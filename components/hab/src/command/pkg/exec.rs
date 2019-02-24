// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

use std::{env,
          ffi::OsString,
          path::PathBuf};

use crate::hcore::{fs::{find_command,
                        FS_ROOT_PATH},
                   os::process,
                   package::{PackageIdent,
                             PackageInstall}};

use crate::error::{Error,
                   Result};

pub fn start<T>(ident: &PackageIdent, command: T, args: Vec<OsString>) -> Result<()>
where
    T: Into<PathBuf>,
{
    let command = command.into();
    let pkg_install = PackageInstall::load(&ident, Some(&*FS_ROOT_PATH))?;
    let cmd_env = pkg_install.environment_for_command()?;

    for (key, value) in cmd_env.into_iter() {
        debug!("Setting: {}='{}'", key, value);
        env::set_var(key, value);
    }
    let command = match find_command(&command) {
        Some(path) => path,
        None => return Err(Error::ExecCommandNotFound(command)),
    };
    let mut display_args = command.to_string_lossy().into_owned();
    for arg in &args {
        display_args.push(' ');
        display_args.push_str(arg.to_string_lossy().as_ref());
    }
    debug!("Running: {}", display_args);
    process::become_command(command, args)?;
    Ok(())
}
