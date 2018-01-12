// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! The Butterfly network abstraction.
//!
//! The abstraction provides communication channels for sending SWIM
//! and gossip messages.

use std::cell::UnsafeCell;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Error as FmtError, Formatter};
use std::marker::Send;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use habitat_core::util::sys as core_sys;

use error::{Error, Result};

use zmq;

// We can get rid of this trait when constraining an associated type
// like "type Address: FromStr where <Self as FromStr>::Err: Debug;"
// is actually implemented.
pub trait MyFromStr: FromStr {
    type MyErr: StdError + From<<Self as FromStr>::Err>;

    fn create_from_str(raw: &str) -> StdResult<Self, Self::MyErr> {
        raw.parse().map_err(|e: Self::Err| e.into())
    }
}

pub trait Address: MyFromStr + Debug + Copy + Clone + Display + Send + Sync + PartialEq {}

pub trait AddressAndPort: MyFromStr + Copy + Clone + Debug + Display + Send + Sync {
    type Address: Address;

    fn new_from_address_and_port(addr: Self::Address, port: u16) -> Self;
    fn get_address(&self) -> Self::Address;
    fn get_port(&self) -> u16;
}

// TODO(krnowak): See a TODO about Debug for Network trait below.
/// A trait for types used for sending SWIM messages.
pub trait SwimSender<A: AddressAndPort>: Send + Debug {
    /// Send a SWIM message (as bytes) to the given address. The
    /// returned value holds a number of bytes sent.
    fn send(&self, buf: &[u8], addr: A) -> Result<usize>;
}

/// A trait for types used for receiving SWIM messages.
pub trait SwimReceiver<A: AddressAndPort>: Send {
    /// Receive a SWIM message (as bytes) from the channel. The
    /// returned value holds the size and an address from where the
    /// bytes came.
    fn receive(&self, buf: &mut [u8]) -> Result<(usize, A)>;
}

/// A trait for types used for sending gossip messages (rumors).
pub trait GossipSender {
    /// Send a rumor (as bytes).
    fn send(&self, buf: &[u8]) -> Result<()>;
}

/// A trait for types used for receiving gossip messages (rumors).
pub trait GossipReceiver {
    /// Receive a rumor (as bytes).
    fn receive(&self) -> Result<Vec<u8>>;
}

// TODO(krnowak): Not sure if this static lifetime specifier here is a
// correct thing to do. It is either here on in several other places
// where generic type N constrained to being an implementation of the
// Network trait is used (trace, expire, inbound, outbound and so
// on). I added it here, because Network is exclusively used by the
// butterfly component.
//
// Same for Debug - Network is used by the butterfly component only so
// I add it here to save me some typing.
/// A trait for types used to provide SWIM and gossip communication
/// channels.
pub trait Network: Send + Sync + Debug + 'static {
    type AddressAndPort: AddressAndPort;
    type SwimSender: SwimSender<Self::AddressAndPort>;
    type SwimReceiver: SwimReceiver<Self::AddressAndPort>;
    type GossipSender: GossipSender;
    type GossipReceiver: GossipReceiver;

    fn get_host_address(&self) -> Result<<Self::AddressAndPort as AddressAndPort>::Address>;
    fn get_swim_addr(&self) -> Self::AddressAndPort;
    fn create_swim_sender(&self) -> Result<Self::SwimSender>;
    fn create_swim_receiver(&self) -> Result<Self::SwimReceiver>;

    fn get_gossip_addr(&self) -> Self::AddressAndPort;
    fn create_gossip_sender(&self, addr: Self::AddressAndPort) -> Result<Self::GossipSender>;
    fn create_gossip_receiver(&self) -> Result<Self::GossipReceiver>;
}

pub type AddressAndPortForNetwork<N> = <N as Network>::AddressAndPort;
pub type AddressForNetwork<N> = <AddressAndPortForNetwork<N> as AddressAndPort>::Address;

impl MyFromStr for IpAddr {
    type MyErr = <Self as FromStr>::Err;
}

impl Address for IpAddr {}

impl MyFromStr for SocketAddr {
    type MyErr = <Self as FromStr>::Err;
}

impl AddressAndPort for SocketAddr {
    type Address = IpAddr;

    fn new_from_address_and_port(addr: IpAddr, port: u16) -> SocketAddr {
        SocketAddr::new(addr, port)
    }

    fn get_address(&self) -> IpAddr {
        self.ip()
    }

    fn get_port(&self) -> u16 {
        self.port()
    }
}

