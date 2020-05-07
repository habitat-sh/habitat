//! The Butterfly server.
//!
//! Creates `Server` structs, that hold everything we need to run the SWIM and Gossip protocol.
//! Winds up with 5 separate threads - inbound (incoming connections), outbound (the Probe
//! protocol), expire (turning Suspect members into Confirmed members), push (the fan-out rumors),
//! and pull (the inbound receipt of rumors.).

mod expire;
mod inbound;
mod incarnation_store;
mod outbound;
mod pull;
mod push;
pub mod timing;

use self::{incarnation_store::IncarnationStore,
           sync::Myself};
use crate::{error::{Error,
                    Result},
            member::{Health,
                     Incarnation,
                     Member,
                     MemberList,
                     MemberListProxy},
            message,
            rumor::{dat_file::{DatFileReader,
                               DatFileWriter},
                    departure::Departure,
                    election::{Election,
                               ElectionRumor,
                               ElectionUpdate},
                    heat::sync::RumorHeat,
                    service::Service,
                    service_config::ServiceConfig,
                    service_file::ServiceFile,
                    service_health::ServiceHealth,
                    ConstIdRumor,
                    Rumor,
                    RumorKey,
                    RumorStore,
                    RumorStoreProxy,
                    RumorType},
            swim::Ack};
use habitat_common::{liveliness_checker,
                     sync::Lock,
                     FeatureFlag};
use habitat_core::crypto::SymKey;
use prometheus::{HistogramTimer,
                 HistogramVec,
                 IntGauge};
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};
use std::{collections::{HashMap,
                        HashSet},
          fmt::{self,
                Debug},
          fs,
          io,
          net::{SocketAddr,
                ToSocketAddrs,
                UdpSocket},
          path::{Path,
                 PathBuf},
          result,
          sync::{atomic::{AtomicBool,
                          AtomicIsize,
                          Ordering},
                 mpsc::{self,
                        channel},
                 Arc,
                 Mutex},
          thread,
          time::{Duration,
                 Instant}};

/// The maximum number of other members we should notify when we shut
/// down and leave the ring.
const SELF_DEPARTURE_RUMOR_FANOUT: usize = 10;

lazy_static! {
    static ref INCARNATION: IntGauge =
        register_int_gauge!(opts!("hab_butterfly_incarnation_number",
                                  "Incarnation number of the supervisor")).unwrap();
    static ref ELECTION_DURATION: HistogramVec =
        register_histogram_vec!("hab_butterfly_election_duration_seconds",
                                "How long it takes to complete an election",
                                &["service_group"]).unwrap();
}

// We need this here to track how long it takes to complete an election. We need to store the timer
// somehow so we can reference it between separate function invocations, and storing it directly in
// the Server struct isn't an option, since HistogramTimer doesn't implement Debug.
struct ElectionTimer(HistogramTimer);

impl fmt::Debug for ElectionTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "An election timer!") }
}

type AckReceiver = mpsc::Receiver<(SocketAddr, Ack)>;
type AckSender = mpsc::Sender<(SocketAddr, Ack)>;

pub trait Suitability: Debug + Send + Sync {
    fn suitability_for_msr(&self, service_group: &str) -> u64;
}

pub(crate) mod sync {
    use super::*;
    use crate::member::Member;
    use habitat_common::sync::{Lock,
                               ReadGuard,
                               WriteGuard};

    pub struct MyselfReadGuard<'a>(ReadGuard<'a, MyselfInner>);

    impl<'a> MyselfReadGuard<'a> {
        fn new(lock: &'a Lock<MyselfInner>) -> Self { Self(lock.read()) }

        pub fn to_member(&self) -> Member { self.0.as_member() }

        pub fn incarnation(&self) -> Incarnation { self.0.incarnation() }
    }

    pub struct MyselfWriteGuard<'a>(WriteGuard<'a, MyselfInner>);

    impl<'a> MyselfWriteGuard<'a> {
        fn new(lock: &'a Lock<MyselfInner>) -> Self { Self(lock.write()) }

        pub fn sync_incarnation(&mut self,
                                store: incarnation_store::IncarnationStore)
                                -> Result<()> {
            self.0.sync_incarnation(store)
        }

        pub fn increment_incarnation(&mut self) { self.0.increment_incarnation() }

        pub fn refute_incarnation(&mut self, incoming: Incarnation) {
            self.0.refute_incarnation(incoming)
        }

        pub fn mark_departed(&mut self) { self.0.mark_departed() }

        pub fn set_persistent(&mut self) { self.0.set_persistent() }
    }

    /// Encapsulate a `Member` with the added understanding that this
    /// represents the identity of this Butterfly Server.
    ///
    /// In particular, this localizes all incarnation increment and
    /// persistence logic.
    #[derive(Debug)]
    pub struct Myself {
        inner: Lock<MyselfInner>,
    }

    impl Myself {
        /// Create a new `Myself` for the given `Member`, whose
        /// incarnation number is backed by `store`
        ///
        /// Currently, `store` should only be `None` when the Butterfly
        /// Server is being initially set up; it will add one in the
        /// course of starting up. This is not ideal.
        ///
        /// It may also be `None` in the context of our current Butterfly
        /// integration tests. This also needs to be fixed, since that
        /// signals a difference between testing and "real life".
        pub fn new(member: Member, incarnation_store: Option<IncarnationStore>) -> Self {
            let inner = MyselfInner { member,
                                      incarnation_store };
            Self { inner: Lock::new(inner), }
        }

        #[must_use]
        pub fn lock_smr(&self) -> MyselfReadGuard { MyselfReadGuard::new(&self.inner) }

        #[must_use]
        pub fn lock_smw(&self) -> MyselfWriteGuard { MyselfWriteGuard::new(&self.inner) }
    }

    #[derive(Debug)]
    struct MyselfInner {
        member:            Member,
        // TODO (CM): This is only optional because the current
        // implementation of Server requires it. See note there for more.
        incarnation_store: Option<incarnation_store::IncarnationStore>,
    }

    impl MyselfInner {
        /// Read the incarnation number stored in the `IncarnationStore`
        /// and set it as our own.
        fn sync_incarnation(&mut self, store: incarnation_store::IncarnationStore) -> Result<()> {
            let value = store.load()?;
            self.incarnation_store = Some(store);
            self.member.incarnation = value;
            INCARNATION.set(value.to_i64());
            debug!("Setting incarnation number to {}", self.member.incarnation);
            Ok(())
        }

        /// Increments the incarnation by 1. A `Member`'s incarnation
        /// number can *only* be incremented by itself.
        ///
        /// This is a facade over `refute_incarnation` (you can think of
        /// it as "refuting yourself"; see its documentation for further
        /// details.
        fn increment_incarnation(&mut self) {
            let i = self.member.incarnation;
            self.refute_incarnation(i);
        }

        /// Increments our incarnation to be one greater than that of the
        /// rumor we're refuting. A `Member`'s incarnation number can
        /// *only* be incremented by itself.
        ///
        /// Ideally, the incoming incarnation *should* be strictly equal
        /// to our own. However, due to historical behavior of the
        /// Butterfly server, in some cases, it is possible for a server
        /// to have a much lower idea of its own incarnation than the rest
        /// of the network (in particular, it is possible in the
        /// transition from a server that doesn't persist its incarnation
        /// to one that does, as well as in the case where a persisting
        /// server cannot write out its number to disk for some reason;
        /// see below for more on that). In this case, to prevent having
        /// to constantly refute the same rumor over and over,
        /// incrementing one-at-a-time until our incarnation number is
        /// greater, we'll just cut to the chase and become one-greater
        /// immediately.
        ///
        /// This should also cut down on network traffic overall, as we'll
        /// be sending out fewer rumors.
        ///
        /// Note that if there was an error while persisting the
        /// incarnation number, we _still continue_. The error will be
        /// logged, but the _in-memory_ incarnation number will still be
        /// incremented. If the file is not writable over a long period of
        /// time, it may be possible for the in-memory incarnation to
        /// diverge from the persisted version.
        ///
        /// Not incrementing the in-memory incarnation number in the face
        /// of a persistence error could cause errors in refutation in the
        /// network, and it is not yet clear that we would want to do
        /// that.
        fn refute_incarnation(&mut self, incoming: Incarnation) {
            self.member.incarnation = incoming + 1;
            INCARNATION.set(self.member.incarnation.to_i64());
            if let Some(ref mut s) = self.incarnation_store {
                if let Err(e) = s.store(self.member.incarnation) {
                    error!("Error persisting incarnation '{}' to disk: {:?}",
                           self.member.incarnation, e);
                }
            }
        }

        /// Returns the current incarnation number.
        fn incarnation(&self) -> Incarnation { self.member.incarnation }

        fn mark_departed(&mut self) { self.member.departed = true }

        /// Return a copy of the underlying `Member`.
        fn as_member(&self) -> Member { self.member.clone() }

        // This is ONLY provided for some integration tests that currently
        // depend on being able to mutate the member. Ideally, the only
        // thing that should be mutable, once you actually have a fully
        // set-up Butterfly server, is the incarnation number, which is
        // accounted for in `Myself::increment_incarnation`.
        fn set_persistent(&mut self) { self.member.persistent = true; }
    }
}

