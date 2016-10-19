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

//! The SWIM server.
//!
//! Creates `Server` structs, that hold everything we need to run the SWIM protocol. Winds up with
//! 3 separate threads - inbound (incoming connections), outbound (the Probe protocl), and expire
//! (turning Suspect members into Confirmed members).

pub mod expire;
pub mod inbound;
pub mod outbound;
pub mod pull;
pub mod push;
pub mod timing;

use std::collections::HashSet;
use std::fmt;
use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

use error::{Result, Error};
use member::{Member, Health, MemberList};
use trace::{Trace, TraceKind};
use rumor::{RumorList, RumorKey};

/// The server struct. Is thread-safe.
#[derive(Debug, Clone)]
pub struct Server {
    pub name: Arc<String>,
    pub member_id: Arc<String>,
    pub member: Arc<RwLock<Member>>,
    pub member_list: MemberList,
    pub rumor_list: RumorList,
    pub swim_addr: Arc<RwLock<SocketAddr>>,
    pub gossip_addr: Arc<RwLock<SocketAddr>>,
    // These are all here for testing support
    pub pause: Arc<AtomicBool>,
    pub trace: Arc<RwLock<Trace>>,
    pub rounds: Arc<AtomicIsize>,
    pub blacklist: Arc<RwLock<HashSet<String>>>,
}

impl Server {
    /// Create a new server, bound to the `addr`, hosting a particular `member`, and with a
    /// `Trace` struct.
    pub fn new<A: ToSocketAddrs>(swim_addr: A,
                                 gossip_addr: A,
                                 member: Member,
                                 trace: Trace,
                                 name: Option<String>)
                                 -> Result<Server> {
        let swim_socket_addr = match swim_addr.to_socket_addrs() {
            Ok(mut addrs) => addrs.nth(0).unwrap(),
            Err(e) => return Err(Error::CannotBind(e)),
        };
        let gossip_socket_addr = match gossip_addr.to_socket_addrs() {
            Ok(mut addrs) => addrs.nth(0).unwrap(),
            Err(e) => return Err(Error::CannotBind(e)),
        };
        Ok(Server {
            name: Arc::new(name.unwrap_or(String::from(member.get_id()))),
            member_id: Arc::new(String::from(member.get_id())),
            member: Arc::new(RwLock::new(member)),
            member_list: MemberList::new(),
            rumor_list: RumorList::default(),
            swim_addr: Arc::new(RwLock::new(swim_socket_addr)),
            gossip_addr: Arc::new(RwLock::new(gossip_socket_addr)),
            pause: Arc::new(AtomicBool::new(false)),
            trace: Arc::new(RwLock::new(trace)),
            rounds: Arc::new(AtomicIsize::new(0)),
            blacklist: Arc::new(RwLock::new(HashSet::new())),
        })
    }

    /// Every iteration of the outbound protocol (which means every member has been pinged if they
    /// are available) increments the round. If we exceed an isize in rounds, we reset to 0.
    ///
    /// This is useful in integration testing, to allow tests to time out after a worst-case
    /// boundary in rounds.
    pub fn rounds(&self) -> isize {
        self.rounds.load(Ordering::SeqCst)
    }

    /// Adds 1 to the current round, atomically.
    pub fn update_round(&self) {
        let current_round = self.rounds.load(Ordering::SeqCst);
        match current_round.checked_add(1) {
            Some(_number) => {
                self.rounds.fetch_add(1, Ordering::SeqCst);
            }
            None => {
                debug!("Exceeded an isize integer in rounds. Congratulations, this is a very \
                        long running supervisor!");
                self.rounds.store(0, Ordering::SeqCst);
            }
        }
    }

