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

pub fn start(
    ui: &mut UI,
    args: Vec<OsString>,
    export_cmd: &str,
    export_cmd_envvar: &str,
    export_pkg_ident: &str,
    export_pkg_ident_envvar: &str,
) -> Result<()> {
    inner::start(
        ui,
        args,
        export_cmd,
        export_cmd_envvar,
        export_pkg_ident,
        export_pkg_ident_envvar,
    )
}

#[cfg(target_os = "linux")]
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

    use error::{Error, Result};
    use exec;
    use VERSION;

    pub fn start(
        ui: &mut UI,
        args: Vec<OsString>,
        export_cmd: &str,
        export_cmd_envvar: &str,
        export_pkg_ident: &str,
        export_pkg_ident_envvar: &str,
    ) -> Result<()> {
        let command = match henv::var(export_cmd_envvar) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let ident = match henv::var(export_pkg_ident_envvar) {
                    Ok(ref ident_str) => PackageIdent::from_str(ident_str)?,
                    Err(_) => {
                        let version: Vec<&str> = VERSION.split("/").collect();
                        PackageIdent::from_str(&format!("{}/{}", export_pkg_ident, version[0]))?
                    }
                };
                let cmd = exec::command_from_min_pkg(
                    ui,
                    export_cmd,
                    &ident,
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

#[cfg(not(target_os = "linux"))]
mod inner {
    use std::ffi::OsString;

    use common::ui::UI;

    use error::{Error, Result};

    pub fn start(
        ui: &mut UI,
        args: Vec<OsString>,
        export_cmd: &str,
        export_cmd_envvar: &str,
        export_pkg_ident: &str,
        export_pkg_ident_envvar: &str,
    ) -> Result<()> {
        let cmd = export_cmd.replace("hab", "").replace("-", " ");
        ui.warn(format!(
            "Running 'hab {}' on this operating system is not yet supported. \
            Try running this command again on 64-bit Linux.",
            &cmd
        ))?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(cmd.to_string()))
    }
}
