use std::{convert::TryInto,
          env,
          io,
          io::Write,
          net::{IpAddr,
                Ipv4Addr},
          process,
          str::{self}};

use configopt::ConfigOpt;
use habitat_common::{command::package::install::InstallSource,
                     liveliness_checker,
                     output::{self,
                              OutputFormat,
                              OutputVerbosity},
                     outputln,
                     types::GossipListenAddr,
                     ui::{self},
                     FeatureFlag};
use habitat_core::{self,
                   crypto::{self,
                            keys::{KeyCache,
                                   RingKey}},
                   package::{Identifiable,
                             PackageIdent},
                   tls::rustls_wrapper::{CertificateChainCli,
                                         RootCertificateStoreCli},
                   ChannelIdent};
use habitat_launcher_client::{LauncherCli,
                              ERR_NO_RETRY_EXCODE,
                              OK_NO_RETRY_EXCODE};

#[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
use habitat_sup::command;
use habitat_sup::{error::{Error,
                          Result},
                  event::EventStreamConfig,
                  manager::{Manager,
                            ManagerConfig,
                            ServiceRestartConfig,
                            TLSConfig},
                  util};
use habitat_sup_protocol::{self as sup_proto};
use log::{error,
          info,
          warn};

use habitat_sup::cli_v2::cli;

use hab::cli::hab::{sup::SupRun,
                    svc};

static LOGKEY: &str = "MN";

pub(crate) fn boot() -> Option<LauncherCli> {
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
pub(crate) async fn start_rsr_imlw_mlw_gsw_smw_rhw_msw(feature_flags: FeatureFlag) -> Result<()> {
    if feature_flags.contains(FeatureFlag::TEST_BOOT_FAIL) {
        outputln!("Simulating boot failure");
        return Err(Error::TestBootFail);
    }
    liveliness_checker::spawn_thread_alive_checker();
    let launcher = crate::cli::boot();

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
pub(crate) async fn sub_bash() -> Result<()> { command::shell::bash().await }

/// # Locking (see locking.md)
/// * `RumorStore::list` (read)
/// * `MemberList::initial_members` (write)
/// * `MemberList::entries` (write)
/// * `GatewayState::inner` (write)
/// * `Server::member` (write)
/// * `RumorHeat::inner` (write)
/// * `ManagerServices::inner` (write)
pub(crate) async fn sub_run_rsr_imlw_mlw_gsw_smw_rhw_msw(sup_run: SupRun,
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
pub(crate) async fn sub_sh() -> Result<()> { command::shell::sh().await }

pub(crate) fn sub_term() -> Result<()> {
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
pub(crate) async fn split_apart_sup_run(
    sup_run: SupRun,
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

    let channel = if let Some(ref channel) = shared_load.channel {
        channel.clone()
    } else {
        ChannelIdent::stable()
    };

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
                        update_channel: channel,
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
        let ident: &PackageIdent = install_source.as_ref();
        let channel = if let Some(ref channel) = shared_load.channel {
            channel.clone()
        } else if ident.origin() == "core" {
            ChannelIdent::base()
        } else {
            ChannelIdent::stable()
        };

        let ident = match install_source {
            source @ InstallSource::Archive(_) => {
                // Install the archive manually then explicitly set the pkg ident to the version
                // found in the archive. This will lock the software to this specific version.
                let install =
                    util::pkg::install(&mut ui::ui(), &bldr_url, &source, &channel).await?;
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

pub(crate) fn get_ring_key(sup_run: &SupRun) -> Result<Option<RingKey>> {
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

pub(crate) fn set_supervisor_logging_options(sup_run: &SupRun) {
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
