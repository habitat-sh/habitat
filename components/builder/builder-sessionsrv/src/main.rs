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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate diesel;
extern crate diesel_migrations;
extern crate env_logger;
extern crate habitat_core as hab_core;
extern crate habitat_builder_sessionsrv as sessionsrv;
#[macro_use]
extern crate log;

use std::fmt;
use std::process;

use hab_core::config::ConfigFile;
use sessionsrv::{Config, SrvResult};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/builder-sessionsrv/config.toml";

fn main() {
    env_logger::init();
    let matches = app().get_matches();
    debug!("CLI matches: {:?}", matches);
    let (subcmd, config) = match subcmd_and_config_from_args(&matches) {
        Ok((s, c)) => (s, c),
        Err(e) => return exit_with(e, 1),
    };

    match subcmd {
        "migrate" => {
            match sessionsrv::server::migrate(config) {
                Ok(_) => process::exit(0),
                Err(e) => exit_with(e, 1),
            }
        }
        "start" => {
            match sessionsrv::server::run(config) {
                Ok(_) => process::exit(0),
                Err(e) => exit_with(e, 1),
            }
        }
        _ => unreachable!(),
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(BuilderSessionSrv =>
        (version: VERSION)
        (about: "Habitat builder-sessionsrv")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand migrate =>
            (about: "Run database migrations")
            (@arg config: -c --config +takes_value +global
                "Filepath to configuration file. [default: /hab/svc/builder-originsrv/config.toml]")
        )
        (@subcommand start =>
            (about: "Run a Habitat-Builder session server")
            (@arg config: -c --config +takes_value
            "Filepath to configuration file. \
            [default: /hab/svc/builder-sessionsrv/config.toml]")
        )
    )
}

fn subcmd_and_config_from_args<'a>(matches: &'a clap::ArgMatches) -> SrvResult<(&'a str, Config)> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let config = match args.value_of("config") {
        Some(cfg_path) => Config::from_file(cfg_path)?,
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    Ok((cmd, config))
}

fn exit_with<T>(err: T, code: i32)
where
    T: fmt::Display,
{
    println!("{}", err);
    process::exit(code)
}
