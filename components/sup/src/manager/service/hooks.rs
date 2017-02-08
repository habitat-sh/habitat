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
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Child;

use hcore;
use hcore::service::ServiceGroup;

use error::{Error, Result};
use manager::service::config::ServiceConfig;
use package::Package;
use templating::Template;
use util;

pub const HOOK_PERMISSIONS: u32 = 0o755;
pub const INIT_FILENAME: &'static str = "init";
pub const HEALTHCHECK_FILENAME: &'static str = "health_check";
pub const FILEUPDATED_FILENAME: &'static str = "file_updated";
pub const RECONFIGURE_FILENAME: &'static str = "reconfigure";
pub const SMOKETEST_FILENAME: &'static str = "smoke_test";
pub const RUN_FILENAME: &'static str = "run";

static LOGKEY: &'static str = "HK";

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

    /// Run a compiled hook.
    pub fn run(&self, service_group: &ServiceGroup) -> Result<()> {
        let mut child = try!(util::create_command(&self.path, &self.user, &self.group).spawn());
        self.stream_output(service_group, &mut child);
        let exit_status = try!(child.wait());
        if exit_status.success() {
            Ok(())
        } else {
            Err(sup_error!(Error::HookFailed(self.htype, exit_status.code().unwrap_or(-1))))
        }
    }

    /// Compile a hook into it's destination service directory.
    pub fn compile(&self, cfg: &ServiceConfig) -> Result<()> {
        let mut template = Template::new();
        try!(template.register_template_file("hook", &self.template));
        let toml = try!(cfg.to_toml());
        let svc_data = util::convert::toml_to_json(toml);
        let data = try!(template.render("hook", &svc_data));
        let mut file = try!(fs::File::create(&self.path));
        try!(file.write_all(data.as_bytes()));
        try!(hcore::util::perm::set_owner(&self.path, &self.user, &self.group));
        try!(hcore::util::perm::set_permissions(&self.path, HOOK_PERMISSIONS));
        Ok(())
    }

    fn stream_output(&self, service_group: &ServiceGroup, process: &mut Child) {
        let preamble_str = self.stream_preamble(service_group);
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
    pub init: Option<Hook>,
    pub health_check: Option<Hook>,
    pub reconfigure: Option<Hook>,
    pub file_updated: Option<Hook>,
    pub run: Option<Hook>,
    pub smoke_test: Option<Hook>,
    package: &'a Package,
    service_group: &'a ServiceGroup,
}

impl<'a> HookTable<'a> {
    pub fn new(package: &'a Package, service_group: &'a ServiceGroup) -> Self {
        HookTable {
            file_updated: None,
            health_check: None,
            init: None,
            reconfigure: None,
            run: None,
            smoke_test: None,
            package: package,
            service_group: service_group,
        }
    }

    /// Compile all loaded hooks from the table into their destination service directory.
    pub fn compile(&mut self, cfg: &ServiceConfig) {
        if let Some(ref hook) = self.file_updated {
            self.compile_one(hook, cfg);
        }
        if let Some(ref hook) = self.health_check {
            self.compile_one(hook, cfg);
        }
        if let Some(ref hook) = self.init {
            self.compile_one(hook, cfg);
        }
        if let Some(ref hook) = self.reconfigure {
            self.compile_one(hook, cfg);
        }
        if let Some(ref hook) = self.run {
            self.compile_one(hook, cfg);
        }
        if let Some(ref hook) = self.smoke_test {
            self.compile_one(hook, cfg);
        }
    }

    /// Read all available hook templates from the table's package directory into the table.
    pub fn load_hooks(&mut self) -> &mut Self {
        let path = &self.package.config_from().join("hooks");
        match fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() {
                    self.init = self.load_hook(HookType::Init);
                    self.file_updated = self.load_hook(HookType::FileUpdated);
                    self.reconfigure = self.load_hook(HookType::Reconfigure);
                    self.health_check = self.load_hook(HookType::HealthCheck);
                    self.run = self.load_hook(HookType::Run);
                    self.smoke_test = self.load_hook(HookType::SmokeTest);
                }
            }
            Err(_) => {}
        }
        self
    }

    /// Run the hook of the given type if the table has a hook of that type loaded and compiled.
    ///
    /// Returns affirmatively if the service does not have the desired hook.
    pub fn try_run(&self, hook: HookType) -> Result<()> {
        let hook = match hook {
            HookType::FileUpdated => &self.file_updated,
            HookType::HealthCheck => &self.health_check,
            HookType::Init => &self.init,
            HookType::Reconfigure => &self.reconfigure,
            HookType::Run => &self.run,
            HookType::SmokeTest => &self.smoke_test,
        };
        match *hook {
            Some(ref h) => h.run(&self.service_group),
            None => Ok(()),
        }
    }

    fn compile_one(&self, hook: &Hook, cfg: &ServiceConfig) {
        hook.compile(cfg).unwrap_or_else(|e| {
            outputln!(preamble self.service_group, "Failed to compile {} hook: {}", hook.htype, e);
        });
    }

    fn load_hook(&self, hook_type: HookType) -> Option<Hook> {
        let template = hook_template_path(&self.package, &hook_type);
        let concrete = hook_path(&self.package, &hook_type);
        let (user, group) = util::users::get_user_and_group(&self.package.pkg_install)
            .expect("Can't determine user:group");
        match fs::metadata(&template) {
            Ok(_) => Some(Hook::new(hook_type, template, concrete, user, group)),
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HookType {
    HealthCheck,
    Reconfigure,
    FileUpdated,
    Run,
    Init,
    SmokeTest,
}

impl fmt::Display for HookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HookType::Init => write!(f, "init"),
            HookType::HealthCheck => write!(f, "health_check"),
            HookType::FileUpdated => write!(f, "file_updated"),
            HookType::Reconfigure => write!(f, "reconfigure"),
            HookType::Run => write!(f, "run"),
            HookType::SmokeTest => write!(f, "smoke_test"),
        }
    }
}

pub fn hook_path(package: &Package, hook_type: &HookType) -> PathBuf {
    let base = package.pkg_install.svc_hooks_path();
    match *hook_type {
        HookType::Init => base.join(INIT_FILENAME),
        HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
        HookType::FileUpdated => base.join(FILEUPDATED_FILENAME),
        HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
        HookType::Run => base.join(RUN_FILENAME),
        HookType::SmokeTest => base.join(SMOKETEST_FILENAME),
    }
}

pub fn hook_template_path(package: &Package, hook_type: &HookType) -> PathBuf {
    let base = package.config_from().join("hooks");
    match *hook_type {
        HookType::Init => base.join(INIT_FILENAME),
        HookType::HealthCheck => base.join(HEALTHCHECK_FILENAME),
        HookType::FileUpdated => base.join(FILEUPDATED_FILENAME),
        HookType::Reconfigure => base.join(RECONFIGURE_FILENAME),
        HookType::Run => base.join(RUN_FILENAME),
        HookType::SmokeTest => base.join(SMOKETEST_FILENAME),
    }
}
