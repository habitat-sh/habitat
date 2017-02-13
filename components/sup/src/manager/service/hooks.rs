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

use std;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Child;

use hcore;
use hcore::service::ServiceGroup;

use error::{Error, Result};
use manager::service::ServiceConfig;
use supervisor::RuntimeConfig;
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

#[derive(Debug, Serialize)]
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
        let mut file = try!(std::fs::File::create(&self.path));
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

#[derive(Debug, Default, Serialize)]
pub struct HookTable {
    pub init: Option<Hook>,
    pub health_check: Option<Hook>,
    pub reconfigure: Option<Hook>,
    pub file_updated: Option<Hook>,
    pub run: Option<Hook>,
    pub smoke_test: Option<Hook>,
    cfg_incarnation: u64,
}

impl HookTable {
    /// Compile all loaded hooks from the table into their destination service directory.
    pub fn compile(&mut self, service_group: &ServiceGroup, config: &ServiceConfig) {
        if self.cfg_incarnation != 0 && config.incarnation <= self.cfg_incarnation {
            return;
        }
        self.cfg_incarnation = config.incarnation;
        if let Some(ref hook) = self.file_updated {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.health_check {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.init {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.reconfigure {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.run {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.smoke_test {
            self.compile_one(hook, service_group, config);
        }
    }

    /// Read all available hook templates from the table's package directory into the table.
    pub fn load_hooks<T, U>(mut self, cfg: &RuntimeConfig, hooks: T, templates: U) -> Self
        where T: AsRef<Path>,
              U: AsRef<Path>
    {
        if let Some(meta) = std::fs::metadata(templates.as_ref()).ok() {
            if meta.is_dir() {
                self.init = self.load_hook(HookType::Init, cfg, &hooks, &templates);
                self.file_updated = self.load_hook(HookType::FileUpdated, cfg, &hooks, &templates);
                self.reconfigure = self.load_hook(HookType::Reconfigure, cfg, &hooks, &templates);
                self.health_check = self.load_hook(HookType::HealthCheck, cfg, &hooks, &templates);
                self.run = self.load_hook(HookType::Run, cfg, &hooks, &templates);
                self.smoke_test = self.load_hook(HookType::SmokeTest, cfg, &hooks, &templates);
            }
        }
        self
    }

    /// Run the hook of the given type if the table has a hook of that type loaded and compiled.
    ///
    /// Returns affirmatively if the service does not have the desired hook.
    pub fn try_run(&self, hook: HookType, group: &ServiceGroup) -> Result<()> {
        let hook = match hook {
            HookType::FileUpdated => &self.file_updated,
            HookType::HealthCheck => &self.health_check,
            HookType::Init => &self.init,
            HookType::Reconfigure => &self.reconfigure,
            HookType::Run => &self.run,
            HookType::SmokeTest => &self.smoke_test,
        };
        match *hook {
            Some(ref h) => h.run(group),
            None => Ok(()),
        }
    }

    fn compile_one(&self, hook: &Hook, service_group: &ServiceGroup, config: &ServiceConfig) {
        hook.compile(config).unwrap_or_else(|e| {
            outputln!(preamble service_group,
                "Failed to compile {} hook: {}", hook.htype, e);
        });
    }

    fn load_hook<T, U>(&self,
                       hook_type: HookType,
                       runtime_cfg: &RuntimeConfig,
                       hooks: T,
                       templates: U)
                       -> Option<Hook>
        where T: AsRef<Path>,
              U: AsRef<Path>
    {
        let template = hook_path(&hook_type, templates);
        let concrete = hook_path(&hook_type, hooks);
        match std::fs::metadata(&template) {
            Ok(_) => {
                Some(Hook::new(hook_type,
                               template,
                               concrete,
                               runtime_cfg.svc_user.clone(),
                               runtime_cfg.svc_group.clone()))
            }
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
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

pub fn hook_path<T>(hook_type: &HookType, path: T) -> PathBuf
    where T: AsRef<Path>
{
    match *hook_type {
        HookType::Init => path.as_ref().join(INIT_FILENAME),
        HookType::HealthCheck => path.as_ref().join(HEALTHCHECK_FILENAME),
        HookType::FileUpdated => path.as_ref().join(FILEUPDATED_FILENAME),
        HookType::Reconfigure => path.as_ref().join(RECONFIGURE_FILENAME),
        HookType::Run => path.as_ref().join(RUN_FILENAME),
        HookType::SmokeTest => path.as_ref().join(SMOKETEST_FILENAME),
    }
}
