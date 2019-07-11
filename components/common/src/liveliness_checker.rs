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
///
/// If the thread exits the loop where mark_thread_alive will be called, it must be unregistered
/// from the liveliness checker or else be subject to false positives. In general, the idiom is
/// ```
/// use habitat_common::liveliness_checker;
/// # let error = false;
/// let _: liveliness_checker::ThreadUnregistered<_, _> = loop {
///     let checked_thread = liveliness_checker::mark_thread_alive();
///     if error {
///         break checked_thread.unregister(Err("some description"));
///     } else {
///         break checked_thread.unregister(Ok(()));
///     }
/// };
/// ```
pub fn mark_thread_alive() -> CheckedThread {
    mark_thread_alive_impl(&mut THREAD_STATUSES.lock().expect("THREAD_STATUSES poisoned"))
}

fn mark_thread_alive_impl(statuses: &mut ThreadStatusMap) -> CheckedThread {
    let thread = thread::current();
    statuses.insert(thread.id(),
                    (thread.name().map(str::to_string),
                     Status::Alive { last_heartbeat: Instant::now(), }));
    CheckedThread(std::ptr::null())
}

/// A type to enforce that all code paths to exit a thread call unregister_thread. Since this
/// type can't be instantiated outside this module, the only way to have consistent return
/// types is to call unregister_thread on all returning paths. This can't prevent the error
/// that *no* code paths call unregister_thread, but it should help avoid the situation of a
/// particular one being overlooked.
///
/// Additionally, since code paths which the threads pass through while unregistering may wish
/// to communicate a Result back to the caller, this type wraps an arbitrary `Result` which can
/// be then recovered via `into_result`, or if an arbitrary value is desired, via `into_ok`.
#[must_use]
pub struct ThreadUnregistered<T = (), E = std::convert::Infallible>(Result<T, E>);

impl<T, E> ThreadUnregistered<T, E> {
    pub fn into_result(self) -> Result<T, E> { self.0 }
}

impl<T> ThreadUnregistered<T, std::convert::Infallible> {
    pub fn into_ok(self) -> T {
        match self.0 {
            Ok(v) => v,
            Err(_) => unreachable!(),
        }
    }
}

/// A type to provide the option to unregister from liveliness checking. Since this type can
/// only be obtained by calling `mark_thread_alive`, it ensures only threads which have
/// registered with the liveliness checker can unregister. And since it is `must_use`, it helps
/// ensure that threads unregister appropriately (or mark themselves as never exiting with
/// `and_divergent`).
// ---
// Since negative trait bounds aren't available yet, we use a raw pointer to force this type
// to be neither Send nor Sync. It wouldn't do much good if it could be sent between threads.
//
// See https://github.com/rust-lang/rust/issues/13231
#[must_use]
pub struct CheckedThread(*const ());

impl CheckedThread {
    /// Call this method to indicate the checker shouldn't expect future heartbeats.
    /// If the thread is exiting as part of expected operation, `reason` should be `Ok(())`
    /// and the thread will no longer be checked. If the thread is unregistering due to an error,
    /// `reason` should be an `Err(_)` explaining why.
    ///
    /// The `ThreadUnregistered` serves as a sentinel value to ensure all code paths away from the
    /// loop which calls mark_thread_alive properly unregister from the checker, otherwise false
    /// positives for exited threads could result. See that type's documentation for more.
    // TODO: make this a method on a must_use type returned by mark_thread_alive, so there's no way
    // to call this unless the thread was previously marked alive.
    pub fn unregister<T, E: ToString>(self, reason: Result<T, E>) -> ThreadUnregistered<T, E> {
        unregister_thread_impl(self,
                               &mut THREAD_STATUSES.lock().expect("THREAD_STATUSES poisoned"),
                               reason)
    }

    /// In general, the return of mark_thread_alive must be used, to help ensure that threads
    /// are unregistered from checking when they exit their work loop. However, this does not
    /// apply to divergent threads (that is, ones that never return), so instead of storing
    /// an ignored value to satisfy the `must_use` of `mark_thread_alive`, call this function
    /// to indicate the intent to never terminate.
    ///
    /// See http://doc.rust-lang.org/std/primitive.never.html
    ///
    /// ```
    /// use habitat_common::liveliness_checker;
    ///
    /// fn run_loop() -> ! {
    ///     loop {
    ///         liveliness_checker::mark_thread_alive().and_divergent();
    ///     }
    /// }
    /// ```
    pub fn and_divergent(self) {}
}

fn unregister_thread_impl<T, E: ToString>(_: CheckedThread,
                                          statuses: &mut ThreadStatusMap,
                                          reason: Result<T, E>)
                                          -> ThreadUnregistered<T, E> {
    let thread = thread::current();
    let thread_id = &thread.id();

    match &reason {
        Ok(_) => {
            statuses.remove(thread_id);
        }
        Err(e) => {
            if let Some(entry) = statuses.get_mut(thread_id) {
                entry.1 = Status::DeadWithError { time_of_death: Instant::now(),
                                                  error:         e.to_string(), };
            } else {
                error!("unregister_thread called for untracked thread: {} id: {:?}",
                       thread.name().unwrap_or("Unnamed thread"),
                       thread_id);
            }
        }
    }

    ThreadUnregistered(reason)
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
        thread::spawn(move || {
            let _ = mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
        }).join()
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
                                  {
                                      let _ =
                                          mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
                                  }
                              })
                              .unwrap()
                              .join()
                              .unwrap();
        thread::spawn(move || {
            while !test_done2.load(Ordering::Relaxed) {
                let _ = mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
                thread::sleep(TEST_THRESHOLD / 2);
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
            let _ = mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
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
            let _ = mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
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
        let _ =
            thread::spawn(move || -> ThreadUnregistered<(), _> {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                let checked_thread = mark_thread_alive_impl(statuses);
                unregister_thread_impl(checked_thread, statuses, Err("thread error description"))
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
        let _ = thread::spawn(move || -> ThreadUnregistered {
                    let statuses = &mut HEARTBEATS.lock().unwrap();
                    let checked_thread = mark_thread_alive_impl(statuses);
                    unregister_thread_impl(checked_thread, statuses, Ok(()))
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
            let _ = mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
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
            let _ = mark_thread_alive_impl(&mut HEARTBEATS.lock().unwrap());
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
        let _ = thread::spawn(move || -> ThreadUnregistered {
                    let statuses = &mut HEARTBEATS.lock().unwrap();
                    let checked_thread = mark_thread_alive_impl(statuses);
                    unregister_thread_impl(checked_thread, statuses, Ok(()))
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
        let _ =
            thread::spawn(move || -> ThreadUnregistered<(), _> {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                let checked_thread = mark_thread_alive_impl(statuses);
                unregister_thread_impl(checked_thread, statuses, Err("thread error description"))
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
        let _ =
            thread::spawn(move || -> ThreadUnregistered<(), _> {
                let statuses = &mut HEARTBEATS.lock().unwrap();
                let checked_thread = mark_thread_alive_impl(statuses);
                unregister_thread_impl(checked_thread, statuses, Err("thread error description"))
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
