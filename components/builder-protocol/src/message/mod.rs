// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::hash::Hasher;

use protobuf;

use sharding::InstaId;

pub mod depotsrv;
pub mod jobsrv;
pub mod net;
pub mod routesrv;
pub mod sessionsrv;
pub mod vault;

#[derive(Debug)]
pub struct Message<'a, T: 'a + protobuf::Message>(&'a T);

impl<'a, T: 'a + protobuf::Message> Message<'a, T> {
    pub fn new(inner: &'a T) -> MessageBuilder<'a, T> {
        MessageBuilder::new(Message(inner))
    }

    pub fn protocol(&self) -> net::Protocol {
        match self.0.descriptor().full_name().rsplit(".").last() {
            Some("jobsrv") => net::Protocol::JobSrv,
            Some("net") => net::Protocol::Net,
            Some("routesrv") => net::Protocol::RouteSrv,
            Some("sessionsrv") => net::Protocol::SessionSrv,
            Some("vault") => net::Protocol::VaultSrv,
            Some(_) | None => {
                unreachable!("no protocol defined for message, name={}",
                             self.0.descriptor().full_name())
            }
        }
    }
}

#[derive(Debug)]
pub struct MessageBuilder<'a, T: 'a + protobuf::Message> {
    pub route_info: Option<net::RouteInfo>,
    msg: Message<'a, T>,
}

impl<'a, T: 'a + protobuf::Message> MessageBuilder<'a, T> {
    pub fn new(msg: Message<'a, T>) -> Self {
        MessageBuilder {
            msg: msg,
            route_info: None,
        }
    }

    pub fn routing(mut self, hash: Option<u64>) -> Self {
        let mut route_info = net::RouteInfo::new();
        route_info.set_protocol(self.msg.protocol());
        if let Some(h) = hash {
            route_info.set_hash(h);
        }
        self.route_info = Some(route_info);
        self
    }

    pub fn build(self) -> ::net::Msg {
        let mut msg = net::Msg::new();
        msg.set_body(self.msg.0.write_to_bytes().unwrap());
        msg.set_message_id(self.msg.0.descriptor().name().to_string());
        if let Some(route_info) = self.route_info {
            msg.set_route_info(route_info);
        }
        msg
    }
}

/// Defines a contract for protocol messages to be persisted to a datastore.
pub trait Persistable: protobuf::Message + protobuf::MessageStatic {
    /// Type of the primary key
    type Key: fmt::Display;

    /// Returns the value of the primary key.
    fn primary_key(&self) -> Self::Key;

    /// Sets the primary key to the given value.
    fn set_primary_key(&mut self, value: Self::Key) -> ();
}

/// Defines a contract for protocol messages to be routed through `RouteSrv`.
pub trait Routable: protobuf::Message {
    /// Type of the route key
    type H: RouteKey + fmt::Display;

    /// Return a `RouteKey` for `RouteSrv` to know which key's value to route on.
    ///
    /// If `Some(T)`, the message will be routed by hashing the value of the route key and modding
    /// it against the shard count. This is known as "randomly deterministic routing".
    ///
    /// If `None`, the message will be randomly routed to an available node.
    fn route_key(&self) -> Option<Self::H>;
}

/// Provides an interface for hashing the implementing type for `Routable` messages.
///
/// Some types contain "hints" that help `RouteSrv` to identify the destination shard for a
/// message. You can leverage this trait to take any hints into account. See the implementation of
/// this trait for `InstaId` for an example.
pub trait RouteKey {
    /// Hashes a route key providing a route hash.
    fn hash(&self, hasher: &mut Hasher) -> u64;
}

impl RouteKey for String {
    fn hash(&self, hasher: &mut Hasher) -> u64 {
        hasher.write(self.as_bytes());
        hasher.finish()
    }
}

impl RouteKey for InstaId {
    fn hash(&self, _hasher: &mut Hasher) -> u64 {
        self.shard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_protocol() {
        assert_eq!(Message(&jobsrv::Job::new()).protocol(),
                   net::Protocol::JobSrv);
        assert_eq!(Message(&net::Ping::new()).protocol(), net::Protocol::Net);
        assert_eq!(Message(&routesrv::Connect::new()).protocol(),
                   net::Protocol::RouteSrv);
        assert_eq!(Message(&sessionsrv::Session::new()).protocol(),
                   net::Protocol::SessionSrv);
        assert_eq!(Message(&vault::Origin::new()).protocol(),
                   net::Protocol::VaultSrv);
    }
}
