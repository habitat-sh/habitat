// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
extern crate habitat_http_client as http_client;

extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate regex;
extern crate rustc_serialize;
extern crate toml;
extern crate url;
// Temporary depdency for gossip/rumor injection code duplication.
extern crate utp;
extern crate uuid;

mod analytics;
mod cli;
mod command;
mod config;
mod error;
mod exec;
mod gossip;

use std::env;
use std::ffi::OsString;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;
use std::thread;

use ansi_term::Colour::Red;
use clap::ArgMatches;

use error::{Error, Result};
use hcore::env as henv;
use hcore::crypto::{init, default_cache_key_path, BoxKeyPair, SigKeyPair, SymKey};
use hcore::crypto::keys::PairType;
use hcore::fs::{cache_artifact_path, cache_analytics_path, FS_ROOT_PATH};
use hcore::service::ServiceGroup;
use hcore::package::PackageIdent;
use hcore::url::{DEFAULT_DEPOT_URL, DEPOT_URL_ENVVAR};

use gossip::hab_gossip;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// Makes the --auth-token CLI param optional when this env var is set
const HABITAT_AUTH_TOKEN_ENVVAR: &'static str = "HAB_AUTH_TOKEN";
/// Makes the --origin CLI param optional when this env var is set
const HABITAT_ORIGIN_ENVVAR: &'static str = "HAB_ORIGIN";
/// Makes the --org CLI param optional when this env var is set
const HABITAT_ORG_ENVVAR: &'static str = "HAB_ORG";
/// Makes the --user CLI param optional when this env var is set
const HABITAT_USER_ENVVAR: &'static str = "HAB_USER";

const FS_ROOT_ENVVAR: &'static str = "FS_ROOT";

const DEFAULT_BINLINK_DIR: &'static str = "/bin";

const MAX_FILE_UPLOAD_SIZE_BYTES: u64 = 4096;

fn main() {
    env_logger::init().unwrap();
    thread::spawn(|| analytics::instrument_subcommand());
    if let Err(e) = start() {
        println!("{}",
                 Red.bold().paint(format!("✗✗✗\n✗✗✗ {}\n✗✗✗", e)));
        std::process::exit(1)
    }
}

