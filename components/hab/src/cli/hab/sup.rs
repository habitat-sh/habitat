use super::{svc::{SharedLoad,
                  DEFAULT_SVC_CONFIG_DIR},
            util::{tls::{CertificateChainCli,
                         PrivateKeyCli,
                         RootCertificateStoreCli},
                   CacheKeyPath,
                   DurationProxy,
                   RemoteSup,
                   SocketAddrProxy,
                   SubjectAlternativeName}};
use crate::VERSION;
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
                   package::PackageIdent,
                   util as core_util};
use rants::{error::Error as RantsError,
            Address as NatsAddress};
use serde::{Deserialize,
            Serialize};
use std::{fmt,
          net::IpAddr,
          path::PathBuf,
          str::FromStr};

use clap::Parser;

// All commands relating to the Supervisor (ie commands handled by both the `hab` and `hab-sup`
// binary)
#[derive(Parser)]
#[allow(clippy::large_enum_variant)]
pub enum HabSup {
    /// Depart a Supervisor from the gossip ring; kicking and banning the target from joining again
    /// with the same member-id
    #[clap(aliases = &["d", "de", "dep", "depa", "depart"])]
    Depart {
        /// The member-id of the Supervisor to depart
        #[clap(name = "MEMBER_ID")]
        member_id: String,

        #[command(flatten)]
        remote_sup: RemoteSup,
    },

    #[clap(aliases = &["sec", "secr"])]
    Secret(Secret),

    /// Query the status of Habitat services
    #[clap(aliases = &["stat", "statu"])]
    Status {
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
        #[clap(name = "PKG_IDENT")]
        pkg_ident: Option<PackageIdent>,

        #[command(flatten)]
        remote_sup: RemoteSup,
    },

    /// Restart a Supervisor without restarting its services
    Restart {
        #[command(flatten)]
        remote_sup: RemoteSup,
    },

    #[cfg(not(target_os = "macos"))]
    #[command(flatten)]
    Sup(Sup),
}

// Supervisor commands handled by the `hab-sup` binary
#[derive(Parser)]
#[command(name = "hab-sup",
            version = VERSION,
            about = "The Habitat Supervisor",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            settings = &[AppSettings::VersionlessSubcommands],
        )]
#[allow(clippy::large_enum_variant)]
pub enum Sup {
    /// Start an interactive Bash-like shell
    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
    #[clap(no_version, aliases = &["b", "ba", "bas"])]
    Bash,

    Run(SupRun),

    /// Start an interactive Bourne-like shell
    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
    Sh,

    /// Gracefully terminate the Habitat Supervisor and all of its running services
    Term,
}

#[derive(Parser)]
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

/// Run the Habitat Supervisor
#[derive(Parser, Deserialize)]
#[serde(deny_unknown_fields)]
#[command(name = "run",
            about = "Run the Habitat Supervisor",
            // set custom usage string, otherwise the binary
            // is displayed confusingly as `hab-sup`
            // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
            usage = "hab sup run [FLAGS] [OPTIONS] [--] [PKG_IDENT_OR_ARTIFACT]",
            rename_all = "screamingsnake",
        )]
