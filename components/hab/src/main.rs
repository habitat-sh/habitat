// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
#[macro_use]
extern crate clap;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate regex;
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

use error::{Error, Result};
use hcore::service::ServiceGroup;
use hcore::package::PackageIdent;
use hcore::url::{DEFAULT_DEPOT_URL, DEPOT_URL_ENVVAR};

const SUP_CMD: &'static str = "hab-sup";
const SUP_CMD_ENVVAR: &'static str = "HAB_SUP_BINARY";
const SUP_PACKAGE_IDENT: &'static str = "core/hab-sup";

/// you can skip the --origin CLI param if you specify this env var
const HABITAT_ORIGIN_ENVVAR: &'static str = "HAB_ORIGIN";

/// you can skip the org CLI param if you specify this env var
const HABITAT_ORG_ENVVAR: &'static str = "HAB_ORG";

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
        ("artifact", Some(matches)) => {
            match matches.subcommand() {
                ("upload", Some(m)) => try!(sub_artifact_upload(m)),
                ("sign", Some(m)) => try!(sub_artifact_sign(m)),
                ("verify", Some(m)) => try!(sub_artifact_verify(m)),
                ("hash", Some(m)) => try!(sub_artifact_hash(m)),
                _ => unreachable!(),
            }
        }
        ("inject", Some(m)) => try!(sub_rumor_inject(m)),
        ("install", Some(m)) => try!(sub_package_install(m)),
        ("origin", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => try!(sub_origin_key_generate(sc)),
                        _ => unreachable!(),
                    }
                }
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
        ("service", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => try!(sub_service_key_generate(sc)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("user", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => try!(sub_user_key_generate(sc)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    };
    Ok(())
}

fn sub_artifact_hash(m: &ArgMatches) -> Result<()> {
    let source = m.value_of("SOURCE").unwrap();
    try!(command::artifact::crypto::hash(&source));
    Ok(())
}

fn sub_artifact_sign(m: &ArgMatches) -> Result<()> {
    let origin = try!(origin_param_or_env(&m));
    let infile = m.value_of("SOURCE").unwrap();
    let outfile = m.value_of("ARTIFACT").unwrap();
    try!(command::artifact::crypto::sign(&origin, &infile, &outfile));
    Ok(())
}

fn sub_artifact_upload(m: &ArgMatches) -> Result<()> {
    let env_or_default = env::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    let artifact_path = m.value_of("ARTIFACT").unwrap();

    try!(command::artifact::upload::start(&url, &artifact_path));
    Ok(())
}

fn sub_artifact_verify(m: &ArgMatches) -> Result<()> {
    let infile = m.value_of("ARTIFACT").unwrap();
    try!(command::artifact::crypto::verify(&infile));
    Ok(())
}

fn sub_origin_key_generate(m: &ArgMatches) -> Result<()> {
    let origin = try!(origin_param_or_env(&m));
    try!(command::artifact::crypto::generate_origin_key(&origin));
    Ok(())
}

fn sub_package_install(m: &ArgMatches) -> Result<()> {
    let env_or_default = env::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    let ident_or_artifact = m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();

    try!(common::command::package::install::start(url, ident_or_artifact));
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

fn sub_service_key_generate(m: &ArgMatches) -> Result<()> {
    let org = try!(org_param_or_env(&m));
    let service_group = m.value_of("SERVICE_GROUP").unwrap(); // clap required
    try!(command::artifact::crypto::generate_service_key(&org, service_group));
    Ok(())

}

fn sub_user_key_generate(m: &ArgMatches) -> Result<()> {
    let user = m.value_of("USER").unwrap(); // clap required
    try!(command::artifact::crypto::generate_user_key(user));
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

// check to see if the user has passed in an ORIGIN param
// if not, check the HABITAT_ORIGIN env var. If that's
// empty too, then error
fn origin_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORIGIN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match env::var(HABITAT_ORIGIN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => return Err(Error::CryptoCLI("No origin specified".to_string())),
            }
        }
    }
}


// check to see if the user has passed in an ORG param
// if not, check the HABITAT_ORG env var. If that's
// empty too, then error
fn org_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORG") {
        Some(o) => Ok(o.to_string()),
        None => {
            match env::var(HABITAT_ORG_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => return Err(Error::CryptoCLI("No organization specified".to_string())),
            }
        }
    }
}
