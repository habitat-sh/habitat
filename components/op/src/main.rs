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
#[macro_use]
extern crate log;
extern crate op;

use op::config::Config;
use op::error::{Error, Result};
use op::util;

use std::process;

const VERSION: &'static str = "0.1.0";

fn main() {
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
    clap_app!(Op =>
        (version: VERSION)
        (about: "Habitat operational tool")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand hash =>
            (about: "Return the BLAKE2b hash for a file")
            (@arg file: --file +takes_value "File to hash")
        )
        (@subcommand shard =>
            (about: "Return the shard number for an origin")
            (@arg origin: --origin +takes_value "Origin")
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let mut config = Config::default();

    if let Some(origin) = args.value_of("origin") {
        config.origin = Some(origin.to_string());
    }

    if let Some(file) = args.value_of("file") {
        config.file = Some(file.to_string());
    }

    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("hash") => util::hash(config),
        Some("shard") => util::shard(config),
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
