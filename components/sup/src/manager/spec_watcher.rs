//! Provides facilities for notifying when any spec files have changed
//! on disk. This is how we know when to start, stop, or restart
//! services in response to the various `hab svc` commands.

use super::spec_dir::SpecDir;
use crate::error::{Error,
                   Result};
use habitat_core::env::Config as EnvConfig;
use notify::{DebouncedEvent,
             RecommendedWatcher,
             RecursiveMode,
             Watcher};
use std::{num::ParseIntError,
          str::FromStr,
          sync::mpsc,
          thread::Builder,
          time::Duration};

static LOGKEY: &'static str = "SW";

/// How long should we wait to consolidate filesystem events?
///
/// This should strike a balance between responsiveness and
/// too-granular a series of events.
///
/// See https://docs.rs/notify/4.0.6/notify/trait.Watcher.html#tymethod.new
struct SpecWatcherDelay(Duration);

impl From<Duration> for SpecWatcherDelay {
    fn from(d: Duration) -> SpecWatcherDelay { SpecWatcherDelay(d) }
}

impl Default for SpecWatcherDelay {
    fn default() -> Self {
        // There's nothing particularly magical about 2s, particularly
        // since we're monitoring at such a coarse level ("something
        // happened in this directory").
        //
        // Smaller is probably fine, but you wouldn't want to go much
        // higher, as this could extend the amount of time you'd need
        // to wait before realizing you need to take action on a
        // service.
        Duration::from_secs(2).into()
    }
}

impl FromStr for SpecWatcherDelay {
    type Err = ParseIntError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Duration::from_millis(s.parse()?).into())
    }
}

impl EnvConfig for SpecWatcherDelay {
    const ENVVAR: &'static str = "HAB_SPEC_WATCHER_DELAY_MS";
}

// TODO (CM): A strong argument could be made for folding the
// SpecWatcher functionality into SpecDir itself.

/// Provides an abstraction layer over filesystem notifications for
/// spec files.
pub struct SpecWatcher {
    // Not actually used; only holding onto it for lifetime / Drop
    // purposes (`Drop` kills the threads that the watcher spawns to do
    // its work).
    _watcher: RecommendedWatcher,
    channel:  mpsc::Receiver<DebouncedEvent>,
}

impl SpecWatcher {
    /// Start up a separate thread to listen for filesystem
    /// events.
    pub fn run(spec_dir: &SpecDir) -> Result<SpecWatcher> {
        // The act of creating a `notify::Watcher` creates threads on
        // its own. It does not, however, allow you to set the _names_
        // of those threads.
        //
        // We're creating a SpecWatcher in a thread just so we can get
        // some control over the name of the threads that the
        // underlying `notify::Watcher` creates (_that_ is what this
        // function's documentation is referring to), which makes
        // monitoring and reasoning about the overall Supervisor
        // process easier. There's no other reason than that; if the
        // `notify` crate allowed us to name the threads, we could
        // just use a slightly modified version of `SpecWatcher::new`
        // instead.

        // I'd rather not have to do this clone, but it's a side
        // effect of this thread business. Better to eat it here
        // rather than at the callsite; having this function take a
        // reference is the true API we want.
        let dir = spec_dir.clone();

        Builder::new().name(String::from("spec-watcher"))
                      .spawn(move || Self::new(&dir))?
                      .join()
                      .map_err(|_| {
                          error!("SpecWatcher spawning thread panicked!");
                          sup_error!(Error::SpecWatcherNotCreated)
                      })?
    }

    /// Isolates the pure creation logic of a `SpecWatcher`, separate
    /// from the thread-based creation we use in `SpecWatcher::run` to
    /// get control over the names of the resulting threads. If we
    /// didn't care what the resulting watcher threads were named,
    /// we'd just use this directly.
    fn new(spec_dir: &SpecDir) -> Result<SpecWatcher> {
        let (tx, rx) = mpsc::channel();
        let delay = SpecWatcherDelay::configured_value();
        let mut watcher = RecommendedWatcher::new(tx, delay.0)?;
        watcher.watch(spec_dir, RecursiveMode::NonRecursive)?;
        Ok(SpecWatcher { _watcher: watcher,
                         channel:  rx, })
    }

