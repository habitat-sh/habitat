// Implemenatation of `hab sup run`

use clap_v4 as clap;

use clap::Args;

use crate::{cli_v4::utils::{CacheKeyPath,
                            DurationProxy,
                            SharedLoad,
                            SocketAddrProxy},
            error::Result as HabResult};

use crate::cli::hab::{svc::DEFAULT_SVC_CONFIG_DIR,
                      util::tls::{CertificateChainCli,
                                  PrivateKeyCli,
                                  RootCertificateStoreCli}};

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
                     ui::UI,
                     FeatureFlag,
                     FEATURE_FLAGS};

use rants::{error::Error as RantsError,
            Address as NatsAddress};

use serde::{Deserialize,
            Serialize};

use std::{fmt,
          net::IpAddr,
          path::PathBuf,
          str::FromStr};

use habitat_core::{env::Config,
                   util as core_util};

#[cfg(not(target_os = "macos"))]
use crate::command;

#[cfg(not(target_os = "macos"))]
use std::{env,
          ffi::OsString};

// TODO (DM): This is unnecessarily difficult due to this issue in serde
// https://github.com/serde-rs/serde/issues/723. The easiest way to get around the issue is by
// using a wrapper type since NatsAddress is not defined in this crate.
#[derive(Deserialize, Serialize, Debug, Clone)]
struct EventStreamAddress(#[serde(with = "core_util::serde::string")] NatsAddress);

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

#[derive(Debug, Clone, Args)]
#[command(arg_required_else_help = true,
    disable_version_flag = true,
    help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                     {usage}\n\n{all-args}\n")]
pub(crate) struct SupRunOptions {
    /// The listen address for the Gossip Gateway.
    #[arg(long = "listen-gossip", 
        env = GossipListenAddr::ENVVAR,
        default_value = GossipListenAddr::default_as_str())]
    listen_gossip: GossipListenAddr,

    /// Initial peer addresses (IP[:PORT]).
    #[arg(long = "peer")]
    peer: Vec<SocketAddrProxy>,

    /// File to watch for connecting to the ring.
    #[arg(long = "peer-watch-file", conflicts_with = "peer")]
    peer_watch_file: Option<PathBuf>,

    /// Start in local gossip mode.
    #[arg(long = "local-gossip-mode", 
        conflicts_with_all = &["listen_gossip", "peer", "peer_watch_file"])]
    local_gossip_mode: bool,

    /// The listen address for the HTTP Gateway.
    #[arg(long = "listen-http",
        env = HttpListenAddr::ENVVAR,
        default_value = HttpListenAddr::default_as_str())]
    listen_http: HttpListenAddr,

    /// Disable the HTTP Gateway.
    #[arg(long = "http-disable", short = 'D')]
    http_disable: bool,

    /// The listen address for the Control Gateway.
    #[arg(long = "listen-ctl",
        env = ListenCtlAddr::ENVVAR,
        default_value = ListenCtlAddr::default_as_str())]
    listen_ctl: ResolvedListenCtlAddr,

    /// The control gateway server’s TLS certificate.
    #[arg(long = "ctl-server-certificate")]
    ctl_server_certificate: Option<CertificateChainCli>,

    /// The control gateway server’s private key.
    #[arg(long = "ctl-server-key")]
    ctl_server_key: Option<PrivateKeyCli>,

    /// The client CA certificate.
    #[arg(long = "ctl-client-ca-certificate")]
    ctl_client_ca_certificate: Option<RootCertificateStoreCli>,

    /// Organization the Supervisor belongs to.
    #[arg(long = "org")]
    organization: Option<String>,

    /// Mark the Supervisor as a permanent peer.
    #[arg(long = "permanent-peer", short = 'I')]
    permanent_peer: bool,

    /// Flattened cache key configuration.
    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    /// The contents of the ring key when running with wire encryption
    ///
    /// This option is explicitly undocumented and for testing purposes only. Do not use it in a
    /// production system. Use the corresponding environment variable instead. (ex:
    /// 'SYM-SEC-1\nfoo-20181113185935\n\nGCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')
    #[arg(long = "ring-key", env = RING_KEY_ENVVAR, hide = true)]
    ring_key: Option<String>,

    /// The name of the ring used by the Supervisor when running with wire encryption
    #[arg(long = "ring", short = 'r',
            env = RING_ENVVAR,
            conflicts_with = "ring_key")]
    ring: Option<String>,

    /// Enable automatic updates for the Supervisor itself
    #[arg(long = "auto-update", short = 'A')]
    auto_update: bool,

    /// Time (seconds) between Supervisor update checks.
    #[arg(long = "auto-update-period", default_value = "60")]
    auto_update_period: DurationProxy,

    /// Time (seconds) between service update checks.
    #[arg(long = "service-update-period", default_value = "60")]
    service_update_period: DurationProxy,

    /// The minimum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[arg(long = "service-min-backoff-period", default_value = "0")]
    service_min_backoff_period: DurationProxy,

    /// The maximum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[arg(long = "service-max-backoff-period", default_value = "0")]
    service_max_backoff_period: DurationProxy,

    /// The period of time in seconds to wait before assuming that a service started up
    /// successfully after a restart
    #[arg(long = "service-restart-cooldown-period", default_value = "300")]
    service_restart_cooldown_period: DurationProxy,

    /// The private key for HTTP Gateway TLS encryption
    ///
    /// Read the private key from KEY_FILE. This should be an RSA private key or PKCS8-encoded
    /// private key in PEM format.
    #[arg(long = "key")]
    key_file: Option<PathBuf>,

    /// The server certificates for HTTP Gateway TLS encryption
    ///
    /// Read server certificates from CERT_FILE. This should contain PEM-format certificates in
    /// the right order. The first certificate should certify KEY_FILE. The last should be a
    /// root CA.
    #[arg(long = "certs", requires = "key_file")]
    cert_file: Option<PathBuf>,

    /// The CA certificate for HTTP Gateway TLS encryption
    ///
    /// Read the CA certificate from CA_CERT_FILE. This should contain PEM-format certificate that
    /// can be used to validate client requests
    #[arg(long = "ca-certs", requires_all = &["cert_file", "key_file"])]
    ca_cert_file: Option<PathBuf>,

    /// Load a Habitat package as part of the Supervisor startup
    ///
    /// The package can be specified by a package identifier (ex: core/redis) or filepath to a
    /// Habitat artifact (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart).    #[arg()]
    pkg_ident_or_artifact: Option<InstallSource>,

    /// Verbose output showing file and line/column numbers
    #[arg(short = 'v')]
    verbose:  bool,
    /// Disable ANSI color.

    #[arg(long = "no-color")]
    no_color: bool,

    /// Use structured JSON logging for the Supervisor
    ///
    /// This option also sets NO_COLOR.
    #[arg(long = "json-logging")]
    json_logging: bool,

    /// The IPv4 address to use as the `sys.ip` template variable
    ///
    /// If this argument is not set, the supervisor tries to dynamically determine an IP address.
    /// If that fails, the supervisor defaults to using `127.0.0.1`.
    #[arg(long = "sys-ip-address")]
    sys_ip_address: Option<IpAddr>,

    /// The name of the application for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[arg(long = "event-stream-application", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    event_stream_application: Option<String>,

    /// The name of the environment for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[arg(long = "event-stream-environment", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    event_stream_environment: Option<String>,

    /// Event stream connection timeout before exiting the Supervisor
    ///
    /// Set to '0' to immediately start the Supervisor and continue running regardless of the
    /// initial connection status.
    #[arg(long = "event-stream-connect-timeout", env = EventStreamConnectMethod::ENVVAR, default_value = "0")]
    event_stream_connect_timeout: EventStreamConnectMethod,

    /// The authentication token for connecting the event stream to Chef Automate
    #[arg(long = "event-stream-token", env = EventStreamToken::ENVVAR)]
    event_stream_token: Option<EventStreamToken>,

    /// The event stream connection url used to send events to Chef Automate
    ///
    /// This enables the event stream and requires EVENT_STREAM_APPLICATION,
    /// EVENT_STREAM_ENVIRONMENT, and EVENT_STREAM_TOKEN also be set.
    #[arg(long = "event-stream-url", 
            requires_all = &["event_stream_application", 
                            "event_stream_environment",
                            "event_stream_token"])]
    event_stream_url: Option<EventStreamAddress>,

    /// The name of the site where this Supervisor is running for event stream purposes
    #[arg(long = "event-stream-site", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    event_stream_site: Option<String>,

    /// An arbitrary key-value pair to add to each event generated by this Supervisor
    #[arg(long = "event-meta")]
    event_meta: Vec<EventStreamMetaPair>,

    //// The path to Chef Automate's event stream certificate used to establish a TLS connection
    /// The certificate should be in PEM format.
    #[arg(long = "event-stream-server-certificate")]
    event_stream_server_certificate: Option<EventStreamServerCertificate>,

    /// Automatically cleanup old packages
    ///
    /// The Supervisor will automatically cleanup old packages only keeping the
    /// KEEP_LATEST_PACKAGES latest packages. If this argument is not specified, no
    /// automatic package cleanup is performed.
    #[arg(long = "keep-latest-packages", env = "HAB_KEEP_LATEST_PACKAGES")]
    keep_latest_packages: Option<usize>,

    /// Paths to files or directories of service config files to load on startup
    ///
    /// See `hab svc bulkload --help` for details
    #[arg(long = "svc-config-paths",
                default_value = DEFAULT_SVC_CONFIG_DIR,
                hide = !FEATURE_FLAGS.contains(FeatureFlag::SERVICE_CONFIG_FILES))]
    svc_config_paths: Vec<PathBuf>,

    #[command(flatten)]
    pub shared_load: SharedLoad,
}

impl SupRunOptions {
    #[cfg(not(target_os = "macos"))]
    pub(crate) async fn do_run(&self, ui: &mut UI) -> HabResult<()> {
        // Skip "hab" and "sup" so we pass only the subcommand + its args.
        let args: Vec<OsString> = env::args_os().skip(2).collect();

        return command::launcher::start_v4(ui, self.clone(), &args).await;
    }

    #[cfg(target_os = "macos")]
    pub(crate) async fn do_run(&self, _ui: &mut UI) -> HabResult<()> { Ok(()) }
}
