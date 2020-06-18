use crate::{manager::service::Service,
            util};
use habitat_core::{self,
                   package::{FullyQualifiedPackageIdent,
                             PackageIdent},
                   service::ServiceGroup,
                   ChannelIdent};
use habitat_sup_protocol::types::UpdateCondition;
use rand::Rng;
use std::{self,
          time::Duration};
use tokio::{self,
            time};

// TODO (CM): Yes, the variable value should be "period" and not
// "frequency"... we need to fix that.
const PERIOD_BYPASS_CHECK_ENVVAR: &str = "HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK";

// TODO (DM): Remove this deprecated env var
habitat_core::env_config_duration!(
    /// Represents how far apart checks for updates to individual services
    /// are, in milliseconds.
    PackageUpdateWorkerPeriod,
    // TODO (CM): Yes, the variable value should be "period" and not
    // "frequency"... we need to fix that.
    HAB_UPDATE_STRATEGY_FREQUENCY_MS => from_millis,
    PackageUpdateWorkerPeriod::MIN_ALLOWED);

impl PackageUpdateWorkerPeriod {
    const MIN_ALLOWED: Duration = Duration::from_secs(60);

    fn get() -> Option<Duration> {
        if habitat_core::env::var(PackageUpdateWorkerPeriod::ENVVAR).is_err() {
            return None;
        }
        warn!("Using deprecated environment variable `HAB_UPDATE_STRATEGY_FREQUENCY_MS`. Prefer \
               using the `hab sup run --service-update-period` argument or config file setting.");
        let val = PackageUpdateWorkerPeriod::configured_value().into();
        if val >= PackageUpdateWorkerPeriod::MIN_ALLOWED
           || habitat_core::env::var(PERIOD_BYPASS_CHECK_ENVVAR).is_ok()
        {
            Some(val)
        } else {
            Some(PackageUpdateWorkerPeriod::MIN_ALLOWED)
        }
    }
}

/// When `run`, a `PackageUpdateWorker` returns a future that continuously checks for a change in
/// version of the package being run by a service. If a change is detected, the package is installed
/// and its identifier returned.
pub struct PackageUpdateWorker {
    service_group:    ServiceGroup,
    ident:            PackageIdent,
    full_ident:       FullyQualifiedPackageIdent,
    update_condition: UpdateCondition,
    channel:          ChannelIdent,
    builder_url:      String,
    period:           Duration,
}

impl PackageUpdateWorker {
    pub fn new(service: &Service, period: Duration) -> Self {
        Self { service_group: service.service_group.clone(),
               ident: service.spec_ident(),
               full_ident: service.pkg.ident.clone(),
               update_condition: service.update_condition(),
               channel: service.channel(),
               builder_url: service.bldr_url(),
               period }
    }
}

impl PackageUpdateWorker {
    /// Use the specified package ident to search for packages.
    ///
    /// If a fully qualified package ident is used, the future will only resolve when that exact
    /// package is found.
    // TODO (DM): The returned package ident should use FullyQualifiedPackageIdent.
    pub async fn update_to(&self, ident: PackageIdent) -> PackageIdent {
        let period = PackageUpdateWorkerPeriod::get().unwrap_or(self.period);
        let splay = Duration::from_secs(rand::thread_rng().gen_range(0, period.as_secs()));
        debug!("Starting package update worker for {} in {}s",
               ident,
               splay.as_secs());
        time::delay_for(splay).await;
        loop {
            let package_result = match self.update_condition {
                UpdateCondition::Latest => {
                    let install_source = ident.clone().into();
                    util::pkg::install_no_ui(&self.builder_url, &install_source, &self.channel).await
                }
                UpdateCondition::TrackChannel => {
                    util::pkg::install_channel_head(&self.builder_url, &ident, &self.channel).await
                }
            };
            match package_result {
                Ok(package) => {
                    if &package.ident != self.full_ident.as_ref() {
                        debug!("'{}' package update worker found change from '{}' to '{}' for \
                                '{}' in channel '{}' using '{}' update condition",
                               self.service_group,
                               self.full_ident,
                               package.ident,
                               ident,
                               self.channel,
                               self.update_condition);
                        break package.ident;
                    }
                    trace!("'{}' package update worker did not find change from '{}' for '{}' in \
                            channel '{}' using '{}' update condition",
                           self.service_group,
                           self.full_ident,
                           ident,
                           self.channel,
                           self.update_condition)
                }
                Err(err) => {
                    warn!("'{}' package update worker failed to install '{}' from channel '{}', \
                           err: {}",
                          self.service_group, self.ident, self.channel, err)
                }
            }
            trace!("Package update worker for {} delaying for {}s",
                   ident,
                   period.as_secs());
            time::delay_for(period).await;
        }
    }

    /// Use the service spec's package ident to search for packages.
    pub async fn update(&self) -> PackageIdent { self.update_to(self.ident.clone()).await }
}
