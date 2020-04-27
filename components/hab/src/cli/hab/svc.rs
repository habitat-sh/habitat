use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath,
                  ConfigOptPkgIdent,
                  ConfigOptRemoteSup,
                  PkgIdent,
                  RemoteSup};
use configopt::ConfigOpt;
use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::{HealthCheckInterval,
                             ServiceGroup},
                   ChannelIdent};
use habitat_sup_protocol::types::UpdateCondition;
use structopt::StructOpt;
use url::Url;

/// Commands relating to Habitat services
#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
#[allow(clippy::large_enum_variant)]
pub enum Svc {
    Key(Key),
    /// Load a service to be started and supervised by Habitat from a package identifier. If an
    /// installed package doesn't satisfy the given package identifier, a suitable package will be
    /// installed from Builder.
    #[structopt(no_version)]
    Load(Load),
    /// Start a loaded, but stopped, Habitat service.
    Start {
        #[structopt(flatten)]
        pkg_ident:  PkgIdent,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    /// Query the status of Habitat services
    Status {
        /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
        #[structopt(name = "PKG_IDENT")]
        pkg_ident:  Option<PackageIdent>,
        #[structopt(flatten)]
        remote_sup: RemoteSup,
    },
    /// Stop a running Habitat service.
    Stop {
        #[structopt(flatten)]
        pkg_ident:        PkgIdent,
        #[structopt(flatten)]
        remote_sup:       RemoteSup,
        /// The delay (seconds) after sending the shutdown signal to wait before killing a service
        /// process
        ///
        /// The default value is set in the packages plan file.
        #[structopt(name = "SHUTDOWN_TIMEOUT", long = "shutdown-timeout")]
        shutdown_timeout: Option<ShutdownTimeout>,
    },
    /// Unload a service loaded by the Habitat Supervisor. If the service is running it will
    /// additionally be stopped.
    Unload {
        #[structopt(flatten)]
        pkg_ident:        PkgIdent,
        #[structopt(flatten)]
        remote_sup:       RemoteSup,
        /// The delay (seconds) after sending the shutdown signal to wait before killing a service
        /// process
        ///
        /// The default value is set in the packages plan file.
        #[structopt(name = "SHUTDOWN_TIMEOUT", long = "shutdown-timeout")]
        shutdown_timeout: Option<ShutdownTimeout>,
    },
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat service keys
pub enum Key {
    /// Generates a Habitat service key
    Generate {
        /// Target service group service.group[@organization] (ex: redis.default or
        /// foo.default@bazcorp)
        #[structopt(name = "SERVICE_GROUP")]
        service_group:  ServiceGroup,
        /// The service organization
        #[structopt(name = "ORG")]
        org:            Option<String>,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}

lazy_static::lazy_static! {
    static ref CHANNEL_IDENT_DEFAULT: String = String::from(ChannelIdent::default().as_str());
}

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[configopt(attrs(serde))]
#[serde(deny_unknown_fields)]
#[structopt(no_version, rename_all = "screamingsnake")]
#[allow(dead_code)]
pub struct SharedLoad {
    /// Receive updates from the specified release channel
    #[structopt(long = "channel", default_value = &*CHANNEL_IDENT_DEFAULT)]
    channel:               ChannelIdent,
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    // TODO (DM): Nested flattens do no work
    #[structopt(name = "BLDR_URL", short = "u", long = "url")]
    bldr_url:              Option<Url>,
    /// The service group with shared config and topology
    #[structopt(long = "group", default_value = "default")]
    group:                 String,
    /// Service topology
    #[structopt(long = "topology",
            short = "t",
            possible_values = &["standalone", "leader"])]
    topology:              Option<habitat_sup_protocol::types::Topology>,
    /// The update strategy
    // TODO (DM): possible_values = &["none", "at-once", "rolling"]
    #[structopt(long = "strategy", short = "s", default_value = "none")]
    strategy:              habitat_sup_protocol::types::UpdateStrategy,
    /// The condition dictating when this service should update
    ///
    /// `latest`: Runs the latest package that can be found in the configured channel and local
    /// packages.
    ///
    /// `track-channel`: Always run what is at the head of a given channel. This enables service
    /// rollback where demoting a package from a channel will cause the package to rollback to
    /// an older version of the package. A ramification of enabling this condition is packages
    /// newer than the package at the head of the channel will be automatically uninstalled
    /// during a service rollback.
    #[structopt(long = "update-condition",
                default_value = UpdateCondition::Latest.as_str(),
                possible_values = UpdateCondition::VARIANTS)]
    update_condition:      UpdateCondition,
    /// One or more service groups to bind to a configuration
    // TODO (DM): Can this be a better type?
    #[structopt(long = "bind")]
    #[serde(default)]
    bind:                  Vec<String>,
    /// Governs how the presence or absence of binds affects service startup
    ///
    /// `strict`: blocks startup until all binds are present.
    // TODO (DM): possible_values = &["none", "strict", "relaxed"]
    #[structopt(long = "binding-mode", default_value = "strict")]
    binding_mode:          Option<habitat_sup_protocol::types::BindingMode>,
    /// The interval (seconds) on which to run health checks
    // TODO (DM): Add this default value
    #[structopt(long = "health-check-interval", short = "i")]
    health_check_interval: Option<HealthCheckInterval>,
    /// The delay (seconds) after sending the shutdown signal to wait before killing a service
    /// process
    ///
    /// The default value is set in the packages plan file.
    #[structopt(long = "shutdown-timeout")]
    shutdown_timeout:      Option<ShutdownTimeout>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(long = "application", short = "a", takes_value = false, hidden = true)]
    #[serde(skip)]
    application:           Vec<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(long = "environment", short = "e", takes_value = false, hidden = true)]
    #[serde(skip)]
    environment:           Vec<String>,
}

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[configopt(attrs(serde))]
#[serde(deny_unknown_fields)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct Load {
    #[structopt(flatten)]
    #[serde(flatten)]
    pkg_ident:   PkgIdent,
    /// Load or reload an already loaded service. If the service was previously loaded and
    /// running this operation will also restart the service
    #[structopt(name = "FORCE", short = "f", long = "force")]
    force:       bool,
    #[structopt(flatten)]
    #[serde(flatten)]
    remote_sup:  RemoteSup,
    #[cfg(target_os = "windows")]
    /// Password of the service user
    #[structopt(name = "PASSWORD", long = "password")]
    password:    Option<String>,
    #[structopt(flatten)]
    #[serde(flatten)]
    shared_load: SharedLoad,
}
