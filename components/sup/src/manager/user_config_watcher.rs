// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{
    mpsc::{
        channel, sync_channel, Receiver, SendError, Sender, SyncSender, TryRecvError, TrySendError,
    },
    Arc, Mutex,
};
use std::thread::{self, Builder as ThreadBuilder};
use std::time::Duration;

use super::file_watcher::{default_file_watcher_with_no_initial_event, Callbacks};

use crate::common::templating::config::UserConfigPath;
use crate::hcore::fs::USER_CONFIG_FILE;
use crate::hcore::service::ServiceGroup;
use crate::manager::service::Service;

static LOGKEY: &'static str = "UCW";

// This trait exists to ease the testing of functions that receive a Service. Creating Services
// requires a lot of ceremony, so we work around this with this trait.
pub trait Serviceable {
    fn name(&self) -> &str;
    fn user_config_path(&self) -> &UserConfigPath;
    fn service_group(&self) -> &ServiceGroup;
}

impl Serviceable for Service {
    fn name(&self) -> &str {
        &self.pkg.name
    }

    fn user_config_path(&self) -> &UserConfigPath {
        &self.cfg.user_config_path
    }

    fn service_group(&self) -> &ServiceGroup {
        &self.service_group
    }
}

// WorkerState contains the channels the worker uses to communicate
// with the Watcher.
struct WorkerState {
    // This receiver is used by the watcher to be notified when a
    // worker has events.  The channel is a SyncChannel with buffer
    // size 1, as we are only interested in the fact that there were
    // events, not how many there were.
    have_events: Receiver<()>,
    // This sender is used by the watcher to notify a worker to stop
    // running.  It is an async channel because we never want the
    // UserConfigWatcher to block, even if the receiver end of the
    // channel somehow dies and/or fails to consume the message.
    stop_running: Sender<()>,
    // This receiver is used by the watcher tests to be notified when
    // a worker finished setting up the watcher and is about to
    // starting looping it.
    //
    // Silence the dead code warnings from rustc, because it is only
    // used in tests for synchronization purposes.
    #[allow(dead_code)]
    started_watching: Receiver<()>,
}

type ServiceName = String;
pub struct UserConfigWatcher {
    // We use Arc/Mutex here, because this needs to be shareable
    // across threads so we can remove watchers from futures; the
    // Sender and Receiver members of WorkerState aren't themselves
    // shareable.
    states: Arc<Mutex<HashMap<ServiceName, WorkerState>>>,
}

impl UserConfigWatcher {
    pub fn new() -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a service to the User Config Watcher, thereby starting a watcher thread.
    pub fn add<T: Serviceable>(&mut self, service: &T) -> io::Result<()> {
        // It isn't possible to use the `or_insert_with` function here because it can't have a
        // return value, which we need to return the error from `Worker::run`.
        let mut states = self.states.lock().expect("states lock was poisoned");
        if states.get(service.name()).is_none() {
            let user_toml_path = match service.user_config_path() {
                UserConfigPath::Recommended(ref p) => p.join(USER_CONFIG_FILE),
                UserConfigPath::Deprecated(ref p) => {
                    outputln!(
                        preamble service.service_group(),
                        "Not watching {}, because it is located in deprecated path ({}).",
                        USER_CONFIG_FILE,
                        p.display(),
                    );
                    return Ok(());
                }
            };
            // Establish bi-directional communication with the worker by creating two channels.
            // The sync_channel's buffer size is 1 because we want to use it as a boolean, i.e. we
            // are not interested in the events themselves, but only whether at least one has
            // happened.
            let (events_tx, events_rx) = sync_channel(1);
            let (running_tx, running_rx) = channel();
            let (watching_tx, watching_rx) = sync_channel(1);

            Worker::run(user_toml_path, events_tx, running_rx, watching_tx)?;

            outputln!(preamble service.service_group(), "Watching {}", USER_CONFIG_FILE);

            let state = WorkerState {
                have_events: events_rx,
                stop_running: running_tx,
                started_watching: watching_rx,
            };

            states.insert(service.name().to_owned(), state);
        }

        Ok(())
    }

