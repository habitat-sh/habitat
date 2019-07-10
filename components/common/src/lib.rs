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
                           (FeatureFlag::TRIGGER_ELECTION, "HAB_FEAT_TRIGGER_ELECTION")];
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
    use std::{collections::HashMap,
              sync::Mutex,
              thread::{self,
                       ThreadId},
              time::{Duration,
                     Instant}};

    // Threads that end normally, simply aren't tracked any longer
    enum Status {
        Alive {
            last_heartbeat: Instant,
        },
        DeadWithError {
            time_of_death: Instant,
            error:         String,
        },
    }
    type NameAndStatus = (Option<String>, Status);
    type NameAndLastHeartbeat = (Option<String>, Instant);
    type NameAndErrorExitTimeAndReason<'a> = (Option<String>, Instant, &'a str);
    type ThreadStatusMap = HashMap<ThreadId, NameAndStatus>;
    lazy_static::lazy_static! {
        static ref THREAD_STATUSES: Mutex<ThreadStatusMap> = Default::default();
    }

    habitat_core::env_config_duration!(ThreadAliveThreshold,
                                       HAB_THREAD_ALIVE_THRESHOLD_SECS => from_secs,
                                       Duration::from_secs(5 * 60));
    habitat_core::env_config_duration!(ThreadAliveCheckDelay,
                                       HAB_THREAD_ALIVE_CHECK_DELAY_SECS => from_secs,
                                       Duration::from_secs(60));
    habitat_core::env_config_duration!(ThreadDeadIgnoreDelay,
                                       HAB_THREAD_DEAD_IGNORE_DELAY_SECS => from_secs,
                                       Duration::from_secs(60 * 60 * 24 * 7)); // 1 week

    /// Call periodically from a thread which has a work loop to indicate that the thread is
    /// still alive and processing its loop. If this function is not called for more than
    /// `ThreadAliveThreshold`, it will be error logged as a likely deadlock.
    pub fn mark_thread_alive() {
        mark_thread_alive_impl(&mut THREAD_STATUSES.lock().expect("THREAD_STATUSES poisoned"));
    }

    fn mark_thread_alive_impl(statuses: &mut ThreadStatusMap) {
        let thread = thread::current();
        statuses.insert(thread.id(),
                        (thread.name().map(str::to_string),
                         Status::Alive { last_heartbeat: Instant::now(), }));
    }

    // TODO: Rename, more like ThreadUnregisteredResult
    /// A type to enforce that all code paths to exit a thread call mark_thread_dead. Since this
    /// type can't be instantiated outside this module, the only way to have consistent return
    /// types is to call mark_thread_dead on all returning paths. This can't prevent the error
    /// that *no* code paths call mark_thread_dead, but it should help avoid the situation of a
    /// particular one being overlooked.
    pub struct ThreadReturn<T = (), E = std::convert::Infallible>(Result<T, E>);

    impl<T, E> ThreadReturn<T, E> {
        pub fn into_result(self) -> Result<T, E> { self.0 }
    }

    impl<T> ThreadReturn<T, std::convert::Infallible> {
        pub fn into_ok(self) -> T {
            match self.0 {
                Ok(v) => v,
                Err(_) => unreachable!(),
            }
        }
    }

    /// Call when a thread is exiting to indicate the checker shouldn't expect future heartbeats.
    /// If the thread is exiting as part of expected operation, `exit_result` should be `Ok(())`
    /// and the thread will no longer be checked. If the thread exited due to an error,
    /// `exit_result` should be an `Err(&str)` explaining why.
    // TODO: rename more like unregister_thread
    pub fn mark_thread_dead<T, E: ToString>(exit_result: Result<T, E>) -> ThreadReturn<T, E> {
        mark_thread_dead_impl(&mut THREAD_STATUSES.lock().expect("THREAD_STATUSES poisoned"),
                              exit_result)
    }

    fn mark_thread_dead_impl<T, E: ToString>(statuses: &mut ThreadStatusMap,
                                             exit_result: Result<T, E>)
                                             -> ThreadReturn<T, E> {
        let thread_id = &thread::current().id();

        match &exit_result {
            Ok(_) => {
                statuses.remove(thread_id);
            }
            Err(e) => {
                if let Some(entry) = statuses.get_mut(thread_id) {
                    entry.1 = Status::DeadWithError { time_of_death: Instant::now(),
                                                      error:         e.to_string(), };
                } else {
                    // TODO: Better message
                    error!("mark_thread_dead called for untracked thread");
                }
            }
        }

        ThreadReturn(exit_result)
    }

    /// Call once per binary to start the thread which will check that all the threads that
    /// call `mark_thread_alive` continue to do so.
    pub fn spawn_thread_alive_checker() {
        thread::Builder::new().name("thread-alive-check".to_string())
                              .spawn(|| -> ! {
                                  let delay = ThreadAliveCheckDelay::configured_value().into();
                                  let threshold = ThreadAliveThreshold::configured_value().into();
                                  let max_time_since_death =
                                      ThreadDeadIgnoreDelay::configured_value().into();
                                  loop {
                                      let statuses =
                                          &mut THREAD_STATUSES.lock()
                                                              .expect("THREAD_STATUSES poisoned");
                                      check_thread_heartbeats(statuses, threshold);
                                      log_dead_threads(statuses);
                                      cull_dead_threads(statuses, max_time_since_death);
                                      thread::sleep(delay);
                                  }
                              })
                              .expect("Error spawning thread alive checker");
    }

    fn check_thread_heartbeats(statuses: &ThreadStatusMap, threshold: Duration) {
        for (name, last_heartbeat) in threads_missing_heartbeat(statuses, threshold) {
            warn!("No heartbeat from {} in {} seconds; deadlock likely",
                  name.unwrap_or_else(|| { "unnamed thread".to_string() }),
                  last_heartbeat.elapsed().as_secs());
        }
    }

    fn log_dead_threads(statuses: &ThreadStatusMap) {
        for (name, time_of_death, error) in threads_exited_with_error(statuses) {
            warn!("{} exited {} seconds ago with error: {}",
                  name.unwrap_or_else(|| { "Unnamed thread".to_string() }),
                  time_of_death.elapsed().as_secs(),
                  error);
        }
    }

    fn cull_dead_threads(statuses: &mut ThreadStatusMap, max_time_since_death: Duration) {
        statuses.retain(|_thread_id, (_thread_name, status)| {
                    match status {
                        Status::Alive { .. } => true,
                        Status::DeadWithError { time_of_death, .. } => {
                            time_of_death.elapsed() < max_time_since_death
                        }
                    }
                });
    }

    fn threads_missing_heartbeat(statuses: &ThreadStatusMap,
                                 threshold: Duration)
                                 -> Vec<NameAndLastHeartbeat> {
        statuses.iter()
                .filter_map(|(thread_id, (thread_name, status))| {
                    match status {
                        Status::Alive { last_heartbeat } => {
                            let time_since_last_heartbeat = last_heartbeat.elapsed();
                            trace!("{:?} {:?} last heartbeat: {:?} ago",
                                   thread_id,
                                   thread_name,
                                   time_since_last_heartbeat);
                            if time_since_last_heartbeat < threshold {
                                None
                            } else {
                                Some((thread_name.clone(), *last_heartbeat))
                            }
                        }
                        Status::DeadWithError { .. } => None,
                    }
                })
                .collect::<Vec<_>>()
    }

    fn threads_exited_with_error(statuses: &ThreadStatusMap) -> Vec<NameAndErrorExitTimeAndReason> {
        statuses.iter()
                .filter_map(|(thread_id, (thread_name, status))| {
                    match status {
                        Status::Alive { .. } => None,
                        Status::DeadWithError { time_of_death,
                                                error, } => {
                            let time_since_exit = time_of_death.elapsed();
                            trace!("{:?} {:?} time of death: {:?} ago",
                                   thread_id,
                                   thread_name,
                                   time_since_exit);
                            Some((thread_name.clone(), *time_of_death, error.as_ref()))
                        }
                    }
                })
                .collect::<Vec<_>>()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::sync::{atomic::{AtomicBool,
                                 Ordering},
                        Arc};

        const TEST_THRESHOLD: Duration = Duration::from_secs(1);

        #[test]
        fn no_tracking_without_mark_thread_alive() {
            let statuses = HashMap::new();
            thread::spawn(|| {}).join().unwrap();
            thread::sleep(TEST_THRESHOLD * 2);
            assert!(threads_missing_heartbeat(&statuses, TEST_THRESHOLD).is_empty());
        }

        #[test]
        fn one_dead_thread() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
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
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }

            let dead_thread_name = "expected-dead".to_string();
            let test_done: Arc<AtomicBool> = Default::default();
            let test_done2 = Arc::clone(&test_done);

            thread::Builder::new().name(dead_thread_name.clone())
                                  .spawn(move || {
                                      mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap())
                                  })
                                  .unwrap()
                                  .join()
                                  .unwrap();
            thread::spawn(move || {
                while !test_done2.load(Ordering::Relaxed) {
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
            test_done.store(true, Ordering::Relaxed);
        }

        #[test]
        fn threads_missing_heartbeat_includes_panicked_threads() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || {
                mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
                panic!("intentional panic for test");
            }).join()
              .ok();
            thread::sleep(TEST_THRESHOLD * 2);
            assert_eq!(threads_missing_heartbeat(&HEARTBEATS.lock().unwrap(), TEST_THRESHOLD).len(),
                       1);
        }

        #[test]
        fn threads_missing_heartbeat_includes_unexpectedly_ended_threads() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || {
                mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
            }).join()
              .unwrap();
            thread::sleep(TEST_THRESHOLD * 2);
            assert_eq!(threads_missing_heartbeat(&HEARTBEATS.lock().unwrap(), TEST_THRESHOLD).len(),
                       1);
        }

        #[test]
        fn threads_missing_heartbeat_excludes_threads_ending_with_err() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || -> ThreadReturn<(), _> {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                mark_thread_alive_impl(statuses);
                mark_thread_dead_impl(statuses, Err("thread error description"))
            }).join()
              .unwrap();
            thread::sleep(TEST_THRESHOLD * 2);
            assert_eq!(threads_missing_heartbeat(&HEARTBEATS.lock().unwrap(), TEST_THRESHOLD).len(),
                       0);
        }

        #[test]
        fn threads_missing_heartbeat_excludes_threads_ending_with_ok() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || -> ThreadReturn {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                mark_thread_alive_impl(statuses);
                mark_thread_dead_impl(statuses, Ok(()))
            }).join()
              .unwrap();
            thread::sleep(TEST_THRESHOLD * 2);
            assert_eq!(threads_missing_heartbeat(&HEARTBEATS.lock().unwrap(), TEST_THRESHOLD).len(),
                       0);
        }

        #[test]
        fn threads_exited_with_error_excludes_panicked_threads() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || {
                mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
                panic!("intentional panic for test");
            }).join()
              .ok();
            assert_eq!(threads_exited_with_error(&HEARTBEATS.lock().unwrap()).len(),
                       0);
        }

        #[test]
        fn threads_exited_with_error_excludes_unexpectedly_ended_threads() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || {
                mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
            }).join()
              .unwrap();
            assert_eq!(threads_exited_with_error(&HEARTBEATS.lock().unwrap()).len(),
                       0);
        }

        #[test]
        fn threads_exited_with_error_excludes_threads_ending_with_ok() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || -> ThreadReturn {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                mark_thread_alive_impl(statuses);
                mark_thread_dead_impl(statuses, Ok(()))
            }).join()
              .unwrap();
            assert_eq!(threads_exited_with_error(&HEARTBEATS.lock().unwrap()).len(),
                       0);
        }

        #[test]
        fn threads_exited_with_error_includes_threads_ending_with_err() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || -> ThreadReturn<(), _> {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                mark_thread_alive_impl(statuses);
                mark_thread_dead_impl(statuses, Err("thread error description"))
            }).join()
              .unwrap();
            assert_eq!(threads_exited_with_error(&HEARTBEATS.lock().unwrap()).len(),
                       1);
        }

        #[test]
        fn threads_exited_with_error_excludes_threads_ending_with_err_after_expiration_period() {
            lazy_static! {
                static ref HEARTBEATS: Mutex<ThreadStatusMap> = Default::default();
            }
            thread::spawn(move || -> ThreadReturn<(), _> {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                mark_thread_alive_impl(statuses);
                mark_thread_dead_impl(statuses, Err("thread error description"))
            }).join()
              .unwrap();
            let statuses = &mut HEARTBEATS.lock().unwrap();
            cull_dead_threads(statuses, TEST_THRESHOLD);
            assert_eq!(threads_exited_with_error(statuses).len(), 1);
            thread::sleep(TEST_THRESHOLD * 2);
            cull_dead_threads(statuses, TEST_THRESHOLD);
            assert_eq!(threads_exited_with_error(statuses).len(), 0);
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
