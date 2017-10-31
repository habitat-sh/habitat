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

extern crate airlock;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate tempdir;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::result;

use airlock::command;
use airlock::{Error, Result};
use clap::{App, ArgMatches};
use tempdir::TempDir;

const FS_ROOT_ENVVAR: &'static str = "AIRLOCK_FS_ROOT";
const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

fn main() {
    env_logger::init().unwrap();
    if let Err(e) = _main() {
        eprintln!("FATAL: {}", e);
        process::exit(1);
    }
}

fn _main() -> Result<()> {
    let (args, remaining_args) = raw_parse_args();
    debug!("clap cli args: {:?}", &args);
    debug!("remaining cli args: {:?}", &remaining_args);
    let app_matches = cli()
        .get_matches_from_safe(&mut args.iter())
        .unwrap_or_else(|e| e.exit());
    match app_matches.subcommand() {
        ("run", Some(m)) => sub_run(m, remaining_args),
        ("invoke", Some(m)) => sub_invoke(m, remaining_args),
        _ => unreachable!(),
    }
}

fn sub_invoke(m: &ArgMatches, cmd_args: Vec<OsString>) -> Result<()> {
    let rootfs = Path::new(m.value_of("FS_ROOT").unwrap());
    let cmd = m.value_of("CMD").unwrap();
    command::invoke::run(rootfs, cmd, cmd_args)
}

fn sub_run(m: &ArgMatches, cmd_args: Vec<OsString>) -> Result<()> {
    let cmd = m.value_of("CMD").unwrap();

    match env::var(FS_ROOT_ENVVAR) {
        Ok(ref val) => {
            let rootfs = Path::new(val);
            if rootfs.exists() {
                return Err(Error::Rootfs(val.to_string()));
            }
            fs::create_dir(&rootfs)?;
            command::run::run(cmd, cmd_args, rootfs)
        }
        Err(_) => {
            let tmpdir = TempDir::new("rootfs")?;
            command::run::run(cmd, cmd_args, tmpdir.path())
        }
    }
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let program_name = {
        let arg0 = env::args().next().map(|p| PathBuf::from(p));
        arg0.as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|p| p.to_str())
            .unwrap()
            .to_string()
    };
    clap_app!((program_name) =>
        (about: "Airlock: your gateway to a Studio")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand run =>
            (about: "stuff")
            (@arg CMD: +required +takes_value
                "The command to execute (ex: ls)")
            (@arg ARGS: +takes_value +multiple
                "Arguments to the command (ex: -l /tmp)")
        )
        (@subcommand invoke =>
            (@setting Hidden)
            (about: "invoke stuff")
            (@arg FS_ROOT: +required +takes_value {dir_exists}
                "Path to the rootfs (ex: /tmp/rootfs)")
            (@arg CMD: +required +takes_value
                "The command to execute (ex: ls)")
            (@arg ARGS: +takes_value +multiple
                "Arguments to the command (ex: -l /tmp)")
        )
    )
}

fn raw_parse_args() -> (Vec<OsString>, Vec<OsString>) {
    let mut args = env::args();
    match args.nth(1).unwrap_or_default().as_str() {
        "run" => {
            if args.by_ref().count() > 1 {
                return (
                    env::args_os().take(3).collect(),
                    env::args_os().skip(3).collect(),
                );
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        "invoke" => {
            if args.by_ref().count() > 2 {
                return (
                    env::args_os().take(4).collect(),
                    env::args_os().skip(4).collect(),
                );
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        _ => (env::args_os().collect(), Vec::new()),
    }
}

fn dir_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_dir() {
        Ok(())
    } else {
        Err(format!("Directory: '{}' cannot be found", &val))
    }
}