    /// Removes a service from the User Config Watcher, and sends a message to the watcher thread
    /// to stop running.
    pub fn remove<T: Serviceable>(&mut self, service: &T) -> Result<(), SendError<()>> {
        if let Some(state) = self
            .states
            .lock()
            .expect("states lock was poisoned")
            .remove(service.name())
        {
            state.stop_running.send(())?;
        }

        Ok(())
    }

    /// Checks whether the watcher for the specified service has observed any events.
    ///
    /// This also consumes the events.
    pub fn have_events_for<T: Serviceable>(&self, service: &T) -> bool {
        if let Some(state) = self
            .states
            .lock()
            .expect("states lock was poisoned")
            .get(service.name())
        {
            let rx = &state.have_events;

            match rx.try_recv() {
                Ok(_) => {
                    return true;
                }
                Err(TryRecvError::Empty) => return false,
                Err(TryRecvError::Disconnected) => {
                    debug!("UserConfigWatcher worker has died!");
                    return false;
                }
            }
        }

        false
    }
}

struct UserConfigCallbacks {
    have_events: SyncSender<()>,
}

impl Callbacks for UserConfigCallbacks {
    fn file_appeared(&mut self, _: &Path) {
        self.perform();
    }

    fn file_modified(&mut self, _: &Path) {
        self.perform();
    }

    fn file_disappeared(&mut self, _: &Path) {
        self.perform();
    }
}

impl UserConfigCallbacks {
    fn perform(&self) {
        if let Err(TrySendError::Disconnected(_)) = self.have_events.try_send(()) {
            debug!("Worker could not notify Manager of event");
        }
    }
}

struct Worker;

