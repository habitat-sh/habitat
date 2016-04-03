// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate libc;

use std::env;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;

use error::Result;

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
