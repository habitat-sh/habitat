// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate habitat_builder_api as api;
#[macro_use]
extern crate log;

use std::process;
use std::str::FromStr;

use api::{Config, Error, Result};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    clap_app!(BuilderApi =>
        (version: VERSION)
        (about: "Manage a Habitat builder-api")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run a Habitat builder-api")
            (@arg port: --port +takes_value "Listen port. [default: 9632]")
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let mut config = Config::new();
    if let Some(port) = args.value_of("port") {
        if u16::from_str(port).map(|p| config.set_port(p)).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }
    Ok(config)
}

fn exit_with(err: Error, code: i32) {
    println!("{:?}", err);
    process::exit(code)
}

/// Starts the builder-api server.
///
/// # Failures
///
/// * Fails if the depot server fails to start - canot bind to the port, etc.
fn start(config: Config) -> Result<()> {
    api::server::run(config)
}
