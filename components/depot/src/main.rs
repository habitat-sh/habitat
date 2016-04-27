// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_depot as depot;
extern crate habitat_depot_core as depot_core;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::net;
use std::process;
use std::str::FromStr;

use depot::{server, Config, Error, Result};
use hcore::config::ConfigFile;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/hab-depot/config/server.cfg.toml";

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
        (@arg path: -p --path +takes_value +global "Filepath to service storage for the Depot service")
        (@arg config: -c --config +takes_value +global "Filepath to configuration file. [default: /hab/svc/hab-depot/config/server.cfg.toml]")
        (@subcommand start =>
            (about: "Run a Habitat package Depot")
            (@arg port: --port +takes_value "Listen port. [default: 9632]")
        )
        (@subcommand repair =>
            (about: "Verify and repair data integrity of the package Depot")
        )
        (@subcommand view =>
            (about: "Creates or lists views in the package Depot")
            (@subcommand create =>
                (about: "Create a new view over the package Depot")
                (@arg view: <view> +required "Name of the view to create")
            )
            (@subcommand list =>
                (about: "List views in the package Depot")
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
    if let Some(path) = args.value_of("path") {
        config.path = path.to_string();
    }
    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("start") => start(config),
        Some("repair") => repair(config),
        Some(cmd @ "view") => {
            let args = matches.subcommand_matches(cmd).unwrap();
            match args.subcommand_name() {
                Some(cmd @ "create") => {
                    let args = args.subcommand_matches(cmd).unwrap();
                    let name = args.value_of("view").unwrap();
                    view_create(name, config)
                }
                Some("list") => view_list(config),
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
    let depot = try!(depot::Depot::new(config));
    let report = try!(depot::doctor::repair(&depot));
    println!("Report: {:?}", &report);
    Ok(())
}

/// Create a view with the given name in the depot.
///
/// # Failures
///
/// * The database cannot be read
/// * A write transaction cannot be acquired.
fn view_create(view: &str, config: Config) -> Result<()> {
    let depot = try!(depot::Depot::new(config));
    try!(depot.datastore.views.write(&view));
    Ok(())
}

/// List all views in the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A read transaction cannot be acquired.
fn view_list(config: Config) -> Result<()> {
    let depot = try!(depot::Depot::new(config));
    let views = try!(depot.datastore.views.all());
    if views.is_empty() {
        println!("No views. Create one with `hab-depot view create`.");
        return Ok(());
    }
    let iter = views.iter();
    println!("Listing {} view(s)", iter.len());
    for view in iter {
        println!("     {}", view);
    }
    Ok(())
}

fn exit_with(err: Error, code: i32) {
    println!("{}", err);
    process::exit(code)
}