pub struct SupRun {
    /// The listen address for the Gossip Gateway
    #[clap(long = "listen-gossip",
                env = GossipListenAddr::ENVVAR,
                default_value = GossipListenAddr::default_as_str())]
    pub listen_gossip: GossipListenAddr,

    /// Start the supervisor in local mode
    #[clap(long = "local-gossip-mode",
                conflicts_with_all = &["LISTEN_GOSSIP", "PEER", "PEER_WATCH_FILE"])]
    pub local_gossip_mode: bool,

    /// The listen address for the HTTP Gateway
    #[clap(long = "listen-http",
                env = HttpListenAddr::ENVVAR,
                default_value = HttpListenAddr::default_as_str())]
    pub listen_http: HttpListenAddr,

    /// Disable the HTTP Gateway completely
    #[clap(long = "http-disable", short = "D")]
    pub http_disable: bool,

    /// The listen address for the Control Gateway
    #[clap(long = "listen-ctl",
                env = ListenCtlAddr::ENVVAR,
                default_value = ListenCtlAddr::default_as_str())]
    pub listen_ctl: ResolvedListenCtlAddr,

    /// The control gateway server's TLS certificate
    #[clap(long = "ctl-server-certificate", default_value = HAB_CTL_KEYS_CACHE)]
    pub ctl_server_certificate: Option<CertificateChainCli>,

    /// Enable TLS for the control gateway and set the server's private key
    ///
    /// See `--ctl-server-certificate` and `--ctl-client-certificate` for additional settings.
    #[clap(long = "ctl-server-key", default_value = HAB_CTL_KEYS_CACHE)]
    pub ctl_server_key: Option<PrivateKeyCli>,

    /// Enable client authentication for the control gateway and set the certificate authority to
    /// use when authenticating the client
    #[clap(long = "ctl-client-ca-certificate",
                default_value = HAB_CTL_KEYS_CACHE)]
    pub ctl_client_ca_certificate: Option<RootCertificateStoreCli>,

    /// The organization the Supervisor and its services are part of
    #[clap(long = "org")]
    pub organization: Option<String>,

    /// The listen address of one or more initial peers (IP[:PORT])
    #[clap(long = "peer")]
    pub peer: Vec<SocketAddrProxy>,

    /// Make this Supervisor a permanent peer
    #[structopt(long = "permanent-peer", short = "I")]
    pub permanent_peer: bool,

    /// Watch this file for connecting to the ring
    #[clap(long = "peer-watch-file", conflicts_with = "PEER")]
    pub peer_watch_file: Option<PathBuf>,

    #[command(flatten)]
    #[serde(flatten)]
    pub cache_key_path: CacheKeyPath,

    /// The name of the ring used by the Supervisor when running with wire encryption
    #[clap(long = "ring",
                short = "r",
                env = RING_ENVVAR,
                conflicts_with = "RING_KEY")]
    pub ring: Option<String>,

    /// The contents of the ring key when running with wire encryption
    ///
    /// This option is explicitly undocumented and for testing purposes only. Do not use it in a
    /// production system. Use the corresponding environment variable instead. (ex:
    /// 'SYM-SEC-1\nfoo-20181113185935\n\nGCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')
    #[clap(long = "ring-key",
                env = RING_KEY_ENVVAR,
                hidden = true)]
    pub ring_key: Option<String>,

    /// Enable automatic updates for the Supervisor itself
    #[clap(long = "auto-update", short = "A")]
    pub auto_update: bool,

    /// The period of time in seconds between Supervisor update checks
    #[clap(long = "auto-update-period", default_value = "60")]
    pub auto_update_period: DurationProxy,

    /// The period of time in seconds between service update checks
    #[clap(long = "service-update-period", default_value = "60")]
    pub service_update_period: DurationProxy,

    /// The minimum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[clap(long = "service-min-backoff-period", default_value = "0")]
    pub service_min_backoff_period: DurationProxy,

    /// The maximum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[clap(long = "service-max-backoff-period", default_value = "0")]
    pub service_max_backoff_period: DurationProxy,

    /// The period of time in seconds to wait before assuming that a service started up
    /// successfully after a restart
    #[clap(long = "service-restart-cooldown-period", default_value = "300")]
    pub service_restart_cooldown_period: DurationProxy,

    /// The private key for HTTP Gateway TLS encryption
    ///
    /// Read the private key from KEY_FILE. This should be an RSA private key or PKCS8-encoded
    /// private key in PEM format.
    #[clap(long = "key", requires = "CERT_FILE")]
    pub key_file: Option<PathBuf>,

    /// The server certificates for HTTP Gateway TLS encryption
    ///
    /// Read server certificates from CERT_FILE. This should contain PEM-format certificates in
    /// the right order. The first certificate should certify KEY_FILE. The last should be a
    /// root CA.
    #[clap(long = "certs", requires = "KEY_FILE")]
    pub cert_file: Option<PathBuf>,

    /// The CA certificate for HTTP Gateway TLS encryption
    ///
    /// Read the CA certificate from CA_CERT_FILE. This should contain PEM-format certificate that
    /// can be used to validate client requests
    #[clap(long = "ca-certs",
                requires_all = &["CERT_FILE", "KEY_FILE"])]
    pub ca_cert_file: Option<PathBuf>,

    /// Load a Habitat package as part of the Supervisor startup
    ///
    /// The package can be specified by a package identifier (ex: core/redis) or filepath to a
    /// Habitat artifact (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart).
    pub pkg_ident_or_artifact: Option<InstallSource>,

    /// Verbose output showing file and line/column numbers
    #[clap(short = "v")]
    pub verbose: bool,

    /// Turn ANSI color off
    #[clap(long = "no-color")]
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
    #[clap(long = "sys-ip-address")]
    pub sys_ip_address: Option<IpAddr>,

    /// The name of the application for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[clap(long = "event-stream-application", empty_values = false)]
    pub event_stream_application: Option<String>,

    /// The name of the environment for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[clap(long = "event-stream-environment", empty_values = false)]
    pub event_stream_environment: Option<String>,

    /// Event stream connection timeout before exiting the Supervisor
    ///
    /// Set to '0' to immediately start the Supervisor and continue running regardless of the
    /// initial connection status.
    #[clap(long = "event-stream-connect-timeout",
                env = EventStreamConnectMethod::ENVVAR,
                default_value = "0")]
    pub event_stream_connect_timeout: EventStreamConnectMethod,

    /// The event stream connection url used to send events to Chef Automate
    ///
    /// This enables the event stream and requires EVENT_STREAM_APPLICATION,
    /// EVENT_STREAM_ENVIRONMENT, and EVENT_STREAM_TOKEN also be set.
    #[clap(long = "event-stream-url",
                requires_all = &["EVENT_STREAM_APPLICATION",
                                 "EVENT_STREAM_ENVIRONMENT",
                                 EventStreamToken::ARG_NAME])]
    pub event_stream_url: Option<EventStreamAddress>,

    /// The name of the site where this Supervisor is running for event stream purposes
    #[clap(long = "event-stream-site", empty_values = false)]
    pub event_stream_site: Option<String>,

    /// The authentication token for connecting the event stream to Chef Automate
    #[clap(long = "event-stream-token", env = EventStreamToken::ENVVAR)]
    pub event_stream_token: Option<EventStreamToken>,

    /// An arbitrary key-value pair to add to each event generated by this Supervisor
    #[clap(long = "event-meta")]
    pub event_meta: Vec<EventStreamMetaPair>,

    /// The path to Chef Automate's event stream certificate used to establish a TLS connection
    ///
    /// The certificate should be in PEM format.
    #[clap(long = "event-stream-server-certificate")]
    pub event_stream_server_certificate: Option<EventStreamServerCertificate>,

    /// Automatically cleanup old packages
    ///
    /// The Supervisor will automatically cleanup old packages only keeping the
    /// KEEP_LATEST_PACKAGES latest packages. If this argument is not specified, no
    /// automatic package cleanup is performed.
    #[clap(long = "keep-latest-packages", env = "HAB_KEEP_LATEST_PACKAGES")]
    pub keep_latest_packages: Option<usize>,

    /// Paths to files or directories of service config files to load on startup
    ///
    /// See `hab svc bulkload --help` for details
    #[structopt(long = "svc-config-paths",
                default_value = DEFAULT_SVC_CONFIG_DIR,
                hidden = !FEATURE_FLAGS.contains(FeatureFlag::SERVICE_CONFIG_FILES))]
    pub svc_config_paths: Vec<PathBuf>,

    #[command(flatten)]
    #[serde(flatten)]
    pub shared_load: SharedLoad,
}

#[derive(Parser)]
/// Commands relating to a Habitat Supervisor's Control Gateway secret
pub enum Secret {
    /// Generate a secret key to use as a Supervisor's Control Gateway secret
    Generate,
    /// Generate a private key and certificate for the Supervisor's
    /// Control Gateway TLS connection
    GenerateTls {
        /// The DNS name to use in the certificates subject alternative name extension
        #[clap(long = "subject-alternative-name")]
        subject_alternative_name: SubjectAlternativeName,

        /// The directory to store the generated private key and certificate
        #[clap(long = "path", default_value = HAB_CTL_KEYS_CACHE)]
        path: PathBuf,
    },
}
