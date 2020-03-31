use super::util::{CacheKeyPath,
                  RemoteSup};
use crate::VERSION;
use configopt::{ConfigOptDefaults,
                ConfigOptToString,
                Partial};
use habitat_common::{cli::{RING_ENVVAR,
                           RING_KEY_ENVVAR},
                     types::{AutomateAuthToken,
                             EventStreamConnectMethod,
                             EventStreamMetadata,
                             EventStreamServerCertificate,
                             GossipListenAddr,
                             HttpListenAddr,
                             ListenCtlAddr}};
use habitat_core::{env::Config,
                   os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::HealthCheckInterval,
                   util::serde_string};
use habitat_sup_protocol::types::{BindingMode,
                                  Topology,
                                  UpdateCondition,
                                  UpdateStrategy};
use rants::{error::Error as RantsError,
            Address as NatsAddress};
use std::{fmt,
          net::{Ipv4Addr,
                SocketAddr},
          path::PathBuf,
          str::FromStr};
use structopt::{clap::AppSettings,
                StructOpt};
use url::Url;

#[derive(StructOpt)]
#[structopt(name = "hab",
            version = VERSION,
            about = "The Habitat Supervisor",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            usage = "hab sup <SUBCOMMAND>",
            settings = &[AppSettings::VersionlessSubcommands],
        )]
#[allow(clippy::large_enum_variant)]
pub enum Sup {
    /// Start an interactive Bash-like shell
    #[structopt(usage = "hab sup bash", no_version)]
    Bash,
    /// Depart a Supervisor from the gossip ring; kicking and banning the target from joining again
    /// with the same member-id
    #[structopt(no_version)]
    Depart {
        /// The member-id of the Supervisor to depart
        #[structopt(name = "MEMBER_ID")]
        member_id:  String,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    /// Run the Habitat Supervisor
    #[structopt(no_version)]
    Run(SupRun),
    #[structopt(no_version)]
    Secret(Secret),
    /// Start an interactive Bourne-like shell
    #[structopt(usage = "hab sup sh", no_version)]
    Sh,
    /// Query the status of Habitat services
    #[structopt(no_version)]
    Status {
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
        #[structopt(name = "PKG_IDENT")]
        pkg_ident:  Option<PackageIdent>,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    /// Gracefully terminate the Habitat Supervisor and all of its running services
    #[structopt(usage = "hab sup term [OPTIONS]", no_version)]
    Term,
}

// TODO (DM): This is unnecessarily difficult due to the orphan rule and the lack of specialization.
// The `configopt` library could be improved to make this easier.
#[derive(Deserialize, Serialize, Debug)]
struct EventStreamAddress(#[serde(with = "serde_string")] NatsAddress);

impl fmt::Display for EventStreamAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl FromStr for EventStreamAddress {
    type Err = RantsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(EventStreamAddress(s.parse()?)) }
}

impl ConfigOptToString for EventStreamAddress {}

#[derive(ConfigOptDefaults, Partial, StructOpt, Deserialize)]
#[configopt_defaults(type = "PartialSupRun")]
#[partial(derive(Debug, Default, Deserialize), attrs(serde))]
#[serde(deny_unknown_fields)]
#[structopt(name = "run",
            no_version,
            about = "Run the Habitat Supervisor",
            // set custom usage string, otherwise the binary
            // is displayed confusingly as `hab-sup`
            // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
            usage = "hab sup run [FLAGS] [OPTIONS] [--] [PKG_IDENT_OR_ARTIFACT]"
        )]