    /// Returns `true` if _any_ filesystem events were detected in the
    /// watched directory.
    ///
    /// We are opting for this coarse granularity because it is
    /// difficult, if not impossible, to rely on the resulting stream
    /// of notification events as a 100% complete and accurate record
    /// of everything that takes place in the specs directory. For
    /// example, because of how we write files to temporary locations,
    /// then rename in order to achieve "atomic writes", it is
    /// possible to miss the events that would cause you to see this
    /// as a "rename"; instead, you would see a "new" file (even with
    /// a debounce time of 0 ms!)
    ///
    /// In any event, we would _still_ need to examine on-disk state
    /// of the directory and compare to our in-memory state to know
    /// exactly what to do in order to reconcile the two (i.e., you
    /// can't know whether to start, stop, or restart a service just
    /// by knowing that the spec file changed; you have to actually
    /// look at the current contents to figure that out).
    ///
    /// As a result, we're just using this as a coarse "something
    /// changed in the directory" signal. We are not filtering events
    /// to receive only those that affect `*.spec` files, so we may
    /// respond to modifications to temporary files, or indeed any
    /// file, within the specs directory (e.g., running `touch
    /// /hab/sup/default/specs/blahblah` would count as an event). It
    /// is possible to perform this filtering, of course, but it's not
    /// clear that the extra code would be worth it.
    pub fn has_events(&self) -> bool {
        let events = self.channel.try_iter().collect::<Vec<_>>();
        if events.is_empty() {
            false
        } else {
            trace!("SpecWatcher events: {:?}", events);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use habitat_common::locked_env_var;
    use std::{fs::File,
              io::{Error as IoError,
                   Write},
              result::Result as StdResult,
              thread};
    use tempfile::TempDir;

    locked_env_var!(HAB_SPEC_WATCHER_DELAY_MS, lock_delay_var);

    fn file_with_content<C>(dir: &TempDir, filename: &str, contents: C) -> StdResult<(), IoError>
        where C: Into<String>
    {
        let path = dir.path().join(filename);
        let mut buffer = File::create(&path)?;
        buffer.write_all(contents.into().as_bytes())
    }

    /// Sleep for the currently-configured debounce interval, plus a
    /// few milliseconds more, just to be certain our filesystem
    /// events have had plenty of time to process.
    fn wait_for_debounce_interval() {
        let wait_duration = SpecWatcherDelay::configured_value().0 + Duration::from_millis(2);
        error!("wait_for_debounce_interval: waiting {:?}", wait_duration);
        thread::sleep(wait_duration);
    }

    #[test]
    fn can_be_created() {
        let _delay = lock_delay_var();

        let dir = TempDir::new().expect("Could not create directory");
        let spec_dir = SpecDir::new(dir.path()).expect("Couldn't make SpecDir");
        assert!(SpecWatcher::run(&spec_dir).is_ok(),
                "Couldn't create a SpecWatcher!");
    }

    #[test]
    fn can_get_events_for_spec_files() {
        env_logger::try_init().ok();
        error!("can_get_events_for_spec_files starting");
        let _delay = lock_delay_var();

        let dir = TempDir::new().expect("Could not create directory");
        let spec_dir = SpecDir::new(dir.path()).expect("Couldn't make SpecDir");
        let sw = SpecWatcher::run(&spec_dir).expect("Couldn't create a SpecWatcher!");

        assert!(!sw.has_events(), "There should be no events to start");

        file_with_content(&dir, "foo.spec", "fooooooo").expect("couldn't create file");

        assert!(!sw.has_events(),
                "Need to allow for the debounce interval to pass before you can expect events");

        while !sw.has_events() {
            wait_for_debounce_interval();
        }

        assert!(!sw.has_events(),
                "Should be no more events after you've checked");
        error!("can_get_events_for_spec_files done");
    }

    /// Currently, the spec watcher will respond to changes to any
    /// file in the directory, whether it's a `*.spec` file or not.
    ///
    /// This would, for instance, pick up the temp files that
    /// operations like `hab svc stop` lay down before renaming them
    /// to their final `*.spec` form.
    #[test]
    fn can_get_events_for_non_spec_files() {
        env_logger::try_init().ok();
        error!("can_get_events_for_non_spec_files starting");
        let _delay = lock_delay_var();

        let dir = TempDir::new().expect("Could not create directory");
        let spec_dir = SpecDir::new(dir.path()).expect("Couldn't make SpecDir");
        let sw = SpecWatcher::run(&spec_dir).expect("Couldn't create a SpecWatcher!");

        assert!(!sw.has_events(), "There should be no events to start");

        file_with_content(&dir, "foo.abc123xyz", "fooooooo").expect("couldn't create file");

        assert!(!sw.has_events(),
                "Need to allow for the debounce interval to pass before you can expect events");

        while !sw.has_events() {
            wait_for_debounce_interval();
        }

        assert!(!sw.has_events(),
                "Should be no more events after you've checked");
        error!("can_get_events_for_non_spec_files done");
    }

    #[test]
    fn short_debounce_delays_also_work() {
        env_logger::try_init().ok();
        error!("short_debounce_delays_also_work starting");
        let delay = lock_delay_var();
        delay.set("1");

        // Just verifying that our delay variable works correctly
        assert_eq!(SpecWatcherDelay::configured_value().0,
                   Duration::from_millis(1));

        let dir = TempDir::new().expect("Could not create directory");
        let spec_dir = SpecDir::new(dir.path()).expect("Couldn't make SpecDir");
        let sw = SpecWatcher::run(&spec_dir).expect("Couldn't create a SpecWatcher!");

        assert!(!sw.has_events(), "There should be no events to start");

        file_with_content(&dir, "foo.spec", "fooooooo").expect("couldn't create file");

        assert!(!sw.has_events(),
                "Need to allow for the debounce interval to pass before you can expect events");

        while !sw.has_events() {
            wait_for_debounce_interval();
        }

        assert!(!sw.has_events(),
                "Should be no more events after you've checked");
        error!("short_debounce_delays_also_work starting");
    }
}
