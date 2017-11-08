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

use std::path::Path;
use std::result;
use std::str::FromStr;

use clap::{App, Arg};
use hcore::service::ServiceGroup;
use url::Url;

use config::GossipListenAddr;
use http_gateway::ListenAddr;
use manager::service::{Topology, UpdateStrategy};

use super::VERSION;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub fn get<'a, 'b>(command_name: &'static str) -> App<'a, 'b> {
    let base =
        clap_app!((command_name) =>
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
        (subcommand: load())
        (subcommand: unload())
        (subcommand: run(&["r", "ru"]))
        (@subcommand sh =>
            (about: "Start an interactive Bourne-like shell")
            (aliases: &[])
        )
        (subcommand: start())
        (subcommand: status())
        (subcommand: stop())
    );

    maybe_add_term_subcommand(base)
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub fn load<'a, 'b>() -> App<'a, 'b> {
    let base = clap_app!(@subcommand load =>
        (about: "Load a service to be started and supervised by Habitat from a package or \
            artifact. Services started in this manner will persist through Supervisor \
            restarts.")
        (aliases: &["lo", "loa"])
        (@arg PKG_IDENT_OR_ARTIFACT: +required +takes_value
            "A Habitat package identifier (ex: core/redis) or filepath to a Habitat Artifact \
            (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart)")
        (@arg NAME: --("override-name") +takes_value
            "The name for the state directory if there is more than one Supervisor running \
            [default: default]")
        (@arg APPLICATION: --application -a +takes_value requires[ENVIRONMENT]
            "Application name; [default: not set].")
        (@arg ENVIRONMENT: --environment -e +takes_value requires[APPLICATION]
            "Environment name; [default: not set].")
        (@arg CHANNEL: --channel +takes_value
            "Receive package updates from the specified release channel [default: stable]")
        (@arg GROUP: --group +takes_value
            "The service group; shared config and topology [default: default].")
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Receive package updates from Builder at the specified URL \
            [default: https://bldr.habitat.sh]")
        (@arg TOPOLOGY: --topology -t +takes_value {valid_topology}
            "Service topology; [default: none]")
        (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
            "The update strategy; [default: none] [values: none, at-once, rolling]")
        (@arg BIND: --bind +takes_value +multiple
            "One or more service groups to bind to a configuration")
        (@arg FORCE: --force -f "Load or reload an already loaded service. If the service was \
            previously loaded and running this operation will also restart the service")
    );

    maybe_add_password_arg(base)
}

pub fn unload<'a, 'b>() -> App<'a, 'b> {
    clap_app!(@subcommand unload =>
        (about: "Unload a persistent or transient service started by the Habitat \
            Supervisor. If the Supervisor is running when the service is unloaded the \
            service will be stopped.")
        (aliases: &["un", "unl", "unlo", "unloa"])
        (@arg PKG_IDENT: +required +takes_value "A Habitat package identifier (ex: core/redis)")
        (@arg NAME: --("override-name") +takes_value
            "The name for the state directory if there is more than one Supervisor running \
            [default: default]")
    )
}

pub fn run<'a, 'b>(aka: &'static [&'static str]) -> App<'a, 'b> {
    clap_app!(@subcommand run =>
        (about: "Run the Habitat Supervisor")
        (aliases: aka)
        // (aliases: &["r", "ru"])
        (@arg LISTEN_GOSSIP: --("listen-gossip") +takes_value {valid_listen_gossip}
            "The listen address for the gossip system [default: 0.0.0.0:9638]")
        (@arg LISTEN_HTTP: --("listen-http") +takes_value {valid_listen_http}
            "The listen address for the HTTP gateway [default: 0.0.0.0:9631]")
        (@arg NAME: --("override-name") +takes_value
            "The name of the Supervisor if launching more than one [default: default]")
        (@arg ORGANIZATION: --org +takes_value
            "The organization that the Supervisor and it's subsequent services are part of \
            [default: default]")
        (@arg PEER: --peer +takes_value +multiple
            "The listen address of an initial peer (IP[:PORT])")
        (@arg PERMANENT_PEER: --("permanent-peer") -I "If this Supervisor is a permanent peer")
        (@arg PEER_WATCH_FILE: --("peer-watch-file") +takes_value conflicts_with[peer]
            "Watch this file for connecting to the ring"
        )
        (@arg RING: --ring -r +takes_value "Ring key name")
        (@arg CHANNEL: --channel +takes_value
            "Receive Supervisor updates from the specified release channel [default: stable]")
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Receive Supervisor updates from Builder at the specified URL \
            [default: https://bldr.habitat.sh]")
        (@arg AUTO_UPDATE: --("auto-update") -A "Enable automatic updates for the Supervisor \
            itself")
        (@arg EVENTS: --events -n +takes_value {valid_service_group} "Name of the service \
            group running a Habitat EventSrv to forward Supervisor and service event data to")
    )
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub fn start<'a, 'b>() -> App<'a, 'b> {
    let base = clap_app!(@subcommand start =>
        (about: "Start a loaded, but stopped, Habitat service or a transient service from \
            a package or artifact. If the Habitat Supervisor is not already running this \
            will additionally start one for you.")
        (aliases: &["sta", "star"])
        (@arg LISTEN_GOSSIP: --("listen-gossip") +takes_value {valid_listen_gossip}
            "The listen address for the gossip system [default: 0.0.0.0:9638]")
        (@arg LISTEN_HTTP: --("listen-http") +takes_value {valid_listen_http}
            "The listen address for the HTTP gateway [default: 0.0.0.0:9631]")
        (@arg NAME: --("override-name") +takes_value
            "The name for the state directory if launching more than one Supervisor \
            [default: default]")
        (@arg ORGANIZATION: --org +takes_value
            "The organization that the Supervisor and it's subsequent services are part of \
            [default: default]")
        (@arg PEER: --peer +takes_value +multiple
            "The listen address of an initial peer (IP[:PORT])")
        (@arg PERMANENT_PEER: --("permanent-peer") -I "If this Supervisor is a permanent peer")
        (@arg PEER_WATCH_FILE: --("peer-watch-file") +takes_value conflicts_with[peer]
            "Watch this file for connecting to the ring"
        )
        (@arg RING: --ring -r +takes_value "Ring key name")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +takes_value
            "A Habitat package identifier (ex: core/redis) or filepath to a Habitat Artifact \
            (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart)")
        (@arg APPLICATION: --application -a +takes_value requires[ENVIRONMENT]
            "Application name; [default: not set].")
        (@arg ENVIRONMENT: --environment -e +takes_value requires[APPLICATION]
            "Environment name; [default: not set].")
        (@arg CHANNEL: --channel +takes_value
            "Receive package updates from the specified release channel [default: stable]")
        (@arg GROUP: --group +takes_value
            "The service group; shared config and topology [default: default]")
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Receive package updates from Builder at the specified URL \
            [default: https://bldr.habitat.sh]")
        (@arg TOPOLOGY: --topology -t +takes_value {valid_topology}
            "Service topology; [default: none]")
        (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
            "The update strategy; [default: none] [values: none, at-once, rolling]")
        (@arg BIND: --bind +takes_value +multiple
            "One or more service groups to bind to a configuration")
        (@arg CONFIG_DIR: --("config-from") +takes_value {dir_exists}
            "Use package config from this path, rather than the package itself")
        (@arg AUTO_UPDATE: --("auto-update") -A "Enable automatic updates for the Supervisor \
            itself")
        (@arg EVENTS: --events -n +takes_value {valid_service_group} "Name of the service \
            group running a Habitat EventSrv to forward Supervisor and service event data to")
    );

    maybe_add_password_arg(base)
}

pub fn status<'a, 'b>() -> App<'a, 'b> {
    clap_app!(@subcommand status =>
        (about: "Query the status of Habitat services.")
        (aliases: &["stat", "statu", "status"])
        (@arg PKG_IDENT: +takes_value "A Habitat package identifier (ex: core/redis)")
        (@arg NAME: --("override-name") +takes_value
            "The name for the state directory if there is more than one Supervisor running \
            [default: default]")
    )
}

pub fn stop<'a, 'b>() -> App<'a, 'b> {
    clap_app!(@subcommand stop =>
        (about: "Stop a running Habitat service.")
        (aliases: &["sto"])
        (@arg PKG_IDENT: +required +takes_value "A Habitat package identifier (ex: core/redis)")
        (@arg NAME: --("override-name") +takes_value
            "The name for the state directory if there is more than one Supervisor running \
            [default: default]")
    )
}

pub fn maybe_add_term_subcommand<'a, 'b>(base: App<'a, 'b>) -> App<'a, 'b> {
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        base.subcommand(
            clap_app!(("term") =>
                (about: "Gracefully terminate the Habitat Supervisor and all of it's running services")
                (@arg NAME: --("override-name") +takes_value
                    "The name of the Supervisor if more than one is running [default: default]")
            )
        )
    } else {
        base
    }
}

fn maybe_add_password_arg<'a, 'b>(base: App<'a, 'b>) -> App<'a, 'b> {
    if cfg!(target_os = "windows") {
        base.arg(
            Arg::with_name("PASSWORD")
                .long("password")
                .takes_value(true)
                .help("Password of the service user"),
        )
    } else {
        base
    }
}

// CLAP Validation Functions
////////////////////////////////////////////////////////////////////////

fn dir_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_dir() {
        Ok(())
    } else {
        Err(format!("Directory: '{}' cannot be found", &val))
    }
}

fn valid_service_group(val: String) -> result::Result<(), String> {
    match ServiceGroup::validate(&val) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn valid_topology(val: String) -> result::Result<(), String> {
    match Topology::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Service topology: '{}' is not valid", &val)),
    }
}

fn valid_listen_gossip(val: String) -> result::Result<(), String> {
    match GossipListenAddr::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "Listen gossip address should include both IP and port, eg: '0.0.0.0:9700'"
        )),
    }
}

fn valid_listen_http(val: String) -> result::Result<(), String> {
    match ListenAddr::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "Listen http address should include both IP and port, eg: '0.0.0.0:9700'"
        )),
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

////////////////////////////////////////////////////////////////////////
