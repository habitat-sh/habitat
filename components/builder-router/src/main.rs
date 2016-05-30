// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate habitat_builder_router as router;
extern crate habitat_core as hab_core;
#[macro_use]
extern crate log;

use std::process;
use std::str::FromStr;

use hab_core::config::ConfigFile;
use router::{Config, Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/hab-builder-router/config.toml";

fn main() {
    env_logger::init().unwrap();
    let matches = app().get_matches();
    debug!("CLI matches: {:?}", matches);
    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return exit_with(e, 1),
    };
    match start(config) {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with(e, 1),
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(BuilderSessionSrv =>
        (version: VERSION)
        (about: "Manage a Habitat-Builder router")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@arg config: -c --config +takes_value +global
            "Filepath to configuration file. [default: /hab/svc/hab-builder-router/config.toml]")
        (@arg port: --port +takes_value +global "Listen port. [default: 5560]")
        (@subcommand start =>
            (about: "Run a Habitat-Builder router")
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
        if u16::from_str(port).map(|p| config.set_port(p)).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }
    Ok(config)
}

fn exit_with(err: Error, code: i32) {
    println!("{}", err);
    process::exit(code)
}

/// Starts the Habitat-Builder router.
///
/// # Failures
///
/// * Cannot bind to the port
fn start(config: Config) -> Result<()> {
    router::server::run(config)
}
