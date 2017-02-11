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
extern crate habitat_sup as sup;
extern crate habitat_core as hcore;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ansi_term;
extern crate libc;
#[macro_use]
extern crate clap;
extern crate url;

use std::net::{SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;

use ansi_term::Colour::{Red, Yellow};
use clap::{App, ArgMatches};
use hcore::env as henv;
use hcore::crypto::{default_cache_key_path, SymKey};
use hcore::crypto::init as crypto_init;
use hcore::package::{PackageArchive, PackageIdent};
use hcore::url::{DEFAULT_DEPOT_URL, DEPOT_URL_ENVVAR};
use url::Url;

use sup::config::{gcache, Config, GossipListenAddr, HttpGatewayListenAddr};
use sup::error::{Error, Result};
use sup::command;
use sup::http_gateway;
use sup::manager::ManagerConfig;
use sup::manager::service::{ServiceSpec, Topology, UpdateStrategy};

/// Our output key
static LOGKEY: &'static str = "MN";

/// The version number
const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

static RING_ENVVAR: &'static str = "HAB_RING";
static RING_KEY_ENVVAR: &'static str = "HAB_RING_KEY";

fn main() {
    env_logger::init().unwrap();
    if let Err(e) = start() {
        println!("{}", e);
        std::process::exit(1);
    }
}

fn start() -> Result<()> {
    crypto_init();
    let app_matches = cli().get_matches();
    match app_matches.subcommand() {
        ("bash", Some(m)) => sub_bash(m),
        ("sh", Some(m)) => sub_sh(m),
        ("start", Some(m)) => sub_start(m),
        _ => unreachable!(),
    }
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    clap_app!(("hab-sup") =>
        (about: "The Habitat Supervisor")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@arg VERBOSE: -v +global "Verbose output; shows line numbers")
        (@arg NO_COLOR: --("no-color") +global "Turn ANSI color off")
        (@subcommand bash =>
            (about: "Start an interactive Bash-like shell")
            (aliases: &["b", "ba", "bas"])
        )
        (@subcommand sh =>
            (about: "Start an interactive Bourne-like shell")
            (aliases: &[])
        )
        (@subcommand start =>
            (about: "Start a Habitat-supervised service from a package or artifact")
            (aliases: &["st", "sta", "star"])
            (@arg PKG_IDENT_OR_ARTIFACT: +required
                "A Habitat package identifier (ex: acme/redis) or filepath to a Habitat Artifact \
                (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            (@arg BIND: --bind +takes_value +multiple
                "One or more service groups to bind to a configuration")
            (@arg CONFIG_DIR: --("config-from") +takes_value {dir_exists}
                "Use package config from this path, rather than the package itself")
            (@arg DEPOT_URL: --url -u +takes_value {valid_url}
                "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
            (@arg GROUP: --group +takes_value
                "The service group; shared config and topology [default: default].")
            (@arg LISTEN_GOSSIP: --("listen-gossip") +takes_value
                "The listen address for the gossip system [default: 0.0.0.0:9638]")
            (@arg LISTEN_HTTP: --("listen-http") +takes_value
                "The listen address for the HTTP gateway [default: 0.0.0.0:9631]")
            (@arg ORGANIZATION: --org +takes_value "The organization that a service is part of")
            (@arg PEER: --peer +takes_value +multiple
                "The listen address of an initial peer (IP[:PORT])")
            (@arg PERMANENT_PEER: --("permanent-peer") -I "If this service is a permanent peer")
            (@arg RING: --ring -r +takes_value "Ring key name")
            (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
                "The update strategy; [default: none] [values: none, at-once, rolling]")
            (@arg TOPOLOGY: --topology -t +takes_value {valid_topology}
                "Service topology; [default: none]")
        )
    )
}

fn sub_bash(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }
    try!(setup_global_config(m));

    command::shell::bash()
}

fn sub_sh(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }
    try!(setup_global_config(m));

    command::shell::sh()
}

