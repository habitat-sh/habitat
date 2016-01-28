// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The Gossip Server.
//!
//! This module is the beating heart of the gossip system (get it? beating heart?). It has 3 major
//! components:
//!
//! * An inbound listener, which receives SWIM Protocol messages.
//! * An outbound distributor, which initiates outbound pings to members
//! * A failure detector, which tracks outbound connections and times them out

use threadpool::ThreadPool;

use std::net::SocketAddr;
use std::thread;
use std::sync::{Arc, RwLock};
use std::net;

use utp::{UtpListener, UtpSocket};

use census::{Census, CensusEntry};
use gossip::client::Client;
use gossip::member::{Member, MemberList, Health};
use gossip::rumor::{Peer, Protocol, Rumor, RumorList};
use gossip::detector::Detector;
use error::BldrResult;

static LOGKEY: &'static str = "GS";
/// How often do we send an outbound request, in milliseconds
static OUTBOUND_INTERVAL: u32 = 200;
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
    /// Our list of census entries
    pub census: Arc<RwLock<Census>>,
    /// The failure detector
    pub detector: Arc<RwLock<Detector>>,
    /// Our 'peer' entry, used to generate SWIM protocol messages.
    pub peer: Peer,
}

impl Server {
    /// Creates a new Server. Creates our own entry in the census and membership lists, and writes
    /// a rumor that this server is alive.
    pub fn new(listen: String, permanent: bool) -> Server {
        let ce = CensusEntry::new();
        let my_id = ce.id.clone();
        outputln!("Supervisor {}", ce.id);
        let peer_listen = format!("{}:{}", ce.ip, GOSSIP_DEFAULT_PORT);
        let peer_listen2 = peer_listen.clone();

        let server = Server {
            listen: listen,
            member_list: Arc::new(RwLock::new(MemberList::new())),
            rumor_list: Arc::new(RwLock::new(RumorList::new())),
            census: Arc::new(RwLock::new(Census::new(ce))),
            peer: Peer::new(my_id, peer_listen),
            detector: Arc::new(RwLock::new(Detector::new())),
        };

        // Create our member entry from the census
        let member = {
            let census = server.census.read().unwrap();
            Member::new(census.me().unwrap().id.clone(),
                        census.me().unwrap().hostname.clone(),
                        census.me().unwrap().ip.clone(),
                        peer_listen2,
                        permanent.clone())
        };

        // Write our Alive Rumor
        {
            let rumor = Rumor::member(member.clone());
            let mut rl = server.rumor_list.write().unwrap();
            rl.add_rumor(member.id.clone(), rumor);
        }

        // Add ourselves to the Membership list
        {
            let mut ml = server.member_list.write().unwrap();
            ml.insert(member);
        }

        server
    }

    /// Starts the inbound gossip listener.
    ///
    /// # Errors
    ///
    /// * If we cannot bind to the listener
    pub fn start_inbound(&self) -> BldrResult<()> {
        outputln!("Starting inbound gossip listener");
        let ml = self.member_list.clone();
        let rl = self.rumor_list.clone();
        let census = self.census.clone();
        let my_peer = self.peer.clone();
        let detector = self.detector.clone();
        let listener = try!(UtpListener::bind(&self.listen[..]));
        thread::spawn(move || inbound(listener, my_peer, ml, rl, census, detector));
        Ok(())
    }

    /// Starts the outbound gossip distributor.
    pub fn start_outbound(&self) {
        outputln!("Starting outbound gossip distributor");
        let ml = self.member_list.clone();
        let rl = self.rumor_list.clone();
        let my_peer = self.peer.clone();
        let detector = self.detector.clone();
        thread::spawn(move || outbound(my_peer, ml, rl, detector));
    }

    /// Starts the failure detector.
    pub fn start_failure_detector(&self) {
        outputln!("Starting gossip failure detector");
        let ml = self.member_list.clone();
        let rl = self.rumor_list.clone();
        let census = self.census.clone();
        let detector = self.detector.clone();
        thread::spawn(move || failure_detector(ml, rl, census, detector));
    }

