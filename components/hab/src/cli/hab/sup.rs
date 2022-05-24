use super::{svc::{SharedLoad,
                  CHANNEL_IDENT_DEFAULT,
                  DEFAULT_SVC_CONFIG_DIR,
                  GROUP_DEFAULT,
                  HEALTH_CHECK_INTERVAL_DEFAULT},
            util::{tls::{CertificateChainCli,
                         PrivateKeyCli,
                         RootCertificateStoreCli},
                   CacheKeyPath,
                   ConfigOptCacheKeyPath,
                   ConfigOptRemoteSup,
                   DurationProxy,
                   RemoteSup,
                   SubjectAlternativeName}};
use crate::{error::Error,
            VERSION};
use configopt::{self,
                configopt_fields,
                ConfigOpt};
use habitat_common::{cli::{RING_ENVVAR,
                           RING_KEY_ENVVAR},
                     command::package::install::InstallSource,
                     types::{EventStreamConnectMethod,
                             EventStreamMetaPair,
                             EventStreamServerCertificate,
                             EventStreamToken,
                             GossipListenAddr,
                             HttpListenAddr,
                             ListenCtlAddr,
                             ResolvedListenCtlAddr},
                     FeatureFlag,
                     FEATURE_FLAGS};
use habitat_core::{env::Config,
                   fs::HAB_CTL_KEYS_CACHE,
                   os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::ServiceBind,
                   util as core_util,
                   ChannelIdent};
use habitat_sup_protocol::types::{BindingMode,
                                  UpdateCondition,
                                  UpdateStrategy};
use rants::{error::Error as RantsError,
            Address as NatsAddress};
use serde::{Deserialize,
            Serialize};
use std::{fmt,
          net::{IpAddr,
                SocketAddr},
          path::PathBuf,
          str::FromStr};
use structopt::{clap::AppSettings,
                StructOpt};
use url::Url;

// All commands relating to the Supervisor (ie commands handled by both the `hab` and `hab-sup`
// binary)
#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, name = "sup")]
#[allow(clippy::large_enum_variant)]
pub enum HabSup {
    /// Depart a Supervisor from the gossip ring; kicking and banning the target from joining again
    /// with the same member-id
    #[structopt(no_version, aliases = &["d", "de", "dep", "depa", "depart"])]
    Depart {
        /// The member-id of the Supervisor to depart
        #[structopt(name = "MEMBER_ID")]
        member_id:  String,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    #[structopt(no_version, aliases = &["sec", "secr"])]
    Secret(Secret),
    /// Query the status of Habitat services
    #[structopt(no_version, aliases = &["stat", "statu"])]
    Status {
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
        #[structopt(name = "PKG_IDENT")]
        pkg_ident:  Option<PackageIdent>,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    /// Restart a Supervisor without restarting its services
    #[structopt(no_version)]
    Restart {
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    #[structopt(flatten)]
    Sup(Sup),
}

// Supervisor commands handled by the `hab-sup` binary
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "hab-sup",
            version = VERSION,
            about = "The Habitat Supervisor",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            settings = &[AppSettings::VersionlessSubcommands],
        )]
#[allow(clippy::large_enum_variant)]
pub enum Sup {
    /// Start an interactive Bash-like shell
    #[cfg(all(any(target_os = "linux", target_os = "windows"), target_arch = "x86_64"))]
    #[structopt(no_version, aliases = &["b", "ba", "bas"])]
    Bash,
    #[structopt(no_version, aliases = &["r", "ru"])]
    Run(SupRun),
    /// Start an interactive Bourne-like shell
    #[cfg(all(any(target_os = "linux", target_os = "windows"), target_arch = "x86_64"))]
    #[structopt(no_version)]
    Sh,
    /// Gracefully terminate the Habitat Supervisor and all of its running services
    #[structopt(no_version, aliases = &["ter"])]
    Term,
}

#[derive(StructOpt)]
#[structopt(name = "term", no_version)]
pub struct SupTerm {}

// TODO (DM): This is unnecessarily difficult due to this issue in serde
// https://github.com/serde-rs/serde/issues/723. The easiest way to get around the issue is by
// using a wrapper type since NatsAddress is not defined in this crate.
#[derive(Deserialize, Serialize, Debug)]
pub struct EventStreamAddress(#[serde(with = "core_util::serde::string")] NatsAddress);

impl fmt::Display for EventStreamAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl FromStr for EventStreamAddress {
    type Err = RantsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(EventStreamAddress(s.parse()?)) }
}

impl From<EventStreamAddress> for NatsAddress {
    fn from(address: EventStreamAddress) -> Self { address.0 }
}

fn parse_peer(s: &str) -> Result<SocketAddr, Error> {
    Ok(habitat_common::util::resolve_socket_addr_with_default_port(s, GossipListenAddr::DEFAULT_PORT)?.1)
}

/// Run the Habitat Supervisor
#[configopt_fields]
#[derive(ConfigOpt, StructOpt, Deserialize)]
#[configopt(attrs(serde), default_config_file("/hab/sup/default/config/sup.toml"))]
#[serde(deny_unknown_fields)]
#[structopt(name = "run",
            no_version,
            about = "Run the Habitat Supervisor",
            // set custom usage string, otherwise the binary
            // is displayed confusingly as `hab-sup`
            // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
            usage = "hab sup run [FLAGS] [OPTIONS] [--] [PKG_IDENT_OR_ARTIFACT]",
            rename_all = "screamingsnake",
        )]
