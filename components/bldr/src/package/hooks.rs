// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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
        let mut child = try!(Command::new(&self.path)
                                 .stdin(Stdio::null())
                                 .stdout(Stdio::piped())
                                 .stderr(Stdio::piped())
                                 .spawn());
        {
            let mut c_stdout = match child.stdout {
                Some(ref mut s) => s,
                None => {
                    return Err(bldr_error!(ErrorKind::HookFailed(self.htype.clone(),
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
            Err(bldr_error!(ErrorKind::HookFailed(self.htype.clone(),
                                                  exit_status.code().unwrap_or(-1),
                                                  String::from("Failed"))))
        }
    }

    pub fn compile(&self, context: Option<&ServiceConfig>) -> BldrResult<()> {
        if let Some(ctx) = context {
            let template = try!(mustache::compile_path(&self.template));
            let mut out = Vec::new();
            let toml = try!(ctx.to_toml());
            let data = convert::toml_to_mustache(toml);
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

    pub fn load_hooks(&mut self) -> &mut Self {
        let hook_path = self.package.join_path("hooks");
        let path = Path::new(&hook_path);
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
        match fs::metadata(&template) {
            Ok(_) => Some(Hook::new(hook_type, template, concrete)),
            Err(_) => None,
        }
    }
}
