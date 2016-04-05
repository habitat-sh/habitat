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
#[macro_use]
extern crate log;
// Temporary depdency for gossip/rumor injection code duplication.
extern crate rustc_serialize;
extern crate url;
// Temporary depdency for gossip/rumor injection code duplication.
extern crate utp;
// Temporary depdency for gossip/rumor injection code duplication.
extern crate uuid;

mod cli;
mod command;
mod error;
mod exec;

use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::ArgMatches;
use hcore::service::ServiceGroup;
use hcore::package::PackageIdent;
use hcore::url::DEFAULT_DEPOT_URL;

use error::{Error, Result};

const SUP_CMD: &'static str = "hab-sup";
const SUP_CMD_ENVVAR: &'static str = "HABITAT_SUP_BINARY";
const SUP_PACKAGE_IDENT: &'static str = "chef/hab-sup";

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
        ("archive", Some(matches)) => {
            match matches.subcommand() {
                ("upload", Some(m)) => try!(sub_archive_upload(m)),
                _ => unreachable!(),
            }
        }
        ("pkg", Some(matches)) => {
            match matches.subcommand() {
                ("install", Some(m)) => try!(sub_package_install(m)),
                _ => unreachable!(),
            }
        }
        ("rumor", Some(matches)) => {
            match matches.subcommand() {
                ("inject", Some(m)) => try!(sub_rumor_inject(m)),
                _ => unreachable!(),
            }
        }
        ("inject", Some(m)) => try!(sub_rumor_inject(m)),
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

fn sub_rumor_inject(m: &ArgMatches) -> Result<()> {
    let peers_str = m.value_of("PEERS").unwrap_or("127.0.0.1");
    let mut peers: Vec<String> = peers_str.split(",").map(|p| p.into()).collect();
    for p in peers.iter_mut() {
        if p.find(':').is_none() {
            p.push(':');
            p.push_str(&command::rumor::inject::hab_gossip::GOSSIP_DEFAULT_PORT.to_string());
        }
    }
    let sg = try!(ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap()));
    let number = value_t!(m, "VERSION_NUMBER", u64).unwrap_or_else(|e| e.exit());
    let file_path = match m.value_of("FILE") {
        Some("-") | None => None,
        Some(p) => Some(Path::new(p)),
    };

    try!(command::rumor::inject::start(&peers, sg, number, file_path));
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
                    Ok(command) => PathBuf::from(command),
                    Err(_) => {
                        let ident = try!(PackageIdent::from_str(SUP_PACKAGE_IDENT));
                        try!(exec::command_from_pkg(SUP_CMD, &ident, 0))
                    }
                };

                if let Some(cmd) = exec::find_command(command.to_string_lossy().as_ref()) {
                    try!(exec::exec_command(cmd, env::args_os().skip(skip_n).collect()));
                } else {
                    return Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()));
                }
            }
            _ => return Ok(()),
        }
    };
    Ok(())
}
