//! Watcher interface implementation for Habitat Supervisor.
use habitat_core::package::target::{PackageTarget,
                                    AARCH64_DARWIN};
use log::{debug,
          warn};
use notify::{poll::PollWatcher,
             Config,
             EventHandler,
             RecommendedWatcher,
             RecursiveMode,
             Result,
             Watcher,
             WatcherKind};
use std::{env,
          path::Path,
          str::FromStr};

#[derive(Debug)]
pub enum SupWatcher {
    Native(RecommendedWatcher),
    Fallback(PollWatcher),
}

impl Watcher for SupWatcher {
    fn new<F: EventHandler>(event_handler: F, config: Config) -> Result<Self> {
        let target = PackageTarget::from_str(&env::var("HAB_STUDIO_HOST_ARCH").
                                             unwrap_or_default()).
                                             unwrap_or_else(|_| PackageTarget::active_target());
        if target == AARCH64_DARWIN {
            debug!("Using pollwatcher");
            Ok(SupWatcher::Fallback(PollWatcher::new(event_handler, config).unwrap()))
        } else {
            debug!("Using native watcher");
            Ok(SupWatcher::Native(RecommendedWatcher::new(event_handler, config).unwrap()))
        }
    }

    fn watch(&mut self, path: &Path, recursive_mode: RecursiveMode) -> Result<()> {
        match self {
            SupWatcher::Native(watcher) => watcher.watch(path, recursive_mode),
            SupWatcher::Fallback(watcher) => watcher.watch(path, recursive_mode),
        }
    }

    fn unwatch(&mut self, path: &Path) -> Result<()> {
        match self {
            SupWatcher::Native(watcher) => watcher.unwatch(path),
            SupWatcher::Fallback(watcher) => watcher.unwatch(path),
        }
    }

    // For now we are using the default implementation of configure() provided
    // by the notify crate which returns Ok(false) signalling that runtime
    // configuration is not supported.

    fn kind() -> WatcherKind
        where Self: Sized
    {
        // https://github.com/notify-rs/notify/pull/441#discussion_r961970946
        // Lacking a self reference I don't see how it isn't a mistake to include this method in the
        // trait.  Trying to come up with an implementation I went and searched the notify-rs github
        // repo an found the discussion linked above.  That's my best indicator that the maintainers
        // agree with me.  So to satify this API I'm doing the following which is ugly and feels
        // horrible so please do suggest or implement something better if you have something.
        warn!("This implementation of kind() LIES to you. See comment in source code.");
        WatcherKind::NullWatcher
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use habitat_core::locked_env_var;
    use std::{sync::mpsc::channel,
              time::Duration};

    locked_env_var!(HAB_STUDIO_HOST_ARCH, lock_env_var);

    #[test]
    fn sup_watcher_constructor_test_polling() {
        let (sender, _) = channel();
        let delay = Duration::from_millis(1000);
        let config = Config::default().with_poll_interval(delay);

        let lock = lock_env_var();
        lock.set("aarch64-darwin");

        let _sup_watcher = SupWatcher::new(sender, config);
        let watcher_type = match _sup_watcher {
            Ok(SupWatcher::Native(_sup_watcher)) => "Native",
            Ok(SupWatcher::Fallback(_sup_watcher)) => "Fallback",
            _ => "Error",
        };

        lock.unset();

        assert_eq!(watcher_type, "Fallback");
    }

    #[test]
    fn sup_watcher_constructor_test_notify() {
        let (sender, _) = channel();
        let delay = Duration::from_millis(1000);
        let config = Config::default().with_poll_interval(delay);

        let lock = lock_env_var();
        lock.unset();

        let _sup_watcher = SupWatcher::new(sender, config);
        let watcher_type = match _sup_watcher {
            Ok(SupWatcher::Native(_sup_watcher)) => "Native",
            Ok(SupWatcher::Fallback(_sup_watcher)) => "Fallback",
            _ => "Error",
        };

        assert_eq!(watcher_type, "Native");
    }
}
