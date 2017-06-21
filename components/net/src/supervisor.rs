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

use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;

use config::DispatcherCfg;
use dispatcher::Dispatcher;

pub struct Supervisor<T>
where
    T: Dispatcher,
{
    config: Arc<RwLock<T::Config>>,
    workers: Vec<mpsc::Receiver<()>>,
    init_state: <T as Dispatcher>::InitState,
}

impl<T> Supervisor<T>
where
    T: Dispatcher + 'static,
{
    // JW TODO: this should take a struct that implements "application config"
    pub fn new(config: Arc<RwLock<T::Config>>, state: <T as Dispatcher>::InitState) -> Self {
        let worker_count = {
            config.read().unwrap().deref().worker_count()
        };
        Supervisor {
            config: config,
            workers: Vec::with_capacity(worker_count),
            init_state: state,
        }
    }

    /// Start the supervisor and block until all workers are ready.
    pub fn start(mut self) -> super::Result<()> {
        try!(self.init());
        debug!("Supervisor ready");
        self.run()
    }

    // Initialize worker pool blocking until all workers are started and ready to begin processing
    // requests.
    fn init(&mut self) -> super::Result<()> {
        let worker_count = {
            self.config.read().unwrap().worker_count()
        };
        for worker_id in 0..worker_count {
            try!(self.spawn_worker(worker_id));
        }
        Ok(())
    }

    fn run(mut self) -> super::Result<()> {
        let worker_count = {
            self.config.read().unwrap().worker_count()
        };
        thread::spawn(move || {
            loop {
                for i in 0..worker_count {
                    match self.workers[i].try_recv() {
                        Err(mpsc::TryRecvError::Disconnected) => {
                            info!("Worker[{}] restarting...", i);
                            self.spawn_worker(i).unwrap();
                        }
                        Ok(msg) => warn!("Worker[{}] sent unexpected msg: {:?}", i, msg),
                        Err(mpsc::TryRecvError::Empty) => continue,
                    }
                }
                // JW TODO: switching to zmq from channels will allow us to call select across
                // multiple queues and avoid sleeping
                thread::sleep(Duration::from_millis(500));
            }
        });
        Ok(())
    }

    fn spawn_worker(&mut self, worker_id: usize) -> super::Result<()> {
        let cfg = self.config.clone();
        let (tx, rx) = mpsc::sync_channel(1);
        let mut worker = T::new(cfg);
        let init_state = self.init_state.clone();
        thread::spawn(move || {
            let state = try!(worker.init(init_state));
            worker.start(tx, state)
        });
        if rx.recv().is_ok() {
            debug!("Worker[{}] ready", worker_id);
            self.workers.insert(worker_id, rx);
        } else {
            error!("Worker[{}] failed to start", worker_id);
            self.workers.remove(worker_id);
        }
        Ok(())
    }
}
