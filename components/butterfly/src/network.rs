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

use std::fmt::Debug;
use std::marker::Send;
use std::net::SocketAddr;

use error::Result;

// TODO(krnowak): See a TODO about Debug for Network trait below.
/// A trait for types used for sending SWIM messages.
pub trait SwimSender: Send + Debug {
    /// Send a SWIM message (as bytes) to the given address. The
    /// returned value holds a number of bytes sent.
    fn send(&self, buf: &[u8], addr: SocketAddr) -> Result<usize>;
}

/// A trait for types used for receiving SWIM messages.
pub trait SwimReceiver: Send {
    /// Receive a SWIM message (as bytes) from the channel. The
    /// returned value holds the size and an address from where the
    /// bytes came.
    fn receive(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>;
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
    type SwimSender: SwimSender;
    type SwimReceiver: SwimReceiver;
    type GossipSender: GossipSender;
    type GossipReceiver: GossipReceiver;

    fn get_swim_addr(&self) -> SocketAddr;
    fn get_swim_port(&self) -> u16;
    fn get_swim_sender(&self) -> Result<Self::SwimSender>;
    fn get_swim_receiver(&self) -> Result<Self::SwimReceiver>;

    fn get_gossip_addr(&self) -> SocketAddr;
    fn get_gossip_port(&self) -> u16;
    fn get_gossip_sender(&self, addr: SocketAddr) -> Result<Self::GossipSender>;
    fn get_gossip_receiver(&self) -> Result<Self::GossipReceiver>;
}
