use super::IncarnatedPackageIdent;
use crate::{manager::service::Service,
            util};
use habitat_core::{self,
                   ChannelIdent,
                   package::{FullyQualifiedPackageIdent,
                             Identifiable,
                             PackageIdent},
                   service::ServiceGroup};
use habitat_sup_protocol::types::UpdateCondition;
use log::{debug,
          trace,
          warn};
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
        #[allow(clippy::question_mark)]
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
    pub async fn update_to(&self, ident: IncarnatedPackageIdent) -> IncarnatedPackageIdent {
        let period = PackageUpdateWorkerPeriod::get().unwrap_or(self.period);
        loop {
            let install_source = ident.ident.clone().into();

            // Rolling updating followers will always update to a fully qulified ident
            // if we are fully qualified, just update to that version and do not check
            // the channel head. The leader already did that. If a package is rolled
            // back in the middle of an update, this fully qualified version may no longer
            // be in the channel which can cause this loop to run indefinitely. Just
            // finish up this update and let the leader roll the followers back when it
            // determines the new head.
            let package_result = if ident.ident.fully_qualified() {
                util::pkg::install_no_ui(&self.builder_url, &install_source, &self.channel).await
            } else {
                match self.update_condition {
                    UpdateCondition::Latest => {
                        let install_source = ident.ident.clone().into();
                        util::pkg::install_no_ui(&self.builder_url, &install_source, &self.channel).await
                    }
                    UpdateCondition::TrackChannel => {
                        util::pkg::install_channel_head(&self.builder_url,
                                                        &ident.ident,
                                                        &self.channel).await
                    }
                }
            };
            match package_result {
                Ok(package) => {
                    // while this is likely a very slim edge case, if the fully qualified ident
                    // happens to be the same as the current service, go ahead and break out of
                    // the loop otherwise we will remain here forever and ever
                    if ident.ident.fully_qualified() || &package.ident != self.full_ident.as_ref() {
                        debug!("'{}' package update worker found change from '{}' to '{}' for \
                                '{}' in channel '{}' using '{}' update condition",
                               self.service_group,
                               self.full_ident,
                               package.ident,
                               ident.ident,
                               self.channel,
                               self.update_condition);
                        break IncarnatedPackageIdent::new(package.ident, ident.incarnation);
                    }
                    trace!("'{}' package update worker did not find change from '{}' for '{}' in \
                            channel '{}' using '{}' update condition",
                           self.service_group,
                           self.full_ident,
                           ident.ident,
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
                   ident.ident,
                   period.as_secs());
            time::sleep(period).await;
        }
    }

    /// Use the service spec's package ident to search for packages.
    /// This function is called by the at-once updater and by a rolling
    /// update leader. Delay for PackageUpdateWorkerPeriod before performing
    /// the update. update_to is only called directly by this function and
    /// rolling update followers where no delay is desired. We want the followers
    /// to update after the leader ASAP.
    pub async fn update(&self) -> IncarnatedPackageIdent {
        let ident = self.ident.clone();
        let period = PackageUpdateWorkerPeriod::get().unwrap_or(self.period);
        let splay = Duration::from_secs(rand::rng().random_range(0..period.as_secs()));
        debug!("Starting package update worker for {} in {}s",
               ident,
               splay.as_secs());
        time::sleep(splay).await;
        self.update_to(IncarnatedPackageIdent::new(ident, None))
            .await
    }
}
