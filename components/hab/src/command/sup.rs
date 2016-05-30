// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use hcore::crypto::{init, default_cache_key_path};
use hcore::env as henv;
use hcore::fs::find_command;
use hcore::package::PackageIdent;

use error::{Error, Result};
use exec;

const SUP_CMD: &'static str = "hab-sup";
const SUP_CMD_ENVVAR: &'static str = "HAB_SUP_BINARY";
const SUP_PACKAGE_IDENT: &'static str = "core/hab-sup";

pub fn start(args: Vec<OsString>) -> Result<()> {
    let command = match henv::var(SUP_CMD_ENVVAR) {
        Ok(command) => PathBuf::from(command),
        Err(_) => {
            init();
            let ident = try!(PackageIdent::from_str(SUP_PACKAGE_IDENT));
            try!(exec::command_from_pkg(SUP_CMD, &ident, &default_cache_key_path(None), 0))
        }
    };

    if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
        exec::exec_command(cmd, args)
    } else {
        Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()))
    }
}