    /// Start the server, aloung with a `Timing` for outbound connections. Spawns the `inbound`,
    /// `outbound`, and `expire` threads.
    ///
    /// # Errors
    ///
    /// * Returns `Error::CannotBind` if the socket cannot be bound
    /// * Returns `Error::SocketSetReadTimeout` if the socket read timeout cannot be set
    /// * Returns `Error::SocketSetWriteTimeout` if the socket write timeout cannot be set
    pub fn start(&self, timing: timing::Timing) -> Result<()> {
        let (tx_outbound, rx_inbound) = channel();

        let socket =
            match UdpSocket::bind(*self.swim_addr.read().expect("Swim address lock is poisoned")) {
                Ok(socket) => socket,
                Err(e) => return Err(Error::CannotBind(e)),
            };
        try!(socket.set_read_timeout(Some(Duration::from_millis(1000)))
            .map_err(|e| Error::SocketSetReadTimeout(e)));
        try!(socket.set_write_timeout(Some(Duration::from_millis(1000)))
            .map_err(|e| Error::SocketSetReadTimeout(e)));


        let server_a = self.clone();
        let socket_a = match socket.try_clone() {
            Ok(socket_a) => socket_a,
            Err(_) => return Err(Error::SocketCloneError),
        };
        let _ = thread::Builder::new().name(format!("inbound-{}", self.name())).spawn(move || {
            inbound::Inbound::new(&server_a, socket_a, tx_outbound).run();
            panic!("You should never, ever get here, judy");
        });

        let server_b = self.clone();
        let socket_b = match socket.try_clone() {
            Ok(socket_b) => socket_b,
            Err(_) => return Err(Error::SocketCloneError),
        };
        let timing_b = timing.clone();
        let _ = thread::Builder::new().name(format!("outbound-{}", self.name())).spawn(move || {
            outbound::Outbound::new(&server_b, socket_b, rx_inbound, timing_b).run();
            panic!("You should never, ever get here, bob");
        });

        let server_c = self.clone();
        let timing_c = timing.clone();
        let _ = thread::Builder::new().name(format!("expire-{}", self.name())).spawn(move || {
            expire::Expire::new(&server_c, timing_c).run();
            panic!("You should never, ever get here, frank");
        });

        let server_d = self.clone();
        let _ = thread::Builder::new().name(format!("pull-{}", self.name())).spawn(move || {
            pull::Pull::new(&server_d).run();
            panic!("You should never, ever get here, davey");
        });

        let server_e = self.clone();
        let _ = thread::Builder::new().name(format!("push-{}", self.name())).spawn(move || {
            push::Push::new(&server_e, timing).run();
            panic!("You should never, ever get here, liu");
        });

        Ok(())
    }

    /// Blacklist a given address, causing no traffic to be seen.
    pub fn add_to_blacklist(&self, member_id: String) {
        let mut blacklist = self.blacklist.write().expect("Write lock for blacklist is poisoned");
        blacklist.insert(member_id);
    }

    /// Remove a given address from the blacklist.
    pub fn remove_from_blacklist(&self, member_id: &str) {
        let mut blacklist = self.blacklist.write().expect("Write lock for blacklist is poisoned");
        blacklist.remove(member_id);
    }

    /// Check that a given address is on the blacklist.
    pub fn check_blacklist(&self, member_id: &str) -> bool {
        let blacklist = self.blacklist.write().expect("Write lock for blacklist is poisoned");
        blacklist.contains(member_id)
    }

    /// Stop the outbound and inbound threads from processing work.
    pub fn pause(&mut self) {
        self.pause.compare_and_swap(false, true, Ordering::Relaxed);
    }

    /// Allow the outbound and inbound threads to process work.
    pub fn unpause(&mut self) {
        self.pause.compare_and_swap(true, false, Ordering::Relaxed);
    }

    /// Whether this server is currently paused.
    pub fn paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    /// Return the swim address we are bound to
    pub fn swim_addr(&self) -> SocketAddr {
        let sa = self.swim_addr.read().expect("Swim Address lock poisoned");
        sa.clone()
    }


    /// Return the port number of the swim socket we are bound to.
    pub fn swim_port(&self) -> u16 {
        self.swim_addr.read().expect("Swim Address lock poisoned").port()
    }

    /// Return the gossip address we are bound to
    pub fn gossip_addr(&self) -> SocketAddr {
        let ga = self.gossip_addr.read().expect("Gossip Address lock poisoned");
        ga.clone()
    }

    /// Return the port number of the gossip socket we are bound to.
    pub fn gossip_port(&self) -> u16 {
        self.gossip_addr.read().expect("Gossip Address lock poisoned").port()
    }

