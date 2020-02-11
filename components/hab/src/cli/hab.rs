mod bldr;
mod cli;
mod config;
mod file;
mod license;
mod origin;
mod pkg;
mod plan;
mod ring;
#[cfg(test)]
mod tests;
mod util;

use self::{bldr::Bldr,
           cli::Cli,
           config::ServiceConfig,
           file::File,
           license::License,
           origin::Origin,
           pkg::Pkg,
           plan::Plan,
           ring::Ring};
use crate::VERSION;
use configopt::{ConfigOptDefaults,
                Partial};
use habitat_common::{cli::{RING_ENVVAR,
                           RING_KEY_ENVVAR},
                     types::{GossipListenAddr,
                             HttpListenAddr,
                             ListenCtlAddr}};
use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   env::Config,
                   fs::CACHE_KEY_PATH,
                   service::HealthCheckInterval};
use std::{net::{Ipv4Addr,
                SocketAddr},
          path::PathBuf};
use structopt::{clap::AppSettings,
                StructOpt};
use url::Url;

#[derive(StructOpt)]
#[structopt(name = "hab",
            version = VERSION,
            about = "\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            global_settings = &[AppSettings::GlobalVersion],
        )]
pub enum Hab {
    #[structopt(no_version)]
    Bldr(Bldr),
    #[structopt(no_version)]
    Cli(Cli),
    #[structopt(no_version)]
    Config(ServiceConfig),
    #[structopt(no_version)]
    File(File),
    #[structopt(no_version)]
    License(License),
    #[structopt(no_version)]
    Origin(Origin),
    #[structopt(no_version)]
    Pkg(Pkg),
    #[structopt(no_version)]
    Plan(Plan),
    #[structopt(no_version)]
    Ring(Ring),
    /// Commands relating to Habitat Studios
    #[structopt(no_version)]
    Studio,
    /// The Habitat Supervisor
    #[structopt(no_version)]
    Sup,
    /// Create a tarball of Habitat Supervisor data to send to support
    #[structopt(no_version)]
    Supportbundle,
    /// Commands relating to Habitat services
    #[structopt(no_version)]
    Svc,
    /// Commands relating to Habitat users
    #[structopt(no_version)]
    User,
}

