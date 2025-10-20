// Implemenatation of `hab sup run`

use clap_v4 as clap;

use clap::Args;

use crate::{cli_v4::utils::{is_default,
                            CacheKeyPath,
                            DurationProxy,
                            SharedLoad,
                            SocketAddrProxy},
            error::{Error as HabError,
                    Result as HabResult}};

use habitat_core::{fs::HAB_CTL_KEYS_CACHE,
                   tls::rustls_wrapper::{CertificateChainCli,
                                         PrivateKeyCli,
                                         RootCertificateStoreCli}};

use habitat_common::{cli::{clap_validators::FileExistsValueParser,
                           is_toml_file,
                           RING_ENVVAR,
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
                     ui::UI};

use hab_common_derive::GenConfig;

use rants::{error::Error as RantsError,
            Address as NatsAddress};

use serde::{Deserialize,
            Serialize};

use std::{fmt,
          fs,
          net::IpAddr,
          path::{Path,
                 PathBuf},
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

#[derive(GenConfig)]
#[derive(Debug, Clone, Args, Serialize, Deserialize)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub struct SupRunOptions {
    /// The listen address for the Gossip Gateway.
    #[arg(long = "listen-gossip",
          env = GossipListenAddr::ENVVAR,
          default_value = GossipListenAddr::default_as_str())]
    #[serde(default, skip_serializing_if = "is_default")]
    pub listen_gossip: GossipListenAddr,

    /// Initial peer addresses (IP[:PORT]).
    #[arg(long = "peer", value_delimiter = ' ', num_args = 1..)]
    #[serde(default)]
    pub peer: Vec<SocketAddrProxy>,

    /// File to watch for connecting to the ring.
    #[arg(long = "peer-watch-file", conflicts_with = "peer")]
    #[serde(default)]
    pub peer_watch_file: Option<PathBuf>,

    /// Start in local gossip mode.
    #[arg(long = "local-gossip-mode",
        conflicts_with_all = &["listen_gossip", "peer", "peer_watch_file"])]
    #[serde(default)]
    pub local_gossip_mode: bool,

    /// The listen address for the HTTP Gateway.
    #[arg(long = "listen-http",
        env = HttpListenAddr::ENVVAR,
        default_value = HttpListenAddr::default_as_str())]
    #[serde(default)]
    pub listen_http: HttpListenAddr,

    /// Disable the HTTP Gateway.
    #[arg(long = "http-disable", short = 'D')]
    #[serde(default)]
    pub http_disable: bool,

    /// The listen address for the Control Gateway.
    #[arg(long = "listen-ctl",
        env = ListenCtlAddr::ENVVAR,
        default_value = ListenCtlAddr::default_as_str())]
    #[serde(default)]
    pub listen_ctl: ResolvedListenCtlAddr,

    /// The control gateway server's TLS certificate.
    #[arg(long = "ctl-server-certificate", default_missing_value = HAB_CTL_KEYS_CACHE, num_args = 0..)]
    pub ctl_server_certificate: Option<CertificateChainCli>,

    /// The control gateway server's private key.
    #[arg(long = "ctl-server-key", default_missing_value = HAB_CTL_KEYS_CACHE, num_args = 0..)]
    pub ctl_server_key: Option<PrivateKeyCli>,

    /// Enable client authentication for the control gateway and set the certificate authority to
    /// use when authenticating the client.
    #[arg(long = "ctl-client-ca-certificate", default_missing_value = HAB_CTL_KEYS_CACHE, num_args = 0..)]
    pub ctl_client_ca_certificate: Option<RootCertificateStoreCli>,

    /// Organization the Supervisor and it's services are part of.
    #[arg(long = "org")]
    pub organization: Option<String>,

    /// Mark the Supervisor as a permanent peer.
    #[arg(long = "permanent-peer", short = 'I')]
    #[serde(default)]
    pub permanent_peer: bool,

    /// Flattened cache key configuration.
    #[command(flatten)]
    #[serde(flatten)]
    #[serde(default)]
    pub cache_key_path: CacheKeyPath,

    /// The contents of the ring key when running with wire encryption
    ///
    /// This option is explicitly undocumented and for testing purposes only. Do not use it in a
    /// production system. Use the corresponding environment variable instead. (ex:
    /// 'SYM-SEC-1\nfoo-20181113185935\n\nGCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')
    #[arg(long = "ring-key", env = RING_KEY_ENVVAR, hide = true)]
    pub ring_key: Option<String>,

    /// The name of the ring used by the Supervisor when running with wire encryption
    #[arg(long = "ring", short = 'r',
            env = RING_ENVVAR,
            conflicts_with = "ring_key")]
    pub ring: Option<String>,

    /// Enable automatic updates for the Supervisor itself
    #[arg(long = "auto-update", short = 'A')]
    #[serde(default)]
    pub auto_update: bool,

    /// Time (seconds) between Supervisor update checks.
    #[arg(long = "auto-update-period", default_value = "60")]
    #[serde(default = "DurationProxy::from_60")]
    pub auto_update_period: DurationProxy,

    /// Time (seconds) between service update checks.
    #[arg(long = "service-update-period", default_value = "60")]
    #[serde(default = "DurationProxy::from_60")]
    pub service_update_period: DurationProxy,

    /// The minimum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[arg(long = "service-min-backoff-period", default_value = "0")]
    #[serde(default = "DurationProxy::from_0")]
    pub service_min_backoff_period: DurationProxy,

    /// The maximum period of time in seconds to wait before attempting to restart a service
    /// that failed to start up
    #[arg(long = "service-max-backoff-period", default_value = "0")]
    #[serde(default = "DurationProxy::from_0")]
    pub service_max_backoff_period: DurationProxy,

    /// The period of time in seconds to wait before assuming that a service started up
    /// successfully after a restart
    #[arg(long = "service-restart-cooldown-period", default_value = "300")]
    #[serde(default = "DurationProxy::from_300")]
    pub service_restart_cooldown_period: DurationProxy,

    /// The private key for HTTP Gateway TLS encryption
    ///
    /// Read the private key from KEY_FILE. This should be an RSA private key or PKCS8-encoded
    /// private key in PEM format.
    #[arg(long = "key")]
    pub key_file: Option<PathBuf>,

    /// The server certificates for HTTP Gateway TLS encryption
    ///
    /// Read server certificates from CERT_FILE. This should contain PEM-format certificates in
    /// the right order. The first certificate should certify KEY_FILE. The last should be a
    /// root CA.
    #[arg(long = "certs", requires = "key_file")]
    pub cert_file: Option<PathBuf>,

    /// The CA certificate for HTTP Gateway TLS encryption
    ///
    /// Read the CA certificate from CA_CERT_FILE. This should contain PEM-format certificate that
    /// can be used to validate client requests
    #[arg(long = "ca-certs", requires_all = &["cert_file", "key_file"])]
    pub ca_cert_file: Option<PathBuf>,

    /// Load a Habitat package as part of the Supervisor startup
    ///
    /// The package can be specified by a package identifier (ex: core/redis) or filepath to a
    /// Habitat artifact (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart).
    pub pkg_ident_or_artifact: Option<InstallSource>,

    /// Verbose output showing file and line/column numbers
    #[arg(short = 'v')]
    #[serde(default)]
    pub verbose: bool,

    /// Disable ANSI color.
    #[arg(long = "no-color")]
    #[serde(default)]
    pub no_color: bool,

    /// Use structured JSON logging for the Supervisor
    ///
    /// This option also sets NO_COLOR.
    #[arg(long = "json-logging")]
    #[serde(default)]
    pub json_logging: bool,

    /// The IPv4 address to use as the `sys.ip` template variable
    ///
    /// If this argument is not set, the supervisor tries to dynamically determine an IP address.
    /// If that fails, the supervisor defaults to using `127.0.0.1`.
    #[arg(long = "sys-ip-address")]
    pub sys_ip_address: Option<IpAddr>,

    /// The name of the application for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[arg(long = "event-stream-application", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub event_stream_application: Option<String>,

    /// The name of the environment for event stream purposes
    ///
    /// This will be attached to all events generated by this Supervisor.
    #[arg(long = "event-stream-environment", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub event_stream_environment: Option<String>,

    /// Event stream connection timeout before exiting the Supervisor
    ///
    /// Set to '0' to immediately start the Supervisor and continue running regardless of the
    /// initial connection status.
    #[arg(long = "event-stream-connect-timeout", env = EventStreamConnectMethod::ENVVAR, default_value = "0")]
    #[serde(default = "EventStreamConnectMethod::from_0")]
    pub event_stream_connect_timeout: EventStreamConnectMethod,

    /// The authentication token for connecting the event stream to Chef Automate
    #[arg(long = "event-stream-token", env = EventStreamToken::ENVVAR, hide_env_values = true)]
    pub event_stream_token: Option<EventStreamToken>,

    /// The event stream connection url used to send events to Chef Automate
    ///
    /// This enables the event stream and requires EVENT_STREAM_APPLICATION,
    /// EVENT_STREAM_ENVIRONMENT, and EVENT_STREAM_TOKEN also be set.
    #[arg(long = "event-stream-url",
            requires_all = &["event_stream_application",
                            "event_stream_environment",
                            "event_stream_token"])]
    pub event_stream_url: Option<EventStreamAddress>,

    /// The name of the site where this Supervisor is running for event stream purposes
    #[arg(long = "event-stream-site", value_parser = clap::builder::NonEmptyStringValueParser::new())]
    pub event_stream_site: Option<String>,

    /// An arbitrary key-value pair to add to each event generated by this Supervisor
    #[arg(long = "event-meta", value_delimiter = ' ', num_args = 1..)]
    #[serde(default)]
    pub event_meta: Vec<EventStreamMetaPair>,

    //// The path to Chef Automate's event stream certificate used to establish a TLS connection
    /// The certificate should be in PEM format.
    #[arg(long = "event-stream-server-certificate")]
    #[serde(default)]
    pub event_stream_server_certificate: Option<EventStreamServerCertificate>,

    /// Automatically cleanup old packages
    ///
    /// The Supervisor will automatically cleanup old packages only keeping the
    /// KEEP_LATEST_PACKAGES latest packages. If this argument is not specified, no
    /// automatic package cleanup is performed.
    #[arg(long = "keep-latest-packages", env = "HAB_KEEP_LATEST_PACKAGES")]
    pub keep_latest_packages: Option<usize>,

    /// Paths to config files to Read
    #[arg(long = "config-files", value_delimiter=' ', num_args = 1.., value_parser = FileExistsValueParser)]
    #[serde(skip)]
    pub config_files: Vec<PathBuf>,

    /// Generate a TOML Config
    #[arg(long = "generate-config")]
    #[serde(skip)]
    pub generate_config: bool,

    #[command(flatten)]
    #[serde(flatten)]
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