/// The server struct. Is thread-safe.
#[derive(Debug)]
pub struct Server {
    name:                     Arc<String>,
    member_id:                Arc<String>,
    // TODO (CM): This is currently public because butterfly tests
    // depends on it being so. Refactor so it can be private.
    myself:                   Arc<Myself>,
    pub member_list:          Arc<MemberList>,
    ring_key:                 Arc<Option<SymKey>>,
    rumor_heat:               Arc<RumorHeat>,
    pub service_store:        RumorStore<Service>,
    pub service_config_store: RumorStore<ServiceConfig>,
    pub service_file_store:   RumorStore<ServiceFile>,
    pub service_health_store: RumorStore<ServiceHealth>,
    pub election_store:       RumorStore<Election>,
    pub update_store:         RumorStore<ElectionUpdate>,
    pub departure_store:      RumorStore<Departure>,
    swim_addr:                SocketAddr,
    gossip_addr:              SocketAddr,
    suitability_lookup:       Arc<dyn Suitability>,
    data_path:                Option<PathBuf>,
    dat_file:                 Option<Arc<Mutex<DatFileWriter>>>,
    socket:                   Option<UdpSocket>,
    departed:                 Arc<AtomicBool>,
    // These are all here for testing support
    pause:                    Arc<AtomicBool>,
    swim_rounds:              Arc<AtomicIsize>,
    gossip_rounds:            Arc<AtomicIsize>,
    block_list:               Arc<Lock<HashSet<String>>>,
    election_timers:          Arc<Mutex<HashMap<String, ElectionTimer>>>,
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server { name:                 self.name.clone(),
                 member_id:            self.member_id.clone(),
                 myself:               self.myself.clone(),
                 member_list:          self.member_list.clone(),
                 ring_key:             self.ring_key.clone(),
                 rumor_heat:           self.rumor_heat.clone(),
                 service_store:        self.service_store.clone(),
                 service_config_store: self.service_config_store.clone(),
                 service_file_store:   self.service_file_store.clone(),
                 service_health_store: self.service_health_store.clone(),
                 election_store:       self.election_store.clone(),
                 update_store:         self.update_store.clone(),
                 departure_store:      self.departure_store.clone(),
                 swim_addr:            self.swim_addr,
                 gossip_addr:          self.gossip_addr,
                 suitability_lookup:   self.suitability_lookup.clone(),
                 data_path:            self.data_path.clone(),
                 dat_file:             self.dat_file.clone(),
                 departed:             self.departed.clone(),
                 pause:                self.pause.clone(),
                 swim_rounds:          self.swim_rounds.clone(),
                 gossip_rounds:        self.gossip_rounds.clone(),
                 block_list:           self.block_list.clone(),
                 socket:               None,
                 election_timers:      self.election_timers.clone(), }
    }
}

impl Server {
    /// Create a new server, bound to the `addr`, hosting a particular `member`, and with a
    /// ring_key if you want encryption on the wire, and an optional server name.
    #[allow(clippy::too_many_arguments)]
    pub fn new(swim_addr: SocketAddr,
               gossip_addr: SocketAddr,
               mut member: Member,
               ring_key: Option<SymKey>,
               name: Option<String>,
               // TODO (CM): having data_path as optional is only something
               // that's used in testing, but it cascades outward and
               // complicates other parts of this code. We should find a way
               // to remove the optionality.
               data_path: Option<&Path>,
               suitability_lookup: Arc<dyn Suitability>)
               -> Result<Server> {
        let maybe_swim_socket_addr = swim_addr.to_socket_addrs().map(|mut iter| iter.next());
        let maybe_gossip_socket_addr = gossip_addr.to_socket_addrs().map(|mut iter| iter.next());

        match (maybe_swim_socket_addr, maybe_gossip_socket_addr) {
            (Ok(Some(swim_socket_addr)), Ok(Some(gossip_socket_addr))) => {
                member.swim_port = swim_socket_addr.port();
                member.gossip_port = gossip_socket_addr.port();

                let member_id = member.id.clone();

                // TODO (CM): This None really wants to go away. Not
                // currently setting a IncarnationStore, because it
                // depends on whether or not a data_path was actually
                // passed. That, in turn, is currently deeply rooted
                // in the testing framework.
                let myself = Myself::new(member, None);

                Ok(Server { name: Arc::new(name.unwrap_or_else(|| member_id.clone())),
                            // TODO (CM): could replace this with an accessor
                            // on member, if we have a better type
                            member_id: Arc::new(member_id),
                            myself: Arc::new(myself),
                            member_list: Arc::new(MemberList::new()),
                            ring_key: Arc::new(ring_key),
                            rumor_heat: Arc::default(),
                            service_store: RumorStore::default(),
                            service_config_store: RumorStore::default(),
                            service_file_store: RumorStore::default(),
                            service_health_store: RumorStore::default(),
                            election_store: RumorStore::default(),
                            update_store: RumorStore::default(),
                            departure_store: RumorStore::default(),
                            swim_addr: swim_socket_addr,
                            gossip_addr: gossip_socket_addr,
                            suitability_lookup,
                            data_path: data_path.as_ref().map(|p| p.into()),
                            dat_file: None,
                            departed: Arc::new(AtomicBool::new(false)),
                            pause: Arc::new(AtomicBool::new(false)),
                            swim_rounds: Arc::new(AtomicIsize::new(0)),
                            gossip_rounds: Arc::new(AtomicIsize::new(0)),
                            block_list: Arc::new(Lock::new(HashSet::new())),
                            socket: None,
                            election_timers: Arc::new(Mutex::new(HashMap::new())) })
            }
            (Err(e), _) | (_, Err(e)) => Err(Error::CannotBind(e)),
            (Ok(None), _) | (_, Ok(None)) => {
                Err(Error::CannotBind(io::Error::new(io::ErrorKind::AddrNotAvailable,
                                                     "No address discovered.")))
            }
        }
    }

