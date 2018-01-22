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

pub mod jobsrv;
pub mod routesrv;
pub mod sessionsrv;
pub mod originsrv;
mod net;

use std::borrow::Cow;
use std::fmt;
use std::hash::Hasher;
use std::str::FromStr;

use fnv::FnvHasher;
use protobuf::{self, Clear};

pub use self::net::{ErrCode, NetError, NetOk, Protocol};
use error::ProtocolError;
use sharding::InstaId;

const MAX_BODY_LEN: usize = (128 * 1024) * 8;
const MAX_IDENTITIES: usize = 10;

#[derive(Debug)]
pub struct Header(net::Header);

impl Header {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let inner = decode::<net::Header>(bytes)?;
        Ok(Header(inner))
    }

    pub fn message_id(&self) -> &str {
        self.0.get_message_id()
    }

    pub fn has_route_info(&self) -> bool {
        self.0.get_route_info()
    }

    pub fn has_txn(&self) -> bool {
        self.0.get_txn()
    }

    pub fn set_has_route_info(&mut self, value: bool) {
        self.0.set_route_info(value)
    }

    pub fn set_has_txn(&mut self, value: bool) {
        self.0.set_txn(value)
    }

    pub fn set_message_id<T>(&mut self, value: T)
    where
        T: ToString,
    {
        self.0.set_message_id(value.to_string())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        encode(&self.0)
    }
}

impl Clear for Header {
    fn clear(&mut self) {
        self.0.clear()
    }
}

impl Default for Header {
    fn default() -> Self {
        let mut header = net::Header::default();
        header.set_route_info(false);
        header.set_txn(false);
        Header(header)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "message-id={}, has-route-info={}, has-txn={}",
            self.message_id(),
            self.has_route_info(),
            self.has_txn()
        )
    }
}

#[derive(Debug)]
pub struct Message {
    /// Returns the binary representation of the body of the message.
    ///
    /// This can be parsed into a protocol message with `parse()`
    pub body: Vec<u8>,
    /// Ordered list of network identities of servers which have handled the message starting
    /// with the originator.
    pub identities: Vec<Vec<u8>>,
    /// Message buffer for `header` portion of a router message.
    header: Header,
    /// Message buffer for `route_info` portion of a router message.
    route_info: Option<RouteInfo>,
    /// Message buffer for `txn` portion of a router message.
    txn: Option<Txn>,
}

impl Message {
    pub fn build<T>(message: &T) -> Result<Self, ProtocolError>
    where
        T: Routable + protobuf::MessageStatic,
    {
        let mut request = Self::default();
        request.populate(message)?;
        Ok(request)
    }

    /// Returns true if the message is transactional and completed.
    pub fn completed_txn(&self) -> bool {
        if let Some(txn) = self.txn.as_ref() {
            return txn.is_complete();
        }
        false
    }

    pub fn populate<T>(&mut self, message: &T) -> Result<(), ProtocolError>
    where
        T: Routable + protobuf::Message,
    {
        self.reset();
        self.set_body(message)?;
        self.header.set_message_id(
            message.descriptor().name().to_string(),
        );
        self.header.set_has_txn(true);
        self.header.set_has_route_info(true);
        self.route_info = Some(RouteInfo::build(message));
        self.txn = Some(Txn::default());
        Ok(())
    }

    /// Populate a transactional protocol message with a reply.
    pub fn populate_reply<T>(&mut self, message: &T) -> Result<(), ProtocolError>
    where
        T: protobuf::Message,
    {
        self.txn_mut().ok_or(ProtocolError::NoTxn).and_then(|x| {
            Ok(x.set_complete(true))
        })?;
        self.set_body(message)?;
        self.header.set_message_id(
            message.descriptor().name().to_string(),
        );
        Ok(())
    }

