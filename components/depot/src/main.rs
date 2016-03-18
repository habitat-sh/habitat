// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate bldr_depot as depot;
extern crate bldr_depot_core as depot_core;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::process;
use std::str::FromStr;

use depot::{server, Config, Error, Result};
use depot::data_store::{self, Cursor, Database, Transaction};
use depot_core::data_object;

const DEFAULT_PATH: &'static str = "/opt/bldr/srvc/bldr-depot/data";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
        (@arg path: -p --path +takes_value "Filepath to service storage for the Depot service")
        (@subcommand start =>
            (about: "Run a bldr package Depot")
            (@arg port: --port +takes_value "Listen port. [default: 9632]")
        )
        (@subcommand repair =>
            (about: "Verify and repair data integrity of the package Depot")
        )
        (@subcommand repo =>
            (@subcommand create =>
                (about: "Create a new repository in the package Depot")
                (@arg repo: <repo> +required "Name of the repository to create")
            )
            (@subcommand list =>
                (about: "List repositories in the package Depot")
            )
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let mut config = Config::new();
    if let Some(port) = args.value_of("port") {
        if let Some(port) = u16::from_str(port).ok() {
            config.port = depot::ListenPort(port);
        } else {
            return Err(Error::BadPort(port.to_string()));
        }
    }
    config.path = args.value_of("path").unwrap_or(DEFAULT_PATH).to_string();
    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("start") => start(&config),
        Some("repair") => repair(&config),
        Some(cmd @ "repo") => {
            let args = matches.subcommand_matches(cmd).unwrap();
            match args.subcommand_name() {
                Some(cmd @ "create") => {
                    let args = args.subcommand_matches(cmd).unwrap();
                    let name = args.value_of("repo").unwrap();
                    repo_create(name, &config)
                }
                Some("list") => repo_list(&config),
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
fn start(config: &Config) -> Result<()> {
    println!("Starting package Depot at {}", &config.path);
    println!("Depot listening on {:?}", config.depot_addr());
    server::run(&config)
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
pub fn repair(config: &Config) -> Result<()> {
    // JW TODO: should pass config to depot, not this path
    let depot = try!(depot::Depot::new(config.path.clone()));
    let report = try!(depot::doctor::repair(&depot));
    println!("Report: {:?}", &report);
    Ok(())
}

/// Create a repository with the given name in the depot.
///
/// # Failures
///
/// * The database cannot be read
/// * A write transaction cannot be acquired.
fn repo_create(name: &str, config: &Config) -> Result<()> {
    // JW TODO: should pass config to depot, not this path
    let depot = try!(depot::Depot::new(config.path.clone()));
    let txn = try!(depot.datastore.views.txn_rw());
    let object = data_object::View::new(name);
    try!(depot.datastore.views.write(&txn, &object));
    Ok(())
}

/// List all repositories in the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A read transaction cannot be acquired.
fn repo_list(config: &Config) -> Result<()> {
    // JW TODO: should pass config to depot, not this path
    let depot = try!(depot::Depot::new(config.path.clone()));
    let mut views: Vec<data_object::View> = vec![];
    let txn = try!(depot.datastore.views.txn_ro());
    let mut cursor = try!(txn.cursor_ro());
    match cursor.first() {
        Err(Error::MdbError(data_store::MdbError::NotFound)) => {
            println!("No repositories. Create one with `bldr-depot repo create`.");
            return Ok(());
        }
        Err(e) => return Err(e),
        Ok((_, value)) => views.push(value),
    }
    loop {
        match cursor.next() {
            Ok((_, value)) => views.push(value),
            Err(_) => break,
        }
    }
    println!("Listing {} repositories", views.len());
    for view in views.iter() {
        println!("     {}", view);
    }
    Ok(())
}

fn exit_with(err: Error, code: i32) {
    println!("{:?}", err);
    process::exit(code)
}
