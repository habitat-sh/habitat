use clap_v4 as clap;

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;

use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::HealthCheckInterval,
                   ChannelIdent};
use habitat_sup_protocol::types::{BindingMode,
                                  ServiceBind,
                                  Topology,
                                  UpdateCondition,
                                  UpdateStrategy};

use crate::cli_v4::utils::{BldrUrl,
                           RemoteSup};

/// Update how the Supervisor manages an already-running service.
///
/// Depending on the given changes, they may be able to be applied without restarting the service.
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub struct UpdateCommand {
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    pub remote_sup: RemoteSup,

    // This is some unfortunate duplication... everything below this
    // should basically be identical to SharedLoad, except that we
    // don't want to have default values, and everything should be
    // optional.
    /// Receive updates from the specified release channel
    #[arg(long = "channel")]
    pub channel: Option<ChannelIdent>,

    /// Specify an alternate Builder endpoint.
    #[command(flatten)]
    pub bldr_url: Option<BldrUrl>,

    /// The service group with shared config and topology
    #[arg(long = "group")]
    pub group: Option<String>,

    /// Service topology
    #[arg(long = "topology", short = 't')]
    pub topology: Option<Topology>,

    /// The update strategy
    #[arg(long = "strategy", short = 's')]
    pub strategy: Option<UpdateStrategy>,

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
    #[arg(long = "update-condition", default_value=UpdateCondition::Latest.as_str())]
    pub update_condition: Option<UpdateCondition>,

    /// One or more service groups to bind to a configuration
    #[arg(long = "bind")]
    pub bind: Option<Vec<ServiceBind>>,

    /// Governs how the presence or absence of binds affects service startup
    ///
    /// strict: blocks startup until all binds are present.
    #[arg(long = "binding-mode")]
    pub binding_mode: Option<BindingMode>,

    /// The interval in seconds on which to run health checks
    // We can use `HealthCheckInterval` here (cf. `SharedLoad` above),
    // because we don't have to worry about serialization here.
    #[arg(long = "health-check-interval", short = 'i')]
    pub health_check_interval: Option<HealthCheckInterval>,

    /// The delay in seconds after sending the shutdown signal to wait before killing the service
    /// process
    ///
    /// The default value can be set in the packages plan file.
    #[arg(long = "shutdown-timeout")]
    pub shutdown_timeout: Option<ShutdownTimeout>,

    #[cfg(target_os = "windows")]
    /// Password of the service user
    #[arg(long = "password")]
    pub password: Option<String>,
}
