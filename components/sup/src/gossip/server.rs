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

//! The Gossip Server.
//!
//! This module is the beating heart of the gossip system (get it? beating heart?). It has 3 major
//! components:
//!
//! * An inbound listener, which receives SWIM Protocol messages.
//! * An outbound distributor, which initiates outbound pings to members
//! * A failure detector, which tracks outbound connections and times them out

use threadpool::ThreadPool;

use std::thread;
use std::ops::Deref;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::net;

use common::gossip_file::GossipFileList;
use hcore::crypto::{default_cache_key_path, SymKey};
use hcore::service::ServiceGroup;
use utp::{UtpListener, UtpSocket};

use gossip::client::Client;
use gossip::member::{Member, MemberList, Health};
use gossip::rumor::{Peer, Protocol, Rumor, RumorList, Message};
use gossip::detector::Detector;
use election::ElectionList;
use census::{Census, CensusEntry, CensusList};
use error::Result;
use util;

static LOGKEY: &'static str = "GS";
/// How often do we send an outbound request, in milliseconds
static OUTBOUND_INTERVAL: u64 = 200;
/// How many outbound threads do we allow?
static OUTBOUND_MAX_THREADS: usize = 5;
/// How many inbound threads do we allow?
static INBOUND_MAX_THREADS: usize = 5;
/// The default port for the Gossip protocol
pub static GOSSIP_DEFAULT_PORT: usize = 9634;

/// A gossip server
pub struct Server {
    /// The port to listen for inbound connections on
    pub listen: String,
    /// Our list of members in the gossip ring
    pub member_list: Arc<RwLock<MemberList>>,
    /// Our list of rumors to share
    pub rumor_list: Arc<RwLock<RumorList>>,
    /// Our list of censuses
    pub census_list: Arc<RwLock<CensusList>>,
    /// The failure detector
    pub detector: Arc<RwLock<Detector>>,
    /// The list of elections
    pub election_list: Arc<RwLock<ElectionList>>,
    /// The list of gossip files
    pub gossip_file_list: Arc<RwLock<GossipFileList>>,
    /// Our 'peer' entry, used to generate SWIM protocol messages.
    pub peer: Peer,
    /// An optional ring key used to encrypt messages with peers
    ring_key: Arc<Option<SymKey>>,
}

impl Server {
    /// Creates a new Server. Creates our own entry in the census and membership lists, and writes
    /// a rumor that this server is alive.
    pub fn new(listen_ip: String,
               listen_port: u16,
               permanent: bool,
               ring_name_with_rev: Option<String>,
               service: String,
               group: String,
               organization: Option<String>,
               exposes: Option<Vec<String>>,
               port: Option<String>)
               -> Server {

        let hostname = util::sys::hostname().unwrap_or(String::from("unknown"));
        let listen = format!("{}:{}", listen_ip, listen_port);
        let peer_listen = listen.clone();
        let peer_listen2 = peer_listen.clone();

        let member = Member::new(hostname, listen_ip, peer_listen2, permanent);

        let service_group = format!("{}.{}", service, group);
        let mut ce = CensusEntry::new(service.clone(), group.clone(), member.id.clone());
        ce.exposes = exposes;
        ce.port = port;
        let my_id = member.id.clone();
        let leader_id = member.id.clone();
        outputln!("Supervisor {}", member);
        outputln!("Census {}", ce);

        let census_list = CensusList::new(Census::new(ce.clone()));

        let ring_key = match ring_name_with_rev {
            Some(rnwr) => Some(SymKey::get_pair_for(&rnwr, &default_cache_key_path(None)).unwrap()),
            None => None,
        };

        let server = Server {
            listen: listen,
            member_list: Arc::new(RwLock::new(MemberList::new(member.clone()))),
            rumor_list: Arc::new(RwLock::new(RumorList::new())),
            census_list: Arc::new(RwLock::new(census_list)),
            peer: Peer::new(my_id, peer_listen),
            detector: Arc::new(RwLock::new(Detector::new())),
            election_list: Arc::new(RwLock::new(ElectionList::new(service_group, leader_id))),
            gossip_file_list:
                Arc::new(RwLock::new(GossipFileList::new(ServiceGroup::new(service,
                                                                           group,
                                                                           organization)))),
            ring_key: Arc::new(ring_key),
        };

        // Write our Alive Rumor
        {
            let rumor = Rumor::member(member);
            let mut rl = server.rumor_list.write().unwrap();
            rl.add_rumor(rumor);
        }

        // Write our Census Entry Rumor
        {
            let rumor = Rumor::census_entry(ce);
            let mut rl = server.rumor_list.write().unwrap();
            rl.add_rumor(rumor);
        }

        server
    }

