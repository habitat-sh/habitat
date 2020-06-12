use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath,
                  ConfigOptPkgIdent,
                  ConfigOptRemoteSup,
                  PkgIdent,
                  RemoteSup};
use configopt::ConfigOpt;
use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::{ServiceBind,
                             ServiceGroup},
                   ChannelIdent};
use habitat_sup_protocol::types::UpdateCondition;
use std::path::PathBuf;
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
        /// The delay in seconds after sending the shutdown signal to wait before killing the
        /// service process
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
        /// The delay in seconds after sending the shutdown signal to wait before killing the
        /// service process
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
    pub channel:               ChannelIdent,
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    // TODO (DM): Nested flattens do no work
    #[structopt(name = "BLDR_URL", short = "u", long = "url")]
    pub bldr_url:              Option<Url>,
    /// The service group with shared config and topology
    #[structopt(long = "group", default_value = "default")]
    pub group:                 String,
    /// Service topology
    #[structopt(long = "topology",
            short = "t",
            possible_values = &["standalone", "leader"])]
    pub topology:              Option<habitat_sup_protocol::types::Topology>,
    /// The update strategy
    #[structopt(long = "strategy",
                short = "s",
                default_value = "none",
                possible_values = &["none", "at-once", "rolling"])]
    pub strategy:              habitat_sup_protocol::types::UpdateStrategy,
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
    #[structopt(long = "update-condition",
                default_value = UpdateCondition::Latest.as_str(),
                possible_values = UpdateCondition::VARIANTS)]
    pub update_condition:      UpdateCondition,
    /// One or more service groups to bind to a configuration
    #[structopt(long = "bind")]
    #[serde(default)]
    pub bind:                  Vec<ServiceBind>,
    /// Governs how the presence or absence of binds affects service startup
    ///
    /// strict: blocks startup until all binds are present.
    #[structopt(long = "binding-mode",
                default_value = "strict",
                possible_values = &["strict", "relaxed"])]
    pub binding_mode:          habitat_sup_protocol::types::BindingMode,
    /// The interval in seconds on which to run health checks
    // We would prefer to use `HealthCheckInterval`. However, `HealthCheckInterval` uses a map based
    // serialization format. We want to allow the user to simply specify a `u64` to be consistent
    // with the CLI, but we cannot change the serialization because the spec file depends on the map
    // based format.
    #[structopt(long = "health-check-interval", short = "i", default_value = "30")]
    pub health_check_interval: u64,
    /// The delay in seconds after sending the shutdown signal to wait before killing the service
    /// process
    ///
    /// The default value can be set in the packages plan file.
    #[structopt(long = "shutdown-timeout")]
    pub shutdown_timeout:      Option<ShutdownTimeout>,
    #[cfg(target_os = "windows")]
    /// Password of the service user
    #[structopt(long = "password")]
    pub password:              Option<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(long = "application", short = "a", takes_value = false, hidden = true)]
    #[serde(skip)]
    pub application:           Vec<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(long = "environment", short = "e", takes_value = false, hidden = true)]
    #[serde(skip)]
    pub environment:           Vec<String>,
    /// Use the package config from this path rather than the package itself
    #[structopt(long = "config-from")]
    pub config_from:           Option<PathBuf>,
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
    #[structopt(flatten)]
    #[serde(flatten)]
    shared_load: SharedLoad,
}
