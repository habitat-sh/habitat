//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

use mustache;

use error::{BldrResult, ErrorKind};
use package::Package;
use service_config::ServiceConfig;
use util::convert;

static LOGKEY: &'static str = "PH";

#[derive(Debug, Clone)]
pub enum HookType {
    HealthCheck,
    Reconfigure,
    Run,
    Init,
}

impl fmt::Display for HookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HookType::Init => write!(f, "init"),
            &HookType::HealthCheck => write!(f, "health_check"),
            &HookType::Reconfigure => write!(f, "reconfigure"),
            &HookType::Run => write!(f, "run"),
        }
    }
}

pub struct Hook {
    pub htype: HookType,
    pub template: PathBuf,
    pub path: PathBuf,
}

impl Hook {
    pub fn new(htype: HookType, template: PathBuf, path: PathBuf) -> Self {
        Hook {
            htype: htype,
            template: template,
            path: path,
        }
    }

    pub fn run(&self, context: Option<&ServiceConfig>) -> BldrResult<String> {
        try!(self.compile(context));
        match Command::new(&self.path).output() {
            Ok(result) => {
                let output = Self::format_output(&result);
                match result.status.code() {
                    Some(0) => Ok(output),
                    Some(code) =>
                        Err(bldr_error!(ErrorKind::HookFailed(self.htype.clone(), code, output))),
                    None => Err(bldr_error!(ErrorKind::HookFailed(self.htype.clone(), -1, output))),
                }
            }
            Err(_) => {
                let err = format!("couldn't run hook: {}", &self.path.to_string_lossy());
                Err(bldr_error!(ErrorKind::HookFailed(self.htype.clone(), -1, err)))
            }
        }
    }

    pub fn compile(&self, context: Option<&ServiceConfig>) -> BldrResult<()> {
        if let Some(ctx) = context {
            let template = try!(mustache::compile_path(&self.template));
            let mut out = Vec::new();
            let toml = try!(ctx.compile_toml());
            let data = convert::toml_table_to_mustache(toml);
            template.render_data(&mut out, &data);
            let data = try!(String::from_utf8(out));
            let mut file = try!(OpenOptions::new()
                                    .write(true)
                                    .truncate(true)
                                    .create(true)
                                    .read(true)
                                    .mode(0o770)
                                    .open(&self.path));
            try!(write!(&mut file, "{}", data));
            Ok(())
        } else {
            try!(fs::copy(&self.template, &self.path));
            Ok(())
        }
    }

    fn format_output(output: &process::Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("{}\n{}", stdout, stderr)
    }
}

pub struct HookTable<'a> {
    pub package: &'a Package,
    pub init_hook: Option<Hook>,
    pub health_check_hook: Option<Hook>,
    pub reconfigure_hook: Option<Hook>,
    pub run_hook: Option<Hook>,
}

impl<'a> HookTable<'a> {
    pub fn new(package: &'a Package) -> Self {
        HookTable {
            package: package,
            init_hook: None,
            health_check_hook: None,
            reconfigure_hook: None,
            run_hook: None,
        }
    }

    pub fn load_hooks(&mut self) -> &mut Self {
        let hook_path = self.package.join_path("hooks");
        let path = Path::new(&hook_path);
        match fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() {
                    self.init_hook = self.load_hook(HookType::Init);
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
        match fs::metadata(&template) {
            Ok(_) => Some(Hook::new(hook_type, template, concrete)),
            Err(_) => None,
        }
    }
}
