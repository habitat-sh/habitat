// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! Butterfly is the [SWIM](http://www.cs.cornell.edu/projects/Quicksilver/public_pdfs/SWIM.pdf)
//! implementation for Habitat, along with a ZeroMQ based gossip protocol.
//!
//! It implements SWIM+Susp+Inf. It uses Newscast-style "heat" tracking to share membership rumors,
//! while trying to keep UDP packet sizes below 512 bytes. It has the following changes:
//!
//! 1. It uses a single membership rumor with internal logic for applying the rumors state, rather
//!    than sending differential event messages.
//! 1. If an "Alive" membership rumor is received with a higher incarnation, it takes precedent
//!    over "Confirmed" membership rumors.
//! 1. Members can be marked "persistent", which means that they will always be taken through the
//!    Probe cycle, regardless of their status. This allows networks to heal from partitions.
//!
//! The SWIM implementation has three working threads:
//!
//! 1. An inbound thread, handling receipt of SWIM messages.
//! 1. An outbound thread, which handles the Ping->PingReq cycle and protocol timing.
//! 1. An expire thread, which handles timing out suspected members.
//!
//! The Gossip implementation has two working threads:
//!
//! 1. A 'push' thread, which fans out to 5 members every second (or longer, if it takes longer
//!    than 1 second to send all the messages to all the members in the fan-out; no more frequently
//!    than one second).
//! 1. A 'pull' thread, which takes messages from any push source and applies them locally.
//!
//! Start exploring the code base by following the thread of execution in the `server` module.

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate byteorder;
extern crate bytes;
extern crate habitat_core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate mktemp;
#[cfg(test)]
extern crate tempdir;
extern crate time;
extern crate toml;
extern crate uuid;
extern crate zmq;

#[macro_use]
pub mod trace;
pub mod client;
pub mod error;
pub mod member;
pub mod message;
pub mod protocol;
pub mod rumor;
pub mod server;
pub mod swim;

use std::cell::UnsafeCell;

pub use server::Server;

lazy_static! {
    /// A threadsafe shared ZMQ context for consuming services.
    ///
    /// You probably want to use this context to create new ZMQ sockets unless you *do not* want to
    /// connect them together using an in-proc queue.
    pub static ref ZMQ_CONTEXT: Box<ServerContext> = {
        let ctx = ServerContext(UnsafeCell::new(zmq::Context::new()));
        Box::new(ctx)
    };
}

/// This is a wrapper to provide interior mutability of an underlying `zmq::Context` and allows
/// for sharing/sending of a `zmq::Context` between threads.
pub struct ServerContext(UnsafeCell<zmq::Context>);

impl ServerContext {
    pub fn as_mut(&self) -> &mut zmq::Context {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl Send for ServerContext {}
unsafe impl Sync for ServerContext {}