pub struct SupRun {
    /// The listen address for the Gossip Gateway
    #[structopt(long = "listen-gossip",
                env = GossipListenAddr::ENVVAR,
                default_value = GossipListenAddr::default_as_str())]
    pub listen_gossip: GossipListenAddr,
    /// Start the supervisor in local mode
    #[structopt(long = "local-gossip-mode",
                conflicts_with_all = &["LISTEN_GOSSIP", "PEER", "PEER_WATCH_FILE"])]
    pub local_gossip_mode: bool,
    /// The listen address for the HTTP Gateway
    #[structopt(long = "listen-http",
                env = HttpListenAddr::ENVVAR,
                default_value = HttpListenAddr::default_as_str())]
    pub listen_http: HttpListenAddr,
    /// Disable the HTTP Gateway completely
    #[structopt(long = "http-disable", short = "D")]
    pub http_disable: bool,
    /// The listen address for the Control Gateway
    #[structopt(long = "listen-ctl",
                env = ListenCtlAddr::ENVVAR,
                default_value = ListenCtlAddr::default_as_str())]
    pub listen_ctl: ResolvedListenCtlAddr,
    /// The control gateway server's TLS certificate
    #[structopt(long = "ctl-server-certificate", default_value = HAB_CTL_KEYS_CACHE)]
    pub ctl_server_certificate: Option<CertificateChainCli>,
    /// Enable TLS for the control gateway and set the server's private key
    ///
    /// See `--ctl-server-certificate` and `--ctl-client-certificate` for additional settings.
    #[structopt(long = "ctl-server-key", default_value = HAB_CTL_KEYS_CACHE)]
    pub ctl_server_key: Option<PrivateKeyCli>,
    /// Enable client authentication for the control gateway and set the certificate authority to
    /// use when authenticating the client
    #[structopt(long = "ctl-client-ca-certificate",
                default_value = HAB_CTL_KEYS_CACHE)]
    pub ctl_client_ca_certificate: Option<RootCertificateStoreCli>,
    /// The organization the Supervisor and its services are part of
    #[structopt(long = "org")]
    pub organization: Option<String>,
    /// The listen address of one or more initial peers (IP[:PORT])
    // TODO (DM): Currently there is not a good way to use `parse_peer` when deserializing. Due to
    // https://github.com/serde-rs/serde/issues/723. There are workarounds but they are all ugly.
    // This means that you have to specify the port when setting this with a config file.
    #[structopt(long = "peer", parse(try_from_str = parse_peer))]
    pub peer: Vec<SocketAddr>,
    /// Make this Supervisor a permanent peer
    #[structopt(long = "permanent-peer", short = "I")]
    pub permanent_peer: bool,
    /// Watch this file for connecting to the ring
    #[structopt(long = "peer-watch-file", conflicts_with = "PEER")]
    pub peer_watch_file: Option<PathBuf>,
    #[structopt(flatten)]
    #[serde(flatten)]
    pub cache_key_path: CacheKeyPath,
    /// The name of the ring used by the Supervisor when running with wire encryption
    #[structopt(long = "ring",
                short = "r",
                env = RING_ENVVAR,
                conflicts_with = "RING_KEY")]
    pub ring: Option<String>,
    /// The contents of the ring key when running with wire encryption
    ///
    /// This option is explicitly undocumented and for testing purposes only. Do not use it in a
    /// production system. Use the corresponding environment variable instead. (ex:
    /// 'SYM-SEC-1\nfoo-20181113185935\n\nGCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')
    #[structopt(long = "ring-key",
                env = RING_KEY_ENVVAR,
                hidden = true)]
    pub ring_key: Option<String>,
    /// Enable automatic updates for the Supervisor itself
    #[structopt(long = "auto-update", short = "A")]
    pub auto_update: bool,
    /// The period of time in seconds between Supervisor update checks
    #[structopt(long = "auto-update-period", default_value = "60")]
    pub auto_update_period: DurationProxy,
    /// The period of time in seconds between service update checks
    #[structopt(long = "service-update-period", default_value = "60")]
    pub service_update_period: DurationProxy,
    /// The minimum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[structopt(long = "service-min-backoff-period", default_value = "0")]
    pub service_min_backoff_period: DurationProxy,
    /// The maximum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[structopt(long = "service-max-backoff-period", default_value = "0")]
    pub service_max_backoff_period: DurationProxy,
    /// The period of time in seconds to wait before assuming that a service started up
    /// successfully after a restart
    #[structopt(long = "service-restart-cooldown-period", default_value = "300")]
    pub service_restart_cooldown_period: DurationProxy,
    /// The private key for HTTP Gateway TLS encryption
    ///
    /// Read the private key from KEY_FILE. This should be an RSA private key or PKCS8-encoded
    /// private key in PEM format.
    #[structopt(long = "key", requires = "CERT_FILE")]
    pub key_file: Option<PathBuf>,
    /// The server certificates for HTTP Gateway TLS encryption
    ///
    /// Read server certificates from CERT_FILE. This should contain PEM-format certificates in
    /// the right order. The first certificate should certify KEY_FILE. The last should be a
    /// root CA.
    #[structopt(long = "certs", requires = "KEY_FILE")]
    pub cert_file: Option<PathBuf>,
    /// The CA certificate for HTTP Gateway TLS encryption
    ///
    /// Read the CA certificate from CA_CERT_FILE. This should contain PEM-format certificate that
    /// can be used to validate client requests
    #[structopt(long = "ca-certs",
                requires_all = &["CERT_FILE", "KEY_FILE"])]
    pub ca_cert_file: Option<PathBuf>,
    /// Load a Habitat package as part of the Supervisor startup
    ///
    /// The package can be specified by a package identifier (ex: core/redis) or filepath to a
    /// Habitat artifact (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart).
    #[structopt()]
    pub pkg_ident_or_artifact: Option<InstallSource>,
    /// Verbose output showing file and line/column numbers
    #[structopt(short = "v")]
    pub verbose: bool,
    /// Turn ANSI color off
    #[structopt(long = "no-color")]
    pub no_color: bool,
    /// Use structured JSON logging for the Supervisor
    ///
    /// This option also sets NO_COLOR.
    #[structopt(long = "json-logging")]
    pub json_logging: bool,
    /// The IPv4 address to use as the `sys.ip` template variable
    ///
    /// If this argument is not set, the supervisor tries to dynamically determine an IP address.
    /// If that fails, the supervisor defaults to using `127.0.0.1`.
    #[structopt(long = "sys-ip-address")]
    pub sys_ip_address: Option<IpAddr>,
    /// The name of the application for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[structopt(long = "event-stream-application", empty_values = false)]
    pub event_stream_application: Option<String>,
    /// The name of the environment for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[structopt(long = "event-stream-environment", empty_values = false)]
    pub event_stream_environment: Option<String>,
    /// Event stream connection timeout before exiting the Supervisor
    ///
    /// Set to '0' to immediately start the Supervisor and continue running regardless of the
    /// initial connection status.
    #[structopt(long = "event-stream-connect-timeout",
                env = EventStreamConnectMethod::ENVVAR,
                default_value = "0")]
    pub event_stream_connect_timeout: EventStreamConnectMethod,
    /// The event stream connection url used to send events to Chef Automate
    ///
    /// This enables the event stream and requires EVENT_STREAM_APPLICATION,
    /// EVENT_STREAM_ENVIRONMENT, and EVENT_STREAM_TOKEN also be set.
    #[structopt(long = "event-stream-url",
                requires_all = &["EVENT_STREAM_APPLICATION", 
                                 "EVENT_STREAM_ENVIRONMENT",
                                 EventStreamToken::ARG_NAME])]
    pub event_stream_url: Option<EventStreamAddress>,
    /// The name of the site where this Supervisor is running for event stream purposes
    #[structopt(long = "event-stream-site", empty_values = false)]
    pub event_stream_site: Option<String>,
    /// The authentication token for connecting the event stream to Chef Automate
    #[structopt(long = "event-stream-token",
                env = EventStreamToken::ENVVAR)]
    pub event_stream_token: Option<EventStreamToken>,
    /// An arbitrary key-value pair to add to each event generated by this Supervisor
    #[structopt(long = "event-meta")]
    pub event_meta: Vec<EventStreamMetaPair>,
    /// The path to Chef Automate's event stream certificate used to establish a TLS connection
    ///
    /// The certificate should be in PEM format.
    #[structopt(long = "event-stream-server-certificate")]
    pub event_stream_server_certificate: Option<EventStreamServerCertificate>,
    /// Automatically cleanup old packages
    ///
    /// The Supervisor will automatically cleanup old packages only keeping the
    /// KEEP_LATEST_PACKAGES latest packages. If this argument is not specified, no
    /// automatic package cleanup is performed.
    #[structopt(long = "keep-latest-packages", env = "HAB_KEEP_LATEST_PACKAGES")]
    pub keep_latest_packages: Option<usize>,
    /// Paths to files or directories of service config files to load on startup
    ///
    /// See `hab svc bulkload --help` for details
    #[structopt(long = "svc-config-paths",
                default_value = DEFAULT_SVC_CONFIG_DIR,
                hidden = !FEATURE_FLAGS.contains(FeatureFlag::SERVICE_CONFIG_FILES))]
    pub svc_config_paths: Vec<PathBuf>,
    /// Receive updates from the specified release channel
    #[structopt(long = "channel", default_value = &*CHANNEL_IDENT_DEFAULT)]
    #[serde(default)]
    pub channel: ChannelIdent,
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    #[structopt(short = "u", long = "url")]
    bldr_url: Option<Url>,
    /// The service group with shared config and topology
    #[structopt(long = "group", requires = "PKG_IDENT_OR_ARTIFACT")]
    group: Option<String>,
    /// Service topology
    #[structopt(long = "topology",
            short = "t",
            possible_values = &["standalone", "leader"],
            requires = "PKG_IDENT_OR_ARTIFACT")]
    topology: Option<habitat_sup_protocol::types::Topology>,
    /// The update strategy
    #[structopt(long = "strategy",
                short = "s",
                possible_values = &["none", "at-once", "rolling"],
                requires = "PKG_IDENT_OR_ARTIFACT")]
    strategy: Option<habitat_sup_protocol::types::UpdateStrategy>,
    /// The condition dictating when this service should update
    #[structopt(long = "update-condition",
                possible_values = UpdateCondition::VARIANTS,
                requires = "PKG_IDENT_OR_ARTIFACT")]
    update_condition: Option<UpdateCondition>,
    /// One or more service groups to bind to a configuration
    #[structopt(long = "bind", requires = "PKG_IDENT_OR_ARTIFACT")]
    bind: Vec<ServiceBind>,
    /// Governs how the presence or absence of binds affects service startup
    #[structopt(long = "binding-mode",
                possible_values = &["strict", "relaxed"],
                requires = "PKG_IDENT_OR_ARTIFACT")]
    binding_mode: Option<habitat_sup_protocol::types::BindingMode>,
    /// The interval in seconds on which to run health checks
    #[structopt(long = "health-check-interval",
                short = "i",
                requires = "PKG_IDENT_OR_ARTIFACT")]
    health_check_interval: Option<u64>,
    /// The delay in seconds after sending the shutdown signal to wait before killing the service
    /// process
    #[structopt(long = "shutdown-timeout", requires = "PKG_IDENT_OR_ARTIFACT")]
    shutdown_timeout: Option<ShutdownTimeout>,
    #[cfg(target_os = "windows")]
    /// Password of the service user
    #[structopt(long = "password", requires = "PKG_IDENT_OR_ARTIFACT")]
    password: Option<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(long = "application", short = "a", takes_value = false, hidden = true)]
    #[serde(skip)]
    application: Vec<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(long = "environment", short = "e", takes_value = false, hidden = true)]
    #[serde(skip)]
    environment: Vec<String>,
    /// Use the package config from this path rather than the package itself
    #[structopt(long = "config-from", requires = "PKG_IDENT_OR_ARTIFACT")]
    config_from: Option<PathBuf>,
}

