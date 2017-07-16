// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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
extern crate habitat_builder_scheduler as scheduler;
extern crate habitat_core as hab_core;
extern crate builder_core as bldr_core;
#[macro_use]
extern crate log;

use std::process;

use hab_core::config::ConfigFile;
use scheduler::{Config, Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/builder-scheduler/config.toml";

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
    clap_app!(BuilderScheduler =>
        (version: VERSION)
        (about: "Habitat builder-scheduler")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run a Habitat Builder scheduler service")
            (@arg config: -c --config +takes_value
                "Filepath to configuration file. \
                [default: /hab/svc/builder-scheduler/config.toml]")
        )
        (@subcommand migrate =>
            (about: "Migrate data from package archives on-disk into the DB")
            (@arg config: -c --config +takes_value
                "Filepath to configuration file. \
                [default: /hab/svc/builder-scheduler/config.toml]")
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let config = match args.value_of("config") {
        Some(cfg_path) => Config::from_file(cfg_path)?,
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("start") => start(config),
        Some("migrate") => migrate(config),
        Some(cmd) => {
            debug!("Dispatch failed, no match for command: {:?}", cmd);
            Ok(())
        }
        None => Ok(()),
    }
}

fn exit_with(err: Error, code: i32) {
    println!("{}", err);
    process::exit(code)
}

// Starts the builder-scheduler server.
fn start(config: Config) -> Result<()> {
    scheduler::server::run(config)
}

// Migrates package information from on-disk package archives into the Postgres DB.
pub fn migrate(config: Config) -> Result<()> {
    scheduler::migration::run(config)
}