fn start() -> Result<()> {
    try!(exec_subcommand_if_called());

    let (args, remaining_args) = raw_parse_args();
    debug!("clap cli args: {:?}", &args);
    debug!("remaining cli args: {:?}", &remaining_args);
    let app_matches =
        cli::get().get_matches_from_safe_borrow(&mut args.iter()).unwrap_or_else(|e| {
            analytics::instrument_clap_error(&e);
            e.exit();
        });
    match app_matches.subcommand() {
        ("apply", Some(m)) => try!(sub_config_apply(m)),
        ("cli", Some(matches)) => {
            match matches.subcommand() {
                ("setup", Some(_)) => try!(sub_cli_setup()),
                _ => unreachable!(),
            }
        }
        ("config", Some(matches)) => {
            match matches.subcommand() {
                ("apply", Some(m)) => try!(sub_config_apply(m)),
                _ => unreachable!(),
            }
        }
        ("file", Some(matches)) => {
            match matches.subcommand() {
                ("upload", Some(m)) => try!(sub_file_upload(m)),
                _ => unreachable!(),
            }
        }
        ("install", Some(m)) => try!(sub_pkg_install(m)),
        ("origin", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("download", Some(sc)) => try!(sub_origin_key_download(sc)),
                        ("export", Some(sc)) => try!(sub_origin_key_export(sc)),
                        ("generate", Some(sc)) => try!(sub_origin_key_generate(sc)),
                        ("import", Some(_)) => try!(sub_origin_key_import()),
                        ("upload", Some(sc)) => try!(sub_origin_key_upload(sc)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("pkg", Some(matches)) => {
            match matches.subcommand() {
                ("binlink", Some(m)) => try!(sub_pkg_binlink(m)),
                ("build", Some(m)) => try!(sub_pkg_build(m)),
                ("exec", Some(m)) => try!(sub_pkg_exec(m, remaining_args)),
                ("export", Some(m)) => try!(sub_pkg_export(m)),
                ("hash", Some(m)) => try!(sub_pkg_hash(m)),
                ("install", Some(m)) => try!(sub_pkg_install(m)),
                ("path", Some(m)) => try!(sub_pkg_path(m)),
                ("sign", Some(m)) => try!(sub_pkg_sign(m)),
                ("upload", Some(m)) => try!(sub_pkg_upload(m)),
                ("verify", Some(m)) => try!(sub_pkg_verify(m)),
                _ => unreachable!(),
            }
        }
        ("ring", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("export", Some(sc)) => try!(sub_ring_key_export(sc)),
                        ("import", Some(_)) => try!(sub_ring_key_import()),
                        ("generate", Some(sc)) => try!(sub_ring_key_generate(sc)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("service", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => try!(sub_service_key_generate(sc)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("user", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => try!(sub_user_key_generate(sc)),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_cli_setup() -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    init();

    command::cli::setup::start(&default_cache_key_path(fs_root_path),
                               &cache_analytics_path(fs_root_path))
}

fn sub_config_apply(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let peers_str = m.value_of("PEER").unwrap_or("127.0.0.1");
    let mut peers: Vec<String> = peers_str.split(",").map(|p| p.into()).collect();
    for p in peers.iter_mut() {
        if p.find(':').is_none() {
            p.push(':');
            p.push_str(&hab_gossip::GOSSIP_DEFAULT_PORT.to_string());
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
    let sg = try!(ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap()));

    command::config::apply::start(&peers, ring_key.as_ref(), &sg, number, file_path)
}

fn sub_file_upload(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let peers_str = m.value_of("PEER").unwrap_or("127.0.0.1");
    let mut peers: Vec<String> = peers_str.split(",").map(|p| p.into()).collect();
    for p in peers.iter_mut() {
        if p.find(':').is_none() {
            p.push(':');
            p.push_str(&hab_gossip::GOSSIP_DEFAULT_PORT.to_string());
        }
    }
    let number = value_t!(m, "VERSION_NUMBER", u64).unwrap_or_else(|e| e.exit());
    let file_path = Path::new(m.value_of("FILE").unwrap());
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

    let mut sg = try!(ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap()));
    // apply the organization name to the service group, either
    // from HAB_ORG or the --org param
    let org = try!(org_param_or_env(&m));
    sg.organization = Some(org.to_string());
    let service_pair = try!(BoxKeyPair::get_latest_pair_for(&sg.to_string(), &cache));

    let user = try!(user_param_or_env(&m));
    let user_pair = try!(BoxKeyPair::get_latest_pair_for(&user, &cache));

    command::file::upload::start(&peers,
                                 ring_key.as_ref(),
                                 &user_pair,
                                 &service_pair,
                                 number,
                                 file_path)
}

fn sub_origin_key_download(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let origin = m.value_of("ORIGIN").unwrap();
    let revision = m.value_of("REVISION");
    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);

    command::origin::key::download::start(&url,
                                          &origin,
                                          revision,
                                          &default_cache_key_path(fs_root_path))
}

fn sub_origin_key_export(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let origin = m.value_of("ORIGIN").unwrap();
    let pair_type = try!(PairType::from_str(m.value_of("PAIR_TYPE").unwrap()));
    init();

    command::origin::key::export::start(origin, pair_type, &default_cache_key_path(fs_root_path))
}

fn sub_origin_key_generate(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let origin = try!(origin_param_or_env(&m));
    init();

    command::origin::key::generate::start(&origin, &default_cache_key_path(fs_root_path))
}

fn sub_origin_key_import() -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let mut content = String::new();
    try!(io::stdin().read_to_string(&mut content));
    init();

    command::origin::key::import::start(&content, &default_cache_key_path(fs_root_path))
}

fn sub_origin_key_upload(m: &ArgMatches) -> Result<()> {

    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));

    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    let token = try!(auth_token_param_or_env(&m));

    init();

    if m.is_present("ORIGIN") {
        let origin = m.value_of("ORIGIN").unwrap();
        // you can either specify files, or infer the latest key names
        let with_secret = m.is_present("WITH_SECRET");
        command::origin::key::upload_latest::start(url,
                                                   &token,
                                                   origin,
                                                   with_secret,
                                                   &default_cache_key_path(fs_root_path))
    } else {
        let keyfile = Path::new(m.value_of("PUBLIC_FILE").unwrap());
        let secret_keyfile = m.value_of("SECRET_FILE").map(|f| Path::new(f));
        command::origin::key::upload::start(url, &token, &keyfile, secret_keyfile)
    }
}

fn sub_pkg_binlink(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Path::new(&fs_root);
    let ident = try!(PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap()));
    let binary = m.value_of("BINARY").unwrap();
    let dest_dir = Path::new(m.value_of("DEST_DIR").unwrap_or(DEFAULT_BINLINK_DIR));

    command::pkg::binlink::start(&ident, &binary, &dest_dir, &fs_root_path)
}

fn sub_pkg_build(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));

    let plan_context = m.value_of("PLAN_CONTEXT").unwrap();
    let root = m.value_of("HAB_STUDIO_ROOT");
    let src = m.value_of("SRC_PATH");
    let keys_string = match m.values_of("HAB_ORIGIN_KEYS") {
        Some(keys) => {
            init();
            for key in keys.clone() {
                // Validate that all secret keys are present
                let pair = try!(SigKeyPair::get_latest_pair_for(key,
                                &default_cache_key_path(fs_root_path)));
                let _ = pair.secret();
            }
            Some(keys.collect::<Vec<_>>().join(","))
        }
        None => None,
    };
    let keys: Option<&str> = match keys_string.as_ref() {
        Some(s) => Some(s),
        None => None,
    };
    let reuse = m.is_present("REUSE");

    command::pkg::build::start(plan_context, root, src, keys, reuse)
}

fn sub_pkg_exec(m: &ArgMatches, cmd_args: Vec<OsString>) -> Result<()> {
    let ident = try!(PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap()));
    let cmd = m.value_of("CMD").unwrap();

    command::pkg::exec::start(&ident, cmd, cmd_args)
}

