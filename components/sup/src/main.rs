#[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
use crate::sup::command;
use crate::sup::{cli::cli,
                 error::{Error,
                         Result},
                 event::EventStreamConfig,
                 logger,
                 manager::{Manager,
                           ManagerConfig,
                           TLSConfig},
                 util};
use configopt::ConfigOpt;
use hab::cli::hab::{sup::SupRun,
                    svc};
use habitat_common::{command::package::install::InstallSource,
                     liveliness_checker,
                     output::{self,
                              OutputFormat,
                              OutputVerbosity},
                     outputln,
                     types::GossipListenAddr,
                     ui::{self,
                          UI},
                     FeatureFlag};
use habitat_core::{self,
                   crypto::{self,
                            keys::{KeyCache,
                                   RingKey}},
                   os::signals,
                   tls::rustls_wrapper::{CertificateChainCli,
                                         RootCertificateStoreCli}};
use habitat_launcher_client::{LauncherCli,
                              ERR_NO_RETRY_EXCODE,
                              OK_NO_RETRY_EXCODE};
use habitat_sup as sup;
use habitat_sup_protocol::{self as sup_proto};
use log::{debug,
          error,
          info,
          warn};
use std::{convert::TryInto,
          env,
          io,
          io::Write,
          net::{IpAddr,
                Ipv4Addr},
          process,
          str::{self}};
use sup::manager::ServiceRestartConfig;
use tokio::{self,
            runtime::Builder as RuntimeBuilder};

/// Our output key
static LOGKEY: &str = "MN";

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

    let runtime = RuntimeBuilder::new_multi_thread()
        .worker_threads(TokioThreadCount::configured_value().into())
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
    if crypto::init().is_err() {
        error!("Failed to initialization libsodium, make sure it is available in your runtime \
                environment");
        process::exit(1);
    }
    match habitat_launcher_client::env_pipe() {
        Some(pipe) => {
            match LauncherCli::connect(pipe) {
                Ok(launcher) => Some(launcher),
                Err(err) => {
                    error!("Failed to connect to launcher: {:?}",
                           anyhow::Error::new(err));
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
        #[cfg(any(all(target_os = "linux",
                      any(target_arch = "x86_64", target_arch = "aarch64")),
                  all(target_os = "windows", target_arch = "x86_64"),))]
        ("bash", Some(_)) => sub_bash().await,
        ("run", Some(_)) => {
            // TODO (DM): This is a little hacky. Essentially, for `hab sup run` we switch to using
            // structopt/configopt instead of querying clap `ArgMatches` directly. We skip the first
            // arg ("sup") to construct a `SupRun`. Eventually, when we switch to exclusivly using
            // structopt/configopt this will go away and everything will be much cleaner.
            let sup_run = match SupRun::try_from_iter_with_configopt(env::args().skip(1)) {
                Ok(sup) => sup,
                Err(err) => {
                    if launcher.is_some() {
                        let exit_code = if err.use_stderr() {
                            ERR_NO_RETRY_EXCODE
                        } else {
                            OK_NO_RETRY_EXCODE
                        };
                        err.exit_with_codes(exit_code, exit_code);
                    } else {
                        err.exit();
                    }
                }
            };
            let launcher = launcher.ok_or(Error::NoLauncher)?;
            sub_run_rsr_imlw_mlw_gsw_smw_rhw_msw(sup_run, launcher, feature_flags).await
        }
        #[cfg(any(all(target_os = "linux",
                      any(target_arch = "x86_64", target_arch = "aarch64")),
                  all(target_os = "windows", target_arch = "x86_64"),))]
        ("sh", Some(_)) => sub_sh().await,
        ("term", Some(_)) => sub_term(),
        _ => unreachable!(),
    }
}

#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64"),))]
async fn sub_bash() -> Result<()> { command::shell::bash().await }

/// # Locking (see locking.md)
/// * `RumorStore::list` (read)
/// * `MemberList::initial_members` (write)
/// * `MemberList::entries` (write)
/// * `GatewayState::inner` (write)
/// * `Server::member` (write)
/// * `RumorHeat::inner` (write)
/// * `ManagerServices::inner` (write)
async fn sub_run_rsr_imlw_mlw_gsw_smw_rhw_msw(sup_run: SupRun,
                                              launcher: LauncherCli,
                                              feature_flags: FeatureFlag)
                                              -> Result<()> {
    set_supervisor_logging_options(&sup_run);

    let mut svc_load_msgs = if feature_flags.contains(FeatureFlag::SERVICE_CONFIG_FILES) {
        svc::svc_loads_from_paths(&sup_run.svc_config_paths)?.into_iter()
                                                             .map(|svc_load| {
                                                                 Ok(svc_load.try_into()?)
                                                             })
                                                             .collect::<Result<Vec<_>>>()?
    } else {
        vec![]
    };

    let (manager_cfg, maybe_svc_load_msg) = split_apart_sup_run(sup_run, feature_flags).await?;
    if let Some(svc_load_msg) = maybe_svc_load_msg {
        svc_load_msgs.push(svc_load_msg);
    }
    let manager = Manager::load_imlw(manager_cfg, launcher).await?;
    manager.run_rsw_imlw_mlw_gsw_smw_rhw_msw(svc_load_msgs)
           .await
}

#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64"),))]
async fn sub_sh() -> Result<()> { command::shell::sh().await }

fn sub_term() -> Result<()> {
    match Manager::term() {
        Err(e @ Error::LockFileError(..)) => {
            println!("Supervisor not terminated: {}", e);
            Ok(())
        }
        result => result,
    }
}

