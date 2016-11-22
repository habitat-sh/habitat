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
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use handlebars::Handlebars;

use error::{Error, Result};
use hcore::util;
use hcore::os::users;
use package::Package;
use manager::service::config::{ServiceConfig, never_escape_fn};
use util::convert;
use util::handlebars_helpers;
use util::users as hab_users;

pub const HOOK_PERMISSIONS: u32 = 0o755;
static LOGKEY: &'static str = "PH";

#[derive(Debug, Clone)]
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

    pub fn run(&self) -> Result<String> {
        let mut cmd = Command::new(&self.path);
        try!(self.run_platform(&mut cmd));
        let mut child = try!(cmd.spawn());
        {
            let mut c_stdout = match child.stdout {
                Some(ref mut s) => s,
                None => {
                    return Err(sup_error!(Error::HookFailed(self.htype.clone(),
                                                            -1,
                                                            String::from("Failed"))));
                }
            };
            let preamble_str = format!("{}", &self.htype);
            let mut line = output_format!(preamble & preamble_str, "");
            loop {
                let mut buf = [0u8; 1]; // Our byte buffer
                let len = try!(c_stdout.read(&mut buf));
                match len {
                    0 => {
                        // 0 == EOF, so stop writing and finish progress
                        break;
                    }
                    _ => {
                        // Write the buffer to the BufWriter on the Heap
                        let buf_string = String::from_utf8_lossy(&buf[0..len]);
                        line.push_str(&buf_string);
                        if line.contains("\n") {
                            print!("{}", line);
                            line = output_format!(preamble & preamble_str, "");
                        }
                    }
                }
            }
        }
        let exit_status = try!(child.wait());
        if exit_status.success() {
            Ok(String::from("Finished"))
        } else {
            Err(sup_error!(Error::HookFailed(self.htype.clone(),
                                             exit_status.code().unwrap_or(-1),
                                             String::from("Failed"))))
        }
    }

    #[cfg(any(target_os="linux", target_os="macos"))]
    fn run_platform(&self, cmd: &mut Command) -> Result<()> {
        use std::os::unix::process::CommandExt;
        let uid = users::get_uid_by_name(&self.user);
        let gid = users::get_gid_by_name(&self.group);
        if let None = uid {
            panic!("Can't determine uid");
        }

        if let None = gid {
            panic!("Can't determine gid");
        }

        let uid = uid.unwrap();
        let gid = gid.unwrap();
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .uid(uid)
            .gid(gid);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn run_platform(&self, cmd: &mut Command) -> Result<()> {
        unimplemented!();
    }

    pub fn compile(&self, context: Option<&ServiceConfig>) -> Result<()> {
        if let Some(ctx) = context {
            debug!("Rendering hook {:?}", self);
            let mut handlebars = Handlebars::new();
            handlebars.register_helper("json", Box::new(handlebars_helpers::json_helper));
            handlebars.register_helper("toml", Box::new(handlebars_helpers::toml_helper));
            handlebars.register_escape_fn(never_escape_fn);
            try!(handlebars.register_template_file("hook", &self.template));
            let toml = try!(ctx.to_toml());
            let svc_data = convert::toml_to_json(toml);
            let data = try!(handlebars.render("hook", &svc_data));
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
