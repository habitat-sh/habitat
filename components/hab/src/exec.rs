// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate libc;

use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::ptr;

use common;
use hcore;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::url::DEFAULT_DEPOT_URL;

use error::{Error, Result};

const MAX_RETRIES: u8 = 4;

/// Makes an `execv(3)` system call to become a new program.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * Command and/or command arguments cannot be converted into `CString`
pub fn exec_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
    let prog = try!(CString::new(command.into_os_string().into_vec()));
    let mut argv: Vec<*const i8> = Vec::with_capacity(args.len() + 2);
    argv.push(prog.as_ptr());
    for arg in args {
        argv.push(try!(CString::new(arg.into_vec())).as_ptr());
    }
    argv.push(ptr::null());

    // Calls `execv(3)` so this will not return, but rather become the program with the given
    // arguments.
    unsafe {
        libc::execv(prog.as_ptr(), argv.as_mut_ptr());
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
pub fn command_from_pkg(command: &str,
                        ident: &PackageIdent,
                        cache_key_path: &Path,
                        retry: u8)
                        -> Result<PathBuf> {
    if retry > MAX_RETRIES {
        return Err(Error::ExecCommandNotFound(command.to_string()));
    }

    match PackageInstall::load(ident, None) {
        Ok(pi) => {
            match try!(find_command_in_pkg(&command, &pi)) {
                Some(cmd) => Ok(cmd),
                None => return Err(Error::ExecCommandNotFound(command.to_string())),
            }
        }
        Err(hcore::Error::PackageNotFound(_)) => {
            println!("Package for {} not found, installing from depot", &ident);
            try!(common::command::package::install::from_url(DEFAULT_DEPOT_URL,
                                                             ident,
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
fn find_command_in_pkg(command: &str, pkg_install: &PackageInstall) -> Result<Option<PathBuf>> {
    for path in try!(pkg_install.paths()) {
        let candidate = path.join(command);
        if candidate.is_file() {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}
