// Copyright (c) 2017-2017 Chef Software Inc. and/or applicable contributors
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

extern crate ansi_term;
extern crate clap;
extern crate env_logger;
extern crate habitat_common as common;
#[macro_use]
extern crate habitat_core as hcore;
extern crate habitat_launcher_client as launcher_client;
#[macro_use]
extern crate habitat_sup as sup;
extern crate habitat_sup_protocol as protocol;
extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate time;
extern crate tokio_core;
extern crate url;

use std::env;
use std::io::{self, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::process;
use std::str::{self, FromStr};

use clap::ArgMatches;
use common::command::package::install::InstallSource;
use common::ui::{Coloring, NONINTERACTIVE_ENVVAR, UI};
use hcore::channel;
#[cfg(windows)]
use hcore::crypto::dpapi::encrypt;
use hcore::crypto::{self, default_cache_key_path, SymKey};
use hcore::env as henv;
use hcore::service::ServiceGroup;
use hcore::url::{bldr_url_from_env, default_bldr_url};
use launcher_client::{LauncherCli, ERR_NO_RETRY_EXCODE};
use protocol::{
    ctl::ServiceBindList,
    types::{ApplicationEnvironment, BindingMode, ServiceBind, Topology, UpdateStrategy},
};

use sup::cli::cli;
use sup::command;
use sup::config::{GossipListenAddr, GOSSIP_DEFAULT_PORT};
use sup::error::{Error, Result, SupError};
use sup::feat;
use sup::http_gateway;
use sup::manager::{Manager, ManagerConfig};
use sup::util;

/// Our output key
static LOGKEY: &'static str = "MN";

static RING_ENVVAR: &'static str = "HAB_RING";
static RING_KEY_ENVVAR: &'static str = "HAB_RING_KEY";

fn main() {
    env_logger::init();
    enable_features_from_env();
    let result = start();
    let exit_code = match result {
        Ok(_) => 0,
        Err(ref err) => {
            println!("{}", err);
            ERR_NO_RETRY_EXCODE
        }
    };
    debug!("start() returned {:?}; Exiting {}", result, exit_code);
    process::exit(exit_code);
}

fn boot() -> Option<LauncherCli> {
    if !crypto::init() {
        println!("Crypto initialization failed!");
        process::exit(1);
    }
    match launcher_client::env_pipe() {
        Some(pipe) => match LauncherCli::connect(pipe) {
            Ok(launcher) => Some(launcher),
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        },
        None => None,
    }
}

fn start() -> Result<()> {
    if feat::is_enabled(feat::TestBootFail) {
        outputln!("Simulating boot failure");
        return Err(sup_error!(Error::TestBootFail));
    }
    let launcher = boot();
    let app_matches = match cli().get_matches_safe() {
        Ok(matches) => matches,
        Err(err) => {
            let out = io::stdout();
            writeln!(&mut out.lock(), "{}", err.message).expect("Error writing Error to stdout");
            match launcher {
                Some(_) => process::exit(ERR_NO_RETRY_EXCODE),
                // If we weren't started by a launcher, exit 0 for
                // help and version
                None => match err.kind {
                    clap::ErrorKind::HelpDisplayed => process::exit(0),
                    clap::ErrorKind::VersionDisplayed => process::exit(0),
                    _ => process::exit(ERR_NO_RETRY_EXCODE),
                },
            }
        }
    };
    match app_matches.subcommand() {
        ("bash", Some(_)) => sub_bash(),
        ("run", Some(m)) => {
            let launcher = launcher.ok_or(sup_error!(Error::NoLauncher))?;
            sub_run(m, launcher)
        }
        ("sh", Some(_)) => sub_sh(),
        ("term", Some(m)) => sub_term(m),
        _ => unreachable!(),
    }
}

fn sub_bash() -> Result<()> {
    command::shell::bash()
}

fn sub_run(m: &ArgMatches, launcher: LauncherCli) -> Result<()> {
    set_supervisor_logging_options(m);

    let cfg = mgrcfg_from_matches(m)?;
    let manager = Manager::load(cfg, launcher)?;

    // We need to determine if we have an initial service to start
    let svc = if let Some(pkg) = m.value_of("PKG_IDENT_OR_ARTIFACT") {
        let mut msg = protocol::ctl::SvcLoad::default();
        update_svc_load_from_input(m, &mut msg)?;
        // Always force - running with a package ident is a "do what I mean" operation. You
        // don't care if a service was loaded previously or not and with what options. You
        // want one loaded right now and in this way.
        msg.force = Some(true);
        let ident = match pkg.parse::<InstallSource>()? {
            source @ InstallSource::Archive(_) => {
                // Install the archive manually then explicitly set the pkg ident to the
                // version found in the archive. This will lock the software to this
                // specific version.
                let install = util::pkg::install(
                    &mut ui(),
                    msg.bldr_url
                        .as_ref()
                        .unwrap_or(&*protocol::DEFAULT_BLDR_URL),
                    &source,
                    msg.bldr_channel
                        .as_ref()
                        .unwrap_or(&*protocol::DEFAULT_BLDR_CHANNEL),
                )?;
                install.ident.into()
            }
            InstallSource::Ident(ident, _) => ident.into(),
        };
        msg.ident = Some(ident);
        Some(msg)
    } else {
        None
    };
    manager.run(svc)
}

fn sub_sh() -> Result<()> {
    command::shell::sh()
}

fn sub_term(m: &ArgMatches) -> Result<()> {
    let cfg = mgrcfg_from_matches(m)?;
    match Manager::term(&cfg) {
        Err(SupError {
            err: Error::ProcessLockIO(_, _),
            ..
        }) => {
            println!("Supervisor not started.");
            Ok(())
        }
        result => result,
    }
}

// Internal Implementation Details
////////////////////////////////////////////////////////////////////////

fn mgrcfg_from_matches(m: &ArgMatches) -> Result<ManagerConfig> {
    let mut cfg = ManagerConfig {
        auto_update: m.is_present("AUTO_UPDATE"),
        update_url: bldr_url(m),
        update_channel: channel(m),
        http_disable: m.is_present("HTTP_DISABLE"),
        organization: m.value_of("ORGANIZATION").map(|org| org.to_string()),
        gossip_permanent: m.is_present("PERMANENT_PEER"),
        ring_key: get_ring_key(m)?,
        gossip_peers: get_peers(m)?,
        ..Default::default()
    };
    if let Some(addr_str) = m.value_of("LISTEN_GOSSIP") {
        cfg.gossip_listen = GossipListenAddr::from_str(addr_str)?;
    }
    if let Some(addr_str) = m.value_of("LISTEN_HTTP") {
        cfg.http_listen = http_gateway::ListenAddr::from_str(addr_str)?;
    }
    if let Some(addr_str) = m.value_of("LISTEN_CTL") {
        cfg.ctl_listen = SocketAddr::from_str(addr_str)?;
    }
    if let Some(watch_peer_file) = m.value_of("PEER_WATCH_FILE") {
        cfg.watch_peer_file = Some(String::from(watch_peer_file));
    }
    if let Some(events) = m.value_of("EVENTS") {
        cfg.eventsrv_group = ServiceGroup::from_str(events).ok();
    }
    Ok(cfg)
}

// Various CLI Parsing Functions
////////////////////////////////////////////////////////////////////////

fn get_peers(matches: &ArgMatches) -> Result<Vec<SocketAddr>> {
    // TODO fn: Clean this up--using a for loop doesn't feel good however an iterator was
    // causing a lot of developer/compiler type confusion
    let mut gossip_peers = Vec::new();
    if let Some(peers) = matches.values_of("PEER") {
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
            if let Some(addr) = addrs.get(0) {
                gossip_peers.push(*addr);
            }
        }
    }
    Ok(gossip_peers)
}

