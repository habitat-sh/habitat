extern crate clap;
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
extern crate url;

use crate::sup::{cli::cli,
                 command,
                 error::{Error,
                         Result},
                 event::EventStreamConfig,
                 logger,
                 manager::{Manager,
                           ManagerConfig,
                           TLSConfig,
                           PROC_LOCK_FILE},
                 util};
use clap::ArgMatches;
use hab::cli::{hab::util as cli_util,
               parse_optional_arg};
use habitat_common::{cli::cache_key_path_from_matches,
                     command::package::install::InstallSource,
                     liveliness_checker,
                     output::{self,
                              OutputFormat,
                              OutputVerbosity},
                     outputln,
                     types::GossipListenAddr,
                     ui::{UIWriter,
                          NONINTERACTIVE_ENVVAR,
                          UI},
                     FeatureFlag};
#[cfg(windows)]
use habitat_core::crypto::dpapi::encrypt;
use habitat_core::{self,
                   crypto::{self,
                            SymKey},
                   os::{process::ShutdownTimeout,
                        signals},
                   service::HealthCheckInterval,
                   url::{bldr_url_from_env,
                         default_bldr_url},
                   ChannelIdent};
use habitat_launcher_client::{LauncherCli,
                              ERR_NO_RETRY_EXCODE};
use habitat_sup_protocol::{self as sup_proto,
                           ctl::ServiceBindList,
                           types::{BindingMode,
                                   ServiceBind,
                                   Topology,
                                   UpdateCondition,
                                   UpdateStrategy}};
use std::{env,
          io::{self,
               Write},
          net::{IpAddr,
                Ipv4Addr},
          path::{Path,
                 PathBuf},
          process,
          str::{self,
                FromStr}};
#[cfg(test)]
use tempfile::TempDir;
use tokio::{self,
            runtime::Builder as RuntimeBuilder};

/// Our output key
static LOGKEY: &str = "MN";

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

habitat_core::env_config_int!(/// Represents how many threads to start for our main Tokio runtime
                              #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
                              TokioThreadCount,
                              usize,
                              HAB_TOKIO_THREAD_COUNT,
                              // This is the same internal logic used in Tokio itself.
                              // https://docs.rs/tokio/0.1.12/src/tokio/runtime/builder.rs.html#68
                              num_cpus::get().max(1));

fn main() {
    // Set up signal handlers before anything else happens to ensure
    // that all threads spawned thereafter behave properly.
    signals::init();
    logger::init();

    let mut runtime =
        RuntimeBuilder::new().threaded_scheduler()
                             .core_threads(TokioThreadCount::configured_value().into())
                             .enable_all()
                             .build()
                             .expect("Couldn't build Tokio Runtime!");

    let mut ui = UI::default_with_env();
    let flags = FeatureFlag::from_env(&mut ui);

    let result = runtime.block_on(start_rsr_imlw_mlw_gsw_smw_rhw_msw(flags));
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

/// # Locking (see locking.md)
/// * `RumorStore::list` (read)
/// * `MemberList::initial_members` (write)
/// * `MemberList::entries` (write)
/// * `GatewayState::inner` (write)
/// * `Server::member` (write)
/// * `RumorHeat::inner` (write)
/// * `ManagerServices::inner` (write)
async fn start_rsr_imlw_mlw_gsw_smw_rhw_msw(feature_flags: FeatureFlag) -> Result<()> {
    if feature_flags.contains(FeatureFlag::TEST_BOOT_FAIL) {
        outputln!("Simulating boot failure");
        return Err(Error::TestBootFail);
    }
    liveliness_checker::spawn_thread_alive_checker();
    let launcher = boot();
    let app_matches = match cli(feature_flags).get_matches_safe() {
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
        ("bash", Some(_)) => sub_bash().await,
        ("run", Some(m)) => {
            let launcher = launcher.ok_or(Error::NoLauncher)?;
            sub_run_rsr_imlw_mlw_gsw_smw_rhw_msw(m, launcher, feature_flags).await
        }
        ("sh", Some(_)) => sub_sh().await,
        ("term", Some(_)) => sub_term(),
        _ => unreachable!(),
    }
}

async fn sub_bash() -> Result<()> { command::shell::bash().await }

/// # Locking (see locking.md)
/// * `RumorStore::list` (read)
/// * `MemberList::initial_members` (write)
/// * `MemberList::entries` (write)
/// * `GatewayState::inner` (write)
/// * `Server::member` (write)
/// * `RumorHeat::inner` (write)
/// * `ManagerServices::inner` (write)
async fn sub_run_rsr_imlw_mlw_gsw_smw_rhw_msw(m: &ArgMatches<'_>,
                                              launcher: LauncherCli,
                                              feature_flags: FeatureFlag)
                                              -> Result<()> {
    set_supervisor_logging_options(m);

    // TODO (DM): This check can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    if m.is_present("APPLICATION") || m.is_present("ENVIRONMENT") {
        ui().warn("--application and --environment flags are deprecated and ignored.")?;
    }

    let cfg = mgrcfg_from_sup_run_matches(m, feature_flags)?;

    let manager = Manager::load_imlw(cfg, launcher).await?;

    // We need to determine if we have an initial service to start
    let svc = if let Some(pkg) = m.value_of("PKG_IDENT_OR_ARTIFACT") {
        let mut msg = svc_load_from_input(m)?;
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
                                           .expect("svc_load_from_input to always set to Some")).await?;
                install.ident.into()
            }
            InstallSource::Ident(ident, _) => ident.into(),
        };
        msg.ident = Some(ident);
        Some(msg)
    } else {
        None
    };

    manager.run_rsw_imlw_mlw_gsw_smw_rhw_msw(svc).await
}