    /// Clear all fields for message instance.
    ///
    /// Useful if you want to re-use the Message struct without allocating a new one.
    pub fn reset(&mut self) {
        self.identities.clear();
        self.header.clear();
        self.txn = None;
        self.route_info = None;
        self.body.clear();
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn message_id(&self) -> &str {
        self.header.message_id()
    }

    /// Returns the identity of the socket which initially generated this message. Nothing is
    /// returned if the message was not received from a socket thus having no originator.
    pub fn originator(&self) -> Option<&[u8]> {
        self.identities.first().map(Vec::as_slice)
    }

    /// Same as `originator()` but returns a lossy utf8 representation of the originators's
    /// identity.
    pub fn originator_str(&self) -> Option<Cow<str>> {
        self.originator().map(String::from_utf8_lossy)
    }

    pub fn parse<T>(&self) -> Result<T, ProtocolError>
    where
        T: protobuf::MessageStatic,
    {
        decode::<T>(&self.body)
    }

    pub fn route_info(&self) -> Option<&RouteInfo> {
        self.route_info.as_ref()
    }

    pub fn txn(&self) -> Option<&Txn> {
        self.txn.as_ref()
    }

    pub fn txn_mut(&mut self) -> Option<&mut Txn> {
        self.txn.as_mut()
    }

    /// Returns the identity of the socket which sent this message. Nothing is returned if the
    /// message was not received from a socket thus having no sender.
    pub fn sender(&self) -> Option<&[u8]> {
        self.identities.last().map(Vec::as_slice)
    }

    /// Same as `sender()` but returns a lossy utf8 representation of the sender's identity.
    pub fn sender_str(&self) -> Option<Cow<str>> {
        self.sender().map(String::from_utf8_lossy)
    }

    pub fn set_body<T>(&mut self, body: &T) -> Result<(), ProtocolError>
    where
        T: protobuf::Message,
    {
        self.body = encode::<T>(body)?;
        Ok(())
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    pub fn set_route_info(&mut self, route_info: RouteInfo) {
        self.route_info = Some(route_info);
        self.header.set_has_route_info(true);
    }

    pub fn set_txn(&mut self, txn: Txn) {
        self.txn = Some(txn);
        self.header.set_has_txn(true);
    }
}

impl Default for Message {
    fn default() -> Self {
        Message {
            body: Vec::with_capacity(MAX_BODY_LEN),
            identities: Vec::with_capacity(MAX_IDENTITIES),
            header: Header::default(),
            route_info: None,
            txn: None,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = format!("{}", self.header);
        if let Some(ref route_info) = self.route_info {
            msg.push_str(&format!(", {}", route_info));
        }
        if let Some(ref txn) = self.txn {
            msg.push_str(&format!(", {}", txn));
        }
        write!(f, "{}, body={:?}", msg, self.body)
    }
}

#[derive(Debug)]
pub struct RouteInfo(net::RouteInfo);

impl RouteInfo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let inner = decode::<net::RouteInfo>(bytes)?;
        Ok(RouteInfo(inner))
    }

    pub fn build<T>(message: &T) -> Self
    where
        T: Routable,
    {
        let mut route_info = net::RouteInfo::default();
        if let Some(key) = message.route_key() {
            let route_hash = key.hash(&mut FnvHasher::default());
            route_info.set_hash(route_hash);
        }
        route_info.set_protocol(T::protocol());
        RouteInfo(route_info)
    }

    pub fn protocol(&self) -> net::Protocol {
        self.0.get_protocol()
    }

    pub fn hash(&self) -> Option<u64> {
        if self.0.has_hash() {
            Some(self.0.get_hash())
        } else {
            None
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        encode(&self.0)
    }
}

impl Clear for RouteInfo {
    fn clear(&mut self) {
        self.0.clear()
    }
}

impl Default for RouteInfo {
    fn default() -> Self {
        let mut route_info = net::RouteInfo::default();
        route_info.set_protocol(net::Protocol::Net);
        RouteInfo(route_info)
    }
}

impl fmt::Display for RouteInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "protocol={}, hash={:?}",
            self.protocol(),
            self.hash(),
        )
    }
}

#[derive(Debug)]
pub struct Txn(net::Txn);

impl Txn {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let inner = decode::<net::Txn>(bytes)?;
        Ok(Txn(inner))
    }

    pub fn id(&self) -> u64 {
        self.0.get_id()
    }

    pub fn is_complete(&self) -> bool {
        self.0.get_complete()
    }

    pub fn set_complete(&mut self, value: bool) {
        self.0.set_complete(value);
    }

    pub fn set_id(&mut self, value: u64) {
        self.0.set_id(value);
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        encode(&self.0)
    }
}

impl Clear for Txn {
    fn clear(&mut self) {
        self.0.clear()
    }
}

impl Default for Txn {
    fn default() -> Self {
        let mut txn = net::Txn::default();
        txn.set_id(0);
        txn.set_complete(false);
        Txn(txn)
    }
}

impl fmt::Display for Txn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "txn-id={}, txn-complete={}",
            self.id(),
            self.is_complete(),
        )
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
pub trait Routable: protobuf::MessageStatic {
    /// Type of the route key
    type H: RouteKey + fmt::Display;

    fn protocol() -> net::Protocol {
        match Self::descriptor_static(None).full_name().rsplit(".").last() {
            Some(name) => {
                match net::Protocol::from_str(name) {
                    Ok(protocol) => protocol,
                    Err(err) => panic!("{}", err),
                }
            }
            None => {
                panic!(
                    "Malformed protobuf: unable to determine protocol, '{:?}'",
                    Self::descriptor_static(None).full_name()
                )
            }
        }
    }

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

impl RouteKey for u32 {
    fn hash(&self, _hasher: &mut Hasher) -> u64 {
        *self as u64
    }
}

impl RouteKey for u64 {
    fn hash(&self, _hasher: &mut Hasher) -> u64 {
        *self
    }
}

pub fn decode<T>(bytes: &[u8]) -> Result<T, ProtocolError>
where
    T: protobuf::MessageStatic,
{
    protobuf::parse_from_bytes::<T>(bytes).map_err(ProtocolError::Decode)
}

pub fn encode<T>(message: &T) -> Result<Vec<u8>, ProtocolError>
where
    T: protobuf::Message,
{
    message.write_to_bytes().map_err(ProtocolError::Encode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_info_build() {
        let mut msg = sessionsrv::AccountGet::new();
        msg.set_name("reset".to_string());
        let route_info = RouteInfo::build(&msg);
        assert_eq!(route_info.protocol(), net::Protocol::SessionSrv);
        assert_eq!(route_info.hash().map(|x| x % 128), Some(96));
    }
}
