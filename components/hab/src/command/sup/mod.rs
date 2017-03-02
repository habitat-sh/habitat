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

    use error::{Error, Result};
    use exec;
    use VERSION;

    const SUP_CMD: &'static str = "hab-sup";
    const SUP_CMD_ENVVAR: &'static str = "HAB_SUP_BINARY";
    const SUP_PACKAGE_IDENT: &'static str = "core/hab-sup";

    const FEAT_STATIC: &'static str = "HAB_FEAT_SUP_STATIC";
    const SUP_STATIC_PACKAGE_IDENT: &'static str = "core/hab-sup-static";

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let sup_ident = match henv::var(FEAT_STATIC) {
            Ok(_) => {
                debug!("Enabling statically compiled Supervisor from {}",
                       SUP_STATIC_PACKAGE_IDENT);
                SUP_STATIC_PACKAGE_IDENT
            }
            Err(_) => SUP_PACKAGE_IDENT,
        };
        let command = match henv::var(SUP_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split("/").collect();
                let ident = try!(PackageIdent::from_str(&format!("{}/{}", sup_ident, version[0])));
                try!(exec::command_from_min_pkg(ui,
                                                SUP_CMD,
                                                &ident,
                                                &default_cache_key_path(None),
                                                0))
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            Ok(try!(process::become_command(cmd, args)))
        } else {
            Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()))
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
        try!(ui.warn("Launching a native Supervisor on this operating system is not yet supported. \
                   Try running this command again on a 64-bit Linux operating system."));
        try!(ui.br());
        Err(Error::SubcommandNotSupported(subcmd))
    }
}