/// An implementation of the `SwimSender` and `SwimReceiver` traits
/// that uses UdpSocket.
#[derive(Debug)]
pub struct SwimUdpSocket {
    udp: UdpSocket,
}

impl SwimSender<SocketAddr> for SwimUdpSocket {
    fn send(&self, buf: &[u8], addr: SocketAddr) -> Result<usize> {
        self.udp
            .send_to(buf, addr)
            .map_err(|e| Error::SwimSendIOError(e))
    }
}

impl SwimReceiver<SocketAddr> for SwimUdpSocket {
    fn receive(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.udp
            .recv_from(buf)
            .map_err(|e| Error::SwimReceiveIOError(e))
    }
}

/// An implementation of the `GossipSender` and `GossipReceiver`
/// traits that uses `zmq::Socket`.
pub struct GossipZmqSocket {
    zmq: zmq::Socket,
}

impl GossipSender for GossipZmqSocket {
    fn send(&self, buf: &[u8]) -> Result<()> {
        self.zmq
            .send(buf, 0)
            .map_err(|e| Error::GossipSendError(e.description().to_owned()))
    }
}

impl GossipReceiver for GossipZmqSocket {
    fn receive(&self) -> Result<Vec<u8>> {
        self.zmq
            .recv_bytes(0)
            .map_err(|e| Error::GossipReceiveError(e.description().to_owned()))
    }
}

/// This is a wrapper to provide interior mutability of an underlying
/// `zmq::Context` and allows for sharing/sending of a `zmq::Context`
/// between threads.
struct ServerContext(UnsafeCell<zmq::Context>);

impl ServerContext {
    pub fn as_mut(&self) -> &mut zmq::Context {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl Send for ServerContext {}
unsafe impl Sync for ServerContext {}

/// An implementation of the `Network` trait that creates
/// `SwimUdpSocket` instances for SWIM communication, and
/// `GossipZmqSocket` instances for gossip communication.
pub struct RealNetwork {
    swim_addr: SocketAddr,
    gossip_addr: SocketAddr,
    push_socket_linger: i32,
    zmq_context: ServerContext,

    udp_socket: Arc<Mutex<Option<UdpSocket>>>,
}

impl RealNetwork {
    /// Create an instance of `RealNetwork` to be used by clients. It
    /// sets linger time for push zmq gossip sockets to be
    /// indefinite. Use this instance to get pull or push gossip
    /// sockets.
    ///
    /// For getting SWIM channels or gossip receivers, get an instance
    /// with `new_for_server`.
    pub fn new_for_client() -> Self {
        // The client only sends rumors through the gossip sender
        // socket, so we pass some throw away address as arguments for
        // SWIM socket and gossip receiver socket addresses.
        Self::new(Self::throw_away_addr(), Self::throw_away_addr(), -1)
    }

    /// Create an instance of `RealNetwork` to be used by servers.
    pub fn new_for_server(swim_addr: SocketAddr, gossip_addr: SocketAddr) -> Self {
        Self::new(swim_addr, gossip_addr, 1000)
    }

    fn new(swim_addr: SocketAddr, gossip_addr: SocketAddr, push_socket_linger: i32) -> Self {
        RealNetwork {
            swim_addr: swim_addr,
            gossip_addr: gossip_addr,
            push_socket_linger: push_socket_linger,
            zmq_context: ServerContext(UnsafeCell::new(zmq::Context::new())),
            udp_socket: Arc::new(Mutex::new(None)),
        }
    }

    fn throw_away_addr() -> SocketAddr {
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        SocketAddr::new(ip, 0)
    }

    fn context_mut(&self) -> &mut zmq::Context {
        self.zmq_context.as_mut()
    }

    fn create_udp_socket(addr: &SocketAddr) -> Result<UdpSocket> {
        let socket = UdpSocket::bind(addr).map_err(|e| Error::CannotBind(e))?;
        socket
            .set_read_timeout(Some(Duration::from_millis(1000)))
            .map_err(|e| {
                Error::SwimChannelSetupError(format!("Can't set up read timeout, {}", e))
            })?;
        socket
            .set_write_timeout(Some(Duration::from_millis(1000)))
            .map_err(|e| {
                Error::SwimChannelSetupError(format!("Can't set up write timeout, {}", e))
            })?;

        Ok(socket)
    }

    fn get_swim_socket(&self) -> Result<SwimUdpSocket> {
        let mut maybe_socket = self.udp_socket.lock().expect("udp socket lock poisoned");
        if let Some(ref socket) = *maybe_socket {
            let cloned_socket = Self::clone_udp_socket(socket)?;
            return Ok(SwimUdpSocket { udp: cloned_socket });
        }
        let new_socket = Self::create_udp_socket(&self.swim_addr)?;
        *maybe_socket = Some(Self::clone_udp_socket(&new_socket)?);

        Ok(SwimUdpSocket { udp: new_socket })
    }

    fn clone_udp_socket(socket: &UdpSocket) -> Result<UdpSocket> {
        socket
            .try_clone()
            .map_err(|e| Error::SwimChannelSetupError(format!("{}", e)))
    }
}

// Implementing Debug trait explicitly to avoid debug output for zmq_context
impl Debug for RealNetwork {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(
            f,
            "RealNetwork {{ swim_addr: {:?}, gossip_addr: {:?}, push_socket_linger: {:?}, \
             zmq_context: <skipped>, udp_socket: {:?} }}",
            self.swim_addr, self.gossip_addr, self.push_socket_linger, self.udp_socket,
        )
    }
}

impl Network for RealNetwork {
    type AddressAndPort = SocketAddr;
    type SwimSender = SwimUdpSocket;
    type SwimReceiver = SwimUdpSocket;
    type GossipReceiver = GossipZmqSocket;
    type GossipSender = GossipZmqSocket;