async fn sub_sh() -> Result<()> { command::shell::sh().await }

fn sub_term() -> Result<()> {
    // We were generating a ManagerConfig from matches here, but 'hab sup term' takes no options.
    // This means that we were implicitly getting the default ManagerConfig here. Instead of calling
    // a function to generate said config, we can just explicitly pass the default.
    let proc_lock_file = habitat_sup_protocol::sup_root(None).join(PROC_LOCK_FILE);
    match Manager::term(&proc_lock_file) {
        Err(Error::ProcessLockIO(..)) => {
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

    let event_stream_config = if m.value_of("EVENT_STREAM_URL").is_some() {
        Some(EventStreamConfig::from(m))
    } else {
        None
    };

    let gossip_peers =
        cli_util::socket_addrs_with_default_port(m.values_of("PEER").into_iter().flatten(),
                                                 GossipListenAddr::DEFAULT_PORT)?;

    #[rustfmt::skip]
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
        gossip_peers,
        watch_peer_file: m.value_of("PEER_WATCH_FILE").map(str::to_string),
        gossip_listen: if m.is_present("LOCAL_GOSSIP_MODE") {
            GossipListenAddr::local_only()
        } else {
            m.value_of("LISTEN_GOSSIP").and_then(|s| s.parse().ok()).unwrap_or_default()
        },
        ctl_listen: m.value_of("LISTEN_CTL").and_then(|s| s.parse().ok()).unwrap_or_default(),
        http_listen: m.value_of("LISTEN_HTTP").and_then(|s| s.parse().ok()).unwrap_or_default(),
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
        feature_flags,
        event_stream_config,
        keep_latest_packages: m.value_of("NUM_LATEST_PACKAGES_TO_KEEP").and_then(|s| s.parse().ok()),
        sys_ip: m.value_of("SYS_IP_ADDRESS")
            .and_then(|s| IpAddr::from_str(s).ok())
            .or_else(|| {
                let result_ip = habitat_core::util::sys::ip();
                if let Err(e) = &result_ip {
                    warn!("{}", e);
                }
                result_ip.ok()
            })
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)), 
    };

    info!("Using sys IP address {}", cfg.sys_ip);

    Ok(cfg)
}

// Various CLI Parsing Functions
////////////////////////////////////////////////////////////////////////

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
        Some(url) => url,
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

fn get_topology_from_input(m: &ArgMatches) -> Option<Topology> {
    m.value_of("TOPOLOGY")
     .and_then(|f| Topology::from_str(f).ok())
}

