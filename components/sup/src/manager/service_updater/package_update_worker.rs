use crate::{manager::service::Service,
            util};
use habitat_core::{env,
                   package::PackageIdent,
                   service::ServiceGroup,
                   ChannelIdent};
use std::{self,
          time::Duration};
use tokio::{self,
            time};

// TODO (CM): Yes, the variable value should be "period" and not
// "frequency"... we need to fix that.
const PERIOD_BYPASS_CHECK_ENVVAR: &str = "HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK";

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

    fn get() -> Duration {
        let val = PackageUpdateWorkerPeriod::configured_value().into();
        if val >= PackageUpdateWorkerPeriod::MIN_ALLOWED
           || env::var(PERIOD_BYPASS_CHECK_ENVVAR).is_ok()
        {
            val
        } else {
            PackageUpdateWorkerPeriod::MIN_ALLOWED
        }
    }
}

/// When `run`, a `PackageUpdateWorker` returns a future that continuously checks for a change in
/// version of the package being run by a service. If a change is detected, the package is installed
/// and its identifier returned.
pub struct PackageUpdateWorker {
    service_group: ServiceGroup,
    ident:         PackageIdent,
    // TODO (DM): This field should always be fully qualified. We need a
    // type to encapsulate that.
    full_ident:    PackageIdent,
    channel:       ChannelIdent,
    builder_url:   String,
}

impl From<&Service> for PackageUpdateWorker {
    fn from(service: &Service) -> Self {
        Self { service_group: service.service_group.clone(),
               ident:         service.spec_ident.clone(),
               full_ident:    service.pkg.ident.clone(),
               channel:       service.channel.clone(),
               builder_url:   service.bldr_url.clone(), }
    }
}

impl PackageUpdateWorker {
    /// The package update worker future.
    ///
    /// If no `install_ident` is specified the service ident is used when searching for the package.
    /// If an `install_ident` is specified, it is used when searching for a package. If a fully
    /// qualified package ident is used, the future will only resolve when that exact package is
    /// found.
    // TODO (DM): The returned package ident should always be fully qualified. We need a type to
    // encapsulate that.
    pub async fn run(&self, install_ident: Option<PackageIdent>) -> PackageIdent {
        let install_ident = install_ident.unwrap_or_else(|| self.ident.clone());
        let install_source = install_ident.clone().into();
        loop {
            // TODO (DM): The entire behavior of the updater depends on this `install` function. It
            // is what the old updater used, but its exact behavior should be tested/documented.
            match util::pkg::install_no_ui(&self.builder_url, &install_source, &self.channel).await
            {
                Ok(package) => {
                    // Only allow updates never rollbacks.
                    if package.ident > self.full_ident {
                        debug!("'{}' package update worker found change from '{}' to '{}' for \
                                '{}' in channel '{}'",
                               self.service_group,
                               self.full_ident,
                               package.ident,
                               install_ident,
                               self.channel,);
                        break package.ident;
                    } else {
                        trace!("'{}' package update worker did not find change from '{}' for '{}' \
                                in channel '{}'",
                               self.service_group,
                               self.full_ident,
                               install_ident,
                               self.channel,)
                    }
                }
                Err(err) => {
                    warn!("'{}' package update worker failed to install '{}' from channel '{}', \
                           err: {}",
                          self.service_group, self.ident, self.channel, err)
                }
            }
            time::delay_for(PackageUpdateWorkerPeriod::get()).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use habitat_core::locked_env_var;

    #[test]
    fn worker_period_default_is_equal_to_minimum_allowed_value() {
        assert_eq!(PackageUpdateWorkerPeriod::default().0,
                   PackageUpdateWorkerPeriod::MIN_ALLOWED);
    }

    locked_env_var!(HAB_UPDATE_STRATEGY_FREQUENCY_MS, lock_period_var);
    locked_env_var!(HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK, lock_bypass_var);

    #[test]
    fn worker_period_must_be_positive() {
        use std::str::FromStr as _;
        assert!(PackageUpdateWorkerPeriod::from_str("-123").is_err());
        assert!(PackageUpdateWorkerPeriod::from_str("0").is_ok());
        assert!(PackageUpdateWorkerPeriod::from_str("5").is_ok());
    }

    #[test]
    fn worker_period_must_be_bypassed_by_non_empty_value() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();

        period.set("123");
        bypass.set(""); // empty string isn't allowed

        assert_ne!(PackageUpdateWorkerPeriod::get(), Duration::from_millis(123));
        assert_eq!(PackageUpdateWorkerPeriod::default().0,
                   PackageUpdateWorkerPeriod::get());
    }

    #[test]
    fn worker_period_defaults_properly() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();

        period.unset();
        bypass.unset();

        assert_eq!(PackageUpdateWorkerPeriod::default().0,
                   PackageUpdateWorkerPeriod::get());
    }

    #[test]
    fn worker_period_can_be_overridden_by_env_var() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();

        period.set("120000");
        bypass.unset();
        let expected_period: PackageUpdateWorkerPeriod =
            PackageUpdateWorkerPeriod(Duration::from_millis(120_000));
        assert!(expected_period.0 >= PackageUpdateWorkerPeriod::MIN_ALLOWED);
        assert_eq!(expected_period.0, PackageUpdateWorkerPeriod::get());
    }

    #[test]
    fn worker_period_cannot_be_overridden_to_a_very_small_value_by_default() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();

        period.set("1"); // This is TOO low
        bypass.unset();
        assert!(Duration::from_millis(1) < PackageUpdateWorkerPeriod::MIN_ALLOWED);
        assert_eq!(PackageUpdateWorkerPeriod::default().0,
                   PackageUpdateWorkerPeriod::get());
    }

    #[test]
    fn worker_period_cannot_be_overridden_by_a_non_number() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();

        period.set("this is not a number");
        bypass.unset();
        assert_eq!(PackageUpdateWorkerPeriod::default().0,
                   PackageUpdateWorkerPeriod::get());
    }

    #[test]
    fn worker_period_can_be_overridden_by_a_small_value_with_bypass_var() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();

        period.set("5000");
        bypass.set("1");
        let expected_period: PackageUpdateWorkerPeriod =
            PackageUpdateWorkerPeriod(Duration::from_millis(5000));
        assert!(expected_period.0 < PackageUpdateWorkerPeriod::MIN_ALLOWED);
        assert_eq!(expected_period.0, PackageUpdateWorkerPeriod::get());
    }
}
