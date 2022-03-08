//! Watcher interface implementation for Habitat Supervisor.

use habitat_core::package::target::{PackageTarget,
                                    AARCH64_DARWIN};
use notify::{poll::PollWatcher,
             DebouncedEvent,
             RecommendedWatcher,
             RecursiveMode,
             Result,
             Watcher};
use std::{env,
          path::Path,
          str::FromStr,
          sync::mpsc::Sender,
          time::Duration};

pub enum SupWatcher {
    Native(RecommendedWatcher),
    Fallback(PollWatcher),
}

impl Watcher for SupWatcher {
    fn new_raw(tx: Sender<notify::RawEvent>) -> Result<Self> {
        let target = PackageTarget::from_str(&env::var("HAB_STUDIO_HOST_ARCH").
                                             unwrap_or_default()).
                                             unwrap_or_else(|_| PackageTarget::active_target());
        if target == AARCH64_DARWIN {
            Ok(SupWatcher::Fallback(PollWatcher::new_raw(tx).unwrap()))
        } else {
            Ok(SupWatcher::Native(RecommendedWatcher::new_raw(tx).unwrap()))
        }
    }

    fn new(tx: Sender<DebouncedEvent>, delay: Duration) -> Result<Self> {
        let target = PackageTarget::from_str(&env::var("HAB_STUDIO_HOST_ARCH").
                                             unwrap_or_default()).
                                             unwrap_or_else(|_| PackageTarget::active_target());
        if target == AARCH64_DARWIN {
            debug!("Using pollwatcher");
            Ok(SupWatcher::Fallback(PollWatcher::new(tx, delay).unwrap()))
        } else {
            debug!("Using native watcher");
            Ok(SupWatcher::Native(RecommendedWatcher::new(tx, delay).unwrap()))
        }
    }

    fn watch<P: AsRef<Path>>(&mut self, path: P, recursive_mode: RecursiveMode) -> Result<()> {
        match self {
            SupWatcher::Native(watcher) => watcher.watch(path, recursive_mode),
            SupWatcher::Fallback(watcher) => watcher.watch(path, recursive_mode),
        }
    }

    fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        match self {
            SupWatcher::Native(watcher) => watcher.unwatch(path),
            SupWatcher::Fallback(watcher) => watcher.unwatch(path),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env,
              sync::mpsc::channel,
              time::Duration};

    pub const TEST_STUDIO_HOST_ARCH_ENVVAR: &str = "HAB_STUDIO_HOST_ARCH";

    #[test]
    fn set_arch_env_var_test_mac() {
        env::set_var(TEST_STUDIO_HOST_ARCH_ENVVAR, "aarch64-darwin");

        let (sender, _) = channel();
        let delay = Duration::from_millis(1000);
        let _sup_watcher = SupWatcher::new(sender, delay);
        let watcher_type = match _sup_watcher {
            Ok(SupWatcher::Native(_sup_watcher)) => "Native",
            Ok(SupWatcher::Fallback(_sup_watcher)) => "Fallback",
            _ => "Error",
        };
        env::remove_var(TEST_STUDIO_HOST_ARCH_ENVVAR);
        assert_eq!(watcher_type, "Fallback");
    }
}