fn sub_pkg_export(m: &ArgMatches) -> Result<()> {
    let ident = try!(PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap()));
    let format = &m.value_of("FORMAT").unwrap();
    let export_fmt = try!(command::pkg::export::format_for(&format));
    command::pkg::export::start(&ident, &export_fmt)
}

fn sub_pkg_hash(m: &ArgMatches) -> Result<()> {
    let source = m.value_of("SOURCE").unwrap();

    init();
    command::pkg::hash::start(&source)
}

fn sub_pkg_install(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    let ident_or_artifacts = m.values_of("PKG_IDENT_OR_ARTIFACT").unwrap();
    init();

    for ident_or_artifact in ident_or_artifacts {
        try!(common::command::package::install::start(url,
                                                      ident_or_artifact,
                                                      Path::new(&fs_root),
                                                      &cache_artifact_path(fs_root_path),
                                                      &default_cache_key_path(fs_root_path)));
    }
    Ok(())
}

fn sub_pkg_path(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Path::new(&fs_root);
    let ident = try!(PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap()));

    command::pkg::path::start(&ident, &fs_root_path)
}

fn sub_pkg_sign(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let src = Path::new(m.value_of("SOURCE").unwrap());
    let dst = Path::new(m.value_of("DEST").unwrap());
    init();
    let pair = try!(SigKeyPair::get_latest_pair_for(&try!(origin_param_or_env(&m)),
                                                    &default_cache_key_path(fs_root_path)));

    command::pkg::sign::start(&pair, &src, &dst)
}

