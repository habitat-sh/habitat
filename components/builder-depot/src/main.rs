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

extern crate habitat_core as hab_core;
extern crate habitat_depot as depot;
extern crate habitat_net as hab_net;

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate zmq;

use std::net;
use std::process;
use std::str::FromStr;
use std::sync::Arc;

use hab_core::config::ConfigFile;
use hab_net::server::ServerContext;

use depot::{server, Config, Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/hab-depot/config.toml";

fn main() {
    env_logger::init().unwrap();
    let matches = app().get_matches();
    debug!("CLI matches: {:?}", matches);
    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return exit_with(e, 1),
    };
    match dispatch(config, &matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with(e, 1),
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(BldrDepot =>
        (version: VERSION)
        (about: "Manage a package Depot")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@arg path: -p --path +takes_value +global
            "Filepath to service storage for the Depot service")
        (@arg config: -c --config +takes_value +global
            "Filepath to configuration file. [default: /hab/svc/hab-depot/config.toml]")
        (@subcommand start =>
            (about: "Run a Habitat package Depot")
            (@arg port: --port +takes_value "Listen port. [default: 9632]")
            (@arg insecure: --insecure)
        )
        (@subcommand repair =>
            (about: "Verify and repair data integrity of the package Depot")
        )
        (@subcommand channel =>
            (about: "Creates or lists channels in the package Depot")
            (@subcommand create =>
                (about: "Create a new channel over the package Depot")
                (@arg channel: <channel> +required "Name of the channel to create")
            )
            (@subcommand list =>
                (about: "List channels in the package Depot")
            )
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let mut config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    if let Some(port) = args.value_of("port") {
        if let Some(port) = u16::from_str(port).ok() {
            let addr = net::SocketAddrV4::new(*config.listen_addr.ip(), port);
            config.listen_addr = addr;
        } else {
            return Err(Error::BadPort(port.to_string()));
        }
    }

    if args.is_present("insecure") {
        println!("*** Depot is running in insecure mode ***");
        config.insecure = true
    }

    if let Some(path) = args.value_of("path") {
        config.path = path.to_string();
    }
    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("start") => start(config),
        Some("repair") => repair(config),
        Some(cmd @ "channel") => {
            let args = matches.subcommand_matches(cmd).unwrap();
            match args.subcommand_name() {
                Some(cmd @ "create") => {
                    let args = args.subcommand_matches(cmd).unwrap();
                    let name = args.value_of("channel").unwrap();
                    channel_create(name, config)
                }
                Some("list") => channel_list(config),
                Some(cmd) => {
                    debug!("Dispatch failed, no match for command: {:?}", cmd);
                    Ok(())
                }
                None => Ok(()),
            }
        }
        Some(cmd) => {
            debug!("Dispatch failed, no match for command: {:?}", cmd);
            Ok(())
        }
        None => Ok(()),
    }
}

/// Starts the depot server.
///
/// # Failures
///
/// * Fails if the depot server fails to start - canot bind to the port, etc.
fn start(config: Config) -> Result<()> {
    println!("Starting package Depot at {}", &config.path);
    println!("Depot listening on {}", &config.listen_addr);
    server::run(config)
}

/// Analyzes the integrity of the depot's metadata by comparing the metadata with the packages
/// on disk. If a package is found on disk that is not present in the metadata it is added to the
/// metadata and if an entry in the metadata doesn't have a matching package archive on disk the
/// entry is dropped from the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A write transaction cannot be acquired
pub fn repair(config: Config) -> Result<()> {
    let ctx = Arc::new(Box::new(ServerContext::new()));
    let depot = try!(depot::Depot::new(config, ctx));
    let report = try!(depot::doctor::repair(&depot));
    println!("Report: {:?}", &report);
    Ok(())
}

/// Create a channel with the given name in the depot.
///
/// # Failures
///
/// * The database cannot be read
/// * A write transaction cannot be acquired.
fn channel_create(channel: &str, config: Config) -> Result<()> {
    let ctx = Arc::new(Box::new(ServerContext::new()));
    let depot = try!(depot::Depot::new(config, ctx));
    try!(depot.datastore.channels.write(&channel));
    Ok(())
}

/// List all channels in the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A read transaction cannot be acquired.
fn channel_list(config: Config) -> Result<()> {
    let ctx = Arc::new(Box::new(ServerContext::new()));
    let depot = try!(depot::Depot::new(config, ctx));
    let channels = try!(depot.datastore.channels.all());
    if channels.is_empty() {
        println!("No channels. Create one with `hab-depot channel create`.");
        return Ok(());
    }
    let iter = channels.iter();
    println!("Listing {} channel(s)", iter.len());
    for channel in iter {
        println!("     {}", channel);
    }
    Ok(())
}

fn exit_with(err: Error, code: i32) {
    println!("{}", err);
    process::exit(code)
}
