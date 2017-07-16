// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::cell::UnsafeCell;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

use core::os;
use zmq;

lazy_static! {
    /// A threadsafe shared ZMQ context for consuming services.
    ///
    /// You probably want to use this context to create new ZMQ sockets unless you *do not* want to
    /// connect them together using an in-proc queue.
    pub static ref DEFAULT_CONTEXT: Box<SocketContext> = {
        let ctx = SocketContext::new();
        Box::new(ctx)
    };
}

/// This is a wrapper to provide interior mutability of an underlying `zmq::Context` and allows
/// for sharing/sending of a `zmq::Context` between threads.
pub struct SocketContext(UnsafeCell<zmq::Context>);

impl SocketContext {
    pub fn new() -> Self {
        SocketContext(UnsafeCell::new(zmq::Context::new()))
    }

    pub fn as_mut(&self) -> &mut zmq::Context {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl Send for SocketContext {}
unsafe impl Sync for SocketContext {}

/// Convert types into stringy socket addresses for ZeroMQ
pub trait ToAddrString {
    fn to_addr_string(&self) -> String;
}

impl ToAddrString for SocketAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for SocketAddrV4 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for SocketAddrV6 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

/// Generates a Network Socket Identity used when connecting to a RouteSrv to identify where the
/// connecting service.
///
/// This should only be used on sockets connecting directly to a RouteSrv. Do not use this name
/// for inproc sockets.
pub fn srv_ident() -> String {
    let hostname = os::net::hostname().unwrap();
    let pid = os::process::current_pid();
    format!("{}@{}", pid, hostname)
}
