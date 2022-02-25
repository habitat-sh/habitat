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
                                             unwrap_or(PackageTarget::active_target());
        if target == AARCH64_DARWIN {
            Ok(SupWatcher::Fallback(PollWatcher::new_raw(tx).unwrap()))
        } else {
            Ok(SupWatcher::Native(RecommendedWatcher::new_raw(tx).unwrap()))
        }
    }

    fn new(tx: Sender<DebouncedEvent>, delay: Duration) -> Result<Self> {
        let target = PackageTarget::from_str(&env::var("HAB_STUDIO_HOST_ARCH").
                                             unwrap_or_default()).
                                             unwrap_or(PackageTarget::active_target());
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
