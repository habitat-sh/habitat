// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::ffi::OsString;

use common::ui::UI;

use error::Result;

const LAUNCH_CMD: &'static str = "hab-launch";
const LAUNCH_CMD_ENVVAR: &'static str = "HAB_LAUNCH_BINARY";
const LAUNCH_PKG_IDENT: &'static str = "core/hab-launcher";

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
    inner::start(ui, args)
}

#[cfg(not(target_os = "macos"))]
mod inner {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::str::FromStr;

    use common::ui::UI;
    use hcore::crypto::{init, default_cache_key_path};
    use hcore::env as henv;
    use hcore::fs::find_command;
    use hcore::os::process;
    use hcore::package::PackageIdent;

    use super::{LAUNCH_CMD, LAUNCH_CMD_ENVVAR, LAUNCH_PKG_IDENT};
    use error::{Error, Result};
    use exec;

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let command = match henv::var(LAUNCH_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let cmd = exec::command_from_min_pkg(
                    ui,
                    LAUNCH_CMD,
                    &PackageIdent::from_str(LAUNCH_PKG_IDENT)?,
                    &default_cache_key_path(None),
                    0,
                )?;
                PathBuf::from(cmd)
            }
        };
        if let Some(cmd) = find_command(&command) {
            Ok(process::become_command(cmd, args)?)
        } else {
            Err(Error::ExecCommandNotFound(command))
        }
    }
}

#[cfg(target_os = "macos")]
mod inner {
    use std::env;
    use std::ffi::OsString;

    use common::ui::UI;

    use error::{Error, Result};

    pub fn start(ui: &mut UI, _args: Vec<OsString>) -> Result<()> {
        let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
        ui.warn(
            "Launching a native Supervisor on this operating system is not yet supported. \
            Try running this command again on 64-bit Linux or Windows.",
        )?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(subcmd))
    }
}