// Internal Implementation Details
////////////////////////////////////////////////////////////////////////
async fn split_apart_sup_run(sup_run: SupRun,
                             feature_flags: FeatureFlag)
                             -> Result<(ManagerConfig, Option<sup_proto::ctl::SvcLoad>)> {
    let ring_key = get_ring_key(&sup_run)?;
    let shared_load = sup_run.shared_load;
    let event_stream_config = if sup_run.event_stream_url.is_some() {
        Some(EventStreamConfig { environment:
                                     sup_run.event_stream_environment
                                            .expect("Required option for EventStream feature"),
                                 application:
                                     sup_run.event_stream_application
                                            .expect("Required option for EventStream feature"),
                                 site:               sup_run.event_stream_site,
                                 meta:               sup_run.event_meta.into(),
                                 token:
                                     sup_run.event_stream_token
                                            .expect("Required option for EventStream feature"),
                                 url:
                                     sup_run.event_stream_url
                                            .expect("Required option for EventStream feature")
                                            .into(),
                                 connect_method:     sup_run.event_stream_connect_timeout,
                                 server_certificate: sup_run.event_stream_server_certificate, })
    } else {
        None
    };

    let tls_config = if let Some(key_file) = sup_run.key_file {
        let cert_path =
            sup_run.cert_file
                   .expect("`cert_file` should always have a value if `key_file` has a value.");
        Some(TLSConfig { key_path: key_file,
                         cert_path,
                         ca_cert_path: sup_run.ca_cert_file })
    } else {
        None
    };

    let bldr_url = habitat_core::url::bldr_url(shared_load.bldr_url.as_ref());

    let key_cache = KeyCache::new(sup_run.cache_key_path.cache_key_path);
    key_cache.setup()?;
    let cfg =
        ManagerConfig { auto_update: sup_run.auto_update,
                        auto_update_period: sup_run.auto_update_period.into(),
                        service_update_period: sup_run.service_update_period.into(),
                        service_restart_config:
                            ServiceRestartConfig::new(sup_run.service_min_backoff_period.into(),
                                                      sup_run.service_max_backoff_period.into(),
                                                      sup_run.service_restart_cooldown_period
                                                             .into()),
                        custom_state_path: None, // remove entirely?
                        key_cache,
                        update_url: bldr_url.clone(),
                        update_channel: shared_load.channel.clone(),
                        http_disable: sup_run.http_disable,
                        organization: sup_run.organization,
                        gossip_permanent: sup_run.permanent_peer,
                        ring_key,
                        gossip_peers: sup_run.peer.iter().map(Into::into).collect(),
                        watch_peer_file: sup_run.peer_watch_file
                                                .map(|p| p.to_string_lossy().to_string()),
                        gossip_listen: if sup_run.local_gossip_mode {
                            GossipListenAddr::local_only()
                        } else {
                            sup_run.listen_gossip
                        },
                        ctl_listen: sup_run.listen_ctl.into(),
                        ctl_server_certificates: sup_run.ctl_server_certificate
                                                        .map(CertificateChainCli::into_inner),
                        ctl_server_key: sup_run.ctl_server_key
                                               .map(|key| key.into_inner().into()),
                        ctl_client_ca_certificates:
                            sup_run.ctl_client_ca_certificate
                                   .map(RootCertificateStoreCli::into_inner),
                        http_listen: sup_run.listen_http,
                        tls_config,
                        feature_flags,
                        event_stream_config,
                        keep_latest_packages: sup_run.keep_latest_packages,
                        sys_ip: sup_run.sys_ip_address
                                       .or_else(|| {
                                           let result_ip = habitat_core::util::sys::ip();
                                           if let Err(e) = &result_ip {
                                               warn!("{}", e);
                                           }
                                           result_ip.ok()
                                       })
                                       .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)) };

    info!("Using sys IP address {}", cfg.sys_ip);

    // Do we have an initial service to start?
    let maybe_svc_load_msg = if let Some(install_source) = sup_run.pkg_ident_or_artifact {
        let ident = match install_source {
            source @ InstallSource::Archive(_) => {
                // Install the archive manually then explicitly set the pkg ident to the version
                // found in the archive. This will lock the software to this specific version.
                let install = util::pkg::install(&mut ui::ui(),
                                                 &bldr_url,
                                                 &source,
                                                 &shared_load.channel).await?;
                install.ident
            }
            InstallSource::Ident(ident, _) => ident,
        };
        // Always force - running with a package ident is a "do what I mean" operation. You don't
        // care if a service was loaded previously or not and with what options. You want one loaded
        // right now and in this way.
        Some(svc::shared_load_cli_to_ctl(ident, shared_load, true)?)
    } else {
        None
    };

    Ok((cfg, maybe_svc_load_msg))
}

// Various CLI Parsing Functions
////////////////////////////////////////////////////////////////////////

fn get_ring_key(sup_run: &SupRun) -> Result<Option<RingKey>> {
    let cache_key_path = &sup_run.cache_key_path.cache_key_path;
    let cache = KeyCache::new(cache_key_path);
    cache.setup()?;

    match &sup_run.ring {
        Some(key_name) => {
            let key = cache.latest_ring_key_revision(key_name)?;
            Ok(Some(key))
        }
        None => {
            match &sup_run.ring_key {
                Some(key_content) => {
                    let key: RingKey = key_content.parse()?;
                    cache.write_key(&key)?;
                    Ok(Some(key))
                }
                None => Ok(None),
            }
        }
    }
}

// ServiceSpec Modification Functions
////////////////////////////////////////////////////////////////////////

