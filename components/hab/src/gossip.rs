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

use hcore::crypto::SymKey;

use error::Result;

pub fn send_rumors_to_peers(peer_listeners: &Vec<String>,
                            ring_key: Option<&SymKey>,
                            rumor_list: &hab_gossip::RumorList)
                            -> Result<()> {
    let fail_after = 10;
    let mut count = 0;

    if peer_listeners.len() > 0 {
        while count < fail_after {
            if try_peers(peer_listeners, ring_key, rumor_list) {
                return Ok(());
            } else {
                count = count + 1;
                println!("Could not connect to any initial peers; attempt {} of {}.",
                         count,
                         fail_after);
            }
        }
    }
    Ok(())
}

fn try_peers(peer_listeners: &Vec<String>,
             ring_key: Option<&SymKey>,
             rumor_list: &hab_gossip::RumorList)
             -> bool {
    let mut initialized = false;
    for to in peer_listeners {
        println!("Joining peer: {}", to);
        let mut c = match hab_gossip::Client::new(&to[..], ring_key.clone()) {
            Ok(c) => c,
            Err(e) => {
                debug!("Error creating gossip client - {:?}", e);
                println!("Failed to create a gossip client for {}", to);
                continue;
            }
        };

        match c.inject(rumor_list.clone()) {
            Ok(_) => println!("Configuration applied to: {}", to),
            Err(e) => {
                println!("Failed to ping {:?}: {:?}", to, e);
                continue;
            }
        }
        initialized = true;
    }
    initialized
}

pub mod hab_gossip {
    //! **Note** that this is a temporary measure. In order to implement the `rumor inject`
    //! subcommand in a codebase independent of the Supervisor, it was quickest to copy a minimal
    //! implementation of the Gossip subsystem suitable enough to "blind inject" a particular
    //! message type and terminate.  The Supervisor Gossip code needs some further refactoring for
    //! generic and minimal reuse. Until then, enjoy Fletcher's minimal Gossip port - FIN

    use std::collections::HashMap;
    use std::error;
    use std::fmt;
    use std::net::ToSocketAddrs;
    use std::io;
    use std::result;

    use common;
    use common::gossip_file::GossipFile;
    use common::wire_message::WireMessage;
    use hcore::crypto::SymKey;
    use rustc_serialize::json;
    use utp::UtpSocket;
    use uuid::Uuid;

    /// The default port for the Gossip protocol
    pub static GOSSIP_DEFAULT_PORT: usize = 9634;

    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        HabitatCommon(common::Error),
        IO(io::Error),
        JsonEncode(json::EncoderError),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let msg = match *self {
                Error::HabitatCommon(ref e) => format!("{}", e),
                Error::IO(ref err) => format!("{}", err),
                Error::JsonEncode(ref e) => format!("JSON encoding error: {}", e),
            };
            write!(f, "{}", msg)
        }
    }

    impl error::Error for Error {
        fn description(&self) -> &str {
            match *self {
                Error::HabitatCommon(ref err) => err.description(),
                Error::IO(ref err) => err.description(),
                Error::JsonEncode(_) => "JSON encoding error",
            }
        }
    }

    impl From<common::Error> for Error {
        fn from(err: common::Error) -> Self {
            Error::HabitatCommon(err)
        }
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::IO(err)
        }
    }

    impl From<json::EncoderError> for Error {
        fn from(err: json::EncoderError) -> Self {
            Error::JsonEncode(err)
        }
    }

    /// A Gossip Client.
    pub struct Client<'a> {
        pub socket: UtpSocket,
        ring_key: Option<&'a SymKey>,
    }

    impl<'a> Client<'a> {
        /// Create a new client from anything that can become a `SocketAddr`.
        ///
        /// # Errors
        ///
        /// * If we cannot connect the UTP socket
        pub fn new<A: ToSocketAddrs>(dst: A, ring_key: Option<&'a SymKey>) -> Result<Client> {
            let socket = try!(UtpSocket::connect(dst));
            Ok(Client {
                socket: socket,
                ring_key: ring_key,
            })
        }

        pub fn inject(&mut self, rumors_for_remote: RumorList) -> Result<()> {
            try!(self.send_message(Protocol::Inject(rumors_for_remote)));
            Ok(())
        }

        /// Send a message.
        ///
        /// # Errors
        ///
        /// * We cannot encode the `Message`
        /// * We fail to send the encoded buffer to the remote
        pub fn send_message(&mut self, msg: Protocol) -> Result<()> {
            let encoded = {
                let wire_msg = match self.ring_key.as_ref() {
                    Some(key) => try!(WireMessage::encrypted(&msg, &key)),
                    None => try!(WireMessage::plain(&msg)),
                };
                try!(json::encode(&wire_msg))
            };
            debug!("Encoded message {:#?}", encoded);
            try!(self.socket.send_to(encoded.as_bytes()));
            debug!("Sent protocol: {:?}", msg);
            Ok(())
        }
    }

    /// Each member has a Uuid.
    pub type MemberId = Uuid;

    /// A Peer is a representation of a member; it tracks how to contact the member, and whether
    /// this request is actually being proxied during a PingReq operation.
    #[derive(Clone, Debug, RustcEncodable)]
    pub struct Peer {
        pub member_id: MemberId,
        pub listening_on: String,
        pub proxy_through: Option<String>,
        pub proxy_to: Option<String>,
    }

    /// The SWIM Protocol.
    #[derive(Debug, RustcEncodable)]
    pub enum Protocol {
        Inject(RumorList),
    }

    /// Rumors contain Messages as their payload, which are then processed by the correct internal
    /// sub-system.
    #[derive(Debug, RustcEncodable, Clone, PartialEq, Eq)]
    pub enum Message {
        GossipFile(GossipFile),
    }

    /// A UUID for Rumors. In practice, always matches the UUID of a message payload.
    pub type RumorId = Uuid;

    /// A Rumor, which contains a Message.
    #[derive(Debug, RustcEncodable, PartialEq, Eq, Clone)]
    pub struct Rumor {
        pub id: RumorId,
        pub payload: Message,
    }

    impl Rumor {
        /// Create a new rumor with a `Message::GossipFile` payload.
        pub fn gossip_file(cf: GossipFile) -> Rumor {
            Rumor {
                id: Uuid::new_v4(),
                payload: Message::GossipFile(cf),
            }
        }
    }

    /// A list of rumors, and their corresponding heat. Heat determines whether we need to share the
    /// rumor with a given Member
    #[derive(Clone, Debug, RustcEncodable)]
    pub struct RumorList {
        pub rumors: HashMap<RumorId, Rumor>,
        pub heat: HashMap<MemberId, HashMap<RumorId, usize>>,
    }

    impl RumorList {
        /// Create a new RumorList.
        pub fn new() -> RumorList {
            RumorList {
                rumors: HashMap::new(),
                heat: HashMap::new(),
            }
        }

        /// Add a new rumor to the list.
        pub fn add_rumor(&mut self, rumor: Rumor) {
            debug!("Adding rumor {:?}", rumor);
            self.reset_heat_for(&rumor.id);
            self.rumors.insert(rumor.id, rumor);
        }

        /// Resets the heat for a rumor.
        pub fn reset_heat_for(&mut self, rumor_id: &RumorId) {
            for (_member_id, mut rumor_heat) in self.heat.iter_mut() {
                debug!("Reset heat for {:?}", rumor_id);
                if rumor_heat.contains_key(rumor_id) {
                    let mut count = rumor_heat.get_mut(rumor_id).unwrap();
                    *count = 0;
                } else {
                    rumor_heat.insert(rumor_id.clone(), 0);
                }
            }
        }
    }
}
