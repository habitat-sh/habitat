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
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use butterfly;
use common::ui::UI;
use env;
use hcore::package::{PackageIdent, PackageInstall, PackageTarget};
use hcore::service::ServiceGroup;
use launcher_client::LauncherCli;

use census::CensusRing;
use manager::periodic::Periodic;
use manager::service::{Service, Topology, UpdateStrategy};
use time::SteadyTime;
use util;

static LOGKEY: &'static str = "SU";
const FREQUENCY_ENVVAR: &'static str = "HAB_UPDATE_STRATEGY_FREQUENCY_MS";
const FREQUENCY_BYPASS_CHECK_ENVVAR: &'static str = "HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK";
const MIN_ALLOWED_FREQUENCY: i64 = 60_000;
const DEFAULT_FREQUENCY: i64 = MIN_ALLOWED_FREQUENCY;

type UpdaterStateList = HashMap<ServiceGroup, UpdaterState>;

enum UpdaterState {
    AtOnce(Receiver<PackageInstall>, Sender<()>),
    Rolling(RollingState),
}

enum RollingState {
    AwaitingElection,
    InElection,
    Leader(LeaderState),
    Follower(FollowerState),
}

enum LeaderState {
    Polling(Receiver<PackageInstall>, Sender<()>),
    Waiting,
}

/// Current package update state of a follower in a leader-follower
/// topology
enum FollowerState {
    /// Waiting to be told to update
    Waiting,
    /// Currently updating
    Updating(Receiver<PackageInstall>, Sender<()>),
}

