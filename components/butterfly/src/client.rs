//! The Butterfly client library.
//!
//! This will connect to a given butterfly members `Pull` thread, and inject a rumor.

use habitat_core::{crypto::SymKey,
                   service::ServiceGroup};
use zmq;

use crate::{error::{Error,
                    Result},
            message,
            rumor::{departure::Departure,
                    service_config::ServiceConfig,
                    service_file::ServiceFile,
                    Rumor},
            ZMQ_CONTEXT};

/// Holds a ZMQ Push socket, and an optional ring encryption key.
pub struct Client {
    socket:   zmq::Socket,
    ring_key: Option<SymKey>,
}

impl Client {
    /// Connect this client to the address, and optionally encrypt the traffic.
    pub fn new(addr: &str, ring_key: Option<SymKey>) -> Result<Client> {
        let socket = (**ZMQ_CONTEXT).as_mut()
                                    .socket(zmq::PUSH)
                                    .expect("Failure to create the ZMQ push socket");
        socket.set_linger(-1)
              .expect("Failure to set the ZMQ push socket to not linger");
        socket.set_tcp_keepalive(0)
              .expect("Failure to set the ZMQ push socket to not use keepalive");
        socket.set_immediate(true)
              .expect("Failure to set the ZMQ push socket to immediate");
        socket.set_sndhwm(1000)
              .expect("Failure to set the ZMQ push socket hwm");
        socket.set_sndtimeo(500)
              .expect("Failure to set the ZMQ send timeout");
        let to_addr = format!("tcp://{}", addr);
        socket.connect(&to_addr).map_err(Error::ZmqConnectError)?;
        Ok(Client { socket, ring_key })
    }

    /// Create a departure notification and send it to the server.
    pub fn send_departure(&mut self, member_id: &str) -> Result<()> {
        let departure = Departure::new(member_id);
        self.send(&departure)
    }

    /// Create a service configuration and send it to the server.
    pub fn send_service_config(&mut self,
                               service_group: ServiceGroup,
                               incarnation: u64,
                               config: &[u8],
                               encrypted: bool)
                               -> Result<()> {
        let mut sc = ServiceConfig::new("butterflyclient", service_group, config.to_vec());
        sc.incarnation = incarnation;
        sc.encrypted = encrypted;
        self.send(&sc)
    }

    /// Create a service file and send it to the server.
    pub fn send_service_file<S>(&mut self,
                                service_group: ServiceGroup,
                                filename: S,
                                incarnation: u64,
                                body: &[u8],
                                encrypted: bool)
                                -> Result<()>
        where S: Into<String>
    {
        let mut sf = ServiceFile::new("butterflyclient", service_group, filename, body.to_vec());
        sf.incarnation = incarnation;
        sf.encrypted = encrypted;
        self.send(&sf)
    }

    /// Send any `Rumor` to the server.
    pub fn send<T>(&mut self, rumor: &T) -> Result<()>
        where T: Rumor
    {
        let bytes = rumor.write_to_bytes()?;
        let wire_msg = message::generate_wire(bytes, self.ring_key.as_ref())?;
        self.socket.send(&wire_msg, 0).map_err(Error::ZmqSendError)
    }
}
