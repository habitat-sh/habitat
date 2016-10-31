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

extern crate libc;

use std;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

use ansi_term::Colour::Cyan;
use common;
use hcore;
use hcore::fs::cache_artifact_path;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::url::default_depot_url;

use {PRODUCT, VERSION};
use error::{Error, Result};

#[allow(dead_code)] // Currently only used on Linux platforms
const MAX_RETRIES: u8 = 4;

/// Makes an `execv(3)` system call to become a new program.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * Command and/or command arguments cannot be converted into `CString`
pub fn exec_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    // A massive thanks to the `exec` crate which pointed to the correct invocation
    // behavior--namely to pass null-terminated string pointers.
    //
    // Source: https://github.com/faradayio/exec-rs/blob/master/src/lib.rs

    debug!("Calling execv: ({:?}) {:?}", command.display(), &args);
    let prog_cstring = try!(CString::new(command.as_os_str().as_bytes()));
    let arg_cstrings = try!(args.into_iter()
        .map(|arg| CString::new(arg.as_os_str().as_bytes()))
        .collect::<std::result::Result<Vec<_>, _>>());
    let mut arg_charptrs: Vec<_> = arg_cstrings.iter()
        .map(|arg| arg.as_bytes_with_nul().as_ptr() as *const i8)
        .collect();
    arg_charptrs.insert(0,
                        prog_cstring.clone().as_bytes_with_nul().as_ptr() as *const i8);
    arg_charptrs.push(ptr::null());

    unsafe {
        libc::execv(prog_cstring.as_bytes_with_nul().as_ptr() as *const i8,
                    arg_charptrs.as_mut_ptr());
    }
    Ok(())
}

/// Returns the absolute path to the given command from the given package identifier.
///
/// If the package is not locally installed, the package will be installed before recomputing.
/// There are a maximum number of times a re-installation will be attempted before returning an
/// error.
///
/// # Failures
///
/// * If the package is installed but the command cannot be found in the package
/// * If an error occurs when loading the local package from disk
/// * If the maximum number of installation retries has been exceeded
#[allow(dead_code)] // Currently only used on Linux platforms
pub fn command_from_pkg(command: &str,
                        ident: &PackageIdent,
                        cache_key_path: &Path,
                        retry: u8)
                        -> Result<PathBuf> {
    if retry > MAX_RETRIES {
        return Err(Error::ExecCommandNotFound(command.to_string()));
    }

    let fs_root_path = Path::new("/");
    match PackageInstall::load(ident, None) {
        Ok(pi) => {
            match try!(find_command_in_pkg(&command, &pi, fs_root_path)) {
                Some(cmd) => Ok(cmd),
                None => return Err(Error::ExecCommandNotFound(command.to_string())),
            }
        }
        Err(hcore::Error::PackageNotFound(_)) => {
            println!("{}",
                     Cyan.bold()
                         .paint(format!("∵ Package for {} not found, installing", &ident)));
            try!(common::command::package::install::start(&default_depot_url(),
                                                          &ident.to_string(),
                                                          PRODUCT,
                                                          VERSION,
                                                          fs_root_path,
                                                          &cache_artifact_path(None),
                                                          cache_key_path));
            command_from_pkg(&command, &ident, &cache_key_path, retry + 1)
        }
        Err(e) => return Err(Error::from(e)),
    }
}

/// Returns the absolute path to the given command from a given package installation.
///
/// If the command is not found, then `None` is returned.
///
/// # Failures
///
/// * The path entries metadata cannot be loaded
pub fn find_command_in_pkg(command: &str,
                           pkg_install: &PackageInstall,
                           fs_root_path: &Path)
                           -> Result<Option<PathBuf>> {
    for path in try!(pkg_install.paths()) {
        let candidate = fs_root_path.join(try!(path.strip_prefix("/"))).join(command);
        if candidate.is_file() {
            return Ok(Some(path.join(command)));
        }
    }
    Ok(None)
}
