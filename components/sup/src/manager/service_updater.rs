use crate::{census::CensusRing,
            manager::{periodic::Periodic,
                      service::{Service,
                                Topology,
                                UpdateStrategy}},
            util};
use habitat_butterfly;
use habitat_common::{liveliness_checker,
                     outputln,
                     ui::UI};
use habitat_core::{env as henv,
                   package::{PackageIdent,
                             PackageInstall,
                             PackageTarget},
                   service::ServiceGroup,
                   ChannelIdent};
use std::{self,
          collections::HashMap,
          sync::mpsc::{channel,
                       Receiver,
                       Sender,
                       TryRecvError},
          thread,
          time::{Duration,
                 Instant}};

static LOGKEY: &str = "SU";
// TODO (CM): Yes, the variable value should be "period" and not
// "frequency"... we need to fix that.
const PERIOD_BYPASS_CHECK_ENVVAR: &str = "HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK";

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

    butterfly: habitat_butterfly::Server,
}

impl ServiceUpdater {
    pub fn new(butterfly: habitat_butterfly::Server) -> Self {
        ServiceUpdater { states: UpdaterStateList::default(),
                         butterfly }
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
            Some(UpdaterState::Rolling(rs)) => {
                match rs {
                    RollingState::Leader(ls) => {
                        match ls {
                            LeaderState::Polling(_rx, kill_tx) => {
                                if kill_tx.send(()).is_err() {
                                    debug!("Tried to kill the updater thread but it's already \
                                            dead.");
                                }
                            }
                            LeaderState::Waiting => {}
                        }
                    }
                    RollingState::Follower(fs) => {
                        match fs {
                            FollowerState::Updating(_rx, kill_tx) => {
                                if kill_tx.send(()).is_err() {
                                    debug!("Tried to kill the updater thread but it's already \
                                            dead.");
                                }
                            }
                            FollowerState::Waiting => {}
                        }
                    }
                    _ => {}
                }
            }
            None => {
                warn!("Tried to remove {} from the ServiceUpdater, but it wasn't found.",
                      service);
            }
        }
    }

    /// See if the given service has an update. Returns the identifier
    /// of the newly-updated service if a new version was installed,
    /// thus signalling that the service should be restarted.
    // If we ever need to modify this function, it would be an excellent opportunity to
    // simplify the redundant aspects and remove this allow(clippy::cognitive_complexity),
    // but changing it in the absence of other necessity seems like too much risk for the
    // expected reward.
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    #[allow(clippy::cognitive_complexity)]
    pub fn check_for_updated_package_rsw_mlr_rhw(&mut self,
                                                 service: &Service,
                                                 // TODO (CM): Strictly speaking, we don't need
                                                 // to pass CensusRing down into here, just the
                                                 // census group for our service.
                                                 census_ring: &CensusRing)
                                                 -> Option<PackageIdent> {
        debug!("Checking for updated package!");

        // TODO (CM): can we do without this?
        let mut ident = None;

        match self.states.get_mut(&service.service_group) {
            Some(&mut UpdaterState::AtOnce(ref mut rx, ref mut kill_tx)) => {
                match rx.try_recv() {
                    Ok(package) => {
                        return Some(package.ident);
                    }
                    Err(TryRecvError::Empty) => return None,
                    Err(TryRecvError::Disconnected) => {
                        debug!("Service Updater worker has died; restarting...");
                        let (ktx, krx) = channel();
                        *rx = Worker::new(service).start(&service.service_group, None, krx);
                        *kill_tx = ktx;
                    }
                }
            }

            Some(&mut UpdaterState::Rolling(ref mut st @ RollingState::AwaitingElection)) => {
                if let Some(census_group) = census_ring.census_group_for(&service.service_group) {
                    if service.topology == Topology::Leader {
                        debug!("Rolling Update, determining proper suitability because we're in \
                                a leader topology");
                        match (census_group.me(), census_group.leader()) {
                            (Some(me), Some(leader)) => {
                                let suitability = if me.member_id == leader.member_id {
                                    u64::min_value()
                                } else {
                                    u64::max_value()
                                };
                                self.butterfly
                                    .start_update_election_rsw_mlr_rhw(&service.service_group,
                                                                       suitability,
                                                                       0);
                                *st = RollingState::InElection
                            }
                            _ => return None,
                        }
                    } else {
                        debug!("Rolling update, using default suitability");
                        self.butterfly
                            .start_update_election_rsw_mlr_rhw(&service.service_group, 0, 0);
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
                        (Some(_), None) => return None,
                        _ => return None,
                    }
                }
            }
            Some(&mut UpdaterState::Rolling(RollingState::Leader(ref mut state))) => {
                match *state {
                    LeaderState::Polling(ref mut rx, ref mut kill_tx) => {
                        match rx.try_recv() {
                            Ok(package) => {
                                debug!("Rolling Update, polling found a new package");
                                ident = Some(package.ident);
                            }
                            Err(TryRecvError::Empty) => return None,
                            Err(TryRecvError::Disconnected) => {
                                debug!("Service Updater worker has died; restarting...");
                                let (ktx, krx) = channel();
                                *rx = Worker::new(service).start(&service.service_group, None, krx);
                                *kill_tx = ktx;
                            }
                        }
                    }
                    LeaderState::Waiting => {
                        match census_ring.census_group_for(&service.service_group) {
                            Some(census_group) => {
                                if census_group.members()
                                               .any(|cm| cm.pkg != census_group.me().unwrap().pkg)
                                {
                                    debug!("Update leader still waiting for followers...");
                                    return None;
                                }
                                let (kill_tx, kill_rx) = channel();
                                let rx = Worker::new(service).start(&service.service_group,
                                                                    None,
                                                                    kill_rx);
                                *state = LeaderState::Polling(rx, kill_tx);
                            }
                            None => {
                                panic!("Expected census list to have service group '{}'!",
                                       &*service.service_group)
                            }
                        }
                    }
                }
                if ident.is_some() {
                    *state = LeaderState::Waiting;
                }
            }
            Some(&mut UpdaterState::Rolling(RollingState::Follower(ref mut state))) => {
                match *state {
                    FollowerState::Waiting => {
                        match census_ring.census_group_for(&service.service_group) {
                            Some(census_group) => {
                                match (census_group.update_leader(),
                                       census_group.previous_peer(),
                                       census_group.me())
                                {
                                    (Some(leader), Some(peer), Some(me)) => {
                                        if leader.pkg == me.pkg {
                                            debug!("We're not in an update");
                                            return None;
                                        }
                                        if leader.pkg != peer.pkg {
                                            debug!("We're in an update but it's not our turn");
                                            return None;
                                        }
                                        debug!("We're in an update and it's our turn");
                                        let (kill_tx, kill_rx) = channel();
                                        let rx = Worker::new(service).start(&service.service_group,
                                                                            leader.pkg.clone(),
                                                                            kill_rx);
                                        *state = FollowerState::Updating(rx, kill_tx);
                                    }
                                    _ => return None,
                                }
                            }
                            None => {
                                panic!("Expected census list to have service group '{}'!",
                                       &*service.service_group)
                            }
                        }
                    }
                    FollowerState::Updating(ref mut rx, ref mut kill_tx) => {
                        match census_ring.census_group_for(&service.service_group) {
                            Some(census_group) => {
                                match rx.try_recv() {
                                    Ok(package) => {
                                        ident = Some(package.ident);
                                    }
                                    Err(TryRecvError::Empty) => return None,
                                    Err(TryRecvError::Disconnected) => {
                                        debug!("Service Updater worker has died; restarting...");
                                        let package =
                                            census_group.update_leader().unwrap().pkg.clone();
                                        let (ktx, krx) = channel();
                                        *rx = Worker::new(service).start(&service.service_group,
                                                                         package,
                                                                         krx);
                                        *kill_tx = ktx;
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
                if ident.is_some() {
                    *state = FollowerState::Waiting;
                }
            }
            None => {}
        }
        ident
    }
}

habitat_core::env_config_duration!(
    /// Represents how far apart checks for updates to individual services
    /// are, in milliseconds.
    ServiceUpdatePeriod,
    // TODO (CM): Yes, the variable value should be "period" and not
    // "frequency"... we need to fix that.
    HAB_UPDATE_STRATEGY_FREQUENCY_MS => from_millis,
    ServiceUpdatePeriod::MIN_ALLOWED);

impl ServiceUpdatePeriod {
    const MIN_ALLOWED: Duration = Duration::from_secs(60);
}

struct Worker {
    current:     PackageIdent,
    spec_ident:  PackageIdent,
    builder_url: String,
    channel:     ChannelIdent,
}

impl Periodic for Worker {
    // TODO (CM): Consider performing this check once and storing it,
    // instead of re-checking every time.
    fn update_period(&self) -> Duration {
        let val = ServiceUpdatePeriod::configured_value().into();
        if val >= ServiceUpdatePeriod::MIN_ALLOWED || henv::var(PERIOD_BYPASS_CHECK_ENVVAR).is_ok()
        {
            val
        } else {
            ServiceUpdatePeriod::MIN_ALLOWED
        }
    }
}

impl Worker {
    fn new(service: &Service) -> Self {
        Worker { current:     service.pkg.ident.clone(),
                 spec_ident:  service.spec_ident.clone(),
                 builder_url: service.bldr_url.clone(),
                 channel:     service.channel.clone(), }
    }

    /// Start a new update worker.
    ///
    /// Passing an optional package identifier will make the worker perform a run-once update to
    /// retrieve a specific version from Builder. If no package identifier is specified,
    /// then the updater will poll until a newer more suitable package is found.
    fn start(mut self,
             sg: &ServiceGroup,
             ident: Option<PackageIdent>,
             kill_rx: Receiver<()>)
             -> Receiver<PackageInstall> {
        let (tx, rx) = channel();
        thread::Builder::new().name(format!("SU-{}", sg))
                              .spawn(move || {
                                  let _ = match ident {
                                      Some(latest) => self.run_once(&tx, latest, &kill_rx),
                                      None => self.run_poll(&tx, &kill_rx),
                                  };
                              })
                              .expect("unable to start service-updater thread");
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
    fn run_once(&mut self,
                sender: &Sender<PackageInstall>,
                ident: PackageIdent,
                kill_rx: &Receiver<()>)
                -> liveliness_checker::ThreadUnregistered<(), &str> {
        // Fairly certain that this only gets called in a rolling update
        // scenario, where `ident` is always a fully-qualified identifier
        outputln!("Updating from {} to {}", self.current, ident);
        let install_source = (ident, PackageTarget::active_target()).into();
        let mut next_time = Instant::now();

        loop {
            let checked_thread = liveliness_checker::mark_thread_alive();

            match kill_rx.try_recv() {
                Ok(_) => {
                    info!("Received some data on the kill channel. Letting this thread die.");
                    break checked_thread.unregister(Ok(()));
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    let msg = "Service updater has gone away, yikes!";
                    error!("{}", msg);
                    break checked_thread.unregister(Err(msg));
                }
            }

            if Instant::now() >= next_time {
                match util::pkg::install(// We don't want anything in here to print
                                         &mut UI::with_sinks(),
                                         &self.builder_url,
                                         &install_source,
                                         &self.channel)
                {
                    Ok(package) => {
                        self.current = package.ident().clone();
                        sender.send(package).expect("Main thread has gone away!");
                        break checked_thread.unregister(Ok(()));
                    }
                    Err(e) => warn!("Failed to install updated package: {:?}", e),
                }

                next_time = self.next_period_start();
            }

            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Continually poll for a new version of a package, installing it
    /// when found.
    fn run_poll(&mut self,
                sender: &Sender<PackageInstall>,
                kill_rx: &Receiver<()>)
                -> liveliness_checker::ThreadUnregistered<(), &str> {
        let install_source = (self.spec_ident.clone(), PackageTarget::active_target()).into();
        let mut next_time = Instant::now();

        loop {
            let checked_thread = liveliness_checker::mark_thread_alive();

            match kill_rx.try_recv() {
                Ok(_) => {
                    info!("Received some data on the kill channel. Letting this thread die.");
                    break checked_thread.unregister(Ok(()));
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    let msg = "Service updater has gone away, yikes!";
                    error!("{}", msg);
                    break checked_thread.unregister(Err(msg));
                }
            }

            if Instant::now() >= next_time {
                match util::pkg::install(// We don't want anything in here to print
                                         &mut UI::with_sinks(),
                                         &self.builder_url,
                                         &install_source,
                                         &self.channel)
                {
                    Ok(maybe_newer_package) => {
                        if self.current < *maybe_newer_package.ident() {
                            outputln!("Updating from {} to {}",
                                      self.current,
                                      maybe_newer_package.ident());
                            self.current = maybe_newer_package.ident().clone();
                            sender.send(maybe_newer_package)
                                  .expect("Main thread has gone away!");
                            break checked_thread.unregister(Ok(()));
                        } else {
                            debug!("Package found {} is not newer than ours",
                                   maybe_newer_package.ident());
                        }
                    }
                    Err(e) => warn!("Updater failed to get latest package: {:?}", e),
                }

                next_time = self.next_period_start();
            }

            thread::sleep(Duration::from_secs(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use habitat_core::locked_env_var;

    #[test]
    fn default_update_period_is_equal_to_minimum_allowed_value() {
        assert_eq!(ServiceUpdatePeriod::default().0,
                   ServiceUpdatePeriod::MIN_ALLOWED);
    }

    locked_env_var!(HAB_UPDATE_STRATEGY_FREQUENCY_MS, lock_period_var);
    locked_env_var!(HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK, lock_bypass_var);

    fn worker() -> Worker {
        Worker { current:     "core/testing/1.0.0/20181109125930".parse()
                                                                 .expect("Can't parse ident!"),
                 spec_ident:  "core/testing".parse().expect("Can't parse ident!"),
                 builder_url: String::from("https://bldr.habitat.sh"),
                 channel:     ChannelIdent::stable(), }
    }

    #[test]
    fn service_update_period_must_be_positive() {
        use std::str::FromStr as _;
        assert!(ServiceUpdatePeriod::from_str("-123").is_err());
        assert!(ServiceUpdatePeriod::from_str("0").is_ok());
        assert!(ServiceUpdatePeriod::from_str("5").is_ok());
    }

    #[test]
    fn worker_period_must_be_bypassed_by_non_empty_value() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();
        let worker = worker();

        period.set("123");
        bypass.set(""); // empty string isn't allowed

        assert_ne!(worker.update_period(), Duration::from_millis(123));
        assert_eq!(ServiceUpdatePeriod::default().0, worker.update_period());
    }

    #[test]
    fn worker_period_defaults_properly() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();
        let worker = worker();

        period.unset();
        bypass.unset();

        assert_eq!(ServiceUpdatePeriod::default().0, worker.update_period());
    }

    #[test]
    fn worker_period_can_be_overridden_by_env_var() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();
        let worker = worker();

        period.set("120000");
        bypass.unset();
        let expected_period: ServiceUpdatePeriod =
            ServiceUpdatePeriod(Duration::from_millis(120_000));
        assert!(expected_period.0 >= ServiceUpdatePeriod::MIN_ALLOWED);
        assert_eq!(expected_period.0, worker.update_period());
    }

    #[test]
    fn worker_period_cannot_be_overridden_to_a_very_small_value_by_default() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();
        let worker = worker();

        period.set("1"); // This is TOO low
        bypass.unset();
        assert!(Duration::from_millis(1) < ServiceUpdatePeriod::MIN_ALLOWED);
        assert_eq!(ServiceUpdatePeriod::default().0, worker.update_period());
    }

    #[test]
    fn worker_period_cannot_be_overridden_by_a_non_number() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();
        let worker = worker();

        period.set("this is not a number");
        bypass.unset();
        assert_eq!(ServiceUpdatePeriod::default().0, worker.update_period());
    }

    #[test]
    fn worker_period_can_be_overridden_by_a_small_value_with_bypass_var() {
        let period = lock_period_var();
        let bypass = lock_bypass_var();
        let worker = worker();

        period.set("5000");
        bypass.set("1");
        let expected_period: ServiceUpdatePeriod = ServiceUpdatePeriod(Duration::from_millis(5000));
        assert!(expected_period.0 < ServiceUpdatePeriod::MIN_ALLOWED);
        assert_eq!(expected_period.0, worker.update_period());
    }
}
