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

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate habitat_sup as sup;
#[cfg(unix)]
extern crate jemalloc_ctl;
#[cfg(unix)]
extern crate jemallocator;
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
extern crate rustls;
extern crate tempfile;
extern crate time;
extern crate url;

use crate::sup::{cli::cli,
                 command,
                 error::{Error,
                         Result,
                         SupError},
                 manager::{Manager,
                           ManagerConfig,
                           TLSConfig,
                           PROC_LOCK_FILE},
                 util};
use clap::ArgMatches;
use habitat_common::{cli::{cache_key_path_from_matches,
                           GOSSIP_DEFAULT_PORT},
                     command::package::install::InstallSource,
                     output::{self,
                              OutputFormat,
                              OutputVerbosity},
                     outputln,
                     ui::{NONINTERACTIVE_ENVVAR,
                          UI},
                     FeatureFlag};
#[cfg(windows)]
use habitat_core::crypto::dpapi::encrypt;
use habitat_core::{crypto::{self,
                            SymKey},
                   url::{bldr_url_from_env,
                         default_bldr_url},
                   ChannelIdent};
use habitat_launcher_client::{LauncherCli,
                              ERR_NO_RETRY_EXCODE};
use habitat_sup_protocol::{ctl::ServiceBindList,
                           types::{ApplicationEnvironment,
                                   BindingMode,
                                   ServiceBind,
                                   Topology,
                                   UpdateStrategy}};
use std::{env,
          io::{self,
               Write},
          net::{Ipv4Addr,
                SocketAddr,
                ToSocketAddrs},
          path::{Path,
                 PathBuf},
          process,
          str::{self,
                FromStr}};
#[cfg(test)]
use tempfile::TempDir;

/// Our output key
static LOGKEY: &'static str = "MN";

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    let flags = FeatureFlag::from_env(&mut ui);
    let result = start(flags);
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
    match habitat_launcher_client::env_pipe() {
        Some(pipe) => {
            match LauncherCli::connect(pipe) {
                Ok(launcher) => Some(launcher),
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }
        None => None,
    }
}

fn start(feature_flags: FeatureFlag) -> Result<()> {
    if feature_flags.contains(FeatureFlag::TEST_BOOT_FAIL) {
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
                None => {
                    match err.kind {
                        clap::ErrorKind::HelpDisplayed => process::exit(0),
                        clap::ErrorKind::VersionDisplayed => process::exit(0),
                        _ => process::exit(ERR_NO_RETRY_EXCODE),
                    }
                }
            }
        }
    };
    match app_matches.subcommand() {
        ("bash", Some(_)) => sub_bash(),
        ("run", Some(m)) => {
            let launcher = launcher.ok_or(sup_error!(Error::NoLauncher))?;
            sub_run(m, launcher, feature_flags)
        }
        ("sh", Some(_)) => sub_sh(),
        ("term", Some(_)) => sub_term(),
        _ => unreachable!(),
    }
}

fn sub_bash() -> Result<()> { command::shell::bash() }