fn sub_pkg_upload(m: &ArgMatches) -> Result<()> {
    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    let token = try!(auth_token_param_or_env(&m));
    let artifact_paths = m.values_of("HART_FILE").unwrap();
    for artifact_path in artifact_paths {
        try!(command::pkg::upload::start(&url, &token, &artifact_path));
    }
    Ok(())
}

fn sub_pkg_verify(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let src = Path::new(m.value_of("SOURCE").unwrap());
    init();

    command::pkg::verify::start(&src, &default_cache_key_path(fs_root_path))
}

fn sub_ring_key_export(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let ring = m.value_of("RING").unwrap();
    init();

    command::ring::key::export::start(ring, &default_cache_key_path(fs_root_path))
}

fn sub_ring_key_generate(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let ring = m.value_of("RING").unwrap();
    init();

    command::ring::key::generate::start(ring, &default_cache_key_path(fs_root_path))
}

fn sub_ring_key_import() -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let mut content = String::new();
    try!(io::stdin().read_to_string(&mut content));
    init();

    command::ring::key::import::start(&content, &default_cache_key_path(fs_root_path))
}

fn sub_service_key_generate(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let org = try!(org_param_or_env(&m));
    let service_group = try!(ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap()));
    init();

    command::service::key::generate::start(&org,
                                           &service_group,
                                           &default_cache_key_path(fs_root_path))
}

fn sub_user_key_generate(m: &ArgMatches) -> Result<()> {
    let fs_root = henv::var(FS_ROOT_ENVVAR).unwrap_or(FS_ROOT_PATH.to_string());
    let fs_root_path = Some(Path::new(&fs_root));
    let user = m.value_of("USER").unwrap(); // clap required
    init();

    command::user::key::generate::start(user, &default_cache_key_path(fs_root_path))
}

fn exec_subcommand_if_called() -> Result<()> {
    let mut args = env::args();
    match (args.nth(1).unwrap_or_default().as_str(), args.next().unwrap_or_default().as_str()) {
        ("stu", _) | ("stud", _) | ("studi", _) | ("studio", _) => {
            command::studio::start(env::args_os().skip(2).collect())
        }
        ("sup", _) => command::sup::start(env::args_os().skip(2).collect()),
        ("start", _) => command::sup::start(env::args_os().skip(1).collect()),
        _ => Ok(()),
    }
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

/// Check to see if the user has passed in an AUTH_TOKEN param. If not, check the
/// HABITAT_AUTH_TOKEN env var. If not, check the CLI config to see if there is a default auth
/// token set. If that's empty too, then error.
fn auth_token_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("AUTH_TOKEN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(HABITAT_AUTH_TOKEN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = try!(config::load());
                    match config.auth_token {
                        Some(v) => Ok(v),
                        None => return Err(Error::ArgumentError("No auth token specified")),
                    }
                }
            }
        }
    }
}

/// Check to see if the user has passed in an ORIGIN param.  If not, check the HABITAT_ORIGIN env
/// var. If not, check the CLI config to see if there is a default origin set. If that's empty too,
/// then error.
fn origin_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORIGIN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(HABITAT_ORIGIN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = try!(config::load());
                    match config.origin {
                        Some(v) => Ok(v),
                        None => return Err(Error::CryptoCLI("No origin specified".to_string())),
                    }
                }
            }
        }
    }
}

/// Check to see if the user has passed in an ORG param.
/// If not, check the HABITAT_ORG env var. If that's
/// empty too, then error.
fn org_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORG") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(HABITAT_ORG_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => return Err(Error::CryptoCLI("No organization specified".to_string())),
            }
        }
    }
}

/// Check to see if the user has passed in a USER param.
/// If not, check the HAB_USER env var. If that's
/// empty too, then return an error.
fn user_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("USER") {
        Some(u) => Ok(u.to_string()),
        None => {
            match env::var(HABITAT_USER_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => return Err(Error::CryptoCLI("No user specified".to_string())),
            }
        }
    }
}
