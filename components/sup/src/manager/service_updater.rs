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

use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

use butterfly;
use common::ui::UI;
use depot_client;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::service::ServiceGroup;
use hcore::crypto::default_cache_key_path;
use hcore::fs::{CACHE_ARTIFACT_PATH, FS_ROOT_PATH};
use time::{SteadyTime, Duration as TimeDuration};

use {PRODUCT, VERSION};
use error::Result;
use manager::census::CensusList;
use manager::service::{Service, Topology, UpdateStrategy};

static LOGKEY: &'static str = "SU";
const UPDATE_STRATEGY_FREQUENCY_MS: i64 = 60_000;

type UpdaterStateList = HashMap<ServiceGroup, UpdaterState>;

enum UpdaterState {
    AtOnce(Receiver<PackageInstall>),
    Rolling(RollingState),
}

enum RollingState {
    AwaitingElection,
    InElection,
    Leader(LeaderState),
    Follower(FollowerState),
}

enum LeaderState {
    Polling(Receiver<PackageInstall>),
    Waiting,
}

enum FollowerState {
    Waiting,
    Updating(Receiver<PackageInstall>),
}

pub struct ServiceUpdater {
    states: UpdaterStateList,
    butterfly: butterfly::Server,
}

impl ServiceUpdater {
    pub fn new(butterfly: butterfly::Server) -> Self {
        ServiceUpdater {
            states: UpdaterStateList::default(),
            butterfly: butterfly,
        }
    }

    pub fn add(&mut self, service: &Service) -> bool {
        match service.update_strategy {
            UpdateStrategy::None => false,
            UpdateStrategy::AtOnce => {
                self.states
                    .entry(service.service_group.clone())
                    .or_insert_with(|| {
                                        let rx = Worker::new(service)
                                            .start(&service.service_group, None);
                                        UpdaterState::AtOnce(rx)
                                    });
                true
            }
            UpdateStrategy::Rolling => {
                self.states
                    .entry(service.service_group.clone())
                    .or_insert(UpdaterState::Rolling(RollingState::AwaitingElection));
                true
            }
        }
    }