fn sub_run(m: &ArgMatches, launcher: LauncherCli, feature_flags: FeatureFlag) -> Result<()> {
    set_supervisor_logging_options(m);

    let cfg = mgrcfg_from_sup_run_matches(m, feature_flags)?;
    let manager = Manager::load(cfg, launcher)?;

    // We need to determine if we have an initial service to start
    let svc = if let Some(pkg) = m.value_of("PKG_IDENT_OR_ARTIFACT") {
        let mut msg = habitat_sup_protocol::ctl::SvcLoad::default();
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
                let install =
                    util::pkg::install(&mut ui(),
                                       msg.bldr_url
                                          .as_ref()
                                          .unwrap_or(&*habitat_sup_protocol::DEFAULT_BLDR_URL),
                                       &source,
                                       &msg.bldr_channel
                                           .clone()
                                           .map(ChannelIdent::from)
                                           .expect("update_svc_load_from_input to always set to \
                                                    Some"))?;
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

fn sub_sh() -> Result<()> { command::shell::sh() }

fn sub_term() -> Result<()> {
    // We were generating a ManagerConfig from matches here, but 'hab sup term' takes no options.
    // This means that we were implicitly getting the default ManagerConfig here. Instead of calling
    // a function to generate said config, we can just explicitly pass the default.
    let proc_lock_file = habitat_sup_protocol::sup_root(None).join(PROC_LOCK_FILE);
    match Manager::term(&proc_lock_file) {
        Err(SupError { err: Error::ProcessLockIO(..),
                       .. }) => {
            println!("Supervisor not started.");
            Ok(())
        }
        result => result,
    }
}

// Internal Implementation Details
////////////////////////////////////////////////////////////////////////

fn mgrcfg_from_sup_run_matches(m: &ArgMatches,
                               feature_flags: FeatureFlag)
                               -> Result<ManagerConfig> {
    let cache_key_path = cache_key_path_from_matches(m);

    let cfg = ManagerConfig {
        auto_update: m.is_present("AUTO_UPDATE"),
        custom_state_path: None, // remove entirely?
        cache_key_path,
        update_url: bldr_url(m),
        update_channel: channel(m),
        http_disable: m.is_present("HTTP_DISABLE"),
        organization: m.value_of("ORGANIZATION").map(str::to_string),
        gossip_permanent: m.is_present("PERMANENT_PEER"),
        ring_key: get_ring_key(m, &cache_key_path_from_matches(m))?,
        gossip_peers: get_peers(m)?,
        watch_peer_file: m.value_of("PEER_WATCH_FILE").map(str::to_string),
        // TODO: Refactor this to remove the duplication
        gossip_listen: if m.is_present("LOCAL_GOSSIP_MODE") {
            // When local gossip mode is used we still startup the gossip layer but set
            // it to listen on 127.0.0.2 to scope it to the local node.
            sup::config::GossipListenAddr::new(Ipv4Addr::new(127, 0, 0, 2), GOSSIP_DEFAULT_PORT)
        } else {
            m.value_of("LISTEN_GOSSIP").map_or_else(
                || {
                    let default = sup::config::GossipListenAddr::default();
                    error!(
                        "Value for LISTEN_GOSSIP has not been set. Using default: {}",
                        default
                    );
                    Ok(default)
                },
                str::parse,
            )?
        },
        ctl_listen: m.value_of("LISTEN_CTL").map_or_else(
            || {
                let default = habitat_common::types::ListenCtlAddr::default();
                error!(
                    "Value for LISTEN_CTL has not been set. Using default: {}",
                    default
                );
                Ok(default)
            },
            str::parse,
        )?,
        http_listen: m.value_of("LISTEN_HTTP").map_or_else(
            || {
                let default = sup::http_gateway::ListenAddr::default();
                error!(
                    "Value for LISTEN_HTTP has not been set. Using default: {}",
                    default
                );
                Ok(default)
            },
            str::parse,
        )?,
        tls_config: m.value_of("KEY_FILE").map(|kf| {
            let cert_path = m
                .value_of("CERT_FILE")
                .map(PathBuf::from)
                .expect("CERT_FILE should always have a value if KEY_FILE has a value.");
            let ca_cert_path = m.value_of("CA_CERT_FILE").map(PathBuf::from);
            TLSConfig {
                key_path: PathBuf::from(kf),
                cert_path,
                ca_cert_path,
            }
        }),
        feature_flags
    };

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
// The use of env variables here makes it difficult to unit test. Since tests are run in parallel,
// setting an env var in one test can adversely effect the results in another test. We need some
// additional abstractions written around env vars in order to make them more testable.
fn get_ring_key(m: &ArgMatches, cache_key_path: &Path) -> Result<Option<SymKey>> {
    match m.value_of("RING") {
        Some(val) => {
            let key = SymKey::get_latest_pair_for(&val, cache_key_path)?;
            Ok(Some(key))
        }
        None => {
            match m.value_of("RING_KEY") {
                Some(val) => {
                    let (key, _) = SymKey::write_file_from_str(&val, cache_key_path)?;
                    Ok(Some(key))
                }
                None => Ok(None),
            }
        }
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
     .map(str::to_string)
     .or_else(bldr_url_from_env)
}

/// Resolve a channel. Taken from CLI args, or (failing that), a
/// default value.
fn channel(matches: &ArgMatches) -> ChannelIdent { channel_from_input(matches).unwrap_or_default() }

/// A channel name, but *only* if the user specified via CLI args.
fn channel_from_input(m: &ArgMatches) -> Option<ChannelIdent> {
    m.value_of("CHANNEL").map(ChannelIdent::from)
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
        Ok(Some(ApplicationEnvironment { application: app.to_string(),
                                         environment: env.to_string(), }))
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
                list.binds.push(ServiceBind::from_str(bind_str)?);
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
        warn!("WARNING: Setting '--config-from' should only be used in development, not \
               production!");
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
fn get_password_from_input(_m: &ArgMatches) -> Result<Option<String>> { Ok(None) }

fn set_supervisor_logging_options(m: &ArgMatches) {
    if m.is_present("VERBOSE") {
        output::set_verbosity(OutputVerbosity::Verbose);
    }
    if m.is_present("NO_COLOR") {
        output::set_format(OutputFormat::NoColor)
    }
    if m.is_present("JSON") {
        output::set_format(OutputFormat::JSON)
    }
}

// Based on UI::default_with_env, but taking into account the setting
// of the global color variable.
//
// TODO: Ideally we'd have a unified way of setting color, so this
// function wouldn't be necessary. In the meantime, though, it'll keep
// the scope of change contained.
fn ui() -> UI {
    let isatty = if env::var(NONINTERACTIVE_ENVVAR).map(|val| val == "1" || val == "true")
                                                   .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    UI::default_with(output::get_format().color_choice(), isatty)
}

/// Set all fields for an `SvcLoad` message that we can from the given opts. This function
/// populates all *shared* options between `run` and `load`.
fn update_svc_load_from_input(m: &ArgMatches,
                              msg: &mut habitat_sup_protocol::ctl::SvcLoad)
                              -> Result<()> {
    msg.bldr_url = Some(bldr_url(m));
    msg.bldr_channel = Some(channel(m).to_string());
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
    use crate::sup::{config::GossipListenAddr,
                     http_gateway};
    use habitat_common::{locked_env_var,
                         types::ListenCtlAddr};

    fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

    mod manager_config {

        use super::*;
        use std::iter::FromIterator;

        locked_env_var!(HAB_CACHE_KEY_PATH, lock_var);

        fn config_from_cmd_str(cmd: &str) -> ManagerConfig {
            let cmd_vec = cmd_vec_from_cmd_str(&cmd);
            config_from_cmd_vec(cmd_vec)
        }

        fn cmd_vec_from_cmd_str(cmd: &str) -> Vec<&str> { Vec::from_iter(cmd.split_whitespace()) }

        fn config_from_cmd_vec(cmd_vec: Vec<&str>) -> ManagerConfig {
            let matches = cli().get_matches_from_safe(cmd_vec)
                               .expect("Error while getting matches");
            let (_, sub_matches) = matches.subcommand();
            let sub_matches = sub_matches.expect("Error getting sub command matches");

            mgrcfg_from_sup_run_matches(&sub_matches, no_feature_flags()).expect("Could not get \
                                                                                  config")
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
            assert_eq!(config.update_channel, ChannelIdent::unstable());
        }

        #[test]
        fn update_channel_is_set_to_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.update_channel, ChannelIdent::stable());
        }

        #[test]
        fn gossip_listen_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --listen-gossip 1.1.1.1:1111");
            let expected_addr =
                GossipListenAddr::from_str("1.1.1.1:1111").expect("Could not create \
                                                                   GossipListenAddr");
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
            let expected_addr =
                http_gateway::ListenAddr::from_str("2.2.2.2:2222").expect("Could not create http \
                                                                           listen addr");
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
                ListenCtlAddr::from_str("3.3.3.3:3333").expect("Could not create ctl listen addr");
            assert_eq!(config.ctl_listen, expected_addr);

            let config = config_from_cmd_str("hab-sup run");
            let expected_addr = ListenCtlAddr::default();
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
            let expected_peers: Vec<SocketAddr> =
                vec!["1.1.1.1:1", "2.2.2.2:1", "3.3.3.3:1"].into_iter()
                                                           .flat_map(|peer| {
                                                               peer.to_socket_addrs()
                                                                   .expect("Failed getting addrs")
                                                           })
                                                           .collect();
            assert_eq!(config.gossip_peers, expected_peers);

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.gossip_peers, Vec::new());
        }

        #[test]
        fn peers_should_have_a_default_port_set() {
            let config = config_from_cmd_str("hab-sup run --peer 1.1.1.1 2.2.2.2 3.3.3.3");
            let expected_peers: Vec<SocketAddr> =
                vec!["1.1.1.1", "2.2.2.2", "3.3.3.3"].into_iter()
                                                     .map(|peer| {
                                                         format!("{}:{}", peer, GOSSIP_DEFAULT_PORT)
                                                     })
                                                     .flat_map(|peer| {
                                                         peer.to_socket_addrs()
                                                             .expect("Failed getting addrs")
                                                     })
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
        fn ring_key_is_set_properly_by_name() {
            let key_cache = TempDir::new().expect("Could not create tempdir");
            let lock = lock_var();
            lock.set(key_cache.path());

            let key_content =
                "SYM-SEC-1\nfoobar-20160504220722\n\nRCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
            let (pair, _) = SymKey::write_file_from_str(key_content, key_cache.path())
                .expect("Could not write key pair");
            let config = config_from_cmd_str("hab-sup run --ring foobar");

            assert_eq!(config.ring_key
                             .expect("No ring key on manager config")
                             .name_with_rev(),
                       pair.name_with_rev());
        }

        #[test]
        fn ring_key_is_set_properly_by_content() {
            let key_cache = TempDir::new().expect("Could not create tempdir");
            let lock = lock_var();
            lock.set(key_cache.path());

            env::set_var("HAB_CACHE_KEY_PATH", key_cache.path());
            let cmd_vec = vec![
                               "hab-sup",
                               "run",
                               "--ring-key",
                               r#"SYM-SEC-1
foobar-20160504220722

RCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE="#,
            ];
            let config = config_from_cmd_vec(cmd_vec);

            assert_eq!(config.ring_key
                             .expect("No ring key on manager config")
                             .name_with_rev(),
                       "foobar-20160504220722");
        }

    }
}
