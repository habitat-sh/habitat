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

extern crate habitat_common as common;
extern crate habitat_core as hcore;
#[macro_use]
extern crate habitat_sup as sup;
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

use sup::VERSION;
use sup::config::{GossipListenAddr, GOSSIP_DEFAULT_PORT};
use sup::error::{Error, Result};
use sup::feat;
use sup::command;
use sup::http_gateway;
use sup::manager::{Manager, ManagerConfig};
use sup::manager::service::{DesiredState, ServiceBind, Topology, UpdateStrategy};
use sup::manager::service::{ServiceSpec, StartStyle};

/// Our output key
static LOGKEY: &'static str = "MN";

static RING_ENVVAR: &'static str = "HAB_RING";
static RING_KEY_ENVVAR: &'static str = "HAB_RING_KEY";

fn main() {
    env_logger::init().unwrap();
    enable_features_from_env();
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
        ("config", Some(m)) => sub_config(m),
        ("load", Some(m)) => sub_load(m),
        ("run", Some(m)) => sub_run(m),
        ("sh", Some(m)) => sub_sh(m),
        ("start", Some(m)) => sub_start(m),
        ("stop", Some(m)) => sub_stop(m),
        ("unload", Some(m)) => sub_unload(m),
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
        (@subcommand config =>
            (about: "Displays the default configuration options for a service")
            (aliases: &["c", "co", "con", "conf", "confi"])
            (@arg PKG_IDENT: +required +takes_value
                "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
        )
        (@subcommand load =>
            (about: "Load a service to be started and supervised by Habitat from a package or \
                artifact. Services started in this manner will persist through Supervisor \
                restarts.")
            (aliases: &["lo", "loa"])
            (@arg PKG_IDENT: +required +takes_value "A Habitat package identifier (ex: core/redis)")
            (@arg NAME: --("override-name") +takes_value
                "The name for the state directory if there is more than one Supervisor running \
                [default: default]")
            (@arg GROUP: --group +takes_value
                "The service group; shared config and topology [default: default].")
            (@arg DEPOT_URL: --url -u +takes_value {valid_url}
                "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
            (@arg TOPOLOGY: --topology -t +takes_value {valid_topology}
                "Service topology; [default: none]")
            (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
                "The update strategy; [default: none] [values: none, at-once, rolling]")
            (@arg BIND: --bind +takes_value +multiple
                "One or more service groups to bind to a configuration")
            (@arg FORCE: --force -f "Load or reload an already loaded service. If the service was \
                previously loaded and running this operation will also restart the service")
        )
        (@subcommand unload =>
            (about: "Unload a persistent or transient service started by the Habitat \
                supervisor. If the Supervisor is running when the service is unloaded the \
                service will be stopped.")
            (aliases: &["un", "unl", "unlo", "unloa"])
            (@arg PKG_IDENT: +required +takes_value "A Habitat package identifier (ex: core/redis)")
            (@arg NAME: --("override-name") +takes_value
                "The name for the state directory if there is more than one Supervisor running \
                [default: default]")
        )
        (@subcommand run =>
            (about: "Start the Habitat Supervisor")
            (aliases: &["r", "ru"])
            (@arg LISTEN_GOSSIP: --("listen-gossip") +takes_value
                "The listen address for the gossip system [default: 0.0.0.0:9638]")
            (@arg LISTEN_HTTP: --("listen-http") +takes_value
                "The listen address for the HTTP gateway [default: 0.0.0.0:9631]")
            (@arg NAME: --("override-name") +takes_value
                "The name for the state directory if launching more than one Supervisor \
                [default: default]")
            (@arg ORGANIZATION: --org +takes_value
                "The organization that the supervisor and it's subsequent services are part of \
                [default: default]")
            (@arg PEER: --peer +takes_value +multiple
                "The listen address of an initial peer (IP[:PORT])")
            (@arg PERMANENT_PEER: --("permanent-peer") -I "If this Supervisor is a permanent peer")
            (@arg RING: --ring -r +takes_value "Ring key name")
        )
        (@subcommand sh =>
            (about: "Start an interactive Bourne-like shell")
            (aliases: &[])
        )
        (@subcommand start =>
            (about: "Start a loaded, but stopped, Habitat service or a transient service from \
                a package or artifact. If the Habitat Supervisor is not already running this \
                will additionally start one for you.")
            (aliases: &["sta", "star"])
            (@arg LISTEN_GOSSIP: --("listen-gossip") +takes_value
                "The listen address for the gossip system [default: 0.0.0.0:9638]")
            (@arg LISTEN_HTTP: --("listen-http") +takes_value
                "The listen address for the HTTP gateway [default: 0.0.0.0:9631]")
            (@arg NAME: --("override-name") +takes_value
                "The name for the state directory if launching more than one Supervisor \
                [default: default]")
            (@arg ORGANIZATION: --org +takes_value
                "The organization that the supervisor and it's subsequent services are part of \
                [default: default]")
            (@arg PEER: --peer +takes_value +multiple
                "The listen address of an initial peer (IP[:PORT])")
            (@arg PERMANENT_PEER: --("permanent-peer") -I "If this Supervisor is a permanent peer")
            (@arg RING: --ring -r +takes_value "Ring key name")
            (@arg PKG_IDENT_OR_ARTIFACT: +required +takes_value
                "A Habitat package identifier (ex: core/redis) or filepath to a Habitat Artifact \
                (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            (@arg GROUP: --group +takes_value
                "The service group; shared config and topology [default: default].")
            (@arg DEPOT_URL: --url -u +takes_value {valid_url}
                "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
            (@arg TOPOLOGY: --topology -t +takes_value {valid_topology}
                "Service topology; [default: none]")
            (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
                "The update strategy; [default: none] [values: none, at-once, rolling]")
            (@arg BIND: --bind +takes_value +multiple
                "One or more service groups to bind to a configuration")
            (@arg CONFIG_DIR: --("config-from") +takes_value {dir_exists}
                "Use package config from this path, rather than the package itself")
        )
        (@subcommand stop =>
            (about: "Stop a running Habitat service.")
            (aliases: &["sto"])
            (@arg PKG_IDENT: +required +takes_value "A Habitat package identifier (ex: core/redis)")
            (@arg NAME: --("override-name") +takes_value
                "The name for the state directory if there is more than one Supervisor running \
                [default: default]")
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

    command::shell::bash()
}

fn sub_config(m: &ArgMatches) -> Result<()> {
    let ident = try!(PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap()));

    try!(common::command::package::config::start(&ident, "/"));
    Ok(())
}

fn sub_load(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let default_spec = ServiceSpec::default_for(ident);
    let spec_file = Manager::spec_path_for(&cfg, &default_spec);
    if let Ok(spec) = ServiceSpec::from_file(&spec_file) {
        if !m.is_present("FORCE") {
            return Err(sup_error!(Error::ServiceLoaded(spec.ident)));
        }
    }
    let mut spec = spec_from_matches(default_spec.ident, m)?;
    spec.start_style = StartStyle::Persistent;
    Manager::save_spec_for(&cfg, spec)
}

fn sub_unload(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let spec = spec_from_matches(ident, m)?;
    let spec_file = Manager::spec_path_for(&cfg, &spec);
    std::fs::remove_file(&spec_file).map_err(|err| {
                                                 sup_error!(Error::ServiceSpecFileIO(spec_file,
                                                                                     err))
                                             })
}

fn sub_run(m: &ArgMatches) -> Result<()> {
    let cfg = mgrcfg_from_matches(m)?;
    let mut manager = Manager::load(cfg)?;
    manager.run()
}

fn sub_sh(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }

    command::shell::sh()
}

fn sub_start(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;
    let mut maybe_local_artifact: Option<&str> = None;
    let maybe_spec = match m.value_of("PKG_IDENT_OR_ARTIFACT") {
        Some(ident_or_artifact) => {
            let ident = if Path::new(ident_or_artifact).is_file() {
                maybe_local_artifact = Some(ident_or_artifact);
                PackageArchive::new(Path::new(ident_or_artifact))
                    .ident()?
            } else {
                PackageIdent::from_str(ident_or_artifact)?
            };
            let default_spec = ServiceSpec::default_for(ident);
            let spec_file = Manager::spec_path_for(&cfg, &default_spec);
            match ServiceSpec::from_file(&spec_file) {
                Ok(mut spec) => {
                    if spec.desired_state == DesiredState::Down {
                        spec.desired_state = DesiredState::Up;
                        Some(spec)
                    } else {
                        if !Manager::is_running(&cfg)? {
                            let mut manager = Manager::load(cfg)?;
                            return manager.run();
                        } else {
                            return Ok(());
                        }
                    }
                }
                Err(_) => Some(spec_from_matches(default_spec.ident, m)?),
            }
        }
        None => None,
    };
    try!(command::start::run(cfg, maybe_spec, maybe_local_artifact));
    Ok(())
}

fn sub_stop(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        sup::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        sup::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let spec_file = Manager::spec_path_for(&cfg, &ServiceSpec::default_for(ident));
    let mut spec = ServiceSpec::from_file(&spec_file)?;
    match spec.start_style {
        StartStyle::Transient => {
            std::fs::remove_file(&spec_file).map_err(|err| {
                sup_error!(Error::ServiceSpecFileIO(spec_file, err))
            })
        }
        StartStyle::Persistent => {
            spec.desired_state = DesiredState::Down;
            Manager::save_spec_for(&cfg, spec)
        }
    }
}

fn mgrcfg_from_matches(m: &ArgMatches) -> Result<ManagerConfig> {
    let mut cfg = ManagerConfig::default();

    if let Some(addr_str) = m.value_of("LISTEN_GOSSIP") {
        cfg.gossip_listen = try!(GossipListenAddr::from_str(addr_str));
    }
    if let Some(addr_str) = m.value_of("LISTEN_HTTP") {
        cfg.http_listen = try!(http_gateway::ListenAddr::from_str(addr_str));
    }
    if let Some(name_str) = m.value_of("NAME") {
        cfg.name = Some(String::from(name_str));
        outputln!("");
        outputln!("{} Running more than one Habitat Supervisor is not recommended for most",
                  Red.bold().paint("CAUTION:".to_string()));
        outputln!("{} users in most use cases. Using one Supervisor per host for multiple",
                  Red.bold().paint("CAUTION:".to_string()));
        outputln!("{} services in one ring will yield much better performance.",
                  Red.bold().paint("CAUTION:".to_string()));
        outputln!("");
        outputln!("{} If you know what you're doing, carry on!",
                  Red.bold().paint("CAUTION:".to_string()));
        outputln!("");
    }
    cfg.organization = m.value_of("ORGANIZATION").map(|org| org.to_string());
    cfg.gossip_permanent = m.is_present("PERMANENT_PEER");
    // TODO fn: Clean this up--using a for loop doesn't feel good however an iterator was
    // causing a lot of developer/compiler type confusion
    let mut gossip_peers: Vec<SocketAddr> = Vec::new();
    if let Some(peers) = m.values_of("PEER") {
        for peer in peers {
            let peer_addr = if peer.find(':').is_some() {
                peer.to_string()
            } else {
                format!("{}:{}", peer, GOSSIP_DEFAULT_PORT)
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
    Ok(cfg)
}

fn spec_from_matches(ident: PackageIdent, m: &ArgMatches) -> Result<ServiceSpec> {
    let mut spec = ServiceSpec::default_for(ident);
    if let Some(group) = m.value_of("GROUP") {
        spec.group = group.to_string();
    }
    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = m.value_of("DEPOT_URL").unwrap_or(&env_or_default);
    spec.depot_url = String::from(url);
    if let Some(topology) = m.value_of("TOPOLOGY") {
        spec.topology = Topology::from_str(topology)?;
    }
    if let Some(ref strategy) = m.value_of("STRATEGY") {
        spec.update_strategy = UpdateStrategy::from_str(strategy)?;
    }
    if let Some(bind_strs) = m.values_of("BIND") {
        let mut binds = Vec::new();
        for bind_str in bind_strs {
            binds.push(ServiceBind::from_str(bind_str)?);
        }
        spec.binds = binds;
    }
    if let Some(ref config_from) = m.value_of("CONFIG_DIR") {
        spec.config_from = Some(PathBuf::from(config_from));
        outputln!("");
        outputln!("{} Setting '{}' should only be used in development, not production!",
                  Red.bold().paint("WARNING:".to_string()),
                  Yellow
                      .bold()
                      .paint(format!("--config-from {}", config_from)));
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

fn enable_features_from_env() {
    let features = vec![(feat::List, "LIST")];

    for feature in &features {
        match henv::var(format!("HAB_FEAT_{}", feature.1)) {
            Ok(ref val) if ["true", "TRUE"].contains(&val.as_str()) => {
                feat::enable(feature.0);
                outputln!("Enabling feature: {:?}", feature.0);
            }
            _ => {}
        }
    }

    if feat::is_enabled(feat::List) {
        outputln!("Listing feature flags environment variables:");
        for feature in &features {
            outputln!("     * {:?}: HAB_FEAT_{}=true", feature.0, feature.1);
        }
        outputln!("The supervisor will start now, enjoy!");
    }
}