fn set_supervisor_logging_options(sup_run: &SupRun) {
    if sup_run.verbose {
        output::set_verbosity(OutputVerbosity::Verbose);
    }
    if sup_run.no_color {
        output::set_format(OutputFormat::NoColor)
    }
    if sup_run.json_logging {
        output::set_format(OutputFormat::Json)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hab::cli::hab::sup::Sup;
    use habitat_common::types::{GossipListenAddr,
                                HttpListenAddr,
                                ListenCtlAddr};
    use habitat_core::{fs::CACHE_KEY_PATH,
                       locked_env_var};
    use habitat_sup_protocol::{ctl::ServiceBindList,
                               types::{BindingMode,
                                       ServiceBind,
                                       Topology,
                                       UpdateCondition,
                                       UpdateStrategy}};
    use std::net::{SocketAddr,
                   ToSocketAddrs};
    use tempfile::TempDir;

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
        use configopt::ConfigOpt;
        use futures::executor;
        use habitat_common::types::EventStreamConnectMethod;
        #[cfg(windows)]
        use habitat_core::crypto::dpapi::decrypt;
        use habitat_core::{crypto::keys::{Key,
                                          NamedRevision},
                           package::PackageIdent,
                           ChannelIdent};
        use std::{collections::HashMap,
                  fs::File,
                  io::Write,
                  str::FromStr,
                  time::Duration};
        use sup::manager::ServiceRestartConfig;

        locked_env_var!(HAB_CACHE_KEY_PATH, lock_var);

        fn cmd_vec_from_cmd_str(cmd: &str) -> Vec<&str> { cmd.split_whitespace().collect() }

        fn sup_run_from_cmd_vec(cmd_vec: Vec<&str>) -> SupRun {
            let sup =
                Sup::try_from_iter_with_configopt(cmd_vec).expect("Error while getting sup_run");
            match sup {
                Sup::Run(sup_run) => sup_run,
                _ => panic!("Error getting sub command sup run"),
            }
        }

        fn sup_run_from_cmd_str(cmd: &str) -> SupRun {
            let cmd_vec = cmd_vec_from_cmd_str(cmd);
            sup_run_from_cmd_vec(cmd_vec)
        }

        fn config_from_cmd_vec(cmd_vec: Vec<&str>) -> ManagerConfig {
            let sup_run = sup_run_from_cmd_vec(cmd_vec);
            executor::block_on(split_apart_sup_run(sup_run, no_feature_flags()))
                .expect(
                    "Could not get split apart \
                                                                     SupRun",
                )
                .0
        }

        fn config_from_cmd_str(cmd: &str) -> ManagerConfig {
            let cmd_vec = cmd_vec_from_cmd_str(cmd);
            config_from_cmd_vec(cmd_vec)
        }

        fn maybe_service_load_from_cmd_str(cmd: &str) -> Option<sup_proto::ctl::SvcLoad> {
            let sup_run = sup_run_from_cmd_str(cmd);
            executor::block_on(split_apart_sup_run(sup_run, no_feature_flags()))
                .expect(
                    "Could not get split apart \
                                                                     SupRun",
                )
                .1
        }

        fn service_load_from_cmd_str(cmd: &str) -> sup_proto::ctl::SvcLoad {
            maybe_service_load_from_cmd_str(cmd).expect("input that contained `SvcLoad` data")
        }

        #[test]
        fn auto_update_should_be_set() {
            let config = config_from_cmd_str("hab-sup run --auto-update");
            assert!(config.auto_update);

            let config = config_from_cmd_str("hab-sup run");
            assert!(!config.auto_update);
        }

        #[test]
        fn update_url_should_be_set() {
            let config = config_from_cmd_str("hab-sup run -u http://fake.example.url");
            assert_eq!(config.update_url, "http://fake.example.url/");
        }

        #[test]
        fn update_url_is_set_to_default_when_not_specified() {
            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(config.update_url, habitat_core::url::default_bldr_url());
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
            assert!(config.http_disable);

            let config = config_from_cmd_str("hab-sup run");
            assert!(!config.http_disable);
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
            assert!(config.gossip_permanent);

            let config = config_from_cmd_str("hab-sup run");
            assert!(!config.gossip_permanent);
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
            let temp_dir = TempDir::new().expect("Could not create tempdir");

            let cache = KeyCache::new(temp_dir.path());
            let lock = lock_var();
            lock.set(temp_dir.path());

            let key_content =
                "SYM-SEC-1\nfoobar-20160504220722\n\nRCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
            let key: RingKey = key_content.parse().unwrap();
            cache.write_key(&key).unwrap();

            let config = config_from_cmd_str("hab-sup run --ring foobar");

            assert_eq!(config.ring_key
                             .expect("No ring key on manager config")
                             .named_revision(),
                       key.named_revision());
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
                             .named_revision(),
                       &"foobar-20160504220722".parse::<NamedRevision>().unwrap());
        }

        const CERT_FILE_CONTENTS: &str = r#"-----BEGIN CERTIFICATE-----
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
"#;

        #[test]
        fn test_hab_sup_run_empty() {
            let lock = lock_var();
            lock.unset();

            let config = config_from_cmd_str("hab-sup run");
            assert_eq!(ManagerConfig { auto_update:                false,
                                       auto_update_period:         Duration::from_secs(60),
                                       service_update_period:      Duration::from_secs(60),
                                       service_restart_config:     ServiceRestartConfig::default(),
                                       custom_state_path:          None,
                                       key_cache:                  KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:             ChannelIdent::default(),
                                       gossip_listen:              GossipListenAddr::default(),
                                       ctl_listen:                 ListenCtlAddr::default(),
                                       ctl_server_certificates:    None,
                                       ctl_server_key:             None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:                HttpListenAddr::default(),
                                       http_disable:               false,
                                       gossip_peers:               vec![],
                                       gossip_permanent:           false,
                                       ring_key:                   None,
                                       organization:               None,
                                       watch_peer_file:            None,
                                       tls_config:                 None,
                                       feature_flags:              FeatureFlag::empty(),
                                       event_stream_config:        None,
                                       keep_latest_packages:       None,
                                       sys_ip:
                                           habitat_core::util::sys::ip().unwrap(), },
                       config);

            let maybe_service_load = maybe_service_load_from_cmd_str("hab-sup run");
            assert!(maybe_service_load.is_none());
        }

        #[test]
        fn test_hab_sup_run_cli_1() {
            let lock = lock_var();
            lock.unset();

            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let temp_dir_str = temp_dir.path().to_str().unwrap();
            let cache = KeyCache::new(temp_dir.path());

            // Setup key file
            let key_content =
                "SYM-SEC-1\ntester-20160504220722\n\nRCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
            let ring_key: RingKey = key_content.parse().unwrap();
            cache.write_key(&ring_key).unwrap();

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
                                --cache-key-path={} --auto-update --auto-update-period 90 \
                                --service-update-period 30 --key={} --certs={} --ca-certs {} \
                                --keep-latest-packages=5 --sys-ip-address 7.8.9.0",
                               temp_dir_str, key_path_str, cert_path_str, ca_cert_path_str);

            let gossip_peers = vec!["1.1.1.1:1111".parse().unwrap(),
                                    "2.2.2.2:2222".parse().unwrap(),
                                    format!("3.3.3.3:{}", GossipListenAddr::DEFAULT_PORT).parse()
                                                                                         .unwrap(),];

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update: true,
                                       auto_update_period: Duration::from_secs(90),
                                       service_update_period: Duration::from_secs(30),
                                       service_restart_config: ServiceRestartConfig::default(),
                                       custom_state_path: None,
                                       key_cache: KeyCache::new(temp_dir_str),
                                       update_url: String::from("https://bldr.habitat.sh"),
                                       update_channel: ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("1.2.3.4:4321").unwrap(),
                                       ctl_listen:
                                           ListenCtlAddr::from_str("7.8.9.1:12").unwrap(),
                                       ctl_server_certificates: None,
                                       ctl_server_key: None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:
                                           HttpListenAddr::from_str("5.5.5.5:11111").unwrap(),
                                       http_disable: true,
                                       gossip_peers,
                                       gossip_permanent: true,
                                       ring_key: Some(ring_key),
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
        }

        #[test]
        fn test_hab_sup_run_cli_2() {
            let lock = lock_var();
            lock.unset();

            let args = "hab-sup run --local-gossip-mode";

            let config = config_from_cmd_str(args);
            assert_eq!(ManagerConfig { auto_update:                false,
                                       auto_update_period:         Duration::from_secs(60),
                                       service_update_period:      Duration::from_secs(60),
                                       service_restart_config:     ServiceRestartConfig::default(),
                                       custom_state_path:          None,
                                       key_cache:                  KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:             ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("127.0.0.2:9638").unwrap(),
                                       ctl_listen:                 ListenCtlAddr::default(),
                                       ctl_server_certificates:    None,
                                       ctl_server_key:             None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:                HttpListenAddr::default(),
                                       http_disable:               false,
                                       gossip_peers:               vec![],
                                       gossip_permanent:           false,
                                       ring_key:                   None,
                                       organization:               None,
                                       watch_peer_file:            None,
                                       tls_config:                 None,
                                       feature_flags:              FeatureFlag::empty(),
                                       event_stream_config:        None,
                                       keep_latest_packages:       None,
                                       sys_ip:
                                           habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_cli_3() {
            let lock = lock_var();
            lock.unset();

            let args = "hab-sup run --peer-watch-file=/some/path";

            let config = config_from_cmd_str(args);
            assert_eq!(ManagerConfig { auto_update:                false,
                                       auto_update_period:         Duration::from_secs(60),
                                       service_update_period:      Duration::from_secs(60),
                                       service_restart_config:     ServiceRestartConfig::default(),
                                       custom_state_path:          None,
                                       key_cache:                  KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:             ChannelIdent::default(),
                                       gossip_listen:              GossipListenAddr::default(),
                                       ctl_listen:                 ListenCtlAddr::default(),
                                       ctl_server_certificates:    None,
                                       ctl_server_key:             None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:                HttpListenAddr::default(),
                                       http_disable:               false,
                                       gossip_peers:               vec![],
                                       gossip_permanent:           false,
                                       ring_key:                   None,
                                       organization:               None,
                                       watch_peer_file:            Some(String::from("/some/path")),
                                       tls_config:                 None,
                                       feature_flags:              FeatureFlag::empty(),
                                       event_stream_config:        None,
                                       keep_latest_packages:       None,
                                       sys_ip:
                                           habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_cli_logging() {
            let args = "hab-sup run";
            let m = sup_run_from_cmd_str(args);
            assert!(!m.verbose);
            assert!(!m.no_color);
            assert!(!m.json_logging);

            let args = "hab-sup run -v --no-color --json-logging";
            let m = sup_run_from_cmd_str(args);
            assert!(m.verbose);
            assert!(m.no_color);
            assert!(m.json_logging);
        }

        #[test]
        fn test_hab_sup_run_cli_event_stream() {
            let lock = lock_var();
            lock.unset();

            let temp_dir = TempDir::new().expect("Could not create tempdir");

            // Setup cert files
            let certificate_path = temp_dir.path().join("certificate.pem");
            let certificate_path_str = certificate_path.to_str().unwrap();
            let mut file = File::create(&certificate_path).unwrap();
            file.write_all(CERT_FILE_CONTENTS.as_bytes()).unwrap();

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
            assert_eq!(
                ManagerConfig {
                    auto_update: false,
                    auto_update_period: Duration::from_secs(60),
                    service_update_period: Duration::from_secs(60),
                    service_restart_config: ServiceRestartConfig::default(),
                    custom_state_path: None,
                    key_cache: KeyCache::new(&*CACHE_KEY_PATH),
                    update_url: String::from("https://bldr.habitat.sh"),
                    update_channel: ChannelIdent::default(),
                    gossip_listen: GossipListenAddr::default(),
                    ctl_listen: ListenCtlAddr::default(),
                    ctl_server_certificates: None,
                    ctl_server_key: None,
                    ctl_client_ca_certificates: None,
                    http_listen: HttpListenAddr::default(),
                    http_disable: false,
                    gossip_peers: vec![],
                    gossip_permanent: false,
                    ring_key: None,
                    organization: None,
                    watch_peer_file: None,
                    tls_config: None,
                    feature_flags: FeatureFlag::empty(),
                    event_stream_config: Some(EventStreamConfig {
                        environment: String::from("MY_ENV"),
                        application: String::from("MY_APP"),
                        site: Some(String::from("my_site")),
                        meta: meta.into(),
                        token: "some_token".parse().unwrap(),
                        url: "127.0.0.1:3456".parse().unwrap(),
                        connect_method: EventStreamConnectMethod::Timeout { secs: 5 },
                        server_certificate: Some(certificate_path_str.parse().unwrap()),
                    }),
                    keep_latest_packages: None,
                    sys_ip: habitat_core::util::sys::ip().unwrap(),
                },
                config,
            );
        }

        #[test]
        fn test_hab_sup_run_cli_svc() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let temp_dir_str = temp_dir.path().to_str().unwrap();

            let args = format!("hab-sup run --channel my_channel --bind one:service1.default \
                                two:service2.default --binding-mode relaxed --url http://my_url.com \
                                --config-from={} --group MyGroup --topology leader \
                                --strategy rolling --update-condition track-channel --health-check-interval 17 \
                                --shutdown-timeout=12 core/redis",
                               temp_dir_str);

            let mut binds = ServiceBindList::default();
            binds.binds
                 .push(ServiceBind::from_str("one:service1.default").unwrap());
            binds.binds
                 .push(ServiceBind::from_str("two:service2.default").unwrap());
            let health_check_interval = sup_proto::types::HealthCheckInterval { seconds: 17 };

            let service_load = service_load_from_cmd_str(&args);
            assert_eq!(sup_proto::ctl::SvcLoad { ident:
                                                     Some("core/redis".parse::<PackageIdent>()
                                                                      .unwrap()
                                                                      .into()),
                                                 binds:                  Some(binds),
                                                 binding_mode:           Some(0),
                                                 bldr_url:
                                                     Some(String::from("http://my_url.com/")),
                                                 bldr_channel:
                                                     Some(String::from("my_channel")),
                                                 config_from:
                                                     Some(String::from(temp_dir_str)),
                                                 force:                  Some(true),
                                                 group:
                                                     Some(String::from("MyGroup")),
                                                 svc_encrypted_password: None,
                                                 topology:
                                                     Some(Topology::Leader.into()),
                                                 update_strategy:
                                                     Some(UpdateStrategy::Rolling.into()),
                                                 health_check_interval:
                                                     Some(health_check_interval),
                                                 shutdown_timeout:       Some(12),
                                                 update_condition:
                                                     Some(UpdateCondition::TrackChannel.into()), },
                       service_load);
        }

        #[test]
        fn test_hab_sup_run_cli_svc_pkg_ident_args() {
            let args = "hab-sup run core/redis";
            let m = sup_run_from_cmd_str(args);
            let pkg = m.pkg_ident_or_artifact.unwrap();
            assert_eq!("core/redis".parse::<InstallSource>().unwrap(), pkg);

            let args = "hab-sup run core/redis/4.0.14/20200421191514";
            let m = sup_run_from_cmd_str(args);
            let pkg = m.pkg_ident_or_artifact.unwrap();
            assert_eq!("core/redis/4.0.14/20200421191514".parse::<InstallSource>()
                                                         .unwrap(),
                       pkg);

            let args = "hab-sup run /some/path/pkg.hrt";
            let m = sup_run_from_cmd_str(args);
            let pkg = m.pkg_ident_or_artifact.unwrap();
            assert_eq!("/some/path/pkg.hrt".parse::<InstallSource>().unwrap(), pkg);
        }

        #[cfg(windows)]
        #[test]
        fn test_hab_sup_run_cli_password() {
            let args = "hab-sup run --password keep_it_secret_keep_it_safe core/redis";
            let service_load = service_load_from_cmd_str(args);
            assert_eq!(decrypt(&service_load.svc_encrypted_password.unwrap()).unwrap(),
                       "keep_it_secret_keep_it_safe");
        }

        #[test]
        fn test_hab_sup_run_config_file_1() {
            let lock = lock_var();
            lock.unset();

            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let temp_dir_str = temp_dir.path().to_str().unwrap();
            let cache = KeyCache::new(temp_dir.path());

            // Setup key file
            let key_content =
                "SYM-SEC-1\ntester-20160504220722\n\nRCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
            let ring_key: RingKey = key_content.parse().unwrap();
            cache.write_key(&ring_key).unwrap();

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

            // Setup config file
            let config_contents = format!(
                                          r#"
listen_gossip = "1.2.3.4:4321"
listen_http = "5.5.5.5:11111"
http_disable = true
listen_ctl = "7.8.9.1:12"
organization = "MY_ORG"
peer = ["1.1.1.1:1111", "2.2.2.2:2222", "3.3.3.3:9638"]
permanent_peer = true
ring = "tester"
cache_key_path = "{}"
auto_update = true
auto_update_period = 3600
service_update_period = 1_000
key_file = "{}"
cert_file = "{}"
ca_cert_file = "{}"
keep_latest_packages = 5
sys_ip_address = "7.8.9.0"
    "#,
                                          temp_dir_str.replace('\\', "/"),
                                          key_path_str.replace('\\', "/"),
                                          cert_path_str.replace('\\', "/"),
                                          ca_cert_path_str.replace('\\', "/")
            );
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let gossip_peers = vec!["1.1.1.1:1111".parse().unwrap(),
                                    "2.2.2.2:2222".parse().unwrap(),
                                    format!("3.3.3.3:{}", GossipListenAddr::DEFAULT_PORT).parse()
                                                                                         .unwrap(),];

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update: true,
                                       auto_update_period: Duration::from_secs(3600),
                                       service_update_period: Duration::from_secs(1_000),
                                       service_restart_config: ServiceRestartConfig::default(),
                                       custom_state_path: None,
                                       key_cache: KeyCache::new(temp_dir_str),
                                       update_url: String::from("https://bldr.habitat.sh"),
                                       update_channel: ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("1.2.3.4:4321").unwrap(),
                                       ctl_listen:
                                           ListenCtlAddr::from_str("7.8.9.1:12").unwrap(),
                                       ctl_server_certificates: None,
                                       ctl_server_key: None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:
                                           HttpListenAddr::from_str("5.5.5.5:11111").unwrap(),
                                       http_disable: true,
                                       gossip_peers,
                                       gossip_permanent: true,
                                       ring_key: Some(ring_key),
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
        }

        #[test]
        fn test_hab_sup_run_config_file_2() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");

            let lock = lock_var();
            lock.unset();

            // Setup config file
            let config_contents = r#"local_gossip_mode = true"#;
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update:                false,
                                       auto_update_period:         Duration::from_secs(60),
                                       service_update_period:      Duration::from_secs(60),
                                       service_restart_config:     ServiceRestartConfig::default(),
                                       custom_state_path:          None,
                                       key_cache:                  KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:             ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("127.0.0.2:9638").unwrap(),
                                       ctl_listen:                 ListenCtlAddr::default(),
                                       ctl_server_certificates:    None,
                                       ctl_server_key:             None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:                HttpListenAddr::default(),
                                       http_disable:               false,
                                       gossip_peers:               vec![],
                                       gossip_permanent:           false,
                                       ring_key:                   None,
                                       organization:               None,
                                       watch_peer_file:            None,
                                       tls_config:                 None,
                                       feature_flags:              FeatureFlag::empty(),
                                       event_stream_config:        None,
                                       keep_latest_packages:       None,
                                       sys_ip:
                                           habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_config_file_3() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");

            let lock = lock_var();
            lock.unset();

            // Setup config file
            let config_contents = r#"peer_watch_file = "/some/path""#;
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update:                false,
                                       auto_update_period:         Duration::from_secs(60),
                                       service_update_period:      Duration::from_secs(60),
                                       service_restart_config:     ServiceRestartConfig::default(),
                                       custom_state_path:          None,
                                       key_cache:                  KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:             ChannelIdent::default(),
                                       gossip_listen:              GossipListenAddr::default(),
                                       ctl_listen:                 ListenCtlAddr::default(),
                                       ctl_server_certificates:    None,
                                       ctl_server_key:             None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:                HttpListenAddr::default(),
                                       http_disable:               false,
                                       gossip_peers:               vec![],
                                       gossip_permanent:           false,
                                       ring_key:                   None,
                                       organization:               None,
                                       watch_peer_file:            Some(String::from("/some/path")),
                                       tls_config:                 None,
                                       feature_flags:              FeatureFlag::empty(),
                                       event_stream_config:        None,
                                       keep_latest_packages:       None,
                                       sys_ip:
                                           habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_config_file_peer() {
            let lock = lock_var();
            lock.unset();

            let temp_dir = TempDir::new().expect("Could not create tempdir");

            // Setup config file
            let config_contents = r#"
            peer = ["1.1.1.1", "localhost"]
            "#.to_string();
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let gossip_peers = vec!["1.1.1.1:9638".parse().unwrap(),
                     format!("127.0.0.1:{}", GossipListenAddr::DEFAULT_PORT).parse()
                                                                            .unwrap(),];

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update: false,
                                       auto_update_period: Duration::from_secs(60),
                                       service_update_period: Duration::from_secs(60),
                                       service_restart_config: ServiceRestartConfig::default(),
                                       custom_state_path: None,
                                       key_cache: KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url: String::from("https://bldr.habitat.sh"),
                                       update_channel: ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("0.0.0.0:9638").unwrap(),
                                       ctl_listen: ListenCtlAddr::default(),
                                       ctl_server_certificates: None,
                                       ctl_server_key: None,
                                       ctl_client_ca_certificates: None,
                                       http_listen: HttpListenAddr::default(),
                                       http_disable: false,
                                       gossip_peers,
                                       gossip_permanent: false,
                                       ring_key: None,
                                       organization: None,
                                       watch_peer_file: None,
                                       tls_config: None,
                                       feature_flags: FeatureFlag::empty(),
                                       event_stream_config: None,
                                       keep_latest_packages: None,
                                       sys_ip: habitat_core::util::sys::ip().unwrap() },
                       config);
        }

        #[test]
        fn test_hab_sup_run_config_file_logging() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");

            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();

            // Setup config file
            let config_contents = r#"
verbose = true
no_color = true
json_logging = true
"#;
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let m = sup_run_from_cmd_str(&args);
            assert!(m.verbose);
            assert!(m.no_color);
            assert!(m.json_logging);

            // Setup config file
            let config_contents = r#"
verbose = true
no_color = false
json_logging = false
"#;
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {} --no-color", config_path_str);

            let m = sup_run_from_cmd_str(&args);
            assert!(m.verbose);
            assert!(m.no_color);
            assert!(!m.json_logging);
        }

        #[test]
        fn test_hab_sup_run_config_file_event_stream() {
            let lock = lock_var();
            lock.unset();

            let temp_dir = TempDir::new().expect("Could not create tempdir");

            // Setup cert files
            let certificate_path = temp_dir.path().join("certificate.pem");
            let certificate_path_str = certificate_path.to_str().unwrap();
            let mut file = File::create(&certificate_path).unwrap();
            file.write_all(CERT_FILE_CONTENTS.as_bytes()).unwrap();

            // Setup config file
            let config_contents = format!(
                                          r#"event_stream_application = "MY_APP"
event_stream_environment = "MY_ENV"
event_stream_connect_timeout = 5
event_stream_url = "127.0.0.1:3456"
event_stream_site = "my_site"
event_stream_token = "some_token"
event_meta = ["key1=val1", "key2=val2", "keyA=valA"]
event_stream_server_certificate = "{}"
"#,
                                          certificate_path_str.replace('\\', "/")
            );
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let config = config_from_cmd_str(&args);
            let mut meta = HashMap::new();
            meta.insert(String::from("key1"), String::from("val1"));
            meta.insert(String::from("key2"), String::from("val2"));
            meta.insert(String::from("keyA"), String::from("valA"));
            assert_eq!(
                ManagerConfig {
                    auto_update: false,
                    auto_update_period: Duration::from_secs(60),
                    service_update_period: Duration::from_secs(60),
                    service_restart_config: ServiceRestartConfig::default(),
                    custom_state_path: None,
                    key_cache: KeyCache::new(&*CACHE_KEY_PATH),
                    update_url: String::from("https://bldr.habitat.sh"),
                    update_channel: ChannelIdent::default(),
                    gossip_listen: GossipListenAddr::default(),
                    ctl_listen: ListenCtlAddr::default(),
                    ctl_server_certificates: None,
                    ctl_server_key: None,
                    ctl_client_ca_certificates: None,
                    http_listen: HttpListenAddr::default(),
                    http_disable: false,
                    gossip_peers: vec![],
                    gossip_permanent: false,
                    ring_key: None,
                    organization: None,
                    watch_peer_file: None,
                    tls_config: None,
                    feature_flags: FeatureFlag::empty(),
                    event_stream_config: Some(EventStreamConfig {
                        environment: String::from("MY_ENV"),
                        application: String::from("MY_APP"),
                        site: Some(String::from("my_site")),
                        meta: meta.into(),
                        token: "some_token".parse().unwrap(),
                        url: "127.0.0.1:3456".parse().unwrap(),
                        connect_method: EventStreamConnectMethod::Timeout { secs: 5 },
                        server_certificate: Some(certificate_path_str.parse().unwrap()),
                    }),
                    keep_latest_packages: None,
                    sys_ip: habitat_core::util::sys::ip().unwrap(),
                },
                config,
            );
        }

        #[test]
        fn test_hab_sup_run_config_file_svc() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let temp_dir_str = temp_dir.path().to_str().unwrap();

            // Setup config file
            let config_contents = format!(
                                          r#"
channel = "my_channel"
bind = ["one:service1.default", "two:service2.default"]
binding_mode = "relaxed"
bldr_url = "http://my_url.com"
config_from = "{}"
group = "MyGroup"
topology = "standalone"
strategy = "at-once"
update_condition = "track-channel"
health_check_interval = 17
shutdown_timeout = 12
pkg_ident_or_artifact = "core/redis"
"#,
                                          temp_dir_str.replace('\\', "/")
            );
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "{}", config_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let mut binds = ServiceBindList::default();
            binds.binds
                 .push(ServiceBind::from_str("one:service1.default").unwrap());
            binds.binds
                 .push(ServiceBind::from_str("two:service2.default").unwrap());
            let health_check_interval = sup_proto::types::HealthCheckInterval { seconds: 17 };

            let service_load = service_load_from_cmd_str(&args);
            assert_eq!(sup_proto::ctl::SvcLoad { ident:
                                                     Some("core/redis".parse::<PackageIdent>()
                                                                      .unwrap()
                                                                      .into()),
                                                 binds:                  Some(binds),
                                                 binding_mode:           Some(0),
                                                 bldr_url:
                                                     Some(String::from("http://my_url.com/")),
                                                 bldr_channel:
                                                     Some(String::from("my_channel")),
                                                 config_from:
                                                     Some(temp_dir_str.replace('\\', "/")),
                                                 force:                  Some(true),
                                                 group:
                                                     Some(String::from("MyGroup")),
                                                 svc_encrypted_password: None,
                                                 topology:
                                                     Some(Topology::Standalone.into()),
                                                 update_strategy:
                                                     Some(UpdateStrategy::AtOnce.into()),
                                                 health_check_interval:
                                                     Some(health_check_interval),
                                                 shutdown_timeout:       Some(12),
                                                 update_condition:
                                                     Some(UpdateCondition::TrackChannel.into()), },
                       service_load);
        }

        #[test]
        fn test_hab_sup_run_config_file_svc_pkg_ident_args() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();

            let args = format!("hab-sup run --config-files {}", config_path_str);

            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "pkg_ident_or_artifact = \"core/redis\"",)
                .expect("to write config file contents");
            let pkg = sup_run_from_cmd_str(&args).pkg_ident_or_artifact.unwrap();
            assert_eq!("core/redis".parse::<InstallSource>().unwrap(), pkg);

            let mut config_file = File::create(&config_path).unwrap();
            write!(
                config_file,
                "pkg_ident_or_artifact = \"core/redis/4.0.14/20200421191514\"",
            )
            .expect("to write config file contents");
            let pkg = sup_run_from_cmd_str(&args).pkg_ident_or_artifact.unwrap();
            assert_eq!("core/redis/4.0.14/20200421191514".parse::<InstallSource>()
                                                         .unwrap(),
                       pkg);

            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file,
                   "pkg_ident_or_artifact = \"/some/path/pkg.hrt\"",).expect("to write config \
                                                                              file contents");
            let pkg = sup_run_from_cmd_str(&args).pkg_ident_or_artifact.unwrap();
            assert_eq!("/some/path/pkg.hrt".parse::<InstallSource>().unwrap(), pkg);
        }

        #[cfg(windows)]
        #[test]
        fn test_hab_sup_run_config_file_password() {
            let temp_dir = TempDir::new().expect("Could not create tempdir");

            // Setup config file
            let config_path = temp_dir.path().join("config.toml");
            let config_path_str = config_path.to_str().unwrap();
            let mut config_file = File::create(&config_path).unwrap();
            write!(config_file, "password = \"keep_it_secret_keep_it_safe\"")
                .expect("to write config file contents");

            let args = format!("hab-sup run core/redis --config-files {}", config_path_str);
            let service_load = service_load_from_cmd_str(&args);
            assert_eq!(decrypt(&service_load.svc_encrypted_password.unwrap()).unwrap(),
                       "keep_it_secret_keep_it_safe");
        }

        #[test]
        fn test_hab_sup_run_config_file_and_cli() {
            let lock = lock_var();
            lock.unset();

            let temp_dir = TempDir::new().expect("Could not create tempdir");

            // Setup config file one
            let config1_path = temp_dir.path().join("config1.toml");
            let config1_path_str = config1_path.to_str().unwrap();
            let config1_contents = r#"
listen_gossip = "1.2.3.4:4321"
listen_http = "5.5.5.5:11111"
listen_ctl = "7.8.9.1:12"
organization = "MY_ORG_FROM_FIRST_CONFG"
"#;
            let mut config1_file = File::create(&config1_path).unwrap();
            write!(config1_file, "{}", config1_contents).expect("to write config file contents");

            // Setup config file two
            let config2_path = temp_dir.path().join("config2.toml");
            let config2_path_str = config2_path.to_str().unwrap();
            let config2_contents = r#"
listen_ctl = "7.7.7.7:7777"
organization = "MY_ORG_FROM_SECOND_CONFG"
"#;
            let mut config2_file = File::create(&config2_path).unwrap();
            write!(config2_file, "{}", config2_contents).expect("to write config file contents");

            let args = format!("hab-sup run --config-files {} {} --listen-http 3.3.3.3:3333",
                               config1_path_str, config2_path_str);

            let config = config_from_cmd_str(&args);
            assert_eq!(ManagerConfig { auto_update:                false,
                                       auto_update_period:         Duration::from_secs(60),
                                       service_update_period:      Duration::from_secs(60),
                                       service_restart_config:     ServiceRestartConfig::default(),
                                       custom_state_path:          None,
                                       key_cache:                  KeyCache::new(&*CACHE_KEY_PATH),
                                       update_url:
                                           String::from("https://bldr.habitat.sh"),
                                       update_channel:             ChannelIdent::default(),
                                       gossip_listen:
                                           GossipListenAddr::from_str("1.2.3.4:4321").unwrap(),
                                       ctl_listen:
                                           ListenCtlAddr::from_str("7.7.7.7:7777").unwrap(),
                                       ctl_server_certificates:    None,
                                       ctl_server_key:             None,
                                       ctl_client_ca_certificates: None,
                                       http_listen:
                                           HttpListenAddr::from_str("3.3.3.3:3333").unwrap(),
                                       http_disable:               false,
                                       gossip_peers:               vec![],
                                       gossip_permanent:           false,
                                       ring_key:                   None,
                                       organization:
                                           Some(String::from("MY_ORG_FROM_SECOND_CONFG")),
                                       watch_peer_file:            None,
                                       tls_config:                 None,
                                       feature_flags:              FeatureFlag::empty(),
                                       event_stream_config:        None,
                                       keep_latest_packages:       None,
                                       sys_ip:
                                           habitat_core::util::sys::ip().unwrap(), },
                       config);
        }

        #[test]
        fn test_hab_sup_run_all_possible_values() {
            let args = "hab-sup run --topology standalone --strategy none --update-condition \
                        latest --binding-mode strict core/redis";
            let svc_load = service_load_from_cmd_str(args);
            assert_eq!(i32::from(Topology::Standalone), svc_load.topology.unwrap());
            assert_eq!(i32::from(UpdateStrategy::None),
                       svc_load.update_strategy.unwrap());
            assert_eq!(i32::from(UpdateCondition::Latest),
                       svc_load.update_condition.unwrap());
            assert_eq!(i32::from(BindingMode::Strict),
                       svc_load.binding_mode.unwrap());

            let args = "hab-sup run --topology leader --strategy at-once --update-condition \
                        track-channel --binding-mode relaxed core/redis";
            let svc_load = service_load_from_cmd_str(args);
            assert_eq!(i32::from(Topology::Leader), svc_load.topology.unwrap());
            assert_eq!(i32::from(UpdateStrategy::AtOnce),
                       svc_load.update_strategy.unwrap());
            assert_eq!(i32::from(UpdateCondition::TrackChannel),
                       svc_load.update_condition.unwrap());
            assert_eq!(i32::from(BindingMode::Relaxed),
                       svc_load.binding_mode.unwrap());

            let args = "hab-sup run --strategy rolling core/redis";
            let svc_load = service_load_from_cmd_str(args);
            assert_eq!(i32::from(UpdateStrategy::Rolling),
                       svc_load.update_strategy.unwrap());
        }
    }
}
