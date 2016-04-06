// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate libc;

use std::env;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;

use common;
use hcore;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::url::DEFAULT_DEPOT_URL;

use error::{Error, Result};

const MAX_RETRIES: u8 = 4;

/// Returns the absolute path for a given command, if it exists, by searching the `PATH`
/// environment variable.
///
/// If the command represents an absolute path, then the `PATH` seaching will not be performed. If
/// no absolute path can be found for the command, then `None` is returned.
pub fn find_command(command: &str) -> Option<PathBuf> {
    // If the command path is absolute and a file exists, then use that.
    let candidate = PathBuf::from(command);
    if candidate.is_absolute() && candidate.is_file() {
        return Some(candidate);
    }

    // Find the command by checking each entry in `PATH`. If we still can't find it, give up and
    // return `None`.
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
            None
        }
        None => None,
    }
}

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
pub fn command_from_pkg(command: &str, ident: &PackageIdent, retry: u8) -> Result<PathBuf> {
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
            try!(common::command::package::install::from_url(DEFAULT_DEPOT_URL, ident));
            command_from_pkg(&command, &ident, retry + 1)
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
