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

use std::ffi::OsString;

use common::ui::UI;

use error::Result;

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
    inner::start(ui, args)
}

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

    const CMD: &'static str = "hab-butterfly";
    const CMD_ENVVAR: &'static str = "HAB_BUTTERFLY_BINARY";

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let sup_ident = "core/hab-butterfly";
        let command = match henv::var(CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let ident = try!(PackageIdent::from_str(sup_ident));
                try!(exec::command_from_pkg(ui, CMD, &ident, &default_cache_key_path(None), 0))
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            Ok(try!(process::become_command(cmd, args)))
        } else {
            Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()))
        }
    }
}