// TODO: Make this more testable.
// The use of env variables here makes it difficult to unit test. Since tests are run in parallel, setting an env var in one test
// can adversely effect the results in another test. We need some additional abstractions written around env vars in order to make
// them more testable.
fn get_ring_key(m: &ArgMatches) -> Result<Option<SymKey>> {
    match m.value_of("RING") {
        Some(val) => {
            let key = SymKey::get_latest_pair_for(&val, &default_cache_key_path(None))?;
            Ok(Some(key))
        }
        None => match henv::var(RING_KEY_ENVVAR) {
            Ok(val) => {
                let (key, _) = SymKey::write_file_from_str(&val, &default_cache_key_path(None))?;
                Ok(Some(key))
            }
            Err(_) => match henv::var(RING_ENVVAR) {
                Ok(val) => {
                    let key = SymKey::get_latest_pair_for(&val, &default_cache_key_path(None))?;
                    Ok(Some(key))
                }
                Err(_) => Ok(None),
            },
        },
    }
}

/// Resolve a Builder URL. Taken from CLI args, the environment, or
/// (failing those) a default value.
fn bldr_url(m: &ArgMatches) -> String {
    match bldr_url_from_input(m) {
        Some(url) => url.to_string(),
        None => default_bldr_url(),
    }
}

