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

use std::env;
use std::ffi::OsString;

use hcore::os::process;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::fs::find_command;

use error::{Error, Result};

pub fn start(ident: &PackageIdent, command: &str, args: Vec<OsString>) -> Result<()> {
    let pkg_install = PackageInstall::load(&ident, None)?;
    let run_env = pkg_install.runtime_environment()?;
    for (key, value) in run_env.into_iter() {
        info!("Setting: {}='{}'", key, value);
        env::set_var(key, value);
    }
    let command = match find_command(command) {
        Some(path) => path,
        None => return Err(Error::ExecCommandNotFound(command.to_string())),
    };
    let mut display_args = command.to_string_lossy().into_owned();
    for arg in &args {
        display_args.push(' ');
        display_args.push_str(arg.to_string_lossy().as_ref());
    }
    info!("Running: {}", display_args);
    Ok(process::become_command(command, args)?)
}