impl Worker {
    // starts a new thread with the file watcher tracking the service's user-config file
    pub fn run(
        path: PathBuf,
        have_events: SyncSender<()>,
        stop_running: Receiver<()>,
        started_watching: SyncSender<()>,
    ) -> io::Result<()> {
        ThreadBuilder::new()
            .name(format!("user-config-watcher-{}", path.display()))
            .spawn(move || {
                debug!(
                    "UserConfigWatcher({}) worker thread starting",
                    path.display(),
                );
                let callbacks = UserConfigCallbacks {
                    have_events: have_events,
                };
                let mut file_watcher =
                    match default_file_watcher_with_no_initial_event(&path, callbacks) {
                        Ok(w) => w,
                        Err(e) => {
                            outputln!(
                                "UserConfigWatcher({}) could not start notifier, \
                                 ending thread ({})",
                                path.display(),
                                e,
                            );
                            return;
                        }
                    };

                let _ = started_watching.try_send(());

                loop {
                    match stop_running.try_recv() {
                        // As long as the `stop_running` channel is
                        // empty, this branch will execute on every
                        // iteration.
                        Err(TryRecvError::Empty) => {
                            if let Err(e) = file_watcher.single_iteration() {
                                outputln!(
                                    "UserConfigWatcher({}) could not run notifier, \
                                     ending thread ({})",
                                    path.display(),
                                    e,
                                );
                                return;
                            };
                        }

                        // If we receive a message on the channel, we stop.
                        Ok(_) => break,

                        // If the channel is disconnected, we stop as well.
                        Err(TryRecvError::Disconnected) => {
                            debug!(
                                "UserConfigWatcher({}) worker thread failed to receive on channel",
                                path.display(),
                            );
                            break;
                        }
                    }

                    thread::sleep(Duration::from_secs(1));
                }
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::{remove_file, File};
    use std::io::Write;
    use std::str::FromStr;
    use std::thread;
    use std::time::{Duration, Instant};

    use crate::manager::file_watcher::WATCHER_DELAY_MS;

    use tempfile::TempDir;

    #[test]
    fn no_events_at_first() {
        let service = TestService::default();
        let mut ucm = UserConfigWatcher::new();
        ucm.add(&service).expect("adding service");

        assert!(!ucm.have_events_for(&service));
    }

    #[test]
    fn events_present_after_adding_config() {
        let service = TestService::default();
        let mut ucm = UserConfigWatcher::new();
        ucm.add(&service).expect("adding service");
        assert!(wait_for_watcher(&ucm, &service));

        File::create(service.user_config_path().get_path().join(USER_CONFIG_FILE))
            .expect("creating file");

        assert!(wait_for_events(&ucm, &service));
    }

    #[test]
    fn events_present_after_changing_config() {
        let service = TestService::default();
        let file_path = service.user_config_path().get_path().join(USER_CONFIG_FILE);
        let mut ucm = UserConfigWatcher::new();

        ucm.add(&service).expect("adding service");
        assert!(wait_for_watcher(&ucm, &service));
        let mut file = File::create(&file_path).expect("creating file");

        file.write_all(b"42").expect(USER_CONFIG_FILE);

        assert!(wait_for_events(&ucm, &service));
    }

    #[test]
    fn events_present_after_removing_config() {
        let service = TestService::default();
        let file_path = service.user_config_path().get_path().join(USER_CONFIG_FILE);
        let mut ucm = UserConfigWatcher::new();

        ucm.add(&service).expect("adding service");
        assert!(wait_for_watcher(&ucm, &service));
        File::create(&file_path).expect("creating file");

        // Allow the watcher to notice that a file was created.
        wait_for_events(&ucm, &service);

        remove_file(&file_path).expect("removing file");

        assert!(wait_for_events(&ucm, &service));
    }

    fn wait_for_watcher<T: Serviceable>(ucm: &UserConfigWatcher, service: &T) -> bool {
        let start = Instant::now();
        let timeout = Duration::from_secs(10);

        while start.elapsed() < timeout {
            let states = ucm.states.lock().expect("states lock was poisoned");
            let state = states.get(service.name()).expect("service added");
            match state.started_watching.try_recv() {
                Ok(_) => {
                    println!("Received data on the start_watching channel. Returning true.");
                    return true;
                }
                Err(TryRecvError::Empty) => {
                    println!("Received nothing on the start_watching channel. Returning ().");
                }
                Err(TryRecvError::Disconnected) => {
                    println!("The start_watching channel was disconnected. Returning false.");
                    return false;
                }
            }

            thread::sleep(Duration::from_millis(100));
        }

        false
    }

    fn wait_for_events<T: Serviceable>(ucm: &UserConfigWatcher, service: &T) -> bool {
        let start = Instant::now();
        let timeout = Duration::from_millis(WATCHER_DELAY_MS * 5);

        while start.elapsed() < timeout {
            if ucm.have_events_for(service) {
                return true;
            }

            thread::sleep(Duration::from_millis(100));
        }

        false
    }

    struct TestService {
        // It is only used as a way to remove the temporary directory
        // when the test finishes. Not used otherwise.
        #[allow(dead_code)]
        tmp: TempDir,
        name: String,
        user_config_path: UserConfigPath,
        service_group: ServiceGroup,
    }

    impl Serviceable for TestService {
        fn name(&self) -> &str {
            &self.name
        }

        fn user_config_path(&self) -> &UserConfigPath {
            &self.user_config_path
        }

        fn service_group(&self) -> &ServiceGroup {
            &self.service_group
        }
    }

    impl Default for TestService {
        fn default() -> Self {
            let tmp = TempDir::new().expect("creating temp dir");
            let path = UserConfigPath::Recommended(tmp.path().to_path_buf());
            Self {
                tmp: tmp,
                name: String::from("foo"),
                user_config_path: path,
                service_group: ServiceGroup::from_str("foo.bar@yoyodine").unwrap(),
            }
        }
    }
}
