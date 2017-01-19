// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Child;

use hcore::service::ServiceGroup;
use hcore::util;

use error::{Error, Result};
use manager::service::config::ServiceConfig;
use package::Package;
use templating::Template;
use util::convert;
use util::users as hab_users;
use util as sup_util;

pub const HOOK_PERMISSIONS: u32 = 0o755;
static LOGKEY: &'static str = "PH";

#[derive(Debug, Clone, Copy)]
pub enum HookType {
    HealthCheck,
    Reconfigure,
    FileUpdated,
    Run,
    Init,
}

impl fmt::Display for HookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HookType::Init => write!(f, "init"),
            &HookType::HealthCheck => write!(f, "health_check"),
            &HookType::FileUpdated => write!(f, "file_updated"),
            &HookType::Reconfigure => write!(f, "reconfigure"),
            &HookType::Run => write!(f, "run"),
        }
    }
}

#[derive(Debug)]
pub struct Hook {
    pub htype: HookType,
    pub template: PathBuf,
    pub path: PathBuf,
    pub user: String,
    pub group: String,
}

impl Hook {
    pub fn new(htype: HookType,
               template: PathBuf,
               path: PathBuf,
               user: String,
               group: String)
               -> Self {
        Hook {
            htype: htype,
            template: template,
            path: path,
            user: user,
            group: group,
        }
    }

    pub fn run(&self, service_group: &ServiceGroup) -> Result<()> {
        let mut child = try!(sup_util::create_command(&self.path, &self.user, &self.group).spawn());
        self.stream_output(service_group, &mut child);
        let exit_status = try!(child.wait());
        if exit_status.success() {
            Ok(())
        } else {
            Err(sup_error!(Error::HookFailed(self.htype, exit_status.code().unwrap_or(-1))))
        }
    }

    pub fn compile(&self, context: Option<&ServiceConfig>) -> Result<()> {
        if let Some(ctx) = context {
            debug!("Rendering hook {:?}", self);
            let mut template = Template::new();
            try!(template.register_template_file("hook", &self.template));
            let toml = try!(ctx.to_toml());
            let svc_data = convert::toml_to_json(toml);
            let data = try!(template.render("hook", &svc_data));
            let mut file = try!(File::create(&self.path));
            try!(file.write_all(data.as_bytes()));
            try!(util::perm::set_owner(&self.path, &self.user, &self.group));
            try!(util::perm::set_permissions(&self.path, HOOK_PERMISSIONS));
            Ok(())
        } else {
            try!(fs::copy(&self.template, &self.path));
            try!(util::perm::set_owner(&self.path, &self.user, &self.group));
            try!(util::perm::set_permissions(&self.path, HOOK_PERMISSIONS));
            Ok(())
        }
    }

    fn stream_output(&self, service_group: &ServiceGroup, process: &mut Child) {
        let preamble_str = self.stream_preamble(service_group);
        // JW TODO: we need to stream this to a file to be read back later in case of error. We
        // can't store the entirity of stdout/stderr in memory because it could crash the
        // supervisor, but we do want to save it for later to show *why* a hook failed to run
        if let Some(ref mut stdout) = process.stdout {
            for line in BufReader::new(stdout).lines() {
                if let Some(ref l) = line.ok() {
                    outputln!(preamble preamble_str, l);
                }
            }
        }
        if let Some(ref mut stderr) = process.stderr {
            for line in BufReader::new(stderr).lines() {
                if let Some(ref l) = line.ok() {
                    outputln!(preamble preamble_str, l);
                }
            }
        }
    }

    fn stream_preamble(&self, service_group: &ServiceGroup) -> String {
        format!("{} hook[{}]:", service_group, self.htype)
    }
}

pub struct HookTable<'a> {
    pub package: &'a Package,
    pub init_hook: Option<Hook>,
    pub health_check_hook: Option<Hook>,
    pub reconfigure_hook: Option<Hook>,
    pub file_updated_hook: Option<Hook>,
    pub run_hook: Option<Hook>,
}

impl<'a> HookTable<'a> {
    pub fn new(package: &'a Package) -> Self {
        HookTable {
            package: package,
            init_hook: None,
            health_check_hook: None,
            reconfigure_hook: None,
            file_updated_hook: None,
            run_hook: None,
        }
    }

    pub fn compile_all(&mut self, context: &ServiceConfig) {
        if let Some(ref hook) = self.init_hook {
            hook.compile(Some(context))
                .unwrap_or_else(|e| outputln!("Failed to compile init hook: {}", e));
        }
        if let Some(ref hook) = self.health_check_hook {
            hook.compile(Some(context))
                .unwrap_or_else(|e| outputln!("Failed to compile health check hook: {}", e));
        }
        if let Some(ref hook) = self.reconfigure_hook {
            hook.compile(Some(context))
                .unwrap_or_else(|e| outputln!("Failed to compile reconfigure hook: {}", e));
        }
        if let Some(ref hook) = self.file_updated_hook {
            hook.compile(Some(context))
                .unwrap_or_else(|e| outputln!("Failed to compile file updated hook: {}", e));
        }
    }

    pub fn load_hooks(&mut self) -> &mut Self {
        let path = &self.package.config_from().join("hooks");
        match fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() {
                    self.init_hook = self.load_hook(HookType::Init);
                    self.file_updated_hook = self.load_hook(HookType::FileUpdated);
                    self.reconfigure_hook = self.load_hook(HookType::Reconfigure);
                    self.health_check_hook = self.load_hook(HookType::HealthCheck);
                    self.run_hook = self.load_hook(HookType::Run);
                }
            }
            Err(_) => {}
        }
        self
    }

    fn load_hook(&self, hook_type: HookType) -> Option<Hook> {
        let template = self.package.hook_template_path(&hook_type);
        let concrete = self.package.hook_path(&hook_type);
        let (user, group) = hab_users::get_user_and_group(&self.package.pkg_install)
            .expect("Can't determine user:group");

        match fs::metadata(&template) {
            Ok(_) => Some(Hook::new(hook_type, template, concrete, user, group)),
            Err(_) => None,
        }
    }
}