    /// Every iteration of the outbound protocol (which means every member has been pinged if they
    /// are available) increments the round. If we exceed an isize in rounds, we reset to 0.
    ///
    /// This is useful in integration testing, to allow tests to time out after a worst-case
    /// boundary in rounds.
    pub fn swim_rounds(&self) -> isize { self.swim_rounds.load(Ordering::SeqCst) }

    /// Adds 1 to the current round, atomically.
    fn update_swim_round(&self) {
        let current_round = self.swim_rounds.load(Ordering::SeqCst);
        match current_round.checked_add(1) {
            Some(_number) => {
                self.swim_rounds.fetch_add(1, Ordering::SeqCst);
            }
            None => {
                debug!("Exceeded an isize integer in swim-rounds. Congratulations, this is a \
                        very long running Supervisor!");
                self.swim_rounds.store(0, Ordering::SeqCst);
            }
        }
    }

    /// Every iteration of the gossip protocol (which means every member has been sent if they
    /// are available) increments the round. If we exceed an isize in rounds, we reset to 0.
    ///
    /// This is useful in integration testing, to allow tests to time out after a worst-case
    /// boundary in rounds.
    pub fn gossip_rounds(&self) -> isize { self.gossip_rounds.load(Ordering::SeqCst) }

    /// Adds 1 to the current round, atomically.
    fn update_gossip_round(&self) {
        let current_round = self.gossip_rounds.load(Ordering::SeqCst);
        match current_round.checked_add(1) {
            Some(_number) => {
                self.gossip_rounds.fetch_add(1, Ordering::SeqCst);
            }
            None => {
                debug!("Exceeded an isize integer in gossip-rounds. Congratulations, this is a \
                        very long running Supervisor!");
                self.gossip_rounds.store(0, Ordering::SeqCst);
            }
        }
    }