/// The ServiceUpdater is in charge of updating a Service when a more recent version of a package
/// has been published to a depot or installed to the local package cache.
/// To use an update strategy, the supervisor must be configured to watch a depot for new versions.
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

    /// Register a new `Service` for updates. Returns `true` if the
    /// `ServiceUpdater` was modified (i.e., the given service has an
    /// `UpdateStrategy` that is not `None`).
    pub fn add(&mut self, service: &Service) -> bool {
        match service.update_strategy {
            UpdateStrategy::None => false,
            UpdateStrategy::AtOnce => {
                self.states
                    .entry(service.service_group.clone())
                    .or_insert_with(|| {
                        let (kill_tx, kill_rx) = channel();
                        let rx = Worker::new(service).start(&service.service_group, None, kill_rx);
                        UpdaterState::AtOnce(rx, kill_tx)
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

    /// Remove a `Service` from updates, e.g. if the service was unloaded.
    pub fn remove(&mut self, service: &Service) {
        match self.states.remove(&service.service_group) {
            Some(UpdaterState::AtOnce(_rx, kill_tx)) => {
                if kill_tx.send(()).is_err() {
                    debug!("Tried to kill the updater thread but it's already dead.");
                }
            }
            Some(UpdaterState::Rolling(rs)) => match rs {
                RollingState::Leader(ls) => match ls {
                    LeaderState::Polling(_rx, kill_tx) => {
                        if kill_tx.send(()).is_err() {
                            debug!("Tried to kill the updater thread but it's already dead.");
                        }
                    }
                    LeaderState::Waiting => {}
                },
                RollingState::Follower(fs) => match fs {
                    FollowerState::Updating(_rx, kill_tx) => {
                        if kill_tx.send(()).is_err() {
                            debug!("Tried to kill the updater thread but it's already dead.");
                        }
                    }
                    FollowerState::Waiting => {}
                },
                _ => {}
            },
            None => {
                warn!(
                    "Tried to remove {} from the ServiceUpdater, but it wasn't found.",
                    service
                );
            }
        }
    }

    /// See if the given service has an update. Returns `true` if a
    /// new version was installed, thus signalling that the service
    /// should be restarted
    pub fn check_for_updated_package(
        &mut self,
        service: &mut Service,
        census_ring: &CensusRing,
        launcher: &LauncherCli,
    ) -> bool {
        let mut updated = false;
        match self.states.get_mut(&service.service_group) {
            Some(&mut UpdaterState::AtOnce(ref mut rx, ref mut kill_tx)) => match rx.try_recv() {
                Ok(package) => {
                    service.update_package(package, launcher);
                    return true;
                }
                Err(TryRecvError::Empty) => return false,
                Err(TryRecvError::Disconnected) => {
                    debug!("Service Updater worker has died; restarting...");
                    let (ktx, krx) = channel();
                    *rx = Worker::new(service).start(&service.service_group, None, krx);
                    *kill_tx = ktx;
                }
            },

            Some(&mut UpdaterState::Rolling(ref mut st @ RollingState::AwaitingElection)) => {
                if let Some(census_group) = census_ring.census_group_for(&service.service_group) {
                    if service.topology == Topology::Leader {
                        debug!(
                            "Rolling Update, determining proper suitability because we're in \
                             a leader topology"
                        );
                        match (census_group.me(), census_group.leader()) {
                            (Some(me), Some(leader)) => {
                                let suitability = if me.member_id == leader.member_id {
                                    u64::min_value()
                                } else {
                                    u64::max_value()
                                };
                                self.butterfly.start_update_election(
                                    &service.service_group,
                                    suitability,
                                    0,
                                );
                                *st = RollingState::InElection
                            }
                            _ => return false,
                        }
                    } else {
                        debug!("Rolling update, using default suitability");
                        self.butterfly
                            .start_update_election(&service.service_group, 0, 0);
                        *st = RollingState::InElection;
                    }
                }
            }
            Some(&mut UpdaterState::Rolling(ref mut st @ RollingState::InElection)) => {
                if let Some(census_group) = census_ring.census_group_for(&service.service_group) {
                    match (census_group.me(), census_group.update_leader()) {
                        (Some(me), Some(leader)) => {
                            if me.member_id == leader.member_id {
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
                    LeaderState::Polling(ref mut rx, ref mut kill_tx) => match rx.try_recv() {
                        Ok(package) => {
                            debug!("Rolling Update, polling found a new package");
                            service.update_package(package, launcher);
                            updated = true;
                        }
                        Err(TryRecvError::Empty) => return false,
                        Err(TryRecvError::Disconnected) => {
                            debug!("Service Updater worker has died; restarting...");
                            let (ktx, krx) = channel();
                            *rx = Worker::new(service).start(&service.service_group, None, krx);
                            *kill_tx = ktx;
                        }
                    },
                    LeaderState::Waiting => match census_ring
                        .census_group_for(&service.service_group)
                    {
                        Some(census_group) => {
                            if census_group.members().iter().any(|cm| {
                                cm.pkg.as_ref().unwrap()
                                    != census_group.me().unwrap().pkg.as_ref().unwrap()
                            }) {
                                debug!("Update leader still waiting for followers...");
                                return false;
                            }
                            let (kill_tx, kill_rx) = channel();
                            let rx =
                                Worker::new(service).start(&service.service_group, None, kill_rx);
                            *state = LeaderState::Polling(rx, kill_tx);
                        }
                        None => panic!(
                            "Expected census list to have service group '{}'!",
                            &*service.service_group
                        ),
                    },
                }
                if updated {
                    *state = LeaderState::Waiting;
                }
            }
            Some(&mut UpdaterState::Rolling(RollingState::Follower(ref mut state))) => {
                match *state {
                    FollowerState::Waiting => {
                        match census_ring.census_group_for(&service.service_group) {
                            Some(census_group) => match (
                                census_group.update_leader(),
                                census_group.previous_peer(),
                                census_group.me(),
                            ) {
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
                                    let (kill_tx, kill_rx) = channel();
                                    let rx = Worker::new(service).start(
                                        &service.service_group,
                                        leader.pkg.clone(),
                                        kill_rx,
                                    );
                                    *state = FollowerState::Updating(rx, kill_tx);
                                }
                                _ => return false,
                            },
                            None => panic!(
                                "Expected census list to have service group '{}'!",
                                &*service.service_group
                            ),
                        }
                    }
                    FollowerState::Updating(ref mut rx, ref mut kill_tx) => {
                        match census_ring.census_group_for(&service.service_group) {
                            Some(census_group) => match rx.try_recv() {
                                Ok(package) => {
                                    service.update_package(package, launcher);
                                    updated = true
                                }
                                Err(TryRecvError::Empty) => return false,
                                Err(TryRecvError::Disconnected) => {
                                    debug!("Service Updater worker has died; restarting...");
                                    let package = census_group.update_leader().unwrap().pkg.clone();
                                    let (ktx, krx) = channel();
                                    *rx = Worker::new(service).start(
                                        &service.service_group,
                                        package,
                                        krx,
                                    );
                                    *kill_tx = ktx;
                                }
                            },
                            None => panic!(
                                "Expected census list to have service group '{}'!",
                                &*service.service_group
                            ),
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
    builder_url: String,
    channel: String,
}

impl Periodic for Worker {
    // TODO (CM): Consider performing this check once and storing it,
    // instead of re-checking every time.
    fn update_period(&self) -> i64 {
        match env::var(FREQUENCY_ENVVAR) {
            Ok(val) => match val.parse::<i64>() {
                Ok(num) => {
                    if (env::var(FREQUENCY_BYPASS_CHECK_ENVVAR).is_ok() && num > 0)
                        || num >= MIN_ALLOWED_FREQUENCY
                    {
                        num
                    } else if num <= 0 {
                        outputln!(
                            "{} has been set, but {} value ({}) is negative ({}) \
                             Falling back to minimal {} MS frequency.",
                            FREQUENCY_BYPASS_CHECK_ENVVAR,
                            FREQUENCY_ENVVAR,
                            num,
                            MIN_ALLOWED_FREQUENCY,
                            MIN_ALLOWED_FREQUENCY
                        );
                        MIN_ALLOWED_FREQUENCY
                    } else {
                        outputln!(
                            "{} value ({}) is below the minimal authorized value ({}) \
                             Falling back to minimal {} MS frequency.",
                            FREQUENCY_ENVVAR,
                            num,
                            MIN_ALLOWED_FREQUENCY,
                            MIN_ALLOWED_FREQUENCY
                        );
                        MIN_ALLOWED_FREQUENCY
                    }
                }
                Err(_) => {
                    outputln!(
                        "Unable to parse '{}' from {} as a valid integer. Falling back \
                         to default {} MS frequency.",
                        val,
                        FREQUENCY_ENVVAR,
                        DEFAULT_FREQUENCY
                    );
                    DEFAULT_FREQUENCY
                }
            },
            Err(_) => DEFAULT_FREQUENCY,
        }
    }
}

impl Worker {
    fn new(service: &Service) -> Self {
        Worker {
            current: service.pkg.ident.clone(),
            spec_ident: service.spec_ident.clone(),
            builder_url: service.bldr_url.clone(),
            channel: service.channel.clone(),
        }
    }

    /// Start a new update worker.
    ///
    /// Passing an optional package identifier will make the worker perform a run-once update to
    /// retrieve a specific version from Builder. If no package identifier is specified,
    /// then the updater will poll until a newer more suitable package is found.
    fn start(
        mut self,
        sg: &ServiceGroup,
        ident: Option<PackageIdent>,
        kill_rx: Receiver<()>,
    ) -> Receiver<PackageInstall> {
        let (tx, rx) = channel();
        thread::Builder::new()
            .name(format!("service-updater-{}", sg))
            .spawn(move || match ident {
                Some(latest) => self.run_once(tx, latest, kill_rx),
                None => self.run_poll(tx, kill_rx),
            }).expect("unable to start service-updater thread");
        rx
    }

    // TODO (CM): A refactor I'd like to do is to tease out the
    // run_once and run_poll cases into two separate "start" functions
    // that describe more what's going on. Passing `None` as the
    // identifier just means to keep going until you get a new
    // one. Passing an identifier (which should probably be a
    // fully-qualified one, right?) just goes until that package gets
    // downloaded.
    //
    // In all cases except for FollowerState::Updating and
    // FollowerState::Waiting, we pass None, so that's easy. In those
    // two states, though, the package can legitimately be an Option
    // (we get it from CensusMember.pkg), but it seems like it can
    // only be None if there was an unparseable identifier in the
    // ServiceRumor the CensusMember was generated from. I suspect
    // that we might be able to refactor the types (or how we handle
    // them) a bit better.
    //
    // I'm also not 100% clear why we have run_poll and run_once,
    // since their implementations are very similar. There may be an
    // opportunity to collapse those.
    /// Polls until a newer version of the specified package is
    /// available. When such a package is found, it is installed, and
    /// the function exits.
    fn run_once(
        &mut self,
        sender: Sender<PackageInstall>,
        ident: PackageIdent,
        kill_rx: Receiver<()>,
    ) {
        // Fairly certain that this only gets called in a rolling update
        // scenario, where `ident` is always a fully-qualified identifier
        outputln!("Updating from {} to {}", self.current, ident);
        let install_source = (ident, *PackageTarget::active_target()).into();
        let mut next_time = SteadyTime::now();

        loop {
            match kill_rx.try_recv() {
                Ok(_) => {
                    info!("Received some data on the kill channel. Letting this thread die.");
                    break;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    info!("Service updater has gone away, yikes!");
                    break;
                }
            }

            if SteadyTime::now() >= next_time {
                match util::pkg::install(
                    // We don't want anything in here to print
                    &mut UI::with_sinks(),
                    &self.builder_url,
                    &install_source,
                    &self.channel,
                ) {
                    Ok(package) => {
                        self.current = package.ident().clone();
                        sender.send(package).expect("Main thread has gone away!");
                        break;
                    }
                    Err(e) => warn!("Failed to install updated package: {:?}", e),
                }

                next_time = self.next_period_start();
            }
        }
    }

    /// Continually poll for a new version of a package, installing it
    /// when found.
    fn run_poll(&mut self, sender: Sender<PackageInstall>, kill_rx: Receiver<()>) {
        let install_source = (self.spec_ident.clone(), *PackageTarget::active_target()).into();
        let mut next_time = SteadyTime::now();

        loop {
            match kill_rx.try_recv() {
                Ok(_) => {
                    info!("Received some data on the kill channel. Letting this thread die.");
                    break;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    info!("Service updater has gone away, yikes!");
                    break;
                }
            }

            if SteadyTime::now() >= next_time {
                match util::pkg::install(
                    // We don't want anything in here to print
                    &mut UI::with_sinks(),
                    &self.builder_url,
                    &install_source,
                    &self.channel,
                ) {
                    Ok(maybe_newer_package) => {
                        if self.current < *maybe_newer_package.ident() {
                            outputln!(
                                "Updating from {} to {}",
                                self.current,
                                maybe_newer_package.ident()
                            );
                            self.current = maybe_newer_package.ident().clone();
                            sender
                                .send(maybe_newer_package)
                                .expect("Main thread has gone away!");
                            break;
                        } else {
                            debug!("Package found is not newer than ours");
                        }
                    }
                    Err(e) => warn!("Updater failed to get latest package: {:?}", e),
                }

                next_time = self.next_period_start();
            }
        }
    }
}