    fn get_host_address(&self) -> Result<IpAddr> {
        core_sys::ip().map_err(|e| e.into())
    }

    fn get_swim_addr(&self) -> SocketAddr {
        self.swim_addr
    }

    fn create_swim_sender(&self) -> Result<SwimUdpSocket> {
        self.get_swim_socket()
    }

    fn create_swim_receiver(&self) -> Result<SwimUdpSocket> {
        self.get_swim_socket()
    }

    fn get_gossip_addr(&self) -> SocketAddr {
        self.gossip_addr
    }

    fn create_gossip_sender(&self, addr: SocketAddr) -> Result<GossipZmqSocket> {
        let socket = self.context_mut().socket(zmq::PUSH).map_err(|e| {
            Error::GossipChannelSetupError(format!("Failed to create the ZMQ push socket: {}", e))
        })?;
        socket.set_linger(self.push_socket_linger).map_err(|e| {
            Error::GossipChannelSetupError(format!(
                "Failed to set the ZMQ push socket linger: {}",
                e
            ))
        })?;
        socket.set_tcp_keepalive(0).map_err(|e| {
            Error::GossipChannelSetupError(format!(
                "Failed to set the ZMQ push socket to not use keepalive: {}",
                e
            ))
        })?;
        socket.set_immediate(true).map_err(|e| {
            Error::GossipChannelSetupError(format!(
                "Failed to set the ZMQ push socket to immediate: {}",
                e
            ))
        })?;
        socket.set_sndhwm(1000).map_err(|e| {
            Error::GossipChannelSetupError(format!("Failed to set the ZMQ push socket hwm: {}", e))
        })?;
        socket.set_sndtimeo(500).map_err(|e| {
            Error::GossipChannelSetupError(format!(
                "Failed to set the ZMQ push socket send timeout: {}",
                e
            ))
        })?;
        socket.connect(&format!("tcp://{}", addr)).map_err(|e| {
            Error::GossipChannelSetupError(format!("Failed to connect to {:?}: {}", addr, e))
        })?;
        Ok(GossipZmqSocket { zmq: socket })
    }

    fn create_gossip_receiver(&self) -> Result<GossipZmqSocket> {
        let socket = self.context_mut().socket(zmq::PULL).map_err(|e| {
            Error::GossipChannelSetupError(format!("Failed to create the ZMQ pull socket: {}", e))
        })?;
        socket.set_linger(0).map_err(|e| {
            Error::GossipChannelSetupError(format!(
                "Failed to set the ZMQ pull socket to not linger: {}",
                e
            ))
        })?;
        socket.set_tcp_keepalive(0).map_err(|e| {
            Error::GossipChannelSetupError(format!(
                "Failed to set the ZMQ pull socket to not use keepalive: {}",
                e
            ))
        })?;
        socket
            .bind(&format!("tcp://{}", self.gossip_addr))
            .map_err(|e| {
                Error::GossipChannelSetupError(format!(
                    "Failed to bind the ZMQ pull socket to the port: {}",
                    e
                ))
            })?;
        Ok(GossipZmqSocket { zmq: socket })
    }
}