    /// Return the member ID of this server.
    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    /// Return the name of this server.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Insert a member to the `MemberList`, and update its `RumorKey` appropriately.
    pub fn insert_member(&self, member: Member, health: Health) {
        let rk: RumorKey = RumorKey::from(&member);
        // NOTE: This sucks so much right here. Check out how we allocate no matter what, because
        // of just how the logic goes. The value of the trace is really high, though, so we suck it
        // for now.
        let trace_member_id = String::from(member.get_id());
        let trace_incarnation = member.get_incarnation();
        let trace_health = health.clone();
        if self.member_list.insert(member, health) {
            trace_it!(MEMBERSHIP: self,
                      TraceKind::MemberUpdate,
                      trace_member_id,
                      trace_incarnation,
                      trace_health);
            self.rumor_list.insert(rk);
        }
    }

    /// Change the helth of a `Member`, and update its `RumorKey`.
    pub fn insert_health(&self, member: &Member, health: Health) {
        let rk: RumorKey = RumorKey::from(&member);
        // NOTE: This sucks so much right here. Check out how we allocate no matter what, because
        // of just how the logic goes. The value of the trace is really high, though, so we suck it
        // for now.
        let trace_member_id = String::from(member.get_id());
        let trace_incarnation = member.get_incarnation();
        let trace_health = health.clone();
        if self.member_list.insert_health(member, health) {
            trace_it!(MEMBERSHIP: self,
                      TraceKind::MemberUpdate,
                      trace_member_id,
                      trace_incarnation,
                      trace_health);
            self.rumor_list.insert(rk);
        }
    }

    /// Insert members from a list of received rumors.
    pub fn insert_from_rumors(&self, members: Vec<(Member, Health)>) {
        for (member, mut health) in members.into_iter() {
            let mut incremented_incarnation = false;
            let rk: RumorKey = RumorKey::from(&member);
            if member.get_id() == self.member_id() {
                if health != Health::Alive {
                    let mut me = self.member.write().expect("Member lock is poisoned");
                    let mut incarnation = me.get_incarnation();
                    incarnation += 1;
                    me.set_incarnation(incarnation);
                    health = Health::Alive;
                    incremented_incarnation = true;
                }
            }
            // NOTE: This sucks so much right here. Check out how we allocate no matter what, because
            // of just how the logic goes. The value of the trace is really high, though, so we suck it
            // for now.
            let trace_member_id = String::from(member.get_id());
            let trace_incarnation = member.get_incarnation();
            let trace_health = health.clone();

            if self.member_list.insert(member, health) || incremented_incarnation {
                trace_it!(MEMBERSHIP: self,
                          TraceKind::MemberUpdate,
                          trace_member_id,
                          trace_incarnation,
                          trace_health);
                self.rumor_list.insert(rk);
            }
        }
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}@{}/{}",
               self.name(),
               self.swim_port(),
               self.gossip_port())
    }
}

#[cfg(test)]
mod tests {
    mod server {
        use server::Server;
        use server::timing::Timing;
        use member::Member;
        use trace::Trace;
        use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

        static SWIM_PORT: AtomicUsize = ATOMIC_USIZE_INIT;
        static GOSSIP_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

        fn start_server() -> Server {
            SWIM_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
            GOSSIP_PORT.compare_and_swap(0, 7777, Ordering::Relaxed);
            let swim_port = SWIM_PORT.fetch_add(1, Ordering::Relaxed);
            let swim_listen = format!("127.0.0.1:{}", swim_port);
            let gossip_port = GOSSIP_PORT.fetch_add(1, Ordering::Relaxed);
            let gossip_listen = format!("127.0.0.1:{}", gossip_port);
            let mut member = Member::new();
            member.set_swim_port(swim_port as i32);
            member.set_gossip_port(gossip_port as i32);
            Server::new(&swim_listen[..],
                        &gossip_listen[..],
                        member,
                        Trace::default(),
                        None)
                .unwrap()
        }

        #[test]
        fn new() {
            start_server();
        }

        #[test]
        fn start_listener() {
            let server = start_server();
            server.start(Timing::default()).expect("Server failed to start");
        }
    }
}
