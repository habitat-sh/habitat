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

use std::collections::HashSet;
use std::clone::Clone;
use std::fmt;
use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

use error::{Result, Error};
use member::{Member, Health, MemberList};
use trace::Trace;
use rumor::{RumorList, RumorKey};

/// The server struct. Is thread-safe.
#[derive(Debug)]
pub struct Server {
    pub name: Arc<RwLock<String>>,
    pub socket: UdpSocket,
    pub member: Arc<RwLock<Member>>,
    pub member_list: MemberList,
    pub rumor_list: RumorList,
    // These are all here for testing support
    pub pause: Arc<AtomicBool>,
    pub trace: Arc<RwLock<Trace>>,
    pub rounds: Arc<AtomicIsize>,
    pub blacklist: Arc<RwLock<HashSet<SocketAddr>>>,
}


impl Server {
    /// Create a new server, bound to the `addr`, hosting a particular `member`, and with a
    /// `Trace` struct.
    ///
    /// # Errors
    ///
    /// * Returns `Error::CannotBind` if the socket cannot be bound
    /// * Returns `Error::SocketSetReadTimeout` if the socket read timeout cannot be set
    /// * Returns `Error::SocketSetWriteTimeout` if the socket write timeout cannot be set
    pub fn new<A: ToSocketAddrs>(addr: A, member: Member, trace: Trace) -> Result<Server> {
        let socket = match UdpSocket::bind(addr) {
            Ok(socket) => socket,
            Err(e) => return Err(Error::CannotBind(e)),
        };
        try!(socket.set_read_timeout(Some(Duration::from_millis(1000))).map_err(|e| Error::SocketSetReadTimeout(e)));
        try!(socket.set_write_timeout(Some(Duration::from_millis(1000))).map_err(|e| Error::SocketSetReadTimeout(e)));
        Ok(Server {
            name: Arc::new(RwLock::new(String::from(member.get_id()))),
            socket: socket,
            member: Arc::new(RwLock::new(member)),
            member_list: MemberList::new(),
            rumor_list: RumorList::default(),
            pause: Arc::new(AtomicBool::new(false)),
            trace: Arc::new(RwLock::new(trace)),
            rounds: Arc::new(AtomicIsize::new(0)),
            blacklist: Arc::new(RwLock::new(HashSet::new())),
        })
    }

    fn try_clone(&self) -> Result<Server> {
        let socket = match self.socket.try_clone() {
            Ok(socket) => socket,
            Err(e) => {
                error!("Failed to clone socket; trying again: {:?}", e);
                return Err(Error::ServerCloneError);
            }
        };
        Ok(Server {
            name: self.name.clone(),
            socket: socket,
            member: self.member.clone(),
            member_list: self.member_list.clone(),
            rumor_list: self.rumor_list.clone(),
            pause: self.pause.clone(),
            trace: self.trace.clone(),
            rounds: self.rounds.clone(),
            blacklist: self.blacklist.clone(),
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
    pub fn start(&self, timing: outbound::Timing) -> Result<()> {
        let (tx_outbound, rx_inbound) = channel();

        let server_a = try!(self.try_clone());
        let server_b = try!(self.try_clone());
        let server_c = try!(self.try_clone());
        let timing_b = timing.clone();

        let _ = thread::Builder::new().name("inbound".to_string()).spawn(move || {
            inbound::Inbound::new(&server_a, tx_outbound).run();
            panic!("You should never, ever get here, judy");
        });
        let _ = thread::Builder::new().name("outbound".to_string()).spawn(move || {
            outbound::Outbound::new(&server_b, rx_inbound, timing).run();
            panic!("You should never, ever get here, bob");
        });
        let _ = thread::Builder::new().name("expire".to_string()).spawn(move || {
            expire::Expire::new(&server_c, timing_b).run();
            panic!("You should never, ever get here, bob");
        });
        Ok(())
    }

    /// Blacklist a given address, causing no traffic to be seen.
    pub fn add_to_blacklist(&self, addr: SocketAddr) {
        let mut blacklist = self.blacklist.write().expect("Write lock for blacklist is poisoned");
        blacklist.insert(addr);
    }

    /// Remove a given address from the blacklist.
    pub fn remove_from_blacklist(&self, addr: &SocketAddr) {
        let mut blacklist = self.blacklist.write().expect("Write lock for blacklist is poisoned");
        blacklist.remove(addr);
    }

    /// Check that a given address is on the blacklist.
    pub fn check_blacklist(&self, addr: &SocketAddr) -> bool {
        let blacklist = self.blacklist.write().expect("Write lock for blacklist is poisoned");
        blacklist.contains(addr)
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

    /// Return the port number of the socket we are bound to.
    pub fn port(&self) -> u16 {
        self.socket
            .local_addr()
            .expect("Cannot get the port number; this socket is very bad")
            .port()
    }

    /// Return the name of this server.
    pub fn name(&self) -> String {
        self.name.read().expect("Server name lock is poisoned").clone()
    }

    /// Insert a member to the `MemberList`, and update its `RumorKey` appropriately.
    pub fn insert_member(&self, member: Member, health: Health) {
        let rk: RumorKey = RumorKey::from(&member);
        if self.member_list.insert(member, health) {
            self.rumor_list.insert(rk);
        }
    }

    /// Change the helth of a `Member`, and update its `RumorKey`.
    pub fn insert_health(&self, member: &Member, health: Health) {
        let rk: RumorKey = RumorKey::from(&member);
        if self.member_list.insert_health(member, health) {
            self.rumor_list.insert(rk);
        }
    }

    /// Insert members from a list of received rumors.
    pub fn insert_from_rumors(&self, members: Vec<(Member, Health)>) {
        let mut me = self.member.write().expect("Member lock is poisoned");
        for (member, mut health) in members.into_iter() {
            let mut incremented_incarnation = false;
            let rk: RumorKey = RumorKey::from(&member);
            if member.get_id() == me.get_id() {
                if health != Health::Alive {
                    let mut incarnation = me.get_incarnation();
                    incarnation += 1;
                    me.set_incarnation(incarnation);
                    health = Health::Alive;
                    incremented_incarnation = true;
                }
            }
            if self.member_list.insert(member, health) || incremented_incarnation {
                self.rumor_list.insert(rk);
            }
        }
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.name(), self.port())
    }
}

#[cfg(test)]
mod tests {
    mod server {
        use server::Server;
        use server::outbound::Timing;
        use member::Member;
        use trace::Trace;
        use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

        static SERVER_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

        fn start_server() -> Server {
            SERVER_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
            let my_port = SERVER_PORT.fetch_add(1, Ordering::Relaxed);
            let listen = format!("127.0.0.1:{}", my_port);
            Server::new(&listen[..], Member::new(), Trace::default()).unwrap()
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