    /// Starts the inbound gossip listener.
    ///
    /// # Errors
    ///
    /// * If we cannot bind to the listener
    pub fn start_inbound(&self) -> Result<()> {
        outputln!("Starting inbound gossip listener");
        let key = self.ring_key.clone();
        let ml = self.member_list.clone();
        let rl = self.rumor_list.clone();
        let cl = self.census_list.clone();
        let my_peer = self.peer.clone();
        let detector = self.detector.clone();
        let el = self.election_list.clone();
        let gfl = self.gossip_file_list.clone();
        let listener = try!(UtpListener::bind(&self.listen[..]));
        let _t = thread::Builder::new()
            .name("inbound".to_string())
            .spawn(move || inbound(listener, key, my_peer, ml, rl, cl, detector, el, gfl));
        Ok(())
    }

    /// Starts the outbound gossip distributor.
    pub fn start_outbound(&self) {
        outputln!("Starting outbound gossip distributor");
        let key = self.ring_key.clone();
        let ml = self.member_list.clone();
        let rl = self.rumor_list.clone();
        let my_peer = self.peer.clone();
        let detector = self.detector.clone();
        let _t = thread::Builder::new()
            .name("outbound".to_string())
            .spawn(move || outbound(key, my_peer, ml, rl, detector));
    }

    /// Starts the failure detector.
    pub fn start_failure_detector(&self) {
        outputln!("Starting gossip failure detector");
        let key = self.ring_key.clone();
        let my_peer = self.peer.clone();
        let ml = self.member_list.clone();
        let rl = self.rumor_list.clone();
        let detector = self.detector.clone();
        let _t = thread::Builder::new()
            .name("failure_detector".to_string())
            .spawn(move || failure_detector(key, my_peer, ml, rl, detector));
    }

    /// Sends blocking SWIM requests to our initial gossip peers.
    ///
    /// # Errors
    ///
    /// * If we cannot contact any of the given peers after 10 attempts through the list.
    pub fn initial_peers(&self, peer_listeners: &[String]) -> Result<()> {

        let fail_after = 10;
        let mut count = 0;

        if peer_listeners.len() > 0 {
            while count < fail_after {
                if self.try_peers(peer_listeners) {
                    return Ok(());
                } else {
                    count = count + 1;
                    outputln!("Could not connect to any initial peers; attempt {} of {}.",
                              count,
                              fail_after);
                }
            }
        }
        Ok(())
    }

    fn try_peers(&self, peer_listeners: &[String]) -> bool {
        let mut initialized = false;
        for to in peer_listeners {
            outputln!("Joining gossip peer at {}", to);
            let mut c = match Client::new(&to[..], self.ring_key.deref().as_ref()) {
                Ok(c) => c,
                Err(e) => {
                    debug!("Error creating gossip client - {:?}", e);
                    outputln!("Failed to create a gossip client for {}", to);
                    continue;
                }
            };

            let ping_rumors = {
                let rl = self.rumor_list.read().unwrap();
                rl.clone()
            };

            match c.ping(self.peer.clone(), ping_rumors) {
                Ok(_) => {}
                Err(e) => {
                    outputln!("Failed to ping {:?}: {:?}", to, e);
                    continue;
                }
            }

            initialized = true;
        }
        initialized
    }
}

