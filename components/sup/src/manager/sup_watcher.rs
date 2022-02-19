//! Watcher interface implementation for Habitat Supervisor.

use notify::{poll::PollWatcher,
             DebouncedEvent,
             RecommendedWatcher,
             RecursiveMode,
             Result,
             Watcher};
use std::{env,
          sync::mpsc::Sender,
          path::Path,
          time::Duration};

pub enum SupWatcher {
    Native(RecommendedWatcher),
    Fallback(PollWatcher)
}

impl Watcher for SupWatcher {
    fn new_raw(tx: Sender<notify::RawEvent>) -> Result<Self> {
        if let Ok(arch_type) = env::var("HAB_STUDIO_HOST_ARCH") {
            match arch_type.as_str() {
                "aarch64-macos" => Ok(SupWatcher::Fallback(PollWatcher::new_raw(tx).unwrap())),
                _ => Ok(SupWatcher::Native(RecommendedWatcher::new_raw(tx).unwrap()))
            }
        } else {
            Ok(SupWatcher::Native(RecommendedWatcher::new_raw(tx).unwrap()))
        }

    }

    fn new(tx: Sender<DebouncedEvent>, delay: Duration) -> Result<Self> {
        if let Ok(arch_type) = env::var("HAB_STUDIO_HOST_ARCH") {
            match arch_type.as_str() {
                "aarch64-macos" => Ok(SupWatcher::Fallback(PollWatcher::new(tx, delay).unwrap())),
                _ => Ok(SupWatcher::Native(RecommendedWatcher::new(tx, delay).unwrap()))
            }
        } else {
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