    /// Sends blocking SWIM requests to our initial gossip peers.
    ///
    /// # Errors
    ///
    /// * If we cannot contact any of the given peers after 10 attempts through the list.
    pub fn initial_peers(&self, peer_listeners: &[String]) -> BldrResult<()> {

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
            let mut c = match Client::new(&to[..]) {
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
               my_peer: Peer,
               member_list: Arc<RwLock<MemberList>>,
               rumor_list: Arc<RwLock<RumorList>>,
               census: Arc<RwLock<Census>>,
               detector: Arc<RwLock<Detector>>) {
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

                let my_peer = my_peer.clone();
                let ml = member_list.clone();
                let rl = rumor_list.clone();
                let census = census.clone();
                let d1 = detector.clone();

                pool.execute(move || receive(socket, src, my_peer, ml, rl, census, d1));
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
           my_peer: Peer,
           member_list: Arc<RwLock<MemberList>>,
           rumor_list: Arc<RwLock<RumorList>>,
           _census: Arc<RwLock<Census>>,
           detector: Arc<RwLock<Detector>>) {
    let mut client = Client::from_socket(socket);
    let msg = match client.recv_message() {
        Ok(msg) => msg,
        Err(e) => {
            debug!("Failed to receive a message: {:#?} {:#?}",
                   client.socket.peer_addr(),
                   e);
            return;
        }
    };

    let mp = my_peer.clone();

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
            let mut c = match Client::new(&respond_to[..]) {
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
            {
                let mut rl = rumor_list.write().unwrap();
                warn!("Rumors from {:#?}", from_peer);
                rl.process_rumors(&mp.member_id, remote_rumor_list, member_list);
            }
        }
        Protocol::Ack(mut from_peer, remote_rumor_list) => {
            // If this is a proxy ack, forward the results on
            if from_peer.proxy_to.is_some() {
                debug!("Proxy Ack for {:?}", from_peer);
                let forward_to = from_peer.proxy_to.take().unwrap();
                let mut c = match Client::new(&forward_to[..]) {
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
                {
                    let mut rl = rumor_list.write().unwrap();
                    warn!("Rumors from {:#?}", from_peer);
                    rl.process_rumors(&mp.member_id, remote_rumor_list, member_list);
                }
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
            let mut c = match Client::new(&proxy_to[..]) {
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
    }
}

/// The outbound distributor. Every OUTBOUND_INTERVAL in milliseconds, it spawns a new connection
/// to the next member.
///
/// Like inbound, it is backed by a thread pool - if we have more than OUTBOUND_MAX_THREADS running
/// at once, we delay the next outbound message until a thread is free.
pub fn outbound(my_peer: Peer,
                member_list: Arc<RwLock<MemberList>>,
                rumor_list: Arc<RwLock<RumorList>>,
                detector: Arc<RwLock<Detector>>) {
    let pool = ThreadPool::new(OUTBOUND_MAX_THREADS);
    loop {
        // Pretty chimpy, but will work for now
        thread::sleep_ms(OUTBOUND_INTERVAL);

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

        // You can skip dead members, unless they are permanent
        if member.health == Health::Confirmed && !member.permanent {
            continue;
        }

        let running_request = {
            detector.read().unwrap().exists(&member.id)
        };

        if !running_request {
            let rl1 = rumor_list.clone();
            let ml1 = member_list.clone();
            let mp1 = my_peer.clone();
            let d1 = detector.clone();

            debug!("Sending ping to {:?}; {} of {} outbound slots",
                   member,
                   pool.active_count(),
                   pool.max_count());
            pool.execute(move || send_outbound(mp1, member, rl1, ml1, d1));
        } else {
            debug!("Skipping ping of {} due to already running request",
                   member.id)
        }
    }
}

/// Send an outbound Ping. If we fail to send, we initiate a PingReq.
pub fn send_outbound(my_peer: Peer,
                     member: Member,
                     rumor_list: Arc<RwLock<RumorList>>,
                     member_list: Arc<RwLock<MemberList>>,
                     detector: Arc<RwLock<Detector>>) {
    {
        let mut d = detector.write().unwrap();
        d.start(member.id.clone());
    }

    let mut c = match Client::new(&member.gossip_listener[..]) {
        Ok(c) => c,
        Err(e) => {
            debug!("Failed to create a gossip connection for {}; sending ping-req: {}",
                   member.id,
                   e);
            send_pingreq(my_peer, member, rumor_list, member_list, detector);
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
            send_pingreq(my_peer, member, rumor_list, member_list, detector);
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
pub fn send_pingreq(my_peer: Peer,
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
        let mut c = match Client::new(&pingreq_member.gossip_listener[..]) {
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
pub fn failure_detector(member_list: Arc<RwLock<MemberList>>,
                        rumor_list: Arc<RwLock<RumorList>>,
                        _census: Arc<RwLock<Census>>,
                        detector: Arc<RwLock<Detector>>) {
    loop {
        // Get a list of all our suspected and confirmed members
        let (failed, confirmed) = {
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
        thread::sleep_ms(100);
    }
}