/// Listens for incoming UTP requests, and spawns a thread to handle each. Handles at most
/// INBOUND_MAX_THREADS concurrent requests.
///
/// New requests are handled by passing them to `receive`.
pub fn inbound(listener: UtpListener,
               ring_key: Arc<Option<SymKey>>,
               my_peer: Peer,
               member_list: Arc<RwLock<MemberList>>,
               rumor_list: Arc<RwLock<RumorList>>,
               census_list: Arc<RwLock<CensusList>>,
               detector: Arc<RwLock<Detector>>,
               election_list: Arc<RwLock<ElectionList>>,
               gossip_file_list: Arc<RwLock<GossipFileList>>) {
    let pool = ThreadPool::new(INBOUND_MAX_THREADS);
    for connection in listener.incoming() {
        loop {
            if pool.active_count() == pool.max_count() {
                info!("{} of {} inbound threads full; delaying this round",
                      pool.active_count(),
                      pool.max_count());
                continue;
            } else {
                break;
            }
        }
        match connection {
            Ok((socket, src)) => {
                debug!("Inbound connection from {:?}; {} of {} slots used",
                       src,
                       pool.active_count(),
                       pool.max_count());

                let key = ring_key.clone();
                let my_peer = my_peer.clone();
                let ml = member_list.clone();
                let rl = rumor_list.clone();
                let cl = census_list.clone();
                let d1 = detector.clone();
                let el = election_list.clone();
                let gfl = gossip_file_list.clone();

                pool.execute(move || {
                    receive(socket, src, key, my_peer, ml, rl, cl, d1, el, gfl)
                });
            }
            _ => {}
        }
    }
}

/// Receives a message from the inbound listener.
///
/// Tries to receive a protocol message on the socket we were passed, then handles it according to what part
/// of the SWIM protocol we received.
///
/// ## Ping(Peer, RumorList)
/// * Respond immediately with an Ack of hot rumors for the requesting Peer, or its Proxy.
/// * Processes any rumors in the RumorList; updating the RumorList and dispatching to the given
/// sub-module.
///
/// ## Ack(Peer, RumorList)
/// * If the Ack was for a proxy, send it along to the proxy member.
/// * Otherwise, process our Rumorlist; updating the RumorList and dispatching to the given
/// sub-module.
///
/// ## PingReq(Peer, RumorList)
/// * Create a connection to the requested Peer
/// * Forward along the RumorList to that Peer as a Proxy Ping.
fn receive(socket: UtpSocket,
           src: net::SocketAddr,
           ring_key: Arc<Option<SymKey>>,
           my_peer: Peer,
           member_list: Arc<RwLock<MemberList>>,
           rumor_list: Arc<RwLock<RumorList>>,
           census_list: Arc<RwLock<CensusList>>,
           detector: Arc<RwLock<Detector>>,
           election_list: Arc<RwLock<ElectionList>>,
           gossip_file_list: Arc<RwLock<GossipFileList>>) {
    let mut client = Client::from_socket(socket, ring_key.deref().as_ref());
    let msg = match client.recv_message() {
        Ok(msg) => msg,
        Err(e) => {
            debug!("Failed to receive a message: {:#?} {:#?}",
                   client.socket.peer_addr(),
                   e);
            return;
        }
    };

    debug!("#{:?} protocol {:?}", src, msg);

    match msg {
        Protocol::Ping(from_peer, remote_rumor_list) => {
            debug!("Ping from {:?}", from_peer);

            // Who are we responding to? The peer, or are we proxied through someone else?
            let respond_to = {
                if let Some(ref proxy_through) = from_peer.proxy_through {
                    debug!("Proxy ping for {:?}", from_peer);
                    proxy_through.clone()
                } else {
                    from_peer.listening_on.clone()
                }
            };

            // Create a client for that peer
            let mut c = match Client::new(&respond_to[..], ring_key.deref().as_ref()) {
                Ok(c) => c,
                Err(e) => {
                    debug!("Failed to create a gossip client for {:?}; aborting: {}",
                           from_peer,
                           e);
                    return;
                }
            };

            // Generate our Ack rumors
            let ack_rumors = {
                let rl = rumor_list.read().unwrap();
                rl.hot_rumors_for(&from_peer.member_id)
            };

            // If we're Ack-ing through a proxy, make sure our reply is correct
            let ack_peer = if from_peer.proxy_to.is_some() {
                let mut ack_peer = Peer::new(my_peer.member_id.clone(),
                                             my_peer.listening_on.clone());
                ack_peer.proxy_to = Some(from_peer.listening_on.clone());
                ack_peer
            } else {
                my_peer
            };

            // Send our Ack
            match c.ack(ack_peer, ack_rumors.clone()) {
                Ok(_) => {}
                Err(e) => debug!("Failed to send Ack to {:?}: {:?}", from_peer, e),
            }

            // Update the heat for the rumors we shared
            {
                let mut rl = rumor_list.write().unwrap();
                rl.update_heat_for(&from_peer.member_id, &ack_rumors);
            }

            // Update our rumors
            debug!("Updating rumors from {:#?}", from_peer);
            process_rumors(remote_rumor_list,
                           rumor_list,
                           member_list,
                           census_list,
                           election_list,
                           gossip_file_list);
        }
        Protocol::Ack(mut from_peer, remote_rumor_list) => {
            // If this is a proxy ack, forward the results on
            if from_peer.proxy_to.is_some() {
                debug!("Proxy Ack for {:?}", from_peer);
                let forward_to = from_peer.proxy_to.take().unwrap();
                let mut c = match Client::new(&forward_to[..], ring_key.deref().as_ref()) {
                    Ok(c) => c,
                    Err(e) => {
                        debug!("Failed to create a gossip client to forward for {:?}; aborting: \
                                {}",
                               from_peer,
                               e);
                        return;
                    }
                };
                from_peer.proxy_through = None;
                match c.ack(from_peer.clone(), remote_rumor_list) {
                    Ok(_) => {}
                    Err(e) => debug!("Failed to send Ack to {:?}: {:?}", from_peer, e),
                }
            } else {
                debug!("Ack from {:?}", from_peer);
                {
                    detector.write().unwrap().success(&from_peer.member_id);
                }
                // Update our rumors
                debug!("Updating rumors via ack from {:#?} ", from_peer);
                process_rumors(remote_rumor_list,
                               rumor_list,
                               member_list,
                               census_list,
                               election_list,
                               gossip_file_list);
            }
        }
        Protocol::PingReq(from_peer, remote_rumor_list) => {
            debug!("PingReq from {:?}", from_peer);
            let proxy_to = match from_peer.proxy_to {
                Some(ref proxy_to) => proxy_to.clone(),
                None => {
                    debug!("Bad proxy request, missing proxy_to {:?}", from_peer);
                    return;
                }
            };
            let mut c = match Client::new(&proxy_to[..], ring_key.deref().as_ref()) {
                Ok(c) => c,
                Err(e) => {
                    debug!("Failed to create a gossip connection for sending ping-req to {} for \
                            {}: {}",
                           proxy_to,
                           from_peer.member_id,
                           e);
                    return;
                }
            };
            match c.ping(from_peer.clone(), remote_rumor_list) {
                Ok(_) => {}
                Err(e) => {
                    debug!("Failed to proxy ping {:?}: {:?}", from_peer, e);
                    return;
                }
            }
        }
        Protocol::Inject(remote_rumor_list) => {
            debug!("Incoming rumor injection: {:?}", remote_rumor_list);
            process_rumors(remote_rumor_list,
                           rumor_list,
                           member_list,
                           census_list,
                           election_list,
                           gossip_file_list);
        }
    }
}