    /// Start the server, along with a `Timing` for outbound connections. Spawns the `inbound`,
    /// `outbound`, and `expire` threads.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `Server::member` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    ///
    /// # Errors
    ///
    /// * Returns `Error::CannotBind` if the socket cannot be bound
    /// * Returns `Error::SocketSetReadTimeout` if the socket read timeout cannot be set
    /// * Returns `Error::SocketSetWriteTimeout` if the socket write timeout cannot be set
    pub fn start_rsw_mlw_smw_rhw_msr(&mut self, timing: &timing::Timing) -> Result<()> {
        debug!("entering habitat_butterfly::server::Server::start");
        let (tx_outbound, rx_inbound) = channel();
        if let Some(ref path) = self.data_path {
            if let Some(err) = fs::create_dir_all(path).err() {
                return Err(Error::BadDataPath(path.to_path_buf(), err));
            }

            let dat_path = path.join(format!("{}.rst", &self.member_id));
            let mut reader = DatFileReader::read_or_create_rsr_mlr(dat_path.clone(),
                                                                   &self.member_list,
                                                                   &self.service_store,
                                                                   &self.service_config_store,
                                                                   &self.service_file_store,
                                                                   &self.election_store,
                                                                   &self.update_store,
                                                                   &self.departure_store)?;

            match reader.read_into_rsw_mlw_rhw_msr(&self) {
                Ok(_) => {
                    debug!("Successfully ingested rumors from {}",
                           reader.path().display())
                }
                Err(Error::DatFileIO(path, err)) => error!("{}", Error::DatFileIO(path, err)),
                Err(err) => return Err(err),
            };

            let writer = DatFileWriter::new(dat_path);
            self.dat_file = Some(Arc::new(Mutex::new(writer)));

            {
                // Set up the incarnation persistence and ensure that
                // our Member is synced with whatever has been
                // persisted previously.
                let mut store = incarnation_store::IncarnationStore::new(path.join("INCARNATION"));
                store.initialize()?;
                self.myself.lock_smw().sync_incarnation(store)?;
            }
        }

        let socket = UdpSocket::bind(self.swim_addr)?;
        socket.set_read_timeout(Some(Duration::from_millis(1000)))
              .map_err(Error::SocketSetReadTimeout)?;
        socket.set_write_timeout(Some(Duration::from_millis(1000)))
              .map_err(Error::SocketSetReadTimeout)?;

        self.socket = Some(clone_socket(&socket)?);

        inbound::spawn_thread(format!("inbound-{}", self.name()),
                              self.clone(),
                              clone_socket(&socket)?,
                              tx_outbound)?;

        outbound::spawn_thread(format!("outbound-{}", self.name()),
                               self.clone(),
                               socket,
                               rx_inbound,
                               timing.clone())?;

        expire::spawn_thread(format!("expire-{}", self.name()),
                             self.clone(),
                             timing.clone())?;

        pull::spawn_thread(format!("pull-{}", self.name()), self.clone())?;

        push::spawn_thread(format!("push-{}", self.name()),
                           self.clone(),
                           timing.clone())?;

        if self.dat_file.is_some() {
            spawn_persist_thread(format!("persist-{}", self.name()), self.clone())?;
        }

        Ok(())
    }

    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read) Additionally `with_closure` is called with this lock held, so
    ///   the closure must not call any functions which take this lock.
    pub fn need_peer_seeding_mlr(&self) -> bool { self.member_list.is_empty_mlr() }

    /// Persistently block a given address, causing no traffic to be seen.
    ///
    /// # Locking (see locking.md)
    /// * `Server::block_list` (write)
    pub fn add_to_block_list_sblw(&self, member_id: String) {
        self.block_list.write().insert(member_id);
    }

    /// Remove a given address from the block_list.
    ///
    /// # Locking (see locking.md)
    /// * `Server::block_list` (write)
    pub fn remove_from_block_list_sblw(&self, member_id: &str) {
        self.block_list.write().remove(member_id);
    }

    /// Check if a given member ID is on the block_list.
    ///
    /// # Locking (see locking.md)
    /// * `Server::block_list` (read)
    fn is_member_blocked_sblr(&self, member_id: &str) -> bool {
        self.block_list.read().contains(member_id)
    }

    /// Stop the outbound and inbound threads from processing work.
    pub fn pause(&mut self) { self.pause.compare_and_swap(false, true, Ordering::Relaxed); }

    /// Whether this server is currently paused.
    pub fn paused(&self) -> bool { self.pause.load(Ordering::Relaxed) }

    /// Return the port number of the swim socket we are bound to.
    fn swim_port(&self) -> u16 { self.swim_addr.port() }

    /// Return the gossip address we are bound to
    pub fn gossip_addr(&self) -> &SocketAddr { &self.gossip_addr }

    /// Return the port number of the gossip socket we are bound to.
    fn gossip_port(&self) -> u16 { self.gossip_addr.port() }

    /// Return the member ID of this server.
    pub fn member_id(&self) -> &str { &self.member_id }

    /// Return the name of this server.
    pub fn name(&self) -> &str { &self.name }

    pub fn myself(&self) -> &Myself { self.myself.as_ref() }

    /// Insert a member to the `MemberList`, and update its `RumorKey` appropriately.
    ///
    /// # Locking (see locking.md)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    pub fn insert_member_mlw_rhw(&self, member: Member, health: Health) {
        let rk: RumorKey = RumorKey::from(&member);
        let member_id = member.id.clone();
        if self.member_list.insert_mlw(member, health) {
            // Purge "heat" information for a member that's
            // gone. Purging doesn't remove Member rumor information,
            // though, since that's how we let others know this member
            // has departed; that's why we subsequently start a "hot"
            // rumor.
            if health == Health::Departed {
                self.rumor_heat.lock_rhw().purge(&member_id);
            }

            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Set our member to departed, then send up to 10 out of order ack messages to other
    /// members to seed our status.
    ///
    /// # Locking (see locking.md)
    /// * `MemberList::entries` (write)
    /// * `Server::member` (write)
    /// * `RumorHeat::inner` (write)
    pub fn set_departed_mlw_smw_rhw(&self) {
        if self.socket.is_some() {
            self.myself.lock_smw().increment_incarnation();
            // TODO (CM): It's not clear that this operation is actually needed.
            self.myself.lock_smw().mark_departed();
            self.member_list.set_departed_mlw(&self.member_id);
            // We need to mark this as "hot" in order to propagate it.
            //
            // TODO (CM): This exact code is present numerous places;
            // factor it out to facilitate further code consolidation.

            // NOT calling RumorHeat::purge here because we'll be
            // shutting down soon anyway.
            self.rumor_heat
                .lock_rhw()
                .start_hot_rumor(RumorKey::new(RumorType::Member, &*self.member_id, ""));

            let check_list = self.member_list.check_list_mlr(&self.member_id);

            // TODO (CM): Even though we marked the rumor as hot
            // above, when we gossip, we send out the 5 "coolest but
            // still warm" rumors. Sending to 10 members increases the
            // chances that we'll get to this hot one now, but I don't
            // think that we can strictly guarantee that this
            // departure health update actually gets out in all cases.
            for member in check_list.iter().take(SELF_DEPARTURE_RUMOR_FANOUT) {
                let addr = member.swim_socket_address();
                // Safe because we checked above
                outbound::ack_mlr_smr_rhw(&self, self.socket.as_ref().unwrap(), member, addr, None);
            }
        } else {
            debug!("No socket present; server was never started, so nothing to depart");
        }
    }

    /// Given a membership record and some health, insert it into the Member List.
    ///
    /// # Locking (see locking.md)
    /// * `MemberList::entries` (write)
    /// * `Server::member` (write)
    /// * `RumorHeat::inner` (write)
    fn insert_member_from_rumor_mlw_smw_rhw(&self, member: Member, mut health: Health) {
        let rk: RumorKey = RumorKey::from(&member);

        if member.id == self.member_id()
           && health != Health::Alive
           && member.incarnation >= self.myself.lock_smr().incarnation()
        {
            self.myself
                .lock_smw()
                .refute_incarnation(member.incarnation);
            health = Health::Alive;
        }

        let member_id = member.id.clone();

        if self.member_list.insert_mlw(member, health) {
            if member_id != self.member_id() && health == Health::Departed {
                self.rumor_heat.lock_rhw().purge(&member_id);
            }
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Insert a service rumor into the service store.
    /// If we're adding a new service group member, we want to avoid the
    /// situation where we could lose quorum due to Confirmed but not yet
    /// departed members. So, if we have any transition one to Departed to
    /// offset the addition of the new member. We choose the member to depart
    /// by ID rather than time since confirmed since different members may
    /// have seen the transition to Confirmed at different times and we don't
    /// want to depart a bunch of different members unnecessarily.
    ///
    /// See https://github.com/habitat-sh/habitat/issues/1994
    /// See Server::check_quorum
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    pub fn insert_service_rsw_mlw_rhw(&self, service: Service) {
        Self::insert_service_impl(service,
                                  &self.service_store,
                                  &self.member_list,
                                  &self.rumor_heat,
                                  |k| self.check_quorum_mlr(k))
    }

    fn insert_service_impl(service: Service,
                           service_store: &RumorStore<Service>,
                           member_list: &MemberList,
                           rumor_heat: &RumorHeat,
                           check_quorum: impl Fn(&str) -> bool) {
        let rk = RumorKey::from(&service);
        let RumorKey { key: service_group,
                       id: member_id,
                       .. } = &rk;

        // True if rumors exist for the service group, but none containing the given member.
        let inserting_new_group_member =
            service_store.lock_rsr()
                         .get(service_group)
                         .map_or(false, |rumors| !rumors.contains_key(member_id));

        if service_store.insert_rsw(service) {
            if inserting_new_group_member && !check_quorum(service_group) {
                // Depart one confirmed member to help maintain quorum. Choose the member with the
                // minimum ID since that will most likely result in the same choice across nodes
                if let Some(member_id_to_depart) =
                    service_store.lock_rsr()
                                 .service_group(service_group)
                                 .rumors()
                                 .map(Rumor::id)
                                 .filter(|id| {
                                     member_list.health_of_by_id_mlr(id) == Some(Health::Confirmed)
                                 })
                                 .min()
                {
                    member_list.set_departed_mlw(&member_id_to_depart);
                    rumor_heat.lock_rhw().purge(&member_id_to_depart);
                    rumor_heat.lock_rhw()
                              .start_hot_rumor(RumorKey::new(RumorType::Member,
                                                             &*member_id_to_depart,
                                                             ""));
                }
            }

            rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Insert a service config rumor into the service store.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `RumorHeat::inner` (write)
    pub fn insert_service_config_rsw_rhw(&self, service_config: ServiceConfig) {
        let rk = RumorKey::from(&service_config);
        if self.service_config_store.insert_rsw(service_config) {
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Insert a service file rumor into the service file store.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `RumorHeat::inner` (write)
    pub fn insert_service_file_rsw_rhw(&self, service_file: ServiceFile) {
        let rk = RumorKey::from(&service_file);
        if self.service_file_store.insert_rsw(service_file) {
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Insert a service health rumor into the service health store.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `RumorHeat::inner` (write)
    pub fn insert_service_health_rsw_rhw(&self, service_health: ServiceHealth) {
        let rk = RumorKey::from(&service_health);
        if self.service_health_store.insert_rsw(service_health) {
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Insert a departure rumor into the departure store.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    pub fn insert_departure_rsw_mlw_rhw(&self, departure: Departure) {
        let rk = RumorKey::from(&departure);
        if *self.member_id == departure.member_id {
            self.departed
                .compare_and_swap(false, true, Ordering::Relaxed);
        }

        self.member_list.set_departed_mlw(&departure.member_id);
        self.rumor_heat.lock_rhw().purge(&departure.member_id);
        self.rumor_heat
            .lock_rhw()
            .start_hot_rumor(RumorKey::new(RumorType::Member, &departure.member_id, ""));

        if self.departure_store.insert_rsw(departure) {
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// Get all the Member ID's who are present in a given service group, and eligible to vote
    /// (alive)
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    fn get_electorate_rsr_mlr(&self, key: &str) -> Vec<String> {
        // This could be converted to a more FP approach and avoid the need for `mut`
        let mut electorate = vec![];
        for s in self.service_store.lock_rsr().service_group(key).rumors() {
            if self.member_list.health_of_by_id_mlr(&s.member_id) == Some(Health::Alive) {
                electorate.push(s.member_id.clone());
            }
        }
        electorate
    }

    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read)
    fn check_in_voting_population_by_id_mlr(&self, member_id: &str) -> bool {
        match self.member_list.health_of_by_id_mlr(member_id) {
            Some(Health::Alive) | Some(Health::Suspect) | Some(Health::Confirmed) => true,
            Some(Health::Departed) | None => false,
        }
    }

    /// Get all the Member ID's who are present in a given service group, and count towards quorum.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    fn get_total_population_rsr_mlr(&self, key: &str) -> Vec<String> {
        // This could be converted to a more FP approach and avoid the need for `mut`
        let mut total_pop = vec![];
        for s in self.service_store.lock_rsr().service_group(key).rumors() {
            if self.check_in_voting_population_by_id_mlr(&s.member_id) {
                total_pop.push(s.member_id.clone());
            }
        }
        total_pop
    }

    /// Check if a given service group has quorum to run an election.
    ///
    /// A group has quorum if a majority of its non-departed members are alive.
    ///
    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read)
    /// * `RumorStore::list` (read)
    fn check_quorum_mlr(&self, key: &str) -> bool {
        let electorate = self.get_electorate_rsr_mlr(key);
        let service_group_members = self.get_total_population_rsr_mlr(key);
        let total_population = service_group_members.len();
        let alive_population = electorate.len();
        #[allow(clippy::integer_division)]
        let has_quorum = alive_population > total_population / 2;

        trace!("check_quorum({}): {}/{} alive/total => {}, electorate: {:?}, service_group: {:?}",
               key,
               alive_population,
               total_population,
               has_quorum,
               electorate,
               service_group_members);

        has_quorum
    }

    /// Start an election for the given service group, declaring this members suitability and the
    /// term for the election.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    pub fn start_election_rsw_mlr_rhw_msr(&self, service_group: &str, term: u64) {
        let suitability = self.suitability_lookup.suitability_for_msr(&service_group);
        let has_quorum = self.check_quorum_mlr(service_group);
        let e = Election::new(self.member_id(),
                              service_group,
                              term,
                              suitability,
                              has_quorum);
        if !has_quorum {
            warn!("start_election check_quorum failed: {:?}", e);
        }
        debug!("start_election: {:?}", e);
        self.rumor_heat
            .lock_rhw()
            .start_hot_rumor(RumorKey::from(&e));
        self.election_store.insert_rsw(e);
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    pub fn start_update_election_rsw_mlr_rhw(&self,
                                             service_group: &str,
                                             suitability: u64,
                                             term: u64) {
        let has_quorum = self.check_quorum_mlr(service_group);
        let e = ElectionUpdate::new(self.member_id(),
                                    service_group,
                                    term,
                                    suitability,
                                    has_quorum);
        if !has_quorum {
            warn!("start_election check_quorum failed: {:?}", e);
        }
        debug!("start_update_election: {:?}", e);
        self.rumor_heat
            .lock_rhw()
            .start_hot_rumor(RumorKey::from(&e));
        self.update_store.insert_rsw(e);
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    fn elections_to_restart_rsr_mlr<T>(&self,
                                       elections: &RumorStore<T>,
                                       feature_flags: FeatureFlag)
                                       -> Vec<(String, u64)>
        where T: Rumor + ElectionRumor + Debug
    {
        Self::elections_to_restart_impl(elections,
                                        &self.service_store,
                                        &self.member_id(),
                                        |k| self.check_quorum_mlr(k),
                                        &self.member_list,
                                        feature_flags,
                                        &self.data_path)
    }

    fn elections_to_restart_impl<T>(elections: &RumorStore<T>,
                                    service_store: &RumorStore<Service>,
                                    myself_member_id: &str,
                                    check_quorum: impl Fn(&str) -> bool,
                                    member_list: &MemberList,
                                    feature_flags: FeatureFlag,
                                    data_path: &Option<PathBuf>)
                                    -> Vec<(String, u64)>
        where T: ConstIdRumor + ElectionRumor + Debug
    {
        let mut elections_to_restart = vec![];

        for (service_group, rumors) in elections.lock_rsr().iter() {
            if service_store.lock_rsr()
                            .service_group(&service_group)
                            .contains_id(myself_member_id)
            {
                // This is safe; there is only one id for a ConstIdRumor
                let election =
                    rumors.get(T::const_id())
                          .expect("Lost an election struct between looking it up and reading it.");
                debug!("elections_to_restart: checking {} -> {:#?}",
                       service_group, election);

                if election_trigger::maybe_trigger(service_group, feature_flags, &data_path) {
                    elections_to_restart.push((String::from(&service_group[..]), election.term()));
                } else {
                    // We're not manually triggering a new election, so we should check to see if we
                    // need to start a new one the old-fashioned way.

                    // If we are finished, and the leader is dead, we should restart the election
                    if election.is_finished() && election.member_id() == myself_member_id {
                        // If we are the leader, and we have lost quorum, we should restart
                        // the election
                        if !check_quorum(election.key()) {
                            warn!("Restarting election with a new term as the leader has lost \
                                   quorum: {:?}",
                                  election);
                            elections_to_restart.push((String::from(&service_group[..]),
                                                       election.term()));
                        }
                    } else if election.is_finished() {
                        let leader_health = member_list.health_of_by_id_mlr(election.member_id())
                                                       .unwrap_or_else(|| {
                                                           debug!("No health information for {}; \
                                                                   treating as Departed",
                                                                  election.member_id());
                                                           Health::Departed
                                                       });
                        if leader_health >= Health::Confirmed {
                            warn!("Restarting election with a new term as the leader is dead {}: \
                                   {:?}",
                                  myself_member_id, election);
                            elections_to_restart.push((String::from(&service_group[..]),
                                                       election.term()));
                        }
                    }
                }
            }
        }

        elections_to_restart
    }

    /// Check to see if this server needs to restart a given election. This happens when:
    ///
    /// a) We are the leader, and we have lost quorum with the rest of the group.
    /// b) We are not the leader, and we have detected that the leader is confirmed dead.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    pub fn restart_elections_rsw_mlr_rhw_msr(&self, feature_flags: FeatureFlag) {
        let elections_to_restart =
            self.elections_to_restart_rsr_mlr(&self.election_store, feature_flags);

        // TODO (CM): not currently triggering update elections!
        // There's only one kind of sentinel file at the moment, and
        // that's for non-update elections. If that file existed,
        // it'll be gone by the time we get here.
        let update_elections_to_restart =
            self.elections_to_restart_rsr_mlr(&self.update_store, feature_flags);

        for (service_group, old_term) in elections_to_restart {
            let term = old_term + 1;
            warn!("Starting a new election for {} {}", service_group, term);
            self.election_store
                .remove_rsw(&service_group, Election::const_id());
            self.start_election_rsw_mlr_rhw_msr(&service_group, term);
        }

        for (service_group, old_term) in update_elections_to_restart {
            let term = old_term + 1;
            warn!("Starting a new election for {} {}", service_group, term);
            self.update_store
                .remove_rsw(&service_group, ElectionUpdate::const_id());
            self.start_update_election_rsw_mlr_rhw(&service_group, 0, term);
        }
    }

    /// Insert an election into the election store. Handles creating a new election rumor for this
    /// member on receipt of an election rumor for a service this server cares about. Also handles
    /// stopping the election if we are the winner and we have enough votes.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    pub fn insert_election_rsw_mlr_rhw_msr(&self, mut election: Election) {
        debug!("insert_election: {:?}", election);
        let rk = RumorKey::from(&election);

        // If this is an election for a service group we care about
        if self.service_store
               .lock_rsr()
               .service_group(&election.service_group)
               .contains_id(self.member_id())
        {
            trace!("{} is a member of {}",
                   self.member_id(),
                   election.service_group);
            // And the election store already has an election rumor for this election
            if self.election_store.lock_rsr().contains_rumor(&election) {
                let new_term = self.election_store
                                   .lock_rsr()
                                   .get_term(election.key())
                                   .map(|stored_term| election.term > stored_term)
                                   .unwrap_or(false);
                if new_term {
                    debug!("removing old rumor and starting new election");
                    self.election_store
                        .remove_rsw(election.key(), election.id());
                    self.start_election_rsw_mlr_rhw_msr(&election.service_group, election.term);
                }
                // If we are the member that this election is voting for, then check to see if the
                // election is over! If it is, mark this election as final before you process it.
                if self.member_id() == election.member_id {
                    if self.check_quorum_mlr(election.key()) {
                        let electorate = self.get_electorate_rsr_mlr(election.key());
                        let mut num_votes = 0;
                        for vote in election.votes.iter() {
                            if electorate.contains(vote) {
                                num_votes += 1;
                            }
                        }
                        if num_votes == electorate.len() {
                            debug!("Election is finished: {:#?}", election);
                            election.finish();
                            // Now we're going to record how long the election took. NOTE that this
                            // will only work as long as the same member starts and finishes the
                            // election (which is how it currently is). If we ever change elections
                            // so that any member can finish an election, this will break.
                            let mut existing_timers = self.election_timers
                                                          .lock()
                                                          .expect("Election timers lock poisoned");

                            // Just to be extra clear, we don't need this timer any more because
                            // once we call observe_duration(), the HistogramVec contained in
                            // ELECTION_DURATION has the data we want, stored in the global
                            // registry.
                            if let Some(timer) = existing_timers.remove(&election.service_group) {
                                timer.0.observe_duration();
                            }
                        } else {
                            debug!("I have quorum, but election is not finished {}/{}",
                                   num_votes,
                                   electorate.len());
                        }
                    } else {
                        election.no_quorum();
                        warn!("Election lacks quorum: {:#?}", election);
                    }
                }
            } else {
                // Otherwise, we need to create a new election object for ourselves prior to
                // merging.
                let timer = ELECTION_DURATION.with_label_values(&[&election.service_group])
                                             .start_timer();
                let mut existing_timers = self.election_timers
                                              .lock()
                                              .expect("Election timers lock poisoned");
                existing_timers.insert(election.service_group.clone(), ElectionTimer(timer));
                self.start_election_rsw_mlr_rhw_msr(&election.service_group, election.term);
            }

            if !election.is_finished() {
                let has_quorum = self.check_quorum_mlr(election.key());
                if has_quorum {
                    election.running();
                } else {
                    election.no_quorum();
                }
            }
        }

        if self.election_store.insert_rsw(election) {
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    pub fn insert_update_election_rsw_mlr_rhw(&self, mut election: ElectionUpdate) {
        debug!("insert_update_election: {:?}", election);
        let rk = RumorKey::from(&election);

        // If this is an election for a service group we care about
        if self.service_store
               .lock_rsr()
               .service_group(&election.service_group)
               .contains_id(self.member_id())
        {
            trace!("{} is a member of {}",
                   self.member_id(),
                   election.service_group);
            // And the election store already has an election rumor for this election
            if self.update_store.lock_rsr().contains_rumor(&election) {
                let new_term = self.update_store
                                   .lock_rsr()
                                   .get_term(election.key())
                                   .map(|stored_term| election.term > stored_term)
                                   .unwrap_or(false);
                if new_term {
                    debug!("removing old rumor and starting new election");
                    self.update_store.remove_rsw(election.key(), election.id());
                    self.start_update_election_rsw_mlr_rhw(&election.service_group,
                                                           0,
                                                           election.term);
                }
                // If we are the member that this election is voting for, then check to see if the
                // election is over! If it is, mark this election as final before you process it.
                if self.member_id() == election.member_id {
                    if self.check_quorum_mlr(election.key()) {
                        let electorate = self.get_electorate_rsr_mlr(election.key());
                        let mut num_votes = 0;
                        for vote in election.votes.iter() {
                            if electorate.contains(vote) {
                                num_votes += 1;
                            }
                        }
                        if num_votes == electorate.len() {
                            debug!("Election is finished: {:#?}", election);
                            election.finish();
                        } else {
                            debug!("I have quorum, but election is not finished {}/{}",
                                   num_votes,
                                   electorate.len());
                        }
                    } else {
                        election.no_quorum();
                        warn!("Election lacks quorum: {:#?}", election);
                    }
                }
            } else {
                // Otherwise, we need to create a new election object for ourselves prior to
                // merging.
                self.start_update_election_rsw_mlr_rhw(&election.service_group, 0, election.term);
            }

            if !election.is_finished() {
                let has_quorum = self.check_quorum_mlr(election.key());
                if has_quorum {
                    election.running();
                } else {
                    election.no_quorum();
                }
            }
        }

        if self.update_store.insert_rsw(election) {
            self.rumor_heat.lock_rhw().start_hot_rumor(rk);
        }
    }

    fn generate_wire(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        message::generate_wire(payload, (*self.ring_key).as_ref())
    }

    fn unwrap_wire(&self, payload: &[u8]) -> Result<Vec<u8>> {
        message::unwrap_wire(payload, (*self.ring_key).as_ref())
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    pub fn persist_data_rsr_mlr(&self) {
        if let Some(ref dat_file_lock) = self.dat_file {
            let dat_file = dat_file_lock.lock().expect("DatFile lock poisoned");
            if let Some(err) = dat_file.write_rsr_mlr(&self.member_list,
                                                      &self.service_store,
                                                      &self.service_config_store,
                                                      &self.service_file_store,
                                                      &self.election_store,
                                                      &self.update_store,
                                                      &self.departure_store)
                                       .err()
            {
                error!("Error persisting rumors to disk, {}", err);
            } else {
                info!("Rumors persisted to disk: {}", dat_file.path().display());
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_departed(&self) -> bool { self.departed.load(Ordering::Relaxed) }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "{}@{}/{}",
               self.name(),
               self.swim_port(),
               self.gossip_port())
    }
}

fn spawn_persist_thread(name: String, server: Server) -> std::io::Result<()> {
    thread::Builder::new().name(name)
                          .spawn(move || -> ! { persist_loop(&server) })
                          .map(|_| ())
}

fn persist_loop(server: &Server) -> ! {
    habitat_core::env_config_duration!(PersistLoopPeriod,
                                       HAB_PERSIST_LOOP_PERIOD_SECS => from_secs,
                                       Duration::from_secs(30));

    let min_loop_period: Duration = PersistLoopPeriod::configured_value().into();

    loop {
        liveliness_checker::mark_thread_alive().and_divergent();

        let before_persist = Instant::now();
        server.persist_data_rsr_mlr();
        let time_to_persist = before_persist.elapsed();
        trace!("persist_data took {:?}", time_to_persist);
        match min_loop_period.checked_sub(time_to_persist) {
            Some(time_to_wait) => thread::sleep(time_to_wait),
            None => {
                warn!("Persisting data took longer than expected: {:?}",
                      time_to_persist)
            }
        }
    }
}

/// There is a bug which surfaced in rust 1.38 where cloned sockets on windows
/// get inherited by child processes and remain open even after the process that
/// created the socket terminates as long as the child processes remain alive.
/// See https://github.com/rust-lang/rust/issues/70719.
/// Until this is fixed, we explicitly clear the HANDLE_FLAG_INHERIT of the
/// socket's handle.
#[cfg(windows)]
fn clone_socket(socket: &UdpSocket) -> Result<UdpSocket> {
    use std::os::windows::io::AsRawSocket;
    use winapi::um::{handleapi,
                     winbase,
                     winnt};

    let cloned = socket.try_clone()?;
    match unsafe {
              handleapi::SetHandleInformation(cloned.as_raw_socket() as winnt::HANDLE,
                                              winbase::HANDLE_FLAG_INHERIT,
                                              0)
          } {
        0 => Err(Error::OsError(io::Error::last_os_error())),
        _ => Ok(cloned),
    }
}

#[cfg(unix)]
fn clone_socket(socket: &UdpSocket) -> std::io::Result<UdpSocket> { socket.try_clone() }

/// This is a proxy struct to represent what information we're writing to the dat file, and
/// therefore what information gets sent out via the HTTP API. Right now, we're just wrapping the
/// actual Server struct, but this will give us something we can refactor against without
/// worrying about breaking the data returned to users.
pub struct ServerProxy<'a>(&'a Server);

impl<'a> ServerProxy<'a> {
    pub fn new(s: &'a Server) -> Self { ServerProxy(&s) }
}

impl<'a> Serialize for ServerProxy<'a> {
    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read)
    /// * `RumorStore::list` (read)
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let dsp = RumorStoreProxy::new(&self.0.departure_store);
        let esp = RumorStoreProxy::new(&self.0.election_store);
        let ssp = RumorStoreProxy::new(&self.0.service_store);
        let eusp = RumorStoreProxy::new(&self.0.update_store);
        let scsp = RumorStoreProxy::new(&self.0.service_config_store);
        let sfsp = RumorStoreProxy::new(&self.0.service_file_store);
        let mlp = MemberListProxy::new(&self.0.member_list);

        let mut strukt = serializer.serialize_struct("butterfly_server", 7)?;
        strukt.serialize_field("member", &self.0.member_list)?;
        strukt.serialize_field("membership", &mlp)?;
        strukt.serialize_field("service", &self.0.service_store)?;
        strukt.serialize_field("services", &ssp)?;
        strukt.serialize_field("service_config", &self.0.service_config_store)?;
        strukt.serialize_field("latest_service_config", &scsp)?;
        strukt.serialize_field("service_file", &self.0.service_file_store)?;
        strukt.serialize_field("service_files", &sfsp)?;
        strukt.serialize_field("election", &self.0.election_store)?;
        strukt.serialize_field("latest_election", &esp)?;
        strukt.serialize_field("election_update", &self.0.update_store)?;
        strukt.serialize_field("latest_election_update", &eusp)?;
        strukt.serialize_field("departure", &self.0.departure_store)?;
        strukt.serialize_field("departed_members", &dsp)?;
        strukt.end()
    }
}

// Note: this is a separate module solely to facilitate targeted
// logging, e.g.
//
//     RUST_LOG=habitat_butterfly::server::election_trigger=trace
mod election_trigger {
    use habitat_common::FeatureFlag;
    use std::{fs,
              path::PathBuf};

    /// If the HAB_FEAT_TRIGGER_ELECTION feature flag is enabled,
    /// we'll look on disk to see if a sentinel file for the
    /// triggering of an election of this service group is present. If
    /// so, we should trigger that election.
    ///
    /// An election for service `foo.default` will be triggered if:
    ///
    /// - the supervisor is running with `HAB_FEAT_TRIGGER_ELECTION=1`
    /// - the file `/hab/sup/default/data/trigger_foo.default_election` is present _and can also be
    ///   successfully deleted by the Supervisor_.
    ///
    /// The latter is important to prevent repeated triggering of an
    /// election. The user puts the file in place to signal their
    /// intent, and the Supervisor deletes it as an acknowledgment of
    /// that intent.
    pub(super) fn maybe_trigger(service_group_name: &str,
                                feature_flags: FeatureFlag,
                                data_path: &Option<PathBuf>)
                                -> bool {
        if !feature_flags.contains(FeatureFlag::TRIGGER_ELECTION) {
            trace!("HAB_FEAT_TRIGGER_ELECTION: Feature not enabled");
            return false;
        }

        if data_path.is_none() {
            trace!("HAB_FEAT_TRIGGER_ELECTION: In a test");
            return false;
        }

        let sentinel_file = data_path.as_ref()
                                     .unwrap() // safe; data_path is Some
                                     .join(format!("trigger_{}_election", service_group_name));
        if !sentinel_file.is_file() {
            trace!("HAB_FEAT_TRIGGER_ELECTION: No trigger file found for {}",
                   service_group_name);
            return false;
        }

        if let Err(e) = fs::remove_file(&sentinel_file) {
            // If we trigger without being able to remove
            // the file, then we'll just constantly
            // trigger elections.
            warn!("HAB_FEAT_TRIGGER_ELECTION: Could not delete sentinel file {:?}; NOT \
                   triggering election! {:?}",
                  sentinel_file, e);
            false
        } else {
            info!("HAB_FEAT_TRIGGER_ELECTION: Manually triggering election for {}",
                  service_group_name);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rumor::election::Term;
    use habitat_core::service::ServiceGroup;
    use std::str::FromStr;

    fn check_quorum_returns(val: bool) -> impl Fn(&str) -> bool { move |_: &str| val }

    fn mock_service(member: &Member) -> Service {
        Service { member_id:     member.id.clone(),
                  service_group: ServiceGroup::from_str("group.default").unwrap(),
                  incarnation:   Default::default(),
                  initialized:   Default::default(),
                  pkg:           Default::default(),
                  cfg:           Default::default(),
                  sys:           Default::default(), }
    }

    #[test]
    fn elections_are_restarted_when_leader_health_is_unknown() {
        env_logger::try_init().ok();
        let suitability = 1;
        let term = 0;
        let elections = RumorStore::<Election>::default();
        let service_store = RumorStore::<Service>::default();
        let myself = Member::default();
        let unknown_leader_member_id = "unknown_leader";
        let member_list = MemberList::new();
        let service = mock_service(&myself);

        let mut election_with_unknown_leader = Election::new(unknown_leader_member_id,
                                                             &service.service_group,
                                                             Term::default(),
                                                             suitability,
                                                             true /* has_quorum */);
        election_with_unknown_leader.finish();
        elections.insert_rsw(election_with_unknown_leader);

        service_store.insert_rsw(service.clone());

        let to_restart = Server::elections_to_restart_impl(&elections,
                                                           &service_store,
                                                           &myself.id,
                                                           check_quorum_returns(true),
                                                           &member_list,
                                                           FeatureFlag::empty(),
                                                           &None);

        assert_eq!(to_restart, vec![(service.service_group.to_string(), term)]);
    }

    #[test]
    fn elections_are_restarted_when_leader_is_departed() {
        env_logger::try_init().ok();
        let suitability = 1;
        let term = 0;
        let elections = RumorStore::<Election>::default();
        let service_store = RumorStore::<Service>::default();
        let myself = Member::default();
        let departed_leader = Member::default();
        let member_list = MemberList::new();
        let service = mock_service(&myself);

        let mut election_with_unknown_leader = Election::new(departed_leader.id.clone(),
                                                             &service.service_group,
                                                             Term::default(),
                                                             suitability,
                                                             true /* has_quorum */);
        election_with_unknown_leader.finish();
        elections.insert_rsw(election_with_unknown_leader);

        service_store.insert_rsw(service.clone());

        member_list.insert_mlw(departed_leader, Health::Departed);

        let to_restart = Server::elections_to_restart_impl(&elections,
                                                           &service_store,
                                                           &myself.id,
                                                           check_quorum_returns(true),
                                                           &member_list,
                                                           FeatureFlag::empty(),
                                                           &None);

        assert_eq!(to_restart, vec![(service.service_group.to_string(), term)]);
    }

    #[test]
    fn insert_service_adds_service_to_service_store() {
        let service = mock_service(&Member::default());
        let service_store = RumorStore::default();
        let member_list = MemberList::new();
        let rumor_heat = RumorHeat::default();

        Server::insert_service_impl(service.clone(),
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(false));

        assert!(service_store.lock_rsr().contains_rumor(&service));
    }

    #[test]
    fn insert_service_with_new_member_departs_confirmed_member() {
        let alive_member = Member::default();
        let confirmed_member = Member::default();
        let confirmed_member_service_rumor = mock_service(&confirmed_member);
        let alive_member_service_rumor = mock_service(&alive_member);
        let service_store = RumorStore::default();
        let member_list = MemberList::new();
        let rumor_heat = RumorHeat::default();

        member_list.insert_mlw(alive_member, Health::Alive);
        member_list.insert_mlw(confirmed_member.clone(), Health::Confirmed);

        Server::insert_service_impl(confirmed_member_service_rumor,
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(false));

        assert_eq!(member_list.health_of_mlr(&confirmed_member),
                   Some(Health::Confirmed));

        Server::insert_service_impl(alive_member_service_rumor,
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(false));

        assert_eq!(member_list.health_of_mlr(&confirmed_member),
                   Some(Health::Departed));
    }

    #[test]
    fn insert_service_with_existing_member_does_not_depart_confirmed_member() {
        let alive_member = Member::default();
        let confirmed_member = Member::default();
        let confirmed_member_service_rumor = mock_service(&confirmed_member);
        let alive_member_service_rumor = mock_service(&alive_member);
        let service_store = RumorStore::default();
        let member_list = MemberList::new();
        let rumor_heat = RumorHeat::default();

        member_list.insert_mlw(alive_member, Health::Alive);
        // This member will become confirmed later. If it's already Confirmed
        // when inserted, it could be departed immediately
        member_list.insert_mlw(confirmed_member.clone(), Health::Alive);

        Server::insert_service_impl(alive_member_service_rumor.clone(),
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(false));

        Server::insert_service_impl(confirmed_member_service_rumor,
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(false));

        member_list.insert_mlw(confirmed_member.clone(), Health::Confirmed);

        Server::insert_service_impl(Service { incarnation: alive_member_service_rumor.incarnation
                                                           + 1,
                                              ..alive_member_service_rumor },
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(false));

        assert_eq!(member_list.health_of_mlr(&confirmed_member),
                   Some(Health::Confirmed));
    }

    #[test]
    fn insert_service_with_quorum_does_not_depart_confirmed_member() {
        let alive_member = Member::default();
        let confirmed_member = Member::default();
        let confirmed_member_service_rumor = mock_service(&confirmed_member);
        let alive_member_service_rumor = mock_service(&alive_member);
        let service_store = RumorStore::default();
        let member_list = MemberList::new();
        let rumor_heat = RumorHeat::default();

        member_list.insert_mlw(alive_member, Health::Alive);
        member_list.insert_mlw(confirmed_member.clone(), Health::Confirmed);

        Server::insert_service_impl(confirmed_member_service_rumor,
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(true));

        assert_eq!(member_list.health_of_mlr(&confirmed_member),
                   Some(Health::Confirmed));

        Server::insert_service_impl(alive_member_service_rumor,
                                    &service_store,
                                    &member_list,
                                    &rumor_heat,
                                    check_quorum_returns(true));

        assert_eq!(member_list.health_of_mlr(&confirmed_member),
                   Some(Health::Confirmed));
    }
    mod myself {
        use super::super::*;
        use crate::member::Member;
        use mktemp::Temp;
        use std::path::Path;

        /// Helper function to create an instance of `Myself` for
        /// tests.
        fn myself<P>(path: P) -> Myself
            where P: AsRef<Path>
        {
            let mut i = IncarnationStore::new(path.as_ref());
            i.initialize()
             .expect("Couldn't initialize incarnation store");
            Myself::new(Member::default(), Some(i))
        }

        #[test]
        fn myself_can_increment_its_incarnation() {
            let path = Temp::new_dir().expect("Could not create temp file");
            let me = myself(path.as_ref().join("INCARNATION"));

            assert_eq!(me.lock_smr().incarnation(),
                       Incarnation::default(),
                       "Incarnation should start at the default of {}",
                       Incarnation::default());
            me.lock_smw().increment_incarnation();
            assert_eq!(me.lock_smr().incarnation(),
                       Incarnation::from(1),
                       "Incarnation should have incremented by 1");
        }

        #[test]
        fn refute_an_incarnation() {
            let path = Temp::new_dir().expect("Could not create temp file");
            let me = myself(path.as_ref().join("INCARNATION"));

            assert_eq!(me.lock_smr().incarnation(),
                       Incarnation::default(),
                       "Incarnation should start at the default of {}",
                       Incarnation::default());

            let incarnation_to_refute = Incarnation::from(25);
            me.lock_smw().refute_incarnation(incarnation_to_refute);
            assert_eq!(me.lock_smr().incarnation(),
                       incarnation_to_refute + 1,
                       "Incarnation should be one greater than the refuted incarnation");
        }
    }

    mod server {
        use super::*;
        use crate::{member::Member,
                    server::{timing::Timing,
                             Server,
                             Suitability}};
        use std::{fs::File,
                  io::prelude::*,
                  net::{IpAddr,
                        Ipv4Addr,
                        SocketAddr},
                  sync::Mutex};
        use tempfile::TempDir;

        lazy_static! {
            static ref SWIM_PORT: Mutex<u16> = Mutex::new(6666);
            static ref GOSSIP_PORT: Mutex<u16> = Mutex::new(7777);
        }

        #[derive(Debug)]
        struct ZeroSuitability;
        impl Suitability for ZeroSuitability {
            fn suitability_for_msr(&self, _service_group: &str) -> u64 { 0 }
        }

        fn start_server() -> Server {
            let swim_port;
            {
                let mut swim_port_guard = SWIM_PORT.lock().expect("SWIM_PORT poisoned");
                swim_port = *swim_port_guard;
                *swim_port_guard += 1;
            }
            let swim_listen = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), swim_port);
            let gossip_port;
            {
                let mut gossip_port_guard = GOSSIP_PORT.lock().expect("GOSSIP_PORT poisoned");
                gossip_port = *gossip_port_guard;
                *gossip_port_guard += 1;
            }
            let gossip_listen =
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), gossip_port);
            let mut member = Member::default();
            member.swim_port = swim_port;
            member.gossip_port = gossip_port;
            Server::new(swim_listen,
                        gossip_listen,
                        member,
                        None,
                        None,
                        None,
                        Arc::new(ZeroSuitability)).unwrap()
        }

        fn start_with_corrupt_rumor_file(tmpdir: &TempDir) -> Server {
            let swim_port;
            {
                let mut swim_port_guard = SWIM_PORT.lock().expect("SWIM_PORT poisoned");
                swim_port = *swim_port_guard;
                *swim_port_guard += 1;
            }
            let swim_listen = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), swim_port);
            let gossip_port;
            {
                let mut gossip_port_guard = GOSSIP_PORT.lock().expect("GOSSIP_PORT poisoned");
                gossip_port = *gossip_port_guard;
                *gossip_port_guard += 1;
            }
            let gossip_listen =
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), gossip_port);
            let mut member = Member::default();
            member.swim_port = swim_port;
            member.gossip_port = gossip_port;
            let rumor_name = format!("{}{}", member.id.to_string(), ".rst");
            let file_path = tmpdir.path().to_owned().join(rumor_name);
            let mut rumor_file = File::create(file_path).unwrap();
            writeln!(rumor_file, "This is not a valid rumor file!").unwrap();
            Server::new(swim_listen,
                        gossip_listen,
                        member,
                        None,
                        None,
                        Some(tmpdir.path()),
                        Arc::new(ZeroSuitability)).unwrap()
        }

        #[test]
        fn new() { start_server(); }

        #[test]
        #[should_panic]
        fn new_with_corrupt_rumor_file() {
            let tmpdir = TempDir::new().unwrap();
            let mut server = start_with_corrupt_rumor_file(&tmpdir);
            server.start_rsw_mlw_smw_rhw_msr(&Timing::default())
                  .expect("Server failed to start");
        }

        #[test]
        fn start_listener() {
            let mut server = start_server();
            server.start_rsw_mlw_smw_rhw_msr(&Timing::default())
                  .expect("Server failed to start");
        }
    }
}
