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
pub mod liveliness_checker;
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
        const LIST                 = 0b0000_0000_0001;
        const TEST_EXIT            = 0b0000_0000_0010;
        const TEST_BOOT_FAIL       = 0b0000_0000_0100;
        const REDACT_HTTP          = 0b0000_0000_1000;
        const OFFLINE_INSTALL      = 0b0000_0100_0000;
        const IGNORE_LOCAL         = 0b0000_1000_0000;
        const EVENT_STREAM         = 0b0001_0000_0000;
        const TRIGGER_ELECTION     = 0b0010_0000_0000;
    }
}

lazy_static! {
    static ref ENV_VARS: HashMap<FeatureFlag, &'static str> = {
        let mapping = vec![(FeatureFlag::LIST, "HAB_FEAT_LIST"),
                           (FeatureFlag::TEST_EXIT, "HAB_FEAT_TEST_EXIT"),
                           (FeatureFlag::TEST_BOOT_FAIL, "HAB_FEAT_BOOT_FAIL"),
                           (FeatureFlag::REDACT_HTTP, "HAB_FEAT_REDACT_HTTP"),
                           (FeatureFlag::OFFLINE_INSTALL, "HAB_FEAT_OFFLINE_INSTALL"),
                           (FeatureFlag::IGNORE_LOCAL, "HAB_FEAT_IGNORE_LOCAL"),
                           (FeatureFlag::EVENT_STREAM, "HAB_FEAT_EVENT_STREAM"),
                           (FeatureFlag::TRIGGER_ELECTION, "HAB_FEAT_TRIGGER_ELECTION"),];

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
    use std::time::Duration;

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

    impl<T: Default> Default for Lock<T> {
        fn default() -> Self { Self { inner: InnerLock::new(T::default()), } }
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
