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
use std::path::{Path, PathBuf};

use hcore::os::process;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::fs::{find_command, FS_ROOT_PATH};

use error::{Error, Result};

pub fn start<T>(ident: &PackageIdent, command: T, args: Vec<OsString>) -> Result<()>
where
    T: Into<PathBuf>,
{
    let command = command.into();
    let pkg_install = PackageInstall::load(&ident, Some(&*FS_ROOT_PATH))?;
    let mut run_env = pkg_install.runtime_environment()?;

    let mut paths: Vec<PathBuf> = match run_env.get("PATH") {
        Some(path) => env::split_paths(&path).collect(),
        None => vec![],
    };
    for i in 0..paths.len() {
        if paths[i].starts_with("/") {
            paths[i] = Path::new(&*FS_ROOT_PATH).join(paths[i].strip_prefix("/").unwrap());
        }
    }
    let joined = env::join_paths(paths)?;
    run_env.insert(
        String::from("PATH"),
        joined.into_string().expect(
            "Unable to convert OsStr path to string!",
        ),
    );

    for (key, value) in run_env.into_iter() {
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
    Ok(process::become_command(command, args)?)
}
