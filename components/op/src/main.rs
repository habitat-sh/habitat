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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

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
            (@arg FILE: +required +takes_value "File to hash")
        )
        (@subcommand session =>
            (about: "Decodes a base64 encoded session token and outputs it's contents.")
            (@arg TOKEN: +required +takes_value "Token")
        )
        (@subcommand shard =>
            (about: "Return the shard number for a string or numeric id")
            (@arg SHARD: +required +takes_value "Shard")
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let mut config = Config::default();

    if let Some(s) = args.value_of("SHARD") {
        config.shard = Some(s.to_string());
    }

    if let Some(file) = args.value_of("FILE") {
        config.file = Some(file.to_string());
    }

    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("hash") => util::hash(config),
        Some("shard") => {
            let result = util::shard(config);
            println!("{}", result);
            Ok(())
        }
        Some("session") => {
            let args = matches.subcommand_matches("session").unwrap();
            util::session(args.value_of("TOKEN").unwrap())
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shard_with_string() {
        let mut config = Config::default();
        config.shard = Some("core".to_string());
        let result = util::shard(config);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_shard_with_id() {
        let mut config = Config::default();
        config.shard = Some("721096872374083614".to_string());
        let result = util::shard(config);
        assert_eq!(result, 30);
    }
}