pub fn process_rumors(remote_rumors: RumorList,
                      rumor_list: Arc<RwLock<RumorList>>,
                      member_list: Arc<RwLock<MemberList>>,
                      census_list: Arc<RwLock<CensusList>>,
                      election_list: Arc<RwLock<ElectionList>>,
                      gossip_file_list: Arc<RwLock<GossipFileList>>) {
    for (id, remote_rumor) in remote_rumors.rumors.into_iter() {
        match remote_rumor.payload {
            Message::Member(m) => {
                debug!("Processing member {:#?}", m);
                let processed = {
                    let mut ml = member_list.write().unwrap();
                    ml.process(m)
                };
                if processed {
                    let member = {
                        let ml = member_list.read().unwrap();
                        ml.get(&id).unwrap().clone()
                    };

                    // The internals of the object might have changed, but not by
                    // replacement. Hence, we don't take the rumor as given - we have to go
                    // get the current member for the new rumor
                    {
                        let mut rl = rumor_list.write().unwrap();
                        rl.add_rumor(Rumor::member(member));
                    }
                }
            }
            Message::CensusEntry(ce) => {
                debug!("Processing Census Entry {:#?}", ce);
                let processed = {
                    let mut cl = census_list.write().unwrap();
                    cl.process(ce.clone())
                };
                if processed {
                    let mut rl = rumor_list.write().unwrap();
                    // If we changed, by definition we took the other side.
                    rl.add_rumor(Rumor::census_entry(ce));
                }
            }
            // We are processing - all we need to do is thread it through, and then make sure we
            // destroy the un-neccessary rumors
            Message::Election(election) => {
                debug!("Processing Election {}", election);
                let processed = {
                    let mut el = election_list.write().unwrap();
                    el.process(election.clone())
                };
                if processed {
                    debug!("We processed Election {}", election);

                    let elector = {
                        let el = election_list.read().unwrap();
                        el.get(&election.service_group()).unwrap().clone()
                    };

                    let mut rl = rumor_list.write().unwrap();
                    rl.prune_elections_for(&elector.service_group());
                    rl.add_rumor(Rumor::election(elector));
                }
            }
            Message::GossipFile(gossip_file) => {
                debug!("Processing Gossip File {}", gossip_file);
                let processed = {
                    let mut gf = gossip_file_list.write().unwrap();
                    gf.process(gossip_file.clone())
                };
                if processed {
                    let mut rl = rumor_list.write().unwrap();
                    rl.prune_gossip_files_for(&gossip_file);
                    rl.add_rumor(Rumor::gossip_file(gossip_file));
                }
            }
            Message::Blank => {}
        }
    }
}