fn get_strategy_from_input(m: &ArgMatches) -> Option<UpdateStrategy> {
    m.value_of("STRATEGY")
     .and_then(|f| UpdateStrategy::from_str(f).ok())
}

fn get_update_condition_from_input(m: &ArgMatches<'_>) -> Option<UpdateCondition> {
    m.value_of("UPDATE_CONDITION")
     .and_then(|f| UpdateCondition::from_str(f).ok())
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
        Some((*config_from).to_string())
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
    if m.is_present("JSON_LOGGING") {
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
fn svc_load_from_input(m: &ArgMatches) -> Result<sup_proto::ctl::SvcLoad> {
    let mut msg = sup_proto::ctl::SvcLoad::default();
    msg.bldr_url = Some(bldr_url(m));
    msg.bldr_channel = Some(channel(m).to_string());
    msg.binds = get_binds_from_input(m)?;
    msg.config_from = get_config_from_input(m);
    if m.is_present("FORCE") {
        msg.force = Some(true);
    }
    msg.group = get_group_from_input(m);
    msg.svc_encrypted_password = get_password_from_input(m)?;
    msg.health_check_interval = get_health_check_interval_from_input(m);
    msg.binding_mode = get_binding_mode_from_input(m).map(|v| v as i32);
    msg.topology = get_topology_from_input(m).map(|v| v as i32);
    msg.update_strategy = get_strategy_from_input(m).map(|v| v as i32);
    msg.update_condition = get_update_condition_from_input(m).map(|v| v as i32);
    msg.shutdown_timeout =
        parse_optional_arg::<ShutdownTimeout>("SHUTDOWN_TIMEOUT", m).map(u32::from);
    Ok(msg)
}

fn get_health_check_interval_from_input(m: &ArgMatches<'_>)
                                        -> Option<sup_proto::types::HealthCheckInterval> {
    // Value will have already been validated by `cli::valid_health_check_interval`
    m.value_of("HEALTH_CHECK_INTERVAL")
     .and_then(|s| HealthCheckInterval::from_str(s).ok())
     .map(HealthCheckInterval::into)
}

#[cfg(test)]
mod test {
    use super::*;
    use habitat_common::types::{GossipListenAddr,
                                HttpListenAddr,
                                ListenCtlAddr};
    use habitat_core::locked_env_var;
    use std::net::{SocketAddr,
                   ToSocketAddrs};

    fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

    mod tokio_thread_count {
        use super::*;
        use habitat_core::locked_env_var;

        locked_env_var!(HAB_TOKIO_THREAD_COUNT, lock_thread_count);

        #[test]
        fn default_is_number_of_cpus() {
            let tc = lock_thread_count();
            tc.unset();

            assert_eq!(TokioThreadCount::configured_value().0, num_cpus::get());
        }

        #[test]
        fn can_be_overridden_by_env_var() {
            let tc = lock_thread_count();
            tc.set("128");
            assert_eq!(TokioThreadCount::configured_value().0, 128);
        }
    }

    mod manager_config {

        use super::*;
        use habitat_common::types::EventStreamConnectMethod;
        use habitat_core::fs::CACHE_KEY_PATH;
        use std::{collections::HashMap,
                  fs::File,
                  iter::FromIterator};

        locked_env_var!(HAB_CACHE_KEY_PATH, lock_var);

        fn cmd_vec_from_cmd_str(cmd: &str) -> Vec<&str> { Vec::from_iter(cmd.split_whitespace()) }

        fn matches_from_cmd_vec(cmd_vec: Vec<&str>) -> ArgMatches {
            let matches = cli(no_feature_flags()).get_matches_from_safe(cmd_vec)
                                                 .expect("Error while getting matches");
            matches.subcommand_matches("run")
                   .expect("Error getting sub command matches")
                   .clone()
        }

        fn matches_from_cmd_str(cmd: &str) -> ArgMatches {
            let cmd_vec = cmd_vec_from_cmd_str(cmd);
            matches_from_cmd_vec(cmd_vec)
        }

        fn config_from_cmd_vec(cmd_vec: Vec<&str>) -> ManagerConfig {
            let matches = matches_from_cmd_vec(cmd_vec);
            mgrcfg_from_sup_run_matches(&matches, no_feature_flags()).expect("Could not get \
                                                                              ManagerConfig")
        }

        fn config_from_cmd_str(cmd: &str) -> ManagerConfig {
            let cmd_vec = cmd_vec_from_cmd_str(cmd);
            config_from_cmd_vec(cmd_vec)
        }

        fn service_load_from_cmd_str(cmd: &str) -> sup_proto::ctl::SvcLoad {
            let matches = matches_from_cmd_str(cmd);
            svc_load_from_input(&matches).expect("Could not get SvcLoad")
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
                HttpListenAddr::from_str("2.2.2.2:2222").expect("Could not create http listen \
                                                                 addr");
            assert_eq!(config.http_listen, expected_addr);
        }

        #[test]
        fn http_listen_is_set_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            let expected_addr = HttpListenAddr::default();
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
                                                         format!("{}:{}",
                                                                 peer,
                                                                 GossipListenAddr::DEFAULT_PORT)
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

        #[test]
        fn test_hab_sup_run_no_args() {
            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(ManagerConfig { auto_update:          false,
                                       custom_state_path:    None,
                                       cache_key_path:       (&*CACHE_KEY_PATH).to_path_buf(),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:       ChannelIdent::default(),
                                       gossip_listen:        GossipListenAddr::default(),
                                       ctl_listen:           ListenCtlAddr::default(),
                                       http_listen:          HttpListenAddr::default(),
                                       http_disable:         false,
                                       gossip_peers:         vec![],
                                       gossip_permanent:     false,
                                       ring_key:             None,
                                       organization:         None,
                                       watch_peer_file:      None,
                                       tls_config:           None,
                                       feature_flags:        FeatureFlag::empty(),
                                       event_stream_config:  None,
                                       keep_latest_packages: None,
                                       sys_ip:               habitat_core::util::sys::ip().unwrap(), },
                       config);

            let service_load = service_load_from_cmd_str("hab-sup run");
            assert_eq!(sup_proto::ctl::SvcLoad { ident:                   None,
                                                 application_environment: None,
                                                 binds:                   None,
                                                 specified_binds:         None,
                                                 binding_mode:            None,
                                                 bldr_url:
                                                     Some(String::from("https://bldr.habitat.sh")),
                                                 bldr_channel:
                                                     Some(String::from("stable")),
                                                 config_from:             None,
                                                 force:                   None,
                                                 group:                   None,
                                                 svc_encrypted_password:  None,
                                                 topology:                None,
                                                 update_strategy:         None,
                                                 health_check_interval:   None,
                                                 shutdown_timeout:        None,
                                                 update_condition:        Some(0), },
                       service_load);
        }

        #[test]
        fn test_hab_sup_run_with_args() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let temp_dir_str = temp_dir.path().to_str().unwrap();

            // Setup key file
            let key_content =
                "SYM-SEC-1\ntester-20160504220722\n\nRCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
            let (sym_key, _) = SymKey::write_file_from_str(key_content, temp_dir.path())
                                       .expect("Could not write key pair");

            // Setup cert files
            let key_path = temp_dir.path().join("key");
            let key_path_str = key_path.to_str().unwrap();
            File::create(&key_path).unwrap();
            let cert_path = temp_dir.path().join("cert");
            let cert_path_str = cert_path.to_str().unwrap();
            File::create(&cert_path).unwrap();
            let ca_cert_path = temp_dir.path().join("ca_cert");
            let ca_cert_path_str = ca_cert_path.to_str().unwrap();
            File::create(&ca_cert_path).unwrap();

            let args = format!("hab-sup run --listen-gossip=1.2.3.4:4321 \
                                --listen-http=5.5.5.5:11111 --http-disable \
                                --listen-ctl=7.8.9.1:12 --org=MY_ORG --peer 1.1.1.1:1111 \
                                2.2.2.2:2222 3.3.3.3 --permanent-peer --ring tester \
                                --cache-key-path={} --auto-update --key={} --certs={} --ca-certs \
                                {} --keep-latest-packages=5 --sys-ip-address 7.8.9.0",
                               temp_dir_str, key_path_str, cert_path_str, ca_cert_path_str);

            let gossip_peers = vec!["1.1.1.1:1111".parse().unwrap(),
                                    "2.2.2.2:2222".parse().unwrap(),
                                    format!("3.3.3.3:{}", GossipListenAddr::DEFAULT_PORT).parse()
                                                                                         .unwrap()];

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update: true,
                                       custom_state_path: None,
                                       cache_key_path: PathBuf::from(temp_dir_str),
                                       update_url: String::from("https://bldr.habitat.sh"),
                                       update_channel: ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("1.2.3.4:4321").unwrap(),
                                       ctl_listen:
                                           ListenCtlAddr::from_str("7.8.9.1:12").unwrap(),
                                       http_listen:
                                           HttpListenAddr::from_str("5.5.5.5:11111").unwrap(),
                                       http_disable: true,
                                       gossip_peers,
                                       gossip_permanent: true,
                                       ring_key: Some(sym_key),
                                       organization: Some(String::from("MY_ORG")),
                                       watch_peer_file: None,
                                       tls_config: Some(TLSConfig { cert_path,
                                                                    key_path,
                                                                    ca_cert_path:
                                                                        Some(ca_cert_path) }),
                                       feature_flags: FeatureFlag::empty(),
                                       event_stream_config: None,
                                       keep_latest_packages: Some(5),
                                       sys_ip: "7.8.9.0".parse().unwrap() },
                       config);

            let service_load = service_load_from_cmd_str(&args);
            assert_eq!(sup_proto::ctl::SvcLoad { ident:                   None,
                                                 application_environment: None,
                                                 binds:                   None,
                                                 specified_binds:         None,
                                                 binding_mode:            None,
                                                 bldr_url:
                                                     Some(String::from("https://bldr.habitat.sh")),
                                                 bldr_channel:
                                                     Some(String::from("stable")),
                                                 config_from:             None,
                                                 force:                   None,
                                                 group:                   None,
                                                 svc_encrypted_password:  None,
                                                 topology:                None,
                                                 update_strategy:         None,
                                                 health_check_interval:   None,
                                                 shutdown_timeout:        None,
                                                 update_condition:        Some(0), },
                       service_load);
        }

        #[test]
        fn test_hab_sup_run_with_args2() {
            let args = "hab-sup run --local-gossip-mode";

            let config = config_from_cmd_str(args);
            assert_eq!(ManagerConfig { auto_update:          false,
                                       custom_state_path:    None,
                                       cache_key_path:       (&*CACHE_KEY_PATH).to_path_buf(),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:       ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("127.0.0.2:9638").unwrap(),
                                       ctl_listen:           ListenCtlAddr::default(),
                                       http_listen:          HttpListenAddr::default(),
                                       http_disable:         false,
                                       gossip_peers:         vec![],
                                       gossip_permanent:     false,
                                       ring_key:             None,
                                       organization:         None,
                                       watch_peer_file:      None,
                                       tls_config:           None,
                                       feature_flags:        FeatureFlag::empty(),
                                       event_stream_config:  None,
                                       keep_latest_packages: None,
                                       sys_ip:               habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_with_args3() {
            let args = "hab-sup run --peer-watch-file=/some/path";

            let config = config_from_cmd_str(args);
            assert_eq!(ManagerConfig { auto_update:          false,
                                       custom_state_path:    None,
                                       cache_key_path:       (&*CACHE_KEY_PATH).to_path_buf(),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:       ChannelIdent::default(),
                                       gossip_listen:        GossipListenAddr::default(),
                                       ctl_listen:           ListenCtlAddr::default(),
                                       http_listen:          HttpListenAddr::default(),
                                       http_disable:         false,
                                       gossip_peers:         vec![],
                                       gossip_permanent:     false,
                                       ring_key:             None,
                                       organization:         None,
                                       watch_peer_file:      Some(String::from("/some/path")),
                                       tls_config:           None,
                                       feature_flags:        FeatureFlag::empty(),
                                       event_stream_config:  None,
                                       keep_latest_packages: None,
                                       sys_ip:               habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_logging_args() {
            let args = "hab-sup run";
            let m = matches_from_cmd_str(args);
            assert!(!m.is_present("VERBOSE"));
            assert!(!m.is_present("NO_COLOR"));
            assert!(!m.is_present("JSON_LOGGING"));

            let args = "hab-sup run -v --no-color --json-logging";
            let m = matches_from_cmd_str(args);
            assert!(m.is_present("VERBOSE"));
            assert!(m.is_present("NO_COLOR"));
            assert!(m.is_present("JSON_LOGGING"));
        }

        #[test]
        fn test_hab_sup_run_event_stream_args() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");

            // Setup cert files
            let certificate_path = temp_dir.path().join("certificate.pem");
            let certificate_path_str = certificate_path.to_str().unwrap();
            let mut file = File::create(&certificate_path).unwrap();
            file.write_all(r#"-----BEGIN CERTIFICATE-----
MIIDPTCCAiWgAwIBAgIJAJCSLX9jr5W7MA0GCSqGSIb3DQEBBQUAMHAxCzAJBgNV
BAYTAlVTMQswCQYDVQQIDAJDQTEQMA4GA1UECgwHU3luYWRpYTEQMA4GA1UECwwH
bmF0cy5pbzESMBAGA1UEAwwJbG9jYWxob3N0MRwwGgYJKoZIhvcNAQkBFg1kZXJl
a0BuYXRzLmlvMB4XDTE5MTAxNzEzNTcyNloXDTI5MTAxNDEzNTcyNlowDTELMAkG
A1UEBhMCVVMwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDm+0dlzcmi
La+LzdVqeVQ8B1/rWnErK+VvvjH7FmVodg5Z5+RXyojpd9ZBrVd6QrLSVMQPfFvB
vGGX4yI6Ph5KXUefa31vNOOMhp2FGSmaEVhETKGQ0xRh4VfaAerOP5Cunl0TbSyJ
yjkVa7aeMtcqTEiFL7Ae2EtiMhTrMrYpBDQ8rzm2i1IyTb9DX5v7DUOmrSynQSlV
yXCztRVGNL/kHlItpEku1SHt/AD3ogu8EgqQZFB8xRRw9fubYgh4Q0kx80e4k9Qt
TKncF3B2NGb/ZcE5Z+mmHIBq8J2zKMijOrdd3m5TbQmzDbETEOjs4L1eoZRLcL/c
vYu5gmXdr4F7AgMBAAGjPTA7MBoGA1UdEQQTMBGCCWxvY2FsaG9zdIcEfwAAATAd
BgNVHSUEFjAUBggrBgEFBQcDAgYIKwYBBQUHAwEwDQYJKoZIhvcNAQEFBQADggEB
ADQYaEjWlOb9YzUnFGjfDC06dRZjRmK8TW/4GiDHIDk5TyZ1ROtskvyhVyTZJ5Vs
qXOKJwpps0jK2edtrvZ7xIGw+Y41oPgYYhr5TK2c+oi2UOHG4BXqRbuwz/5cU+nM
ZWOG1OrHBCbrMSeFsn7rzETnd8SZnw6ZE7LI62WstdoCY0lvNfjNv3kY/6hpPm+9
0bVzurZ28pdJ6YEJYgbPcOvxSzGDXTw9LaKEmqknTsrBKI2qm+myVTbRTimojYTo
rw/xjHESAue/HkpOwWnFTOiTT+V4hZnDXygiSy+LWKP4zLnYOtsn0lN9OmD0z+aa
gpoVMSncu2jMIDZX63IkQII=
-----END CERTIFICATE-----
"#.as_bytes())
                .unwrap();

            let args = format!("hab-sup run --event-stream-application=MY_APP \
                                --event-stream-environment=MY_ENV \
                                --event-stream-connect-timeout=5 --event-stream-url \
                                127.0.0.1:3456 --event-stream-site my_site --event-stream-token \
                                some_token --event-meta key1=val1 key2=val2 keyA=valA \
                                --event-stream-server-certificate={}",
                               certificate_path_str);

            let config = config_from_cmd_str(&args);
            let mut meta = HashMap::new();
            meta.insert(String::from("key1"), String::from("val1"));
            meta.insert(String::from("key2"), String::from("val2"));
            meta.insert(String::from("keyA"), String::from("valA"));
            assert_eq!(ManagerConfig { auto_update:          false,
                                       custom_state_path:    None,
                                       cache_key_path:       (&*CACHE_KEY_PATH).to_path_buf(),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:       ChannelIdent::default(),
                                       gossip_listen:        GossipListenAddr::default(),
                                       ctl_listen:           ListenCtlAddr::default(),
                                       http_listen:          HttpListenAddr::default(),
                                       http_disable:         false,
                                       gossip_peers:         vec![],
                                       gossip_permanent:     false,
                                       ring_key:             None,
                                       organization:         None,
                                       watch_peer_file:      None,
                                       tls_config:           None,
                                       feature_flags:        FeatureFlag::empty(),
                                       event_stream_config:  Some(EventStreamConfig {
                                        environment: String::from("MY_ENV"),
                                        application: String::from("MY_APP"),
                                        site: Some(String::from("my_site")),
                                        meta: meta.into(),
                                        token: "some_token".parse().unwrap(),
                                        url: "127.0.0.1:3456".parse().unwrap(),
                                        connect_method: EventStreamConnectMethod::Timeout {secs: 5},
                                        server_certificate: Some(certificate_path_str.parse().unwrap()),
                                       }),
                                       keep_latest_packages: None,
                                       sys_ip:               habitat_core::util::sys::ip().unwrap(), },
                       config,);
        }

        #[test]
        fn test_hab_sup_run_svc_args() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let temp_dir_str = temp_dir.path().to_str().unwrap();

            let args = format!("hab-sup run --channel my_channel --bind one:service1.default \
                                two:service2.default --binding-mode strict --url http://my_url.com \
                                --config-from={} --group MyGroup --topology standalone \
                                --strategy at-once --update-condition track-channel --health-check-interval 17 \
                                --shutdown-timeout=12 core/redis",
                               temp_dir_str);

            let mut binds = ServiceBindList::default();
            binds.binds
                 .push(ServiceBind::from_str("one:service1.default").unwrap());
            binds.binds
                 .push(ServiceBind::from_str("two:service2.default").unwrap());
            let health_check_interval = sup_proto::types::HealthCheckInterval { seconds: 17 };

            let service_load = service_load_from_cmd_str(&args);
            assert_eq!(sup_proto::ctl::SvcLoad { ident:                   None,
                                                 application_environment: None,
                                                 binds:                   Some(binds),
                                                 specified_binds:         None,
                                                 binding_mode:            Some(1),
                                                 bldr_url:
                                                     Some(String::from("http://my_url.com")),
                                                 bldr_channel:
                                                     Some(String::from("my_channel")),
                                                 config_from:
                                                     Some(String::from(temp_dir_str)),
                                                 force:                   None,
                                                 group:
                                                     Some(String::from("MyGroup")),
                                                 svc_encrypted_password:  None,
                                                 topology:                Some(0),
                                                 update_strategy:         Some(1),
                                                 health_check_interval:
                                                     Some(health_check_interval),
                                                 shutdown_timeout:        Some(12),
                                                 update_condition:        Some(1), },
                       service_load);
        }

        #[test]
        fn test_hab_sup_run_svc_pkg_ident() {
            let args = "hab-sup run core/redis";
            let m = matches_from_cmd_str(args);
            let pkg = m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();
            assert_eq!("core/redis".parse::<InstallSource>().unwrap(),
                       pkg.parse::<InstallSource>().unwrap());

            let args = "hab-sup run core/redis/4.0.14/20200421191514";
            let m = matches_from_cmd_str(args);
            let pkg = m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();
            assert_eq!("core/redis/4.0.14/20200421191514".parse::<InstallSource>()
                                                         .unwrap(),
                       pkg.parse::<InstallSource>().unwrap());

            let args = "hab-sup run /some/path/pkg.hrt";
            let m = matches_from_cmd_str(args);
            let pkg = m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();
            assert_eq!("/some/path/pkg.hrt".parse::<InstallSource>().unwrap(),
                       pkg.parse::<InstallSource>().unwrap());
        }
    }
}
