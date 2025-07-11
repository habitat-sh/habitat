use clap_v4 as clap;

use std::{convert::TryFrom,
          iter::FromIterator};

use clap::Parser;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;

use habitat_core::{os::process::ShutdownTimeout,
                   package::PackageIdent,
                   service::{HealthCheckInterval,
                             ServiceBind},
                   ChannelIdent};
use habitat_sup_protocol::{ctl,
                           types::{BindingMode,
                                   Topology,
                                   UpdateCondition,
                                   UpdateStrategy}};

use crate::{cli_v4::utils::{BldrUrl,
                            RemoteSup},
            error::{Error,
                    Result as HabResult},
            gateway_util};

/// Update how the Supervisor manages an already-running service. Depending on the given changes,
/// they may be able to be applied without restarting the service.
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct UpdateCommand {
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,

    // This is some unfortunate duplication... everything below this
    // should basically be identical to SharedLoad, except that we
    // don't want to have default values, and everything should be
    // optional.
    /// Receive updates from the specified release channel
    #[arg(long = "channel")]
    channel: Option<ChannelIdent>,

    /// Specify an alternate Builder endpoint.
    #[command(flatten)]
    bldr_url: Option<BldrUrl>,

    /// The service group with shared config and topology
    #[arg(long = "group")]
    group: Option<String>,

    /// Service topology
    #[arg(long = "topology", short = 't')]
    topology: Option<Topology>,

    /// The update strategy
    #[arg(long = "strategy", short = 's')]
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
    #[arg(long = "update-condition", default_value=UpdateCondition::Latest.as_str())]
    update_condition: Option<UpdateCondition>,

    /// One or more service groups to bind to a configuration
    #[arg(long = "bind")]
    bind: Option<Vec<ServiceBind>>,

    /// Governs how the presence or absence of binds affects service startup
    ///
    /// strict: blocks startup until all binds are present.
    #[arg(long = "binding-mode")]
    binding_mode: Option<BindingMode>,

    /// The interval in seconds on which to run health checks
    // We can use `HealthCheckInterval` here (cf. `SharedLoad` above),
    // because we don't have to worry about serialization here.
    #[arg(long = "health-check-interval", short = 'i')]
    health_check_interval: Option<HealthCheckInterval>,

    /// The delay in seconds after sending the shutdown signal to wait before killing the service
    /// process
    ///
    /// The default value can be set in the packages plan file.
    #[arg(long = "shutdown-timeout")]
    shutdown_timeout: Option<ShutdownTimeout>,

    #[cfg(target_os = "windows")]
    /// Password of the service user
    #[arg(long = "password")]
    password: Option<String>,
}

impl TryFrom<UpdateCommand> for ctl::SvcUpdate {
    type Error = Error;

    fn try_from(u: UpdateCommand) -> HabResult<Self> {
        let msg = ctl::SvcUpdate { ident: Some(From::from(u.pkg_ident)),
                                   // We are explicitly *not* using the environment variable as a
                                   // fallback.
                                   bldr_url: u.bldr_url.map(|u| u.to_string()),
                                   bldr_channel: u.channel.map(Into::into),
                                   binds: u.bind.map(FromIterator::from_iter),
                                   group: u.group,
                                   health_check_interval: u.health_check_interval.map(Into::into),
                                   binding_mode: u.binding_mode.map(|v| v as i32),
                                   topology: u.topology.map(|v| v as i32),
                                   update_strategy: u.strategy.map(|v| v as i32),
                                   update_condition: u.update_condition.map(|v| v as i32),
                                   shutdown_timeout: u.shutdown_timeout.map(Into::into),
                                   #[cfg(windows)]
                                   svc_encrypted_password: u.password,
                                   #[cfg(not(windows))]
                                   svc_encrypted_password: None, };

        // Compiler-assisted validation that the user has indeed
        // specified *something* to change. If they didn't, all the
        // fields would end up as `None`, and that would be an error.
        if let ctl::SvcUpdate { ident: _,
                                binds: None,
                                binding_mode: None,
                                bldr_url: None,
                                bldr_channel: None,
                                group: None,
                                svc_encrypted_password: None,
                                topology: None,
                                update_strategy: None,
                                health_check_interval: None,
                                shutdown_timeout: None,
                                update_condition: None, } = &msg
        {
            Err(Error::ArgumentError("No fields specified for update".to_string()))
        } else {
            Ok(msg)
        }
    }
}

impl UpdateCommand {
    pub(crate) async fn do_command(&self) -> HabResult<()> {
        let ctl_addr = self.remote_sup.clone();
        let msg: ctl::SvcUpdate = TryFrom::try_from(self.clone())?;
        gateway_util::send(ctl_addr.inner(), msg).await
    }
}
