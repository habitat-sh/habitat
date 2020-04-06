use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath,
                  ConfigOptPkgIdent,
                  ConfigOptRemoteSup,
                  PkgIdent,
                  RemoteSup};
use configopt::{configopt_fields,
                ConfigOpt};
use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::{HealthCheckInterval,
                             ServiceGroup}};
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
        /// The number of seconds after sending a shutdown signal to wait before killing a service
        /// process (default: set in plan)
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
        /// The number of seconds after sending a shutdown signal to wait before killing a service
        /// process (default: set in plan)
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

#[derive(ConfigOpt, StructOpt, Deserialize)]
#[configopt(attrs(serde))]
#[serde(deny_unknown_fields)]
#[structopt(no_version)]
#[allow(dead_code)]
pub struct SharedLoad {
    /// Receive updates from the specified release channel
    #[structopt(name = "CHANNEL", long = "channel", default_value = "stable")]
    channel:               String,
    /// Specify an alternate Builder endpoint. If not specified, the value will be taken from
    /// the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    // TODO (DM): This should probably use `env` and `default_value`
    // TODO (DM): Nested flattens do no work
    #[structopt(name = "BLDR_URL", short = "u", long = "url")]
    bldr_url:              Option<Url>,
    /// The service group; shared config and topology [default: default]
    // TODO (DM): This should set a default value
    #[structopt(name = "GROUP", long = "group")]
    group:                 Option<String>,
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
    update_condition:      UpdateCondition,
    /// One or more service groups to bind to a configuration
    #[structopt(name = "BIND", long = "bind")]
    #[serde(default)]
    bind:                  Vec<String>,
    /// Governs how the presence or absence of binds affects service startup. `strict` blocks
    /// startup until all binds are present. [default: strict] [values: relaxed, strict]
    // TODO (DM): This should set default_value and use possible_values
    #[structopt(name = "BINDING_MODE", long = "binding-mode")]
    binding_mode:          Option<habitat_sup_protocol::types::BindingMode>,
    /// The interval (seconds) on which to run health checks [default: 30]
    // TODO (DM): Should use default_value = "30"
    #[structopt(name = "HEALTH_CHECK_INTERVAL",
                long = "health-check-interval",
                short = "i")]
    health_check_interval: Option<HealthCheckInterval>,
    /// The number of seconds after sending a shutdown signal to wait before killing a service
    /// process (default: set in plan)
    #[structopt(name = "SHUTDOWN_TIMEOUT", long = "shutdown-timeout")]
    shutdown_timeout:      Option<ShutdownTimeout>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(name = "APPLICATION",
                long = "application",
                short = "a",
                takes_value = false,
                hidden = true)]
    #[serde(skip)]
    application:           Vec<String>,
    // TODO (DM): This flag can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    /// DEPRECATED
    #[structopt(name = "ENVIRONMENT",
                long = "environment",
                short = "e",
                takes_value = false,
                hidden = true)]
    #[serde(skip)]
    environment:           Vec<String>,
}

#[configopt_fields]
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