#[derive(ConfigOptDefaults, Partial, StructOpt, Deserialize)]
#[configopt_defaults(type = "PartialSubSupRun")]
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
pub struct SubSupRun {
    /// The listen address for the Gossip System Gateway
    #[structopt(name = "LISTEN_GOSSIP",
                long = "listen-gossip",
                env = GossipListenAddr::ENVVAR,
                default_value = GossipListenAddr::default_as_str())]
    listen_gossip:         SocketAddr,
    /// Start the supervisor in local mode
    #[structopt(name = "LOCAL_GOSSIP_MODE",
                long = "local-gossip-mode",
                conflicts_with_all = &["LISTEN_GOSSIP", "PEER", "PEER_WATCH_FILE"])]
    local_gossip_mode:     bool,
    /// The listen address for the HTTP Gateway
    #[structopt(name = "LISTEN_HTTP",
                long = "listen-http",
                env = HttpListenAddr::ENVVAR,
                default_value = HttpListenAddr::default_as_str())]
    listen_http:           SocketAddr,
    /// Disable the HTTP Gateway completely
    #[structopt(name = "HTTP_DISABLE", long = "http-disable", short = "D")]
    http_disable:          bool,
    /// The listen address for the Control Gateway. If not specified, the value will be taken from
    /// the HAB_LISTEN_CTL environment variable if defined
    #[structopt(name = "LISTEN_CTL",
                long = "listen-ctl",
                env = ListenCtlAddr::ENVVAR,
                default_value = ListenCtlAddr::default_as_str())]
    listen_ctl:            SocketAddr,
    /// The organization that the Supervisor and its subsequent services are part of
    #[structopt(name = "ORGANIZATION", long = "org")]
    organization:          Option<String>,
    /// The listen address of one or more initial peers (IP[:PORT])
    #[structopt(name = "PEER", long = "peer")]
    // TODO (DM): This could probably be a different type for better validation (Vec<SockAddr>?)
    peer:                  Vec<String>,
    /// If this Supervisor is a permanent peer
    #[structopt(name = "PERMANENT_PEER", long = "permanent-peer", short = "I")]
    permanent_peer:        bool,
    /// Watch this file for connecting to the ring
    #[structopt(name = "PEER_WATCH_FILE",
                long = "peer-watch-file",
                conflicts_with = "PEER")]
    peer_watch_file:       PathBuf,
    /// Path to search for encryption keys. Default value is hab/cache/keys if root and
    /// .hab/cache/keys under the home directory otherwise
    // TODO (DM): I dont think the default value comment is correct. It looks like it always is set
    // to hab/cache/keys.
    #[structopt(name = "CACHE_KEY_PATH",
                long = "cache-key-path",
                env = CACHE_KEY_PATH_ENV_VAR,
                default_value = CACHE_KEY_PATH,
                hide_default_value = true)]
    cache_key_path:        PathBuf,
    /// The name of the ring used by the Supervisor when running with wire encryption. (ex: hab sup
    /// run --ring myring)
    #[structopt(name = "RING",
                long = "ring",
                short = "r",
                env = RING_ENVVAR,
                conflicts_with = "RING_KEY")]
    ring:                  String,
    /// The contents of the ring key when running with wire encryption. (Note: This option is
    /// explicitly undocumented and for testing purposes only. Do not use it in a production
    /// system. Use the corresponding environment variable instead.) (ex: hab sup run --ring-key
    /// 'SYM-SEC-1 foo-20181113185935GCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')
    #[structopt(name = "RING_KEY",
                long = "ring-key",
                env = RING_KEY_ENVVAR,
                hidden = true,
                conflicts_with = "RING")]
    ring_key:              Option<String>,
    /// Receive Supervisor updates from the specified release channel
    #[structopt(name = "CHANNEL", long = "channel", default_value = "stable")]
    channel:               String,
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from the
    /// HAB_BLDR_URL environment variable if defined (default: https://bldr.habitat.sh)
    #[structopt(name = "BLDR_URL",
                long = "url",
                short = "u",
                // TODO (DM): These fields are not actual set in the clap macro but I think they should
                // env = BLDR_URL_ENVVAR,
                // default_value = DEFAULT_BLDR_URL
            )]
    bldr_url:              Url,
    /// Use package config from this path, rather than the package itself
    #[structopt(name = "CONFIG_DIR", long = "config-from")]
    config_dir:            Option<PathBuf>,
    /// Enable automatic updates for the Supervisor itself
    #[structopt(name = "AUTO_UPDATE", long = "auto-update", short = "A")]
    auto_update:           bool,
    /// Used for enabling TLS for the HTTP gateway. Read private key from KEY_FILE. This should be
    /// a RSA private key or PKCS8-encoded private key, in PEM format
    #[structopt(name = "KEY_FILE", long = "key", requires = "CERT_FILE")]
    key_file:              Option<PathBuf>,
    /// Used for enabling TLS for the HTTP gateway. Read server certificates from CERT_FILE. This
    /// should contain PEM-format certificates in the right order (the first certificate should
    /// certify KEY_FILE, the last should be a root CA)
    #[structopt(name = "CERT_FILE", long = "certs", requires = "KEY_FILE")]
    cert_file:             Option<PathBuf>,
    /// Used for enabling client-authentication with TLS for the HTTP gateway. Read CA certificate
    /// from CA_CERT_FILE. This should contain PEM-format certificate that can be used to validate
    /// client requests
    #[structopt(name = "CA_CERT_FILE",
                long = "ca-certs",
                requires_all = &["CERT_FILE", "KEY_FILE"])]
    ca_cert_file:          Option<PathBuf>,
    /// Load the given Habitat package as part of the Supervisor startup specified by a package
    /// identifier (ex: core/redis) or filepath to a Habitat Artifact (ex:
    /// /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart)
    // TODO (DM): We could probably do better validation here
    #[structopt(name = "PKG_IDENT_OR_ARTIFACT")]
    pkg_ident_or_artifact: Option<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    #[structopt(name = "APPLICATION", long = "application", hidden = true)]
    application:           Vec<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    #[structopt(name = "ENVIRONMENT", long = "environment", hidden = true)]
    environment:           Vec<String>,
    /// The service group; shared config and topology [default: default]
    // TODO (DM): This should set a default value
    #[structopt(name = "GROUP", long = "group")]
    group:                 String,
    /// Service topology; [default: none]
    // TODO (DM): I dont think saying the default is none makes sense here
    #[structopt(name = "TOPOLOGY",
                long = "topology",
                short = "t",
                possible_values = &["standalone", "leader"])]
    topology:              Option<habitat_sup_protocol::types::Topology>,
    /// The update strategy; [default: none] [values: none, at-once, rolling]
    // TODO (DM): this should set a default_value and use possible_values = &["none", "at-once",
    // "rolling"]
    #[structopt(name = "STRATEGY", long = "strategy", short = "s")]
    strategy:              Option<habitat_sup_protocol::types::UpdateStrategy>,
    /// One or more service groups to bind to a configuration
    #[structopt(name = "BIND", long = "bind")]
    bind:                  Vec<String>,
    /// Governs how the presence or absence of binds affects service startup. `strict` blocks
    /// startup until all binds are present. [default: strict] [values: relaxed, strict]
    // TODO (DM): This should set default_value and use possible_values
    #[structopt(name = "BINDING_MODE", long = "binding-mode")]
    binding_mode:          Option<habitat_sup_protocol::types::BindingMode>,
    /// Verbose output; shows file and line/column numbers
    #[structopt(name = "VERBOSE", short = "v")]
    verbose:               bool,
    /// Turn ANSI color off
    #[structopt(name = "NO_COLOR", long = "no-color")]
    no_color:              bool,
    /// Use structured JSON logging for the Supervisor. Implies NO_COLOR
    #[structopt(name = "JSON", long = "json-logging")]
    json_logging:          bool,
    /// The interval (seconds) on which to run health checks [default: 30]
    // TODO (DM): Should use default_value = "30"
    #[structopt(name = "HEALTH_CHECK_INTERVAL",
                long = "health-check-interval",
                short = "i")]
    health_check_interval: HealthCheckInterval,
    /// The IPv4 address to use as the `sys.ip` template variable. If this argument is not set, the
    /// supervisor tries to dynamically determine an IP address. If that fails, the supervisor
    /// defaults to using `127.0.0.1`
    #[structopt(name = "SYS_IP_ADDRESS", long = "sys-ip-address")]
    sys_ip_address:        Option<Ipv4Addr>,
}