/// The outbound distributor. Every OUTBOUND_INTERVAL in milliseconds, it spawns a new connection
/// to the next member.
///
/// Like inbound, it is backed by a thread pool - if we have more than OUTBOUND_MAX_THREADS running
/// at once, we delay the next outbound message until a thread is free.
pub fn outbound(ring_key: Arc<Option<SymKey>>,
                my_peer: Peer,
                member_list: Arc<RwLock<MemberList>>,
                rumor_list: Arc<RwLock<RumorList>>,
                detector: Arc<RwLock<Detector>>) {
    let pool = ThreadPool::new(OUTBOUND_MAX_THREADS);
    loop {
        // Pretty chimpy, but will work for now
        thread::sleep(Duration::from_millis(OUTBOUND_INTERVAL));

        if pool.active_count() == pool.max_count() {
            info!("{} of {} outbound threads full; delaying this round",
                  pool.active_count(),
                  pool.max_count());
            continue;
        }

        let member = {
            let mut ml = member_list.write().unwrap();
            match ml.next() {
                Some(member) => member.clone(),
                None => continue,
            }
        };

        // You can skip yourself, thanks.
        if my_peer.member_id == member.id {
            continue;
        }

        // You can skip dead members, unless they are permanent, or you are isolated
        let isolated = {
            member_list.read().unwrap().isolated(&my_peer.member_id)
        };
        if member.health == Health::Confirmed && !member.permanent && !isolated {
            continue;
        }

        let running_request = {
            detector.read().unwrap().exists(&member.id)
        };

        if !running_request {
            let key1 = ring_key.clone();
            let rl1 = rumor_list.clone();
            let ml1 = member_list.clone();
            let mp1 = my_peer.clone();
            let d1 = detector.clone();

            debug!("Sending ping to {:?}; {} of {} outbound slots",
                   member,
                   pool.active_count(),
                   pool.max_count());
            pool.execute(move || send_outbound(key1, mp1, member, rl1, ml1, d1));
        } else {
            debug!("Skipping ping of {} due to already running request",
                   member.id)
        }
    }
}

/// Send an outbound Ping. If we fail to send, we initiate a PingReq.
pub fn send_outbound(ring_key: Arc<Option<SymKey>>,
                     my_peer: Peer,
                     member: Member,
                     rumor_list: Arc<RwLock<RumorList>>,
                     member_list: Arc<RwLock<MemberList>>,
                     detector: Arc<RwLock<Detector>>) {
    {
        let mut d = detector.write().unwrap();
        d.start(member.id.clone());
    }

    let mut c = match Client::new(&member.gossip_listener[..], ring_key.deref().as_ref()) {
        Ok(c) => c,
        Err(e) => {
            debug!("Failed to create a gossip connection for {}; sending ping-req: {}",
                   member.id,
                   e);
            send_pingreq(ring_key.clone(),
                         my_peer,
                         member,
                         rumor_list,
                         member_list,
                         detector);
            return;
        }
    };

    let ping_rumors = {
        let rl = rumor_list.read().unwrap();
        rl.hot_rumors_for(&member.id)
    };

    debug!("Preparing hot rumors for {:?}: {:#?}", member, ping_rumors);

    {
        detector.write().unwrap().awaiting_ack(&member.id);
    }

    match c.ping(my_peer.clone(), ping_rumors.clone()) {
        Ok(_) => {}
        Err(e) => {
            debug!("Failed to ping {:?}: {:?}", my_peer, e);
            send_pingreq(ring_key.clone(),
                         my_peer,
                         member,
                         rumor_list,
                         member_list,
                         detector);
            return;
        }
    }

    {
        let mut rl = rumor_list.write().unwrap();
        rl.update_heat_for(&member.id, &ping_rumors);
    }
}