    pub fn check_for_updated_package(&mut self,
                                     service: &mut Service,
                                     census_list: &CensusList)
                                     -> bool {
        let mut updated = false;
        match self.states.get_mut(&service.service_group) {
            Some(&mut UpdaterState::AtOnce(ref mut rx)) => {
                match rx.try_recv() {
                    Ok(package) => {
                        service.update_package(package);
                        return true;
                    }
                    Err(TryRecvError::Empty) => return false,
                    Err(TryRecvError::Disconnected) => {}
                }
                outputln!(preamble service.service_group,
                    "Service Updater worker has died {}", "; restarting...");
                *rx = Worker::new(service).start(&service.service_group, None);
            }
            Some(&mut UpdaterState::Rolling(ref mut st @ RollingState::AwaitingElection)) => {
                if let Some(census) = census_list.get(&*service.service_group) {
                    if service.topology == Topology::Leader {
                        debug!("Rolling Update, determining proper suitability because we're in \
                                a leader topology");
                        match (census.me(), census.get_leader()) {
                            (Some(me), Some(leader)) => {
                                let suitability = if me == leader {
                                    u64::min_value()
                                } else {
                                    u64::max_value()
                                };
                                self.butterfly
                                    .start_update_election(service.service_group.clone(),
                                                           suitability,
                                                           0);
                                *st = RollingState::InElection
                            }
                            _ => return false,
                        }
                    } else {
                        debug!("Rolling update, using default suitability");
                        self.butterfly
                            .start_update_election(service.service_group.clone(), 0, 0);
                        *st = RollingState::InElection;
                    }
                }
            }
            Some(&mut UpdaterState::Rolling(ref mut st @ RollingState::InElection)) => {
                if let Some(census) = census_list.get(&*service.service_group) {
                    match (census.me(), census.get_update_leader()) {
                        (Some(me), Some(leader)) => {
                            if me == leader {
                                debug!("We're the leader");
                                // Start in waiting state to ensure all members agree with our
                                // version before attempting a new rolling upgrade.
                                *st = RollingState::Leader(LeaderState::Waiting);
                            } else {
                                debug!("We're a follower");
                                *st = RollingState::Follower(FollowerState::Waiting);
                            }
                        }
                        (Some(_), None) => return false,
                        _ => return false,
                    }
                }
            }
            Some(&mut UpdaterState::Rolling(RollingState::Leader(ref mut state))) => {
                match *state {
                    LeaderState::Polling(ref mut rx) => {
                        match rx.try_recv() {
                            Ok(package) => {
                                debug!("Rolling Update, polling found a new package");
                                service.update_package(package);
                                updated = true;
                            }
                            Err(TryRecvError::Empty) => return false,
                            Err(TryRecvError::Disconnected) => {
                                outputln!(preamble service.service_group,
                                    "Service Updater has died {}", "; restarting...");
                                *rx = Worker::new(service).start(&service.service_group, None);
                            }
                        }
                    }
                    LeaderState::Waiting => {
                        match census_list.get(&*service.service_group) {
                            Some(census) => {
                                if census
                                       .members_ordered()
                                       .iter()
                                       .any(|ce| {
                                                ce.pkg.as_ref().unwrap() !=
                                                census.me().unwrap().pkg.as_ref().unwrap()
                                            }) {
                                    debug!("Update leader still waiting for followers...");
                                    return false;
                                }
                                let rx = Worker::new(service).start(&service.service_group, None);
                                *state = LeaderState::Polling(rx);
                            }
                            None => {
                                panic!("Expected census list to have service group '{}'!",
                                       &*service.service_group)
                            }
                        }
                    }
                }
                if updated {
                    *state = LeaderState::Waiting;
                }
            }
            Some(&mut UpdaterState::Rolling(RollingState::Follower(ref mut state))) => {
                match *state {
                    FollowerState::Waiting => {
                        match census_list.get(&*service.service_group) {
                            Some(census) => {
                                match (census.get_update_leader(),
                                       census.previous_peer(),
                                       census.me()) {
                                    (Some(leader), Some(peer), Some(me)) => {
                                        if leader.pkg == me.pkg {
                                            debug!("We're not in an update");
                                            return false;
                                        }
                                        if leader.pkg != peer.pkg {
                                            debug!("We're in an update but it's not our turn");
                                            return false;
                                        }
                                        debug!("We're in an update and it's our turn");
                                        let rx =
                                            Worker::new(service).start(&service.service_group,
                                                                       leader.pkg.clone());
                                        *state = FollowerState::Updating(rx);
                                    }
                                    _ => return false,
                                }
                            }
                            None => {
                                panic!("Expected census list to have service group '{}'!",
                                       &*service.service_group)
                            }
                        }
                    }
                    FollowerState::Updating(ref mut rx) => {
                        match census_list.get(&*service.service_group) {
                            Some(census) => {
                                match rx.try_recv() {
                                    Ok(package) => {
                                        service.update_package(package);
                                        updated = true
                                    }
                                    Err(TryRecvError::Empty) => return false,
                                    Err(TryRecvError::Disconnected) => {
                                        outputln!(preamble service.service_group,
                                            "Service Updater has died {}", "; restarting...");
                                        let package =
                                            census.get_update_leader().unwrap().pkg.clone();
                                        *rx = Worker::new(service).start(&service.service_group,
                                                                         package);
                                    }
                                }
                            }
                            None => {
                                panic!("Expected census list to have service group '{}'!",
                                       &*service.service_group)
                            }
                        }
                    }
                }
                if updated {
                    *state = FollowerState::Waiting;
                }
            }
            None => {}
        }
        updated
    }
}