impl SupRunOptions {
    /// Merge all the possible configurations
    ///
    /// If the `config-files is non-empty, use those. If the config-files is empty, try to see if
    /// the default config file "/hab/sup/default/config/sup.toml" exists. If that is missing too,
    /// just return the passed 'other' as it is after cleaning the 'config-files' value if any.
    pub fn maybe_merge_from_config_files(mut other: Self) -> HabResult<Self> {
        let config_files = if other.config_files.is_empty() {
            let default_config_path = Path::new("/hab/sup/default/config/sup.toml");
            if default_config_path.exists() && default_config_path.is_file() {
                vec!["/hab/sup/default/config/sup.toml"].into_iter()
                                                        .map(Into::<PathBuf>::into)
                                                        .collect::<Vec<PathBuf>>()
            } else {
                vec![]
            }
        } else {
            other.config_files.drain(..).collect::<Vec<PathBuf>>()
        };

        // We iterate in reverse order so that values from the *last* config file in the list
        // take precedence. This is necessary because the `merge` implementation only sets a field
        // if it is currently at its default value; once a field is set to a non-default value, it
        // will not be overwritten by subsequent merges. For example, if you have two config files:
        //   - config1.toml: { port = 8080 }
        //   - config2.toml: { port = 9090 }
        // and you specify them as [config1.toml, config2.toml], iterating in reverse means
        // config2.toml is merged first, setting port to 9090, and then config1.toml is merged,
        // but since port is already set (non-default), it is not overwritten. Thus, the value from
        // the last file in the list is preserved, matching user expectations for "last one wins".
        for config_file in config_files.into_iter().rev() {
            if is_toml_file(config_file.as_os_str().to_str().unwrap()) {
                let inner_object: Self =
                    toml::from_str(&fs::read_to_string(config_file).expect("File cannot be read"))?;
                other.merge(inner_object);
            } else {
                return Err(HabError::ArgumentError(format!("'{}' is not a valid \
                                                            toml config file.",
                                                           config_file.display())));
            }
        }

        Ok(other)
    }

