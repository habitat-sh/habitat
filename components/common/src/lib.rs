use crate::ui::{UIWriter,
                UI};
use habitat_api_client as api_client;
use habitat_core as hcore;
use lazy_static::lazy_static;
use std::{collections::HashMap,
          env,
          ffi::OsStr,
          iter::FromIterator};

extern crate json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[cfg(windows)]
extern crate winapi;

pub use self::error::{Error,
                      Result};

pub mod cli;
pub mod command;
pub mod error;
pub mod locked_env_var;
pub mod output;
pub mod package_graph;
pub mod templating;
pub mod types;
pub mod ui;
pub mod util;

lazy_static::lazy_static! {
    pub static ref PROGRAM_NAME: String = {
        match env::current_exe() {
            Ok(path) => path.file_stem().and_then(OsStr::to_str).unwrap().to_string(),
            Err(e) => {
                error!("Error getting path of current_exe: {}", e);
                String::from("hab-?")
            }
        }
    };
}

// TODO (CM): It would be nice to come up with a way to more
// programmatically manage these flags. It's a bit of a pain to define
// the flag, and then define the environment variables
// separately. Nothing statically guarantees that you've specified an
// variable for a flag.

// TODO (CM): It'd be great to have a built-in way to document them,
// too.

// TODO (CM): Part of that documentation might be *when* a flag was
// added. In general, long-lived flags are a code-smell.

// TODO (CM): It may also be useful to break out features by area of
// concern. We can have any number of bitflags-generated structs.

bitflags::bitflags! {
    /// All the feature flags that are recogized by Habitat.
    ///
    /// In general, feature flags are enabled by setting the corresponding
    /// environment variable.
    ///
    /// Your binary should call `FeatureFlag::from_env` to get a set
    /// of flags to use.
    ///
    /// To add a new feature flag, you will need to add the bit mask
    /// constant here, as well as a mapping from the feature to the
    /// environment variable to which it corresponds in the `ENV_VARS`
    /// map below.
    pub struct FeatureFlag: u32 {
        const LIST               = 0b0000_0000_0001;
        const TEST_EXIT          = 0b0000_0000_0010;
        const TEST_BOOT_FAIL     = 0b0000_0000_0100;
        const REDACT_HTTP        = 0b0000_0000_1000;
        const IGNORE_SIGNALS     = 0b0000_0001_0000;
        const OFFLINE_INSTALL    = 0b0000_0100_0000;
        const IGNORE_LOCAL       = 0b0000_1000_0000;
        const EVENT_STREAM       = 0b0001_0000_0000;
        const TRIGGER_ELECTION   = 0b0010_0000_0000;
        const CONFIGURE_SHUTDOWN = 0b0100_0000_0000;
    }
}

lazy_static! {
    static ref ENV_VARS: HashMap<FeatureFlag, &'static str> = {
        let mapping = vec![(FeatureFlag::LIST, "HAB_FEAT_LIST"),
                           (FeatureFlag::TEST_EXIT, "HAB_FEAT_TEST_EXIT"),
                           (FeatureFlag::TEST_BOOT_FAIL, "HAB_FEAT_BOOT_FAIL"),
                           (FeatureFlag::REDACT_HTTP, "HAB_FEAT_REDACT_HTTP"),
                           (FeatureFlag::IGNORE_SIGNALS, "HAB_FEAT_IGNORE_SIGNALS"),
                           (FeatureFlag::OFFLINE_INSTALL, "HAB_FEAT_OFFLINE_INSTALL"),
                           (FeatureFlag::IGNORE_LOCAL, "HAB_FEAT_IGNORE_LOCAL"),
                           (FeatureFlag::EVENT_STREAM, "HAB_FEAT_EVENT_STREAM"),
                           (FeatureFlag::TRIGGER_ELECTION, "HAB_FEAT_TRIGGER_ELECTION"),
                           (FeatureFlag::CONFIGURE_SHUTDOWN, "HAB_FEAT_CONFIGURE_SHUTDOWN")];
        HashMap::from_iter(mapping)
    };
}

impl FeatureFlag {
    /// If the environment variable for a flag is set to _anything_ but
    /// the empty string, it is activated.
    pub fn from_env(ui: &mut UI) -> Self {
        let mut flags = FeatureFlag::empty();

        for (feature, env_var) in ENV_VARS.iter() {
            if let Some(val) = env::var_os(env_var) {
                if !val.is_empty() {
                    flags.insert(*feature);
                    ui.warn(&format!("Enabling feature: {:?}", feature))
                      .unwrap();
                }
            }
        }

        // TODO (CM): Once the other TODOs above are done (especially the
        // documentation bits), it would be nice to extract this logic
        // into an actual discoverable CLI subcommand; it's a little weird
        // that you have to know how to enable a feature flag before you
        // can even find out that there *are* feature flags to enable.
        //
        // There's no reason why "list feature flags" should itself be a
        // feature-flag.
        if flags.contains(FeatureFlag::LIST) {
            ui.warn("Listing feature flags environment variables:")
              .unwrap();
            for (feature, env_var) in ENV_VARS.iter() {
                ui.warn(&format!("  * {:?}: {}={:?}",
                                 feature,
                                 env_var,
                                 env::var_os(env_var).unwrap_or_default()))
                  .unwrap();
            }
        }

        flags
    }
}

