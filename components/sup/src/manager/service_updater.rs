// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
use std::path::Path;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

use common::ui::UI;
use depot_client;
use hcore::package::PackageIdent;
use hcore::crypto::default_cache_key_path;
use hcore::fs::{CACHE_ARTIFACT_PATH, FS_ROOT_PATH};
use time::{SteadyTime, Duration as TimeDuration};

use {PRODUCT, VERSION};
use config::{gconfig, UpdateStrategy};
use error::Result;
use manager::service::Service;
use package::Package;

static LOGKEY: &'static str = "SU";
const UPDATE_STRATEGY_FREQUENCY_MS: i64 = 60_000;

type WorkerList = HashMap<PackageIdent, Receiver<Package>>;

#[derive(Default)]
pub struct ServiceUpdater {
    workers: WorkerList,
}

impl ServiceUpdater {
    pub fn add(&mut self, service: &Service) -> bool {
        match self.workers.get(service.package.ident()) {
            None => self.start_worker(service),
            Some(_) => false,
        }
    }

    pub fn check_for_updated_package(&mut self, service: &mut Service) {
        if let Some(worker) = self.workers.get_mut(&service.package.ident()) {
            match worker.try_recv() {
                Ok(package) => {
                    service.package = package;
                    service.needs_restart = true;
                    return;
                }
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => {}
            }
        } else {
            return;
        }
        outputln!(preamble service.service_group_str(),
            "Service Updater worker has died {}", "; restarting...");
        self.start_worker(service);
    }

    fn start_worker(&mut self, service: &Service) -> bool {
        if service.update_strategy == UpdateStrategy::None {
            return false;
        }
        let rx = Worker::new(service).start();
        self.workers.insert(service.package.ident().clone(), rx);
        true
    }
}

struct Worker {
    current: PackageIdent,
    depot: depot_client::Client,
    ui: UI,
}

impl Worker {
    pub fn new(service: &Service) -> Self {
        Worker {
            current: service.package.ident().clone(),
            depot: depot_client::Client::new(gconfig().url(), PRODUCT, VERSION, None).unwrap(),
            ui: UI::default(),
        }
    }

    pub fn start(mut self) -> Receiver<Package> {
        let (tx, rx) = sync_channel(0);
        thread::Builder::new()
            .name(format!("service-updater-{}-{}",
                          &self.current.origin,
                          &self.current.name))
            .spawn(move || self.run(tx))
            .unwrap();
        rx
    }

    fn install(&mut self, package: &PackageIdent) -> Result<Package> {
        let mut archive = try!(self.depot.fetch_package(package,
                                                        &Path::new(FS_ROOT_PATH)
                                                            .join(CACHE_ARTIFACT_PATH),
                                                        self.ui.progress()));
        debug!("Updater downloaded new package to {:?}", archive);
        try!(archive.verify(&default_cache_key_path(None)));
        try!(archive.unpack(None));
        Package::load(archive.ident().as_ref().unwrap(), None)
    }

    fn run(&mut self, sender: SyncSender<Package>) {
        loop {
            let next_check = SteadyTime::now() +
                             TimeDuration::milliseconds(UPDATE_STRATEGY_FREQUENCY_MS);
            match self.depot.show_package(&self.current) {
                Ok(remote) => {
                    let latest: PackageIdent = remote.get_ident().clone().into();
                    if latest > self.current {
                        match self.install(&latest) {
                            Ok(package) => {
                                self.current = latest;
                                sender.send(package).expect("Main thread has gone away!");
                            }
                            Err(e) => warn!("Failed to install updated package: {:?}", e),
                        }
                    } else {
                        debug!("Package found is not newer than ours");
                    }
                }
                Err(e) => warn!("Updater failed to get latest package: {:?}", e),
            }
            let time_to_wait = next_check - SteadyTime::now();
            thread::sleep(Duration::from_millis(time_to_wait.num_milliseconds() as u64));
        }
    }
}