/// Send a PingReq for a failed Ping. We pick targets from the Member List, and then send a PingReq
/// to each of them, with our information filled in.
pub fn send_pingreq(ring_key: Arc<Option<SymKey>>,
                    my_peer: Peer,
                    member: Member,
                    rumor_list: Arc<RwLock<RumorList>>,
                    member_list: Arc<RwLock<MemberList>>,
                    detector: Arc<RwLock<Detector>>) {
    {
        detector.write().unwrap().pingreq(&member.id);
    }

    let ping_rumors = {
        let rl = rumor_list.read().unwrap();
        rl.hot_rumors_for(&member.id)
    };

    let pingreq_targets = {
        let ml = member_list.read().unwrap();
        ml.pingreq_targets(&my_peer.member_id, &member.id)
    };

    for pingreq_member in pingreq_targets.into_iter() {
        debug!("Sending pingreq to {} through {}",
               member.id,
               pingreq_member.id);
        let mut c = match Client::new(&pingreq_member.gossip_listener[..],
                                      ring_key.deref().as_ref()) {
            Ok(c) => c,
            Err(e) => {
                debug!("Failed to create a gossip connection for {}; aborting ping-req request: \
                        {}",
                       pingreq_member.id,
                       e);
                continue;
            }
        };

        let mut proxy_peer = my_peer.clone();
        proxy_peer.proxy_through = Some(pingreq_member.gossip_listener.clone());
        proxy_peer.proxy_to = Some(member.gossip_listener.clone());

        match c.pingreq(proxy_peer, ping_rumors.clone()) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to pingreq {:?}: {:?}; aborting ping-req request",
                       my_peer,
                       e);
            }
        }
    }
}

/// The failure detector. Every 100ms, we check for any failed for confirmed timeouts within the
/// detector. If we find a timeout, we update our rumor and the members entry. Additionally, if we
/// mark a member as Suspect through a rumor we were passed, we set up its entry in the detector.
pub fn failure_detector(ring_key: Arc<Option<SymKey>>,
                        my_peer: Peer,
                        member_list: Arc<RwLock<MemberList>>,
                        rumor_list: Arc<RwLock<RumorList>>,
                        detector: Arc<RwLock<Detector>>) {
    loop {
        // Get a list of all our suspected and confirmed members
        let (pingreq, failed, confirmed) = {
            let mut fd = detector.write().unwrap();
            fd.expire()
        };

        // For each failed member, mark it as suspect and update
        for member_id in failed.iter() {
            {
                let mut ml = member_list.write().unwrap();
                ml.suspect(member_id);
            }
            let suspect_member = {
                let ml = member_list.read().unwrap();
                ml.get(&member_id).unwrap().clone()
            };
            {
                let mut rl = rumor_list.write().unwrap();
                rl.update_rumor(Rumor::member(suspect_member));
            }
        }

        // For each suspect confirmed failed, confirm it!
        for member_id in confirmed.iter() {
            {
                let mut ml = member_list.write().unwrap();
                ml.confirm(member_id);
            }
            let confirmed_member = {
                let ml = member_list.read().unwrap();
                ml.get(&member_id).unwrap().clone()
            };
            {
                let mut rl = rumor_list.write().unwrap();
                rl.update_rumor(Rumor::member(confirmed_member));
            }
        }

        // Account for any suspects who come to us via rumors
        {
            let ml = member_list.read().unwrap();
            for member_id in ml.suspect_members() {
                let member_detected = {
                    let fd = detector.read().unwrap();
                    fd.exists(member_id)
                };
                if !member_detected {
                    let mut fd = detector.write().unwrap();
                    fd.start(member_id.clone());
                    fd.failed(&member_id);
                }
            }
        }

        // For each pingreq target, send the pingreq!
        for member_id in pingreq.iter() {
            let ml = member_list.read().unwrap();
            let member = ml.get(&member_id).unwrap().clone();
            send_pingreq(ring_key.clone(),
                         my_peer.clone(),
                         member,
                         rumor_list.clone(),
                         member_list.clone(),
                         detector.clone());
        }

        thread::sleep(Duration::from_millis(100));
    }
}
