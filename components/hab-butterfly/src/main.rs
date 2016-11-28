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

extern crate hab;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate hab_butterfly;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate handlebars;
#[macro_use]
extern crate log;

use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::str::FromStr;
use std::thread;

use clap::ArgMatches;

use common::ui::UI;
use hcore::env as henv;
use hcore::crypto::{init, default_cache_key_path, BoxKeyPair, SymKey};
use hcore::fs::FS_ROOT_PATH;
use hcore::service::ServiceGroup;

use hab_butterfly::{analytics, cli, command};
use hab_butterfly::error::{Error, Result};


/// Makes the --org CLI param optional when this env var is set
const HABITAT_ORG_ENVVAR: &'static str = "HAB_ORG";
/// Makes the --user CLI param optional when this env var is set
const HABITAT_USER_ENVVAR: &'static str = "HAB_USER";

const FS_ROOT_ENVVAR: &'static str = "FS_ROOT";

const HABITAT_BUTTERFLY_PORT: u64 = 9638;

const MAX_FILE_UPLOAD_SIZE_BYTES: u64 = 4096;

fn main() {
    env_logger::init().unwrap();
    let mut ui = UI::default();
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
    let app_matches =
        cli::get().get_matches_from_safe_borrow(&mut args.iter()).unwrap_or_else(|e| {
            analytics::instrument_clap_error(&e);
            e.exit();
        });
    match app_matches.subcommand() {
        ("config", Some(matches)) => {
            match matches.subcommand() {
                ("apply", Some(m)) => try!(sub_config_apply(ui, m)),
                _ => unreachable!(),
            }
        }
        ("file", Some(matches)) => {
            match matches.subcommand() {
                ("upload", Some(m)) => try!(sub_file_upload(ui, m)),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_config_apply(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let peers_str = m.value_of("PEER").unwrap_or("127.0.0.1");
    let mut peers: Vec<String> = peers_str.split(",").map(|p| p.into()).collect();
    for p in peers.iter_mut() {
        if p.find(':').is_none() {
            p.push(':');
            p.push_str(&HABITAT_BUTTERFLY_PORT.to_string());
        }
    }
    let number = value_t!(m, "VERSION_NUMBER", u64).unwrap_or_else(|e| e.exit());
    let file_path = match m.value_of("FILE") {
        Some("-") | None => None,
        Some(p) => Some(Path::new(p)),
    };

    init();
    let cache = default_cache_key_path(fs_root_path);
    let ring_key = match m.value_of("RING") {
        Some(name) => Some(try!(SymKey::get_latest_pair_for(&name, &cache))),
        None => None,
    };

    let mut sg = try!(ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap())); // Required via clap

    let org = org_param_or_env(&m);
    let mut service_pair = None;
    if let Some(org_name) = org {
        sg.organization = Some(org_name.clone());
        service_pair = Some(try!(BoxKeyPair::get_latest_pair_for(&sg.to_string(), &cache)));
    }

    let user = user_param_or_env(&m);
    let mut user_pair = None;
    if let Some(user_name) = user {
        user_pair = Some(try!(BoxKeyPair::get_latest_pair_for(&user_name, &cache)));
    }

    command::config::apply::start(ui,
                                  &sg,
                                  number,
                                  file_path,
                                  &peers,
                                  ring_key.as_ref(),
                                  user_pair.as_ref(),
                                  service_pair.as_ref())
}

fn sub_file_upload(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));

    let peers_str = m.value_of("PEER").unwrap_or("127.0.0.1");
    let mut peers: Vec<String> = peers_str.split(",").map(|p| p.into()).collect();
    for p in peers.iter_mut() {
        if p.find(':').is_none() {
            p.push(':');
            p.push_str(&HABITAT_BUTTERFLY_PORT.to_string());
        }
    }
    let number = value_t!(m, "VERSION_NUMBER", u64).unwrap_or_else(|e| e.exit());
    let file_path = Path::new(m.value_of("FILE").unwrap()); // Required via clap
    match file_path.metadata() {
        Ok(md) => {
            if md.len() > MAX_FILE_UPLOAD_SIZE_BYTES {
                return Err(Error::CryptoCLI(format!("Maximum encrypted file size is {} bytes",
                                                    MAX_FILE_UPLOAD_SIZE_BYTES)));
            }
        }
        Err(e) => {
            return Err(Error::CryptoCLI(format!("Error checking file metadata: {}", e)));

        }
    };

    init();
    let cache = default_cache_key_path(fs_root_path);
    let ring_key = match m.value_of("RING") {
        Some(name) => Some(try!(SymKey::get_latest_pair_for(&name, &cache))),
        None => None,
    };

    let mut sg = try!(ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap())); // Required via clap

    let org = org_param_or_env(&m);
    let mut service_pair = None;
    if let Some(org_name) = org {
        sg.organization = Some(org_name.clone());
        service_pair = Some(try!(BoxKeyPair::get_latest_pair_for(&sg.to_string(), &cache)));
    }

    let user = user_param_or_env(&m);
    let mut user_pair = None;
    if let Some(user_name) = user {
        user_pair = Some(try!(BoxKeyPair::get_latest_pair_for(&user_name, &cache)));
    }

    command::file::upload::start(ui,
                                 &sg,
                                 number,
                                 file_path,
                                 &peers,
                                 ring_key.as_ref(),
                                 user_pair.as_ref(),
                                 service_pair.as_ref())
}

/// Parse the raw program arguments and split off any arguments that will skip clap's parsing.
///
/// **Note** with the current version of clap there is no clean way to ignore arguments after a
/// certain point, especially if those arguments look like further options and flags.
fn raw_parse_args() -> (Vec<OsString>, Vec<OsString>) {
    let mut args = env::args();
    match (args.nth(1).unwrap_or_default().as_str(), args.next().unwrap_or_default().as_str()) {
        ("pkg", "exec") => {
            if args.by_ref().count() > 2 {
                return (env::args_os().take(5).collect(), env::args_os().skip(5).collect());
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        _ => (env::args_os().collect(), Vec::new()),
    }
}

/// Check to see if the user has passed in an ORG param.
/// If not, check the HABITAT_ORG env var. If that's
/// empty too, then error.
fn org_param_or_env(m: &ArgMatches) -> Option<String> {
    match m.value_of("ORG") {
        Some(o) => Some(o.to_string()),
        None => {
            match henv::var(HABITAT_ORG_ENVVAR) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
    }
}

/// Check to see if the user has passed in a USER param.
/// If not, check the HAB_USER env var. If that's
/// empty too, then return an error.
fn user_param_or_env(m: &ArgMatches) -> Option<String> {
    match m.value_of("USER") {
        Some(u) => Some(u.to_string()),
        None => {
            match env::var(HABITAT_USER_ENVVAR) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
    }
}