pub mod sync {
    use habitat_core::env::Config as EnvConfig;
    use std::{collections::HashMap,
              sync::Mutex,
              thread::{self,
                       ThreadId},
              time::{Duration,
                     Instant}};

    type NameAndLastHeartbeat = (Option<String>, Instant);
    type HeartbeatMap = HashMap<ThreadId, NameAndLastHeartbeat>;
    lazy_static::lazy_static! {
        static ref THREAD_HEARTBEATS: Mutex<HeartbeatMap> = Default::default();
    }

    struct ThreadAliveThreshold(Duration);

    impl EnvConfig for ThreadAliveThreshold {
        const ENVVAR: &'static str = "HAB_THREAD_ALIVE_THRESHOLD_SECS";
    }

    impl Default for ThreadAliveThreshold {
        fn default() -> Self { Self(Duration::from_secs(5 * 60)) }
    }

    impl std::str::FromStr for ThreadAliveThreshold {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(Duration::from_secs(s.parse()?)))
        }
    }

    struct ThreadAliveCheckDelay(Duration);

    impl EnvConfig for ThreadAliveCheckDelay {
        const ENVVAR: &'static str = "HAB_THREAD_ALIVE_THRESHOLD_SECS";
    }

    impl Default for ThreadAliveCheckDelay {
        fn default() -> Self { Self(Duration::from_secs(60)) }
    }

    impl std::str::FromStr for ThreadAliveCheckDelay {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(Duration::from_secs(s.parse()?)))
        }
    }

    /// Call periodically from a thread which has a work loop to indicate that the thread is
    /// still alive and processing its loop. If this function is not called for more than
    /// `ThreadAliveThreshold`, it will be error logged as a likely deadlock.
    pub fn mark_thread_alive() {
        mark_thread_alive_impl(&mut THREAD_HEARTBEATS.lock()
                                                     .expect("THREAD_HEARTBEATS poisoned"));
    }

    fn mark_thread_alive_impl(heartbeats: &mut HeartbeatMap) {
        let thread = thread::current();
        heartbeats.insert(thread.id(),
                          (thread.name().map(str::to_string), Instant::now()));
    }

    /// Call once per binary to start the thread which will check that all the threads that
    /// call `mark_thread_alive` continue to do so.
    pub fn spawn_thread_alive_checker() {
        thread::Builder::new().name("thread-alive-check".to_string())
                              .spawn(|| {
                                  loop {
                                      check_thread_heartbeats();
                                      thread::sleep(ThreadAliveCheckDelay::configured_value().0);
                                  }
                              })
                              .expect("Error spawning thread alive checker");
    }

    fn check_thread_heartbeats() {
        for (name, last_heartbeat) in
            threads_missing_heartbeat(&THREAD_HEARTBEATS.lock()
                                                        .expect("THREAD_HEARTBEATS poisoned"),
                                      ThreadAliveThreshold::configured_value().0)
        {
            error!("No heartbeat from {} in {} seconds; deadlock likely",
                   name.unwrap_or_else(|| { "unnamed thread".to_string() }),
                   last_heartbeat.elapsed().as_secs());
        }
    }

    fn threads_missing_heartbeat(heartbeats: &HeartbeatMap,
                                 threshold: Duration)
                                 -> Vec<NameAndLastHeartbeat> {
        heartbeats.iter()
                  .filter_map(|(thread_id, (thread_name, last_heartbeat))| {
                      let time_since_last_heartbeat = last_heartbeat.elapsed();
                      trace!("{:?} {:?} last heartbeat: {:?} ago",
                             thread_id,
                             thread_name,
                             time_since_last_heartbeat);
                      if time_since_last_heartbeat < threshold {
                          None
                      } else {
                          Some((thread_name.clone(), last_heartbeat.clone()))
                      }
                  })
                  .collect::<Vec<_>>()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const TEST_THRESHOLD: Duration = Duration::from_millis(10);

        #[test]
        fn no_tracking_without_mark_thread_alive() {
            let heartbeats = HashMap::new();
            thread::spawn(|| {}).join().unwrap();
            thread::sleep(TEST_THRESHOLD * 2);
            assert!(threads_missing_heartbeat(&heartbeats, TEST_THRESHOLD).is_empty());
        }

        #[test]
        fn one_dead_thread() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<HeartbeatMap> = Default::default();
            }
            thread::spawn(move || mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap())).join()
                                                                                          .unwrap();
            thread::sleep(TEST_THRESHOLD * 2);
            assert_eq!(threads_missing_heartbeat(&HEARTBEATS.lock().unwrap(), TEST_THRESHOLD).len(),
                       1);
        }

        #[test]
        fn one_dead_one_alive() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<HeartbeatMap> = Default::default();
            }

            let dead_thread_name = "expected-dead".to_string();

            thread::Builder::new().name(dead_thread_name.clone())
                                  .spawn(move || {
                                      mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap())
                                  })
                                  .unwrap()
                                  .join()
                                  .unwrap();
            thread::spawn(move || {
                loop {
                    mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
                    thread::sleep(TEST_THRESHOLD / 2)
                }
            });

            thread::sleep(TEST_THRESHOLD * 2);

            let dead_thread_names = threads_missing_heartbeat(&HEARTBEATS.lock().unwrap(),
                                                              TEST_THRESHOLD).iter()
                                                                             .map(|(name, _)| name.clone())
                                                                             .collect::<Vec<_>>();
            assert_eq!(dead_thread_names, vec![Some(dead_thread_name)]);
        }
    }

    #[cfg(feature = "deadlock_detection")]
    mod deadlock_detection {
        use super::*;
        use parking_lot::deadlock;
        use std::{sync::Once,
                  thread};

        static INIT: Once = Once::new();

        pub fn init() { INIT.call_once(spawn_deadlock_detector_thread); }

        fn spawn_deadlock_detector_thread() {
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(10));
                    let deadlocks = deadlock::check_deadlock();
                    if deadlocks.is_empty() {
                        continue;
                    }

                    println!("{} deadlocks detected", deadlocks.len());
                    for (i, threads) in deadlocks.iter().enumerate() {
                        println!("Deadlock #{}", i);
                        for t in threads {
                            println!("Thread Id {:#?}", t.thread_id());
                            println!("{:#?}", t.backtrace());
                        }
                    }

                    // Unfortunately, we can't do anything to resolve the deadlock and
                    // continue, so we have to abort the whole process
                    std::process::exit(1);
                }
            });
        }
    }

    #[cfg(any(feature = "lock_as_rwlock", not(feature = "lock_as_mutex")))]
    type InnerLock<T> = parking_lot::RwLock<T>;
    #[cfg(feature = "lock_as_mutex")]
    type InnerLock<T> = parking_lot::Mutex<T>;

    #[cfg(any(feature = "lock_as_rwlock", not(feature = "lock_as_mutex")))]
    pub type ReadGuard<'a, T> = parking_lot::RwLockReadGuard<'a, T>;
    #[cfg(feature = "lock_as_mutex")]
    pub type ReadGuard<'a, T> = parking_lot::MutexGuard<'a, T>;

    #[cfg(any(feature = "lock_as_rwlock", not(feature = "lock_as_mutex")))]
    pub type WriteGuard<'a, T> = parking_lot::RwLockWriteGuard<'a, T>;
    #[cfg(feature = "lock_as_mutex")]
    pub type WriteGuard<'a, T> = parking_lot::MutexGuard<'a, T>;

    /// A lock which provides the interface of a read/write lock, but which has the option to
    /// internally use either a RwLock or a Mutex in order to make it easier to expose erroneous
    /// recursive locking in tests while still using an RwLock in production to avoid deadlocking
    /// as much as possible.
    #[derive(Debug)]
    pub struct Lock<T> {
        inner: InnerLock<T>,
    }

    impl<T> Lock<T> {
        pub fn new(val: T) -> Self {
            #[cfg(feature = "lock_as_mutex")]
            println!("Lock::new is using Mutex to help find recursive locking");

            #[cfg(feature = "deadlock_detection")]
            deadlock_detection::init();

            Self { inner: InnerLock::new(val), }
        }

        /// This acquires a read lock and will not deadlock if the same thread tries to acquire
        /// the lock recursively. However, it may result in writer starvation. Once we are confident
        /// that all recursive locking has been eliminated, we may replace this implementation
        /// and try_read_for (or add an additional methods) to provide fair locking for readers
        /// and writers.
        ///
        /// See https://github.com/habitat-sh/habitat/issues/6435
        pub fn read(&self) -> ReadGuard<T> {
            #[cfg(any(feature = "lock_as_rwlock", not(feature = "lock_as_mutex")))]
            {
                self.inner.read_recursive()
            }
            #[cfg(feature = "lock_as_mutex")]
            {
                self.inner.lock()
            }
        }

        pub fn try_read_for(&self, timeout: Duration) -> Option<ReadGuard<T>> {
            #[cfg(any(feature = "lock_as_rwlock", not(feature = "lock_as_mutex")))]
            {
                self.inner.try_read_recursive_for(timeout)
            }
            #[cfg(feature = "lock_as_mutex")]
            {
                self.inner.try_lock_for(timeout)
            }
        }

        pub fn write(&self) -> WriteGuard<T> {
            #[cfg(any(feature = "lock_as_rwlock", not(feature = "lock_as_mutex")))]
            {
                self.inner.write()
            }
            #[cfg(feature = "lock_as_mutex")]
            {
                self.inner.lock()
            }
        }
    }
}