struct Worker {
    current: PackageIdent,
    spec_ident: PackageIdent,
    depot: depot_client::Client,
    ui: UI,
}

impl Worker {
    fn new(service: &Service) -> Self {
        Worker {
            current: service.package().ident().clone(),
            spec_ident: service.spec_ident.clone(),
            depot: depot_client::Client::new(&service.depot_url, PRODUCT, VERSION, None).unwrap(),
            ui: UI::default(),
        }
    }

    /// Start a new update worker.
    ///
    /// Passing an optional package identifier will make the worker perform a run-once update to
    /// retrieve a specific version from a remote Depot. If no package identifier is specified,
    /// then the updater will poll until a newer more suitable package is found.
    fn start(mut self, sg: &ServiceGroup, ident: Option<PackageIdent>) -> Receiver<PackageInstall> {
        let (tx, rx) = sync_channel(0);
        thread::Builder::new()
            .name(format!("service-updater-{}", sg))
            .spawn(move || match ident {
                       Some(latest) => self.run_once(tx, latest),
                       None => self.run_poll(tx),
                   })
            .expect("unable to start service-updater thread");
        rx
    }

    fn run_once(&mut self, sender: SyncSender<PackageInstall>, ident: PackageIdent) {
        outputln!("Updating from {} to {}", self.current, ident);
        loop {
            let next_check = SteadyTime::now() +
                             TimeDuration::milliseconds(UPDATE_STRATEGY_FREQUENCY_MS);
            match self.install(&ident, true) {
                Ok(package) => {
                    self.current = package.ident().clone();
                    sender.send(package).expect("Main thread has gone away!");
                    break;
                }
                Err(e) => warn!("Failed to install updated package: {:?}", e),
            }
            let time_to_wait = (next_check - SteadyTime::now()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            }
        }
    }

    fn run_poll(&mut self, sender: SyncSender<PackageInstall>) {
        loop {
            let next_check = SteadyTime::now() +
                             TimeDuration::milliseconds(UPDATE_STRATEGY_FREQUENCY_MS);
            match self.depot.show_package(&self.spec_ident) {
                Ok(remote) => {
                    let latest: PackageIdent = remote.get_ident().clone().into();
                    if latest > self.current {
                        outputln!("Updating from {} to {}", self.current, latest);
                        match self.install(&latest, true) {
                            Ok(package) => {
                                self.current = latest;
                                sender.send(package).expect("Main thread has gone away!");
                                break;
                            }
                            Err(e) => warn!("Failed to install updated package: {:?}", e),
                        }
                    } else {
                        info!("Package found is not newer than ours");
                    }
                }
                Err(e) => warn!("Updater failed to get latest package: {:?}", e),
            }
            let time_to_wait = (next_check - SteadyTime::now()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            }
        }
    }

    fn install(&mut self, package: &PackageIdent, recurse: bool) -> Result<PackageInstall> {
        let package = match PackageInstall::load(package, Some(&*FS_ROOT_PATH)) {
            Ok(pkg) => pkg,
            Err(_) => try!(self.download(package)),
        };
        if recurse {
            for ident in package.tdeps()?.iter() {
                try!(self.install(&ident, false));
            }
        }
        Ok(package)
    }

    fn download(&mut self, package: &PackageIdent) -> Result<PackageInstall> {
        outputln!("Downloading {}", package);
        let mut archive = try!(self.depot
                                   .fetch_package(package,
                                                  &Path::new(&*FS_ROOT_PATH)
                                                       .join(CACHE_ARTIFACT_PATH),
                                                  self.ui.progress()));
        try!(archive.verify(&default_cache_key_path(None)));
        outputln!("Installing {}", package);
        try!(archive.unpack(None));
        let pkg = PackageInstall::load(archive.ident().as_ref().unwrap(), Some(&*FS_ROOT_PATH))?;
        Ok(pkg)
    }
}