/// A Builder URL, but *only* if the user specified it via CLI args or
/// the environment
fn bldr_url_from_input(m: &ArgMatches) -> Option<String> {
    m.value_of("BLDR_URL")
        .and_then(|u| Some(u.to_string()))
        .or_else(|| bldr_url_from_env())
}

/// Resolve a channel. Taken from CLI args, or (failing that), a
/// default value.
fn channel(matches: &ArgMatches) -> String {
    channel_from_input(matches).unwrap_or(channel::default())
}

/// A channel name, but *only* if the user specified via CLI args.
fn channel_from_input(m: &ArgMatches) -> Option<String> {
    m.value_of("CHANNEL").and_then(|c| Some(c.to_string()))
}

// ServiceSpec Modification Functions
////////////////////////////////////////////////////////////////////////

fn get_group_from_input(m: &ArgMatches) -> Option<String> {
    m.value_of("GROUP").map(ToString::to_string)
}

/// If the user provides both --application and --environment options,
/// parse and set the value on the spec.
fn get_app_env_from_input(m: &ArgMatches) -> Result<Option<ApplicationEnvironment>> {
    if let (Some(app), Some(env)) = (m.value_of("APPLICATION"), m.value_of("ENVIRONMENT")) {
        Ok(Some(ApplicationEnvironment {
            application: app.to_string(),
            environment: env.to_string(),
        }))
    } else {
        Ok(None)
    }
}

fn get_topology_from_input(m: &ArgMatches) -> Option<Topology> {
    m.value_of("TOPOLOGY")
        .and_then(|f| Topology::from_str(f).ok())
}

fn get_strategy_from_input(m: &ArgMatches) -> Option<UpdateStrategy> {
    m.value_of("STRATEGY")
        .and_then(|f| UpdateStrategy::from_str(f).ok())
}

fn get_binds_from_input(m: &ArgMatches) -> Result<Option<ServiceBindList>> {
    match m.values_of("BIND") {
        Some(bind_strs) => {
            let mut list = ServiceBindList::default();
            for bind_str in bind_strs {
                list.binds.push(ServiceBind::from_str(bind_str)?.into());
            }
            Ok(Some(list))
        }
        None => Ok(None),
    }
}

fn get_binding_mode_from_input(m: &ArgMatches) -> Option<BindingMode> {
    // There won't be errors, because we validate with `valid_binding_mode`
    m.value_of("BINDING_MODE")
        .and_then(|b| BindingMode::from_str(b).ok())
}

