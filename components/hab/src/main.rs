// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate url;

mod cli;
mod command;
mod error;
mod exec;

use std::env;

use clap::ArgMatches;
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
                ("install", Some(m)) => try!(sub_package_install(m)),
                _ => unreachable!(),
            }
        }
        ("archive", Some(matches)) => {
            match matches.subcommand() {
                ("upload", Some(m)) => try!(sub_archive_upload(m)),
                _ => unreachable!(),
            }
        }
        ("install", Some(m)) => try!(sub_package_install(m)),
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_archive_upload(m: &ArgMatches) -> Result<()> {
    let url = m.value_of("DEPOT_URL").unwrap_or(DEFAULT_DEPOT_URL);
    let archive_path = m.value_of("ARCHIVE").unwrap();
    try!(command::archive::upload::start(&url, &archive_path));
    Ok(())
}

fn sub_package_install(m: &ArgMatches) -> Result<()> {
    let url = m.value_of("REPO_URL").unwrap_or(DEFAULT_DEPOT_URL);
    let ident_or_archive = m.value_of("PKG_IDENT_OR_ARCHIVE").unwrap();
    try!(common::command::package::install::start(url, ident_or_archive));
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

                if let Some(cmd) = exec::find_command(&command) {
                    try!(exec::exec_command(cmd, env::args_os().skip(skip_n).collect()));
                } else {
                    return Err(Error::ExecCommandNotFound(command));
                }
            }
            _ => return Ok(()),
        }
    };
    Ok(())
}