    #[allow(clippy::cognitive_complexity)]
    fn merge(&mut self, other: Self) {
        if self.listen_gossip == GossipListenAddr::default() {
            self.listen_gossip = other.listen_gossip;
        }
        if self.peer.is_empty() {
            self.peer.extend_from_slice(&other.peer);
        }
        if self.peer_watch_file.is_none() {
            self.peer_watch_file = other.peer_watch_file;
        }

        self.local_gossip_mode |= other.local_gossip_mode;

        if self.listen_http == HttpListenAddr::default() {
            self.listen_http = other.listen_http;
        }
        self.http_disable |= other.http_disable;

        if self.listen_ctl == ResolvedListenCtlAddr::default() {
            self.listen_ctl = other.listen_ctl;
        }

        if self.ctl_server_certificate.is_none() {
            self.ctl_server_certificate = other.ctl_server_certificate;
        }

        if self.ctl_server_key.is_none() {
            self.ctl_server_key = other.ctl_server_key;
        }

        if self.ctl_client_ca_certificate.is_none() {
            self.ctl_client_ca_certificate = other.ctl_client_ca_certificate;
        }

        if self.organization.is_none() {
            self.organization = other.organization;
        }
        self.permanent_peer |= other.permanent_peer;

        if self.cache_key_path == CacheKeyPath::default() {
            self.cache_key_path = other.cache_key_path;
        }

        if self.ring_key.is_none() {
            self.ring_key = other.ring_key;
        }

        if self.ring.is_none() {
            self.ring = other.ring;
        }

        self.auto_update |= other.auto_update;

        if self.auto_update_period == 60_u64.into() {
            self.auto_update_period = other.auto_update_period;
        }

        if self.service_update_period == 60_u64.into() {
            self.service_update_period = other.service_update_period;
        }

        if self.service_min_backoff_period == 0_u64.into() {
            self.service_min_backoff_period = other.service_min_backoff_period;
        }

        if self.service_max_backoff_period == 0_u64.into() {
            self.service_max_backoff_period = other.service_max_backoff_period;
        }

        if self.service_restart_cooldown_period == 300_u64.into() {
            self.service_restart_cooldown_period = other.service_restart_cooldown_period;
        }

        if self.key_file.is_none() {
            self.key_file = other.key_file;
        }

        if self.cert_file.is_none() {
            self.cert_file = other.cert_file;
        }

        if self.ca_cert_file.is_none() {
            self.ca_cert_file = other.ca_cert_file;
        }

        if self.pkg_ident_or_artifact.is_none() {
            self.pkg_ident_or_artifact = other.pkg_ident_or_artifact;
        }

        self.verbose |= other.verbose;

        self.no_color |= other.no_color;

        self.json_logging |= other.json_logging;

        if self.sys_ip_address.is_none() {
            self.sys_ip_address = other.sys_ip_address;
        }

        if self.event_stream_application.is_none() {
            self.event_stream_application = other.event_stream_application;
        }

        if self.event_stream_token.is_none() {
            self.event_stream_token = other.event_stream_token;
        }

        if self.event_stream_environment.is_none() {
            self.event_stream_environment = other.event_stream_environment;
        }

        if self.event_stream_connect_timeout == 0_u64.into() {
            self.event_stream_connect_timeout = other.event_stream_connect_timeout;
        }

        if self.event_stream_url.is_none() {
            self.event_stream_url = other.event_stream_url;
        }

        if self.event_stream_site.is_none() {
            self.event_stream_site = other.event_stream_site;
        }

        if self.event_meta.is_empty() {
            self.event_meta.extend_from_slice(&other.event_meta);
        }

        if self.event_stream_server_certificate.is_none() {
            self.event_stream_server_certificate = other.event_stream_server_certificate;
        }

        if self.keep_latest_packages.is_none() {
            self.keep_latest_packages = other.keep_latest_packages;
        }

        self.config_files.clear();

        if self.shared_load == SharedLoad::default() {
            self.shared_load = other.shared_load;
        }
    }
}
