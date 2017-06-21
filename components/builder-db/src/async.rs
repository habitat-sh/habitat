// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::fmt;

use rand::{self, Rng};
use time::SteadyTime;
use time::Duration as SteadyDuration;
use threadpool::ThreadPool;

use pool::Pool;
use error::Result;

pub type DispatchKey = String;
pub type EventFunction = fn(Pool) -> Result<EventOutcome>;

pub enum EventOutcome {
    Finished,
    Retry,
}

const BACKOFF_SLOT_TIME_MS: u64 = 100;
const FAILURE_COUNT_UPPER_BOUND: usize = 10;

#[derive(Clone)]
pub struct AsyncServer {
    pool: Pool,
    stop: Arc<AtomicBool>,
    pub dispatch: Arc<RwLock<HashMap<DispatchKey, EventFunction>>>,
    pub failure_count: Arc<RwLock<HashMap<DispatchKey, usize>>>,
    pub retry: Arc<RwLock<HashMap<DispatchKey, (SteadyTime, EventFunction)>>>,
    pub running: Arc<RwLock<HashSet<DispatchKey>>>,
}

impl fmt::Debug for AsyncServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AsyncServer")
    }
}

impl AsyncServer {
    pub fn new(pool: Pool) -> AsyncServer {
        AsyncServer {
            pool: pool,
            stop: Arc::new(AtomicBool::new(false)),
            dispatch: Arc::new(RwLock::new(HashMap::new())),
            failure_count: Arc::new(RwLock::new(HashMap::new())),
            retry: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn schedule(&self, dispatch_key: &str) -> Result<()> {
        let has_key = {
            let d = self.dispatch.read().expect(
                "Async dispatch lock is poisoned",
            );
            d.contains_key(dispatch_key)
        };
        let is_scheduled = {
            let r = self.retry.read().expect("Async retry lock is poisoned");
            r.contains_key(dispatch_key)
        };
        if has_key && !is_scheduled {
            let function = {
                let d = self.dispatch.read().expect(
                    "Async dispatch lock is poisoned",
                );
                // Safe because we checked above
                d.get(dispatch_key).unwrap().clone()
            };
            let mut r = self.retry.write().expect("Async retry lock is poisoned");
            r.insert(dispatch_key.to_string(), (SteadyTime::now(), function));
        }
        Ok(())
    }

    pub fn register(&self, dispatch_key: DispatchKey, callback: EventFunction) {
        {
            let mut d = self.dispatch.write().expect(
                "Async dispatch lock is poisoned",
            );
            d.insert(dispatch_key.clone(), callback.clone());
        }
        {
            let mut r = self.retry.write().expect("Async retry lock is poisoned");
            r.insert(dispatch_key, (SteadyTime::now(), callback));
        }
    }

    fn get_num_events(&self, available_jobs: usize) -> Vec<(DispatchKey, EventFunction)> {
        let r = self.retry.read().expect("Async retry lock is poisoned");
        r.iter()
            .filter(|&(_dispatch_key, &(event_time, _callback))| {
                SteadyTime::now() >= event_time
            })
            .take(available_jobs)
            .map(|(dispatch_key, &(_event_time, callback))| {
                (dispatch_key.to_string(), callback.clone())
            })
            .collect()
    }

    pub fn run_event(&self, key: DispatchKey, event: EventFunction) {
        let remove_key = key.clone();
        match event(self.pool.clone()) {
            Ok(EventOutcome::Finished) => {
                debug!("Event finished {}", key);
                let mut r = self.retry.write().expect("Async retry lock poisoned");
                let mut f = self.failure_count.write().expect(
                    "Async failure count lock poisoned",
                );
                f.remove(&key);
                r.remove(&key);
            }
            Ok(EventOutcome::Retry) => {
                self.retry_failed_event(key, event);
            }
            Err(e) => {
                warn!("Event {} failed, {}", key, e);
                self.retry_failed_event(key, event);
            }
        }
        let mut running = self.running.write().expect("Running lock is poisoned");
        running.remove(&remove_key);
    }

    fn retry_failed_event(&self, key: DispatchKey, event: EventFunction) {
        warn!("Scheduling retry of {:?}", key);
        let failure_count = {
            let mut f = self.failure_count.write().expect(
                "Async failure count lock poisoned",
            );
            let mut value = f.entry(key.clone()).or_insert(0);
            *value += 1;
            if *value >= FAILURE_COUNT_UPPER_BOUND {
                *value
            } else {
                FAILURE_COUNT_UPPER_BOUND
            }
        };
        let mut rng = rand::thread_rng();
        let high_end = 2u64.pow(failure_count as u32);
        let cycles = rng.gen_range(0, high_end);
        let next_event = SteadyTime::now() +
            SteadyDuration::milliseconds((cycles * BACKOFF_SLOT_TIME_MS) as i64);
        info!("Backing off {:?} for {:?}", key, next_event);
        let mut r = self.retry.write().expect("Async retry lock poisoned");
        r.insert(key, (next_event, event));
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
    }

    pub fn start(self, workers: usize) {
        let server = self;

        let _ = thread::Builder::new()
            .name("async-events".to_string())
            .spawn(move || {
                let threadpool =
                    ThreadPool::new_with_name("async-event-worker".to_string(), workers);
                loop {
                    if server.stop.load(Ordering::SeqCst) {
                        return;
                    }
                    let available_jobs = threadpool.max_count() - threadpool.active_count();
                    if available_jobs == 0 {
                        info!("Async Pool is full; delaying all events for 500ms");
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                    let events = server.get_num_events(available_jobs);
                    for (key, event) in events.into_iter() {
                        let is_running = {
                            let running_events =
                                server.running.read().expect("Running events lock poisoned");
                            running_events.contains(&key)
                        };
                        if !is_running {
                            let sa = server.clone();
                            error!("Dispatching {}", key);
                            {
                                let mut running_events = server.running.write().expect(
                                    "Running events lock poisoned",
                                );
                                running_events.insert(key.clone());
                            }
                            {
                                let mut r =
                                    server.retry.write().expect("Async write lock poisoned");
                                r.remove(&key);
                            }
                            threadpool.execute(move || sa.run_event(key, event));
                        }
                    }
                    thread::sleep(Duration::from_millis(200));
                }
            });
    }
}