fn get_config_from_input(m: &ArgMatches) -> Option<String> {
    if let Some(ref config_from) = m.value_of("CONFIG_DIR") {
        warn!("");
        warn!(
            "WARNING: Setting '--config-from' should only be used in development, not production!"
        );
        warn!("");
        Some(config_from.to_string())
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
fn get_password_from_input(m: &ArgMatches) -> Result<Option<String>> {
    if let Some(password) = m.value_of("PASSWORD") {
        Ok(Some(encrypt(password.to_string())?))
    } else {
        Ok(None)
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_password_from_input(_m: &ArgMatches) -> Result<Option<String>> {
    Ok(None)
}

fn enable_features_from_env() {
    let features = vec![
        (feat::List, "LIST"),
        (feat::TestExit, "TEST_EXIT"),
        (feat::TestBootFail, "BOOT_FAIL"),
        (feat::RedactHTTP, "REDACT_HTTP"),
        (feat::IgnoreSignals, "IGNORE_SIGNALS"),
    ];

    // If the environment variable for a flag is set to _anything_ but
    // the empty string, it is activated.
    for feature in &features {
        match henv::var(format!("HAB_FEAT_{}", feature.1)) {
            Ok(_) => {
                feat::enable(feature.0);
                outputln!("Enabling feature: {:?}", feature.0);
            }
            _ => {}
        }
    }

    if feat::is_enabled(feat::List) {
        outputln!("Listing feature flags environment variables:");
        for feature in &features {
            outputln!(
                "     * {:?}: HAB_FEAT_{}={}",
                feature.0,
                feature.1,
                henv::var(format!("HAB_FEAT_{}", feature.1)).unwrap_or("".to_string())
            );
        }
        outputln!("The Supervisor will start now, enjoy!");
    }
}

fn set_supervisor_logging_options(m: &ArgMatches) {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }
    if m.is_present("JSON") {
        hcore::output::set_json(true)
    }
}

// Based on UI::default_with_env, but taking into account the setting
// of the global color variable.
//
// TODO: Ideally we'd have a unified way of setting color, so this
// function wouldn't be necessary. In the meantime, though, it'll keep
// the scope of change contained.
fn ui() -> UI {
    let coloring = if hcore::output::is_color() {
        Coloring::Auto
    } else {
        Coloring::Never
    };
    let isatty = if env::var(NONINTERACTIVE_ENVVAR)
        .map(|val| val == "1" || val == "true")
        .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    UI::default_with(coloring, isatty)
}

/// Set all fields for an `SvcLoad` message that we can from the given opts. This function
/// populates all *shared* options between `run` and `load`.
fn update_svc_load_from_input(m: &ArgMatches, msg: &mut protocol::ctl::SvcLoad) -> Result<()> {
    msg.bldr_url = Some(bldr_url(m));
    msg.bldr_channel = Some(channel(m));
    msg.application_environment = get_app_env_from_input(m)?;
    msg.binds = get_binds_from_input(m)?;
    msg.config_from = get_config_from_input(m);
    if m.is_present("FORCE") {
        msg.force = Some(true);
    }
    msg.group = get_group_from_input(m);
    msg.svc_encrypted_password = get_password_from_input(m)?;
    msg.binding_mode = get_binding_mode_from_input(m).map(|v| v as i32);
    msg.topology = get_topology_from_input(m).map(|v| v as i32);
    msg.update_strategy = get_strategy_from_input(m).map(|v| v as i32);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    mod manager_config {

        use super::*;
        use std::iter::FromIterator;

        fn config_from_cmd_str(cmd: &str) -> ManagerConfig {
            let cmd_vec = Vec::from_iter(cmd.split_whitespace());
            let matches = cli()
                .get_matches_from_safe(cmd_vec)
                .expect("Error while getting matches");
            let (_, sub_matches) = matches.subcommand();
            let sub_matches = sub_matches.expect("Error getting sub command matches");

            mgrcfg_from_matches(&sub_matches).expect("Could not get config")
        }

        #[test]
        fn auto_update_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --auto-update");
            assert_eq!(config.auto_update, true);

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.auto_update, false);
        }

        #[test]
        fn update_url_should_be_set() {
            let config = config_from_cmd_str("hab-sup run -u http://fake.example.url");
            assert_eq!(config.update_url, "http://fake.example.url");
        }

        #[test]
        fn update_url_is_set_to_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.update_url, default_bldr_url());
        }

        #[test]
        fn update_channel_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --channel unstable");
            assert_eq!(config.update_channel, "unstable");
        }

        #[test]
        fn update_channel_is_set_to_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.update_channel, "stable");
        }

        #[test]
        fn gossip_listen_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --listen-gossip 1.1.1.1:1111");
            let expected_addr = GossipListenAddr::from_str("1.1.1.1:1111")
                .expect("Could not create GossipListenAddr");
            assert_eq!(config.gossip_listen, expected_addr);
        }

        #[test]
        fn gossip_listen_is_set_to_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            let expected_addr = GossipListenAddr::default();
            assert_eq!(config.gossip_listen, expected_addr);
        }

        #[test]
        fn http_listen_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --listen-http 2.2.2.2:2222");
            let expected_addr = http_gateway::ListenAddr::from_str("2.2.2.2:2222")
                .expect("Could not create http listen addr");
            assert_eq!(config.http_listen, expected_addr);
        }

        #[test]
        fn http_listen_is_set_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            let expected_addr = http_gateway::ListenAddr::default();
            assert_eq!(config.http_listen, expected_addr);
        }

        #[test]
        fn http_disable_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --http-disable");
            assert_eq!(config.http_disable, true);

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.http_disable, false);
        }

        #[test]
        fn ctl_listen_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --listen-ctl 3.3.3.3:3333");
            let expected_addr =
                SocketAddr::from_str("3.3.3.3:3333").expect("Could not create ctl listen addr");
            assert_eq!(config.ctl_listen, expected_addr);

            let config = config_from_cmd_str("hab-sup run");
            let expected_addr = protocol::ctl::default_addr();
            assert_eq!(config.ctl_listen, expected_addr);
        }

        #[test]
        fn organization_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --org foobar");
            assert_eq!(config.organization, Some("foobar".to_string()));

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.organization, None);
        }

        #[test]
        fn gossip_permanent_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --permanent-peer");
            assert_eq!(config.gossip_permanent, true);

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.gossip_permanent, false);
        }

        #[test]
        fn peers_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --peer 1.1.1.1:1 2.2.2.2:1 3.3.3.3:1");
            let expected_peers: Vec<SocketAddr> = vec!["1.1.1.1:1", "2.2.2.2:1", "3.3.3.3:1"]
                .into_iter()
                .flat_map(|peer| peer.to_socket_addrs().expect("Failed getting addrs"))
                .collect();
            assert_eq!(config.gossip_peers, expected_peers);

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.gossip_peers, Vec::new());
        }

        #[test]
        fn peers_should_have_a_default_port_set() {
            let config = config_from_cmd_str("hab-sup run --peer 1.1.1.1 2.2.2.2 3.3.3.3");
            let expected_peers: Vec<SocketAddr> = vec!["1.1.1.1", "2.2.2.2", "3.3.3.3"]
                .into_iter()
                .map(|peer| format!("{}:{}", peer, GOSSIP_DEFAULT_PORT))
                .flat_map(|peer| peer.to_socket_addrs().expect("Failed getting addrs"))
                .collect();
            assert_eq!(config.gossip_peers, expected_peers);
        }

        #[test]
        fn watch_peer_file_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --peer-watch-file foobar");
            assert_eq!(config.watch_peer_file, Some("foobar".to_string()));

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.watch_peer_file, None);
        }

        #[test]
        fn events_is_set() {
            let config = config_from_cmd_str("hab-sup run --events event.service");
            let eventsrv_group = config.eventsrv_group;
            let expected_group: Option<hcore::service::ServiceGroup> =
                ServiceGroup::from_str("event.service").ok().map(Into::into);

            assert_eq!(eventsrv_group, expected_group);
        }

    }
}