fn sub_start(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }

    let mut cfg = ManagerConfig::default();

    if let Some(addr_str) = m.value_of("LISTEN_GOSSIP") {
        cfg.gossip_listen = try!(GossipListenAddr::from_str(addr_str));
    }
    if let Some(addr_str) = m.value_of("LISTEN_HTTP") {
        cfg.http_listen = try!(http_gateway::ListenAddr::from_str(addr_str));
    }
    if m.is_present("PERMANENT_PEER") {
        cfg.gossip_permanent = true;
    }
    // TODO fn: Clean this up--using a for loop doesn't feel good however an iterator was
    // causing a lot of developer/compiler type confusion
    let mut gossip_peers: Vec<SocketAddr> = Vec::new();
    if let Some(peers) = m.values_of("PEER") {
        for peer in peers {
            let peer_addr = if peer.find(':').is_some() {
                peer.to_string()
            } else {
                format!("{}:{}", peer, 9638)
            };
            let addrs: Vec<SocketAddr> = match peer_addr.to_socket_addrs() {
                Ok(addrs) => addrs.collect(),
                Err(e) => {
                    outputln!("Failed to resolve peer: {}", peer_addr);
                    return Err(sup_error!(Error::NameLookup(e)));
                }
            };
            let addr: SocketAddr = addrs[0];
            gossip_peers.push(addr);
        }
    }
    cfg.gossip_peers = gossip_peers;
    let ring = match m.value_of("RING") {
        Some(val) => Some(try!(SymKey::get_latest_pair_for(&val, &default_cache_key_path(None)))),
        None => {
            match henv::var(RING_KEY_ENVVAR) {
                Ok(val) => {
                    let (key, _) = try!(SymKey::write_file_from_str(&val,
                                                                    &default_cache_key_path(None)));
                    Some(key)
                }
                Err(_) => {
                    match henv::var(RING_ENVVAR) {
                        Ok(val) => {
                            Some(try!(SymKey::get_latest_pair_for(&val,
                                                                  &default_cache_key_path(None))))
                        }
                        Err(_) => None,
                    }
                }
            }
        }
    };
    if let Some(ring) = ring {
        cfg.ring = Some(ring.name_with_rev());
    }

    let mut local_artifact: Option<&str> = None;
    let ident = {
        let ident_or_artifact = m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();
        if Path::new(ident_or_artifact).is_file() {
            local_artifact = Some(ident_or_artifact);
            try!(PackageArchive::new(Path::new(ident_or_artifact)).ident())
        } else {
            try!(PackageIdent::from_str(ident_or_artifact))
        }
    };
    let spec = try!(spec_from_matches(&ident, m));

    try!(setup_global_config(m));

    outputln!("Starting {}", Yellow.bold().paint(ident.to_string()));
    try!(command::start::package(cfg, spec, local_artifact));
    outputln!("Finished with {}", Yellow.bold().paint(ident.to_string()));
    Ok(())
}

// TODO fn: Once the remaining fields of Config are gone, the struct and this function can be
// deleted
fn setup_global_config(m: &ArgMatches) -> Result<()> {
    let mut config = Config::default();
    // TODO fn: remove once ServiceConfig doesn't depend on global config
    if let Some(addr_str) = m.value_of("LISTEN_HTTP") {
        let listener_addr = try!(http_gateway::ListenAddr::from_str(addr_str));
        config.service_config_http_listen.set(listener_addr);
    }
    debug!("Config:\n{:?}", config);
    gcache(config);
    Ok(())
}

fn spec_from_matches(ident: &PackageIdent, m: &ArgMatches) -> Result<ServiceSpec> {
    let mut spec = ServiceSpec::default_for(ident.clone());

    if let Some(group) = m.value_of("GROUP") {
        spec.group = group.to_string();
    }
    if let Some(org) = m.value_of("ORGANIZATION") {
        spec.organization = Some(org.to_string());
    }
    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    spec.depot_url = String::from(url);
    if let Some(topology) = m.value_of("TOPOLOGY") {
        spec.topology = try!(Topology::from_str(topology));
    }
    if let Some(ref strategy) = m.value_of("STRATEGY") {
        spec.update_strategy = try!(UpdateStrategy::from_str(strategy));
    }
    if let Some(binds) = m.values_of("BIND") {
        spec.binds = binds.map(|s| s.to_string()).collect();
    }
    if let Some(ref config_from) = m.value_of("CONFIG_DIR") {
        spec.config_from = Some(PathBuf::from(config_from));
        outputln!("");
        outputln!("{} Setting '{}' should only be used in development, not production!",
                  Red.bold().paint("WARNING:".to_string()),
                  Yellow.bold().paint(format!("--config-from {}", config_from)));
        outputln!("");
    }

    Ok(spec)
}

fn dir_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_dir() {
        Ok(())
    } else {
        Err(format!("Directory: '{}' cannot be found", &val))
    }
}

fn valid_topology(val: String) -> result::Result<(), String> {
    match Topology::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Service topology: '{}' is not valid", &val)),
    }
}

fn valid_update_strategy(val: String) -> result::Result<(), String> {
    match UpdateStrategy::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Update strategy: '{}' is not valid", &val)),
    }
}

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
