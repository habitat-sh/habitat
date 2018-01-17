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

use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::marker::Send;
use std::result::Result as StdResult;
use std::str::FromStr;

use error::Result;

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