#[allow(dead_code)]
pub struct SupRun {
    /// The listen address for the Gossip System Gateway
    #[structopt(name = "LISTEN_GOSSIP",
                long = "listen-gossip",
                env = GossipListenAddr::ENVVAR,
                default_value = GossipListenAddr::default_as_str())]
    listen_gossip: SocketAddr,
    /// Start the supervisor in local mode
    #[structopt(name = "LOCAL_GOSSIP_MODE",
                long = "local-gossip-mode",
                conflicts_with_all = &["LISTEN_GOSSIP", "PEER", "PEER_WATCH_FILE"])]
    local_gossip_mode: bool,
    /// The listen address for the HTTP Gateway
    #[structopt(name = "LISTEN_HTTP",
                long = "listen-http",
                env = HttpListenAddr::ENVVAR,
                default_value = HttpListenAddr::default_as_str())]
    listen_http: SocketAddr,
    /// Disable the HTTP Gateway completely
    #[structopt(name = "HTTP_DISABLE", long = "http-disable", short = "D")]
    http_disable: bool,
    /// The listen address for the Control Gateway. If not specified, the value will be taken from
    /// the HAB_LISTEN_CTL environment variable if defined
    #[structopt(name = "LISTEN_CTL",
                long = "listen-ctl",
                env = ListenCtlAddr::ENVVAR,
                default_value = ListenCtlAddr::default_as_str())]
    listen_ctl: SocketAddr,
    /// The organization that the Supervisor and its subsequent services are part of
    #[structopt(name = "ORGANIZATION", long = "org")]
    organization: Option<String>,
    /// The listen address of one or more initial peers (IP[:PORT])
    #[structopt(name = "PEER", long = "peer")]
    // TODO (DM): This could probably be a different type for better validation (Vec<SockAddr>?)
    peer: Vec<String>,
    /// If this Supervisor is a permanent peer
    #[structopt(name = "PERMANENT_PEER", long = "permanent-peer", short = "I")]
    permanent_peer: bool,
    /// Watch this file for connecting to the ring
    #[structopt(name = "PEER_WATCH_FILE",
                long = "peer-watch-file",
                conflicts_with = "PEER")]
    peer_watch_file: Option<PathBuf>,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
    /// The name of the ring used by the Supervisor when running with wire encryption. (ex: hab sup
    /// run --ring myring)
    #[structopt(name = "RING",
                long = "ring",
                short = "r",
                env = RING_ENVVAR,
                conflicts_with = "RING_KEY")]
    ring: Option<String>,
    /// The contents of the ring key when running with wire encryption. (Note: This option is
    /// explicitly undocumented and for testing purposes only. Do not use it in a production
    /// system. Use the corresponding environment variable instead.) (ex: hab sup run --ring-key
    /// 'SYM-SEC-1 foo-20181113185935 GCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')
    #[structopt(name = "RING_KEY",
                long = "ring-key",
                env = RING_KEY_ENVVAR,
                hidden = true,
                conflicts_with = "RING")]
    ring_key: Option<String>,
    /// Receive Supervisor updates from the specified release channel
    #[structopt(name = "CHANNEL", long = "channel", default_value = "stable")]
    channel: String,
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from the
    /// HAB_BLDR_URL environment variable if defined (default: https://bldr.habitat.sh)
    #[structopt(name = "BLDR_URL",
                long = "url",
                short = "u",
                // TODO (DM): These fields are not actual set in the clap macro but I think they should
                // env = BLDR_URL_ENVVAR,
                // default_value = DEFAULT_BLDR_URL
            )]
    bldr_url: Option<Url>,
    /// Use package config from this path, rather than the package itself
    #[structopt(name = "CONFIG_DIR", long = "config-from")]
    config_dir: Option<PathBuf>,
    /// Enable automatic updates for the Supervisor itself
    #[structopt(name = "AUTO_UPDATE", long = "auto-update", short = "A")]
    auto_update: bool,
    /// Used for enabling TLS for the HTTP gateway. Read private key from KEY_FILE. This should be
    /// a RSA private key or PKCS8-encoded private key, in PEM format
    #[structopt(name = "KEY_FILE", long = "key", requires = "CERT_FILE")]
    key_file: Option<PathBuf>,
    /// Used for enabling TLS for the HTTP gateway. Read server certificates from CERT_FILE. This
    /// should contain PEM-format certificates in the right order (the first certificate should
    /// certify KEY_FILE, the last should be a root CA)
    #[structopt(name = "CERT_FILE", long = "certs", requires = "KEY_FILE")]
    cert_file: Option<PathBuf>,
    /// Used for enabling client-authentication with TLS for the HTTP gateway. Read CA certificate
    /// from CA_CERT_FILE. This should contain PEM-format certificate that can be used to validate
    /// client requests
    #[structopt(name = "CA_CERT_FILE",
                long = "ca-certs",
                requires_all = &["CERT_FILE", "KEY_FILE"])]
    ca_cert_file: Option<PathBuf>,
    /// Load the given Habitat package as part of the Supervisor startup specified by a package
    /// identifier (ex: core/redis) or filepath to a Habitat Artifact (ex:
    /// /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart)
    // TODO (DM): We could probably do better validation here
    #[structopt(name = "PKG_IDENT_OR_ARTIFACT")]
    pkg_ident_or_artifact: Option<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(name = "APPLICATION",
                long = "application",
                short = "a",
                takes_value = false,
                hidden = true)]
    application: Vec<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(name = "ENVIRONMENT",
                long = "environment",
                short = "e",
                takes_value = false,
                hidden = true)]
    environment: Vec<String>,
    /// The service group; shared config and topology [default: default]
    // TODO (DM): This should set a default value
    #[structopt(name = "GROUP", long = "group")]
    group: Option<String>,
    /// Service topology; [default: none]
    // TODO (DM): I dont think saying the default is none makes sense here
    #[structopt(name = "TOPOLOGY",
                long = "topology",
                short = "t",
                possible_values = &["standalone", "leader"])]
    topology: Option<Topology>,
    /// The update strategy; [default: none] [values: none, at-once, rolling]
    // TODO (DM): this should set a default_value and use possible_values = &["none", "at-once",
    // "rolling"]
    #[structopt(name = "STRATEGY", long = "strategy", short = "s")]
    strategy: Option<UpdateStrategy>,
    /// The condition dictating when this service should update
    ///
    /// latest: Runs the latest package that can be found in the configured channel and local
    /// packages.
    ///
    /// track-channel: Always run what is at the head of a given channel. This enables service
    /// rollback where demoting a package from a channel will cause the package to rollback to
    /// an older version of the package. A ramification of enabling this condition is packages
    /// newer than the package at the head of the channel will be automatically uninstalled
    /// during a service rollback.
    #[structopt(name = "UPDATE_CONDITION",
                long = "update-condition",
                default_value = UpdateCondition::Latest.as_str(),
                possible_values = UpdateCondition::VARIANTS)]
    update_condition: UpdateCondition,
    /// One or more service groups to bind to a configuration
    #[structopt(name = "BIND", long = "bind")]
    bind: Vec<String>,
    /// Governs how the presence or absence of binds affects service startup. `strict` blocks
    /// startup until all binds are present. [default: strict] [values: relaxed, strict]
    // TODO (DM): This should set default_value and use possible_values
    #[structopt(name = "BINDING_MODE", long = "binding-mode")]
    binding_mode: Option<BindingMode>,
    /// Verbose output; shows file and line/column numbers
    #[structopt(name = "VERBOSE", short = "v")]
    verbose: bool,
    /// Turn ANSI color off
    #[structopt(name = "NO_COLOR", long = "no-color")]
    no_color: bool,
    /// Use structured JSON logging for the Supervisor. Implies NO_COLOR
    #[structopt(name = "JSON", long = "json-logging")]
    json_logging: bool,
    /// The interval (seconds) on which to run health checks [default: 30]
    // TODO (DM): Should use default_value = "30"
    #[structopt(name = "HEALTH_CHECK_INTERVAL",
                long = "health-check-interval",
                short = "i")]
    health_check_interval: Option<HealthCheckInterval>,
    /// The IPv4 address to use as the `sys.ip` template variable. If this argument is not set, the
    /// supervisor tries to dynamically determine an IP address. If that fails, the supervisor
    /// defaults to using `127.0.0.1`
    #[structopt(name = "SYS_IP_ADDRESS", long = "sys-ip-address")]
    sys_ip_address: Option<Ipv4Addr>,
    /// The name of the application for event stream purposes. This will be attached to all events
    /// generated by this Supervisor
    #[structopt(name = "EVENT_STREAM_APPLICATION", long = "event-stream-application")]
    event_stream_application: Option<String>,
    /// The name of the environment for event stream purposes. This will be attached to all events
    /// generated by this Supervisor
    #[structopt(name = "EVENT_STREAM_ENVIRONMENT", long = "event-stream-environment")]
    event_stream_environment: Option<String>,
    /// How long in seconds to wait for an event stream connection before exiting the Supervisor.
    /// Set to '0' to immediately start the Supervisor and continue running regardless of the
    /// initial connection status
    #[structopt(name = "EVENT_STREAM_CONNECT_TIMEOUT",
                long = "event-stream-connect-timeout",
                default_value = "0",
                env = EventStreamConnectMethod::ENVVAR)]
    event_stream_connect_timeout: u64,
    /// The event stream connection string (host:port) used by this Supervisor to send events to
    /// Chef Automate. This enables the event stream and requires --event-stream-application,
    /// --event-stream-environment, and --event-stream-token also be set
    #[structopt(name = "EVENT_STREAM_URL",
                long = "event-stream-url",
                requires_all = &["EVENT_STREAM_APPLICATION", 
                                 "EVENT_STREAM_ENVIRONMENT",
                                 AutomateAuthToken::ARG_NAME])]
    event_stream_url: Option<EventStreamAddress>,
    /// The name of the site where this Supervisor is running for event stream purposes
    #[structopt(name = "EVENT_STREAM_SITE", long = "event-stream-site")]
    event_stream_site: Option<String>,
    /// The authentication token for connecting the event stream to Chef Automate
    #[structopt(name = "EVENT_STREAM_TOKEN",
                long = "event-stream-token",
                env = AutomateAuthToken::ENVVAR,
                validator = AutomateAuthToken::validate)]
    automate_auth_token: Option<String>,
    /// An arbitrary key-value pair to add to each event generated by this Supervisor
    #[structopt(name = "EVENT_STREAM_METADATA",
                long = "event-meta",
                validator = EventStreamMetadata::validate)]
    event_meta: Vec<String>,
    /// The path to Chef Automate's event stream certificate in PEM format used to establish a TLS
    /// connection
    #[structopt(name = "EVENT_STREAM_SERVER_CERTIFICATE",
                long = "event-stream-server-certificate",
                validator = EventStreamServerCertificate::validate)]
    event_stream_server_certificate: Option<String>,
    /// The number of seconds after sending a shutdown signal to wait before killing a service
    /// process (default: set in plan)
    #[structopt(name = "SHUTDOWN_TIMEOUT", long = "shutdown-timeout")]
    shutdown_timeout: Option<ShutdownTimeout>,
    /// Automatically cleanup old packages.
    ///
    /// If enabled, service startup will initiate an uninstall of all packages except for the
    /// `KEEP_LATEST_PACKAGES` most recent packages. The same logic applies to the `core/hab-sup`
    /// package on Supervisor startup.
    #[structopt(name = "KEEP_LATEST_PACKAGES",
                long = "keep-latest-packages",
                env = "HAB_KEEP_LATEST_PACKAGES")]
    keep_latest_packages: Option<usize>,
}

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to a Habitat Supervisor's Control Gateway secret
pub enum Secret {
    /// Generate a secret key to use as a Supervisor's Control Gateway secret
    Generate,
}
