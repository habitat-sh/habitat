// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_common as common;
#[macro_use]
extern crate clap;
extern crate libc;
extern crate url;

mod cli;
mod error;

use std::path::PathBuf;
use std::env;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::ptr;
use std::str::FromStr;

use clap::ArgMatches;
use common::package;
use hcore::package::PackageIdent;
use hcore::url::DEFAULT_DEPOT_URL;

use error::{Error, Result};

const SUP_CMD: &'static str = "hab-sup";
const SUP_CMD_ENVVAR: &'static str = "HABITAT_SUP_BINARY";

fn main() {
    if let Err(e) = run_hab() {
        println!("{}", e);
        std::process::exit(1)
    }
}

fn run_hab() -> Result<()> {
    try!(exec_subcommand_if_called());

    let app_matches = cli::get().get_matches();
    match app_matches.subcommand() {
        ("pkg", Some(matches)) => {
            match matches.subcommand() {
                ("install", Some(m)) => try!(sub_pkg_install(m)),
                _ => unreachable!(),
            }
        }
        ("install", Some(m)) => try!(sub_pkg_install(m)),
        _ => unreachable!(),
    };
    Ok(())
}

fn exec_subcommand_if_called() -> Result<()> {
    if let Some(subcmd) = env::args().nth(1) {
        match subcmd.as_str() {
            "sup" | "start" => {
                let skip_n = if subcmd == "sup" {
                    2
                } else {
                    1
                };
                let command = match env::var(SUP_CMD_ENVVAR) {
                    Ok(value) => value,
                    Err(_) => SUP_CMD.to_string(),
                };

                if let Some(cmd) = find_command(&command) {
                    try!(exec_command(cmd, env::args_os().skip(skip_n).collect()));
                } else {
                    return Err(Error::ExecCommandNotFound(command));
                }
            }
            _ => return Ok(()),
        }
    };
    Ok(())
}

fn find_command(command: &str) -> Option<PathBuf> {
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

fn exec_command(command: PathBuf, args: Vec<OsString>) -> Result<()> {
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

fn sub_pkg_install(m: &ArgMatches) -> Result<()> {
    let url = m.value_of("REPO_URL").unwrap_or(DEFAULT_DEPOT_URL);
    let ident_or_archive = m.value_of("PKG_IDENT").unwrap();

    if Path::new(ident_or_archive).is_file() {
        try!(package::from_archive(url, &ident_or_archive));
    } else {
        let ident = try!(PackageIdent::from_str(ident_or_archive));
        try!(package::from_url(url, &ident));
    }
    Ok(())
}
