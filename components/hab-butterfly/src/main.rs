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

extern crate clap;
extern crate env_logger;
extern crate hab;
extern crate hab_butterfly;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::thread;

use clap::ArgMatches;

use common::ui::{UI, UIWriter};
use hcore::env as henv;
use hcore::crypto::{init, default_cache_key_path, SymKey};

use hab_butterfly::{analytics, cli, command};
use hab_butterfly::error::Result;

const HABITAT_BUTTERFLY_PORT: u64 = 9638;

lazy_static! {
    /// The default filesystem root path to base all commands from. This is lazily generated on
    /// first call and reflects on the presence and value of the environment variable keyed as
    /// `FS_ROOT_ENVVAR`.
    static ref FS_ROOT: PathBuf = {
        use hcore::fs::FS_ROOT_ENVVAR;
        if let Some(root) = henv::var(FS_ROOT_ENVVAR).ok() {
            PathBuf::from(root)
        } else {
            PathBuf::from("/")
        }
    };
}

fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    thread::spawn(|| analytics::instrument_subcommand());
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let (args, remaining_args) = raw_parse_args();
    debug!("clap cli args: {:?}", &args);
    debug!("remaining cli args: {:?}", &remaining_args);
    let app_matches = cli::get()
        .get_matches_from_safe_borrow(&mut args.iter())
        .unwrap_or_else(|e| {
            analytics::instrument_clap_error(&e);
            e.exit();
        });
    match app_matches.subcommand() {
        ("depart", Some(matches)) => {
            try!(sub_depart(ui, matches));
        }
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_depart(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let peers_str = m.value_of("PEER").unwrap_or("127.0.0.1");
    let mut peers: Vec<String> = peers_str.split(",").map(|p| p.into()).collect();
    for p in peers.iter_mut() {
        if p.find(':').is_none() {
            p.push(':');
            p.push_str(&HABITAT_BUTTERFLY_PORT.to_string());
        }
    }
    let member_id = m.value_of("MEMBER_ID").unwrap();

    init();
    let cache = default_cache_key_path(Some(&*FS_ROOT));
    let ring_key = match m.value_of("RING") {
        Some(name) => Some(SymKey::get_latest_pair_for(&name, &cache)?),
        None => None,
    };
    command::depart::run(ui, member_id, peers, ring_key)
}

/// Parse the raw program arguments and split off any arguments that will skip clap's parsing.
///
/// **Note** with the current version of clap there is no clean way to ignore arguments after a
/// certain point, especially if those arguments look like further options and flags.
fn raw_parse_args() -> (Vec<OsString>, Vec<OsString>) {
    let mut args = env::args();
    match (
        args.nth(1).unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
    ) {
        ("pkg", "exec") => {
            if args.by_ref().count() > 2 {
                return (
                    env::args_os().take(5).collect(),
                    env::args_os().skip(5).collect(),
                );
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        _ => (env::args_os().collect(), Vec::new()),
    }
}
