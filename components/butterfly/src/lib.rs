

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

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prometheus;
extern crate prost;
extern crate prost_derive;

#[macro_use]
extern crate serde_derive;

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

pub use crate::server::Server;
use std::cell::UnsafeCell;

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
    #[allow(clippy::mut_from_ref)]
    pub fn as_mut(&self) -> &mut zmq::Context { unsafe { &mut *self.0.get() } }
}

unsafe impl Send for ServerContext {}
unsafe impl Sync for ServerContext {}
