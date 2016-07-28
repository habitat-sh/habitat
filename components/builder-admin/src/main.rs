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

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate habitat_builder_admin as admin;
extern crate habitat_core as hab_core;
#[macro_use]
extern crate log;

use std::process;
use std::str::FromStr;

use hab_core::config::ConfigFile;
use admin::{Config, Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/hab-builder-admin/config.toml";

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
        (about: "Habitat builder-admin")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@arg config: -c --config +takes_value +global
            "Filepath to configuration file. [default: /hab/svc/hab-builder-admin/config.toml]")
        (@arg port: --port +takes_value +global "Listen port. [default: 9636]")
        (@subcommand start =>
            (about: "Run the builder-admin server")
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

/// Starts the builder-admin server.
///
/// # Failures
///
/// * Fails if the admin server fails to start - canot bind to the port, etc.
fn start(config: Config) -> Result<()> {
    admin::server::run(config)
}