impl From<&SupRun> for SharedLoad {
    fn from(sup_run: &SupRun) -> Self {
        Self { channel: sup_run.channel.clone(),
               bldr_url: sup_run.bldr_url.clone(),
               group: sup_run.group
                             .as_ref()
                             .unwrap_or(&GROUP_DEFAULT::get())
                             .to_string(),
               topology: sup_run.topology,
               strategy: sup_run.strategy.unwrap_or(UpdateStrategy::None),
               bind: sup_run.bind.clone(),
               update_condition: sup_run.update_condition.unwrap_or(UpdateCondition::Latest),
               binding_mode: sup_run.binding_mode.unwrap_or(BindingMode::Strict),
               health_check_interval:
                   sup_run.health_check_interval
                          .unwrap_or_else(|| u64::from_str(HEALTH_CHECK_INTERVAL_DEFAULT).unwrap()),
               shutdown_timeout: sup_run.shutdown_timeout,
               #[cfg(target_os = "windows")]
               password: sup_run.password.clone(),
               environment: sup_run.environment.clone(),
               application: sup_run.application.clone(),
               config_from: sup_run.config_from.clone(), }
    }
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to a Habitat Supervisor's Control Gateway secret
pub enum Secret {
    /// Generate a secret key to use as a Supervisor's Control Gateway secret
    Generate,
    /// Generate a private key and certificate for the Supervisor's
    /// Control Gateway TLS connection
    GenerateTls {
        /// The DNS name to use in the certificates subject alternative name extension
        #[structopt(long = "subject-alternative-name")]
        subject_alternative_name: SubjectAlternativeName,
        /// The directory to store the generated private key and certificate
        #[structopt(long = "path", default_value = HAB_CTL_KEYS_CACHE)]
        path:                     PathBuf,
    },
}
