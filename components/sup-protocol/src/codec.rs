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

//! Binary protocol encoder and decoder (codec).
//!
//! This module contains functions and types for serializing and deserializing messages sent to
//! and from sockets speaking `SrvProtocol`. Messages are framed into 3 to 4 segments.
//!
//! # Header Segment
//!
//! Contains information to a message decoder regarding the count of segments to read and the
//! length of variable segments. The header segment is a 32 bit integer in big-endian format and
//! is packed as follows:
//!
//! * Header Segment (32-bit)
//!     * is_txn (1-bit) - flag to determine if there is a transaction segment included in this
//!                        message.
//!     * flags (5-bit) - reserved
//!     * message_id_len (6-bit) - length of the message ID segment in bytes
//!     * body_len (20-bits) - length of the message body segment in bytes
//!
//! # Transaction Segment
//!
//! An optional message segment containing transaction information for requests and responses.
//! This segment is encoded or decoded only if the `is_txn` bit is set on the header segment. The
//! transaction segment is a 32 bit integer in big-endian format and is packed as follows:
//!
//! * Transaction Segment (32-bit)
//!     * is_response (1-bit) - flag to determine if the message is a response to a request. If
//!                             not set it is a request, if set it is a reply.
//!     * is_complete (1-bit) - flag to determine if the message is the last in a stream.
//!     * txn_identifier (30-bit) - the actual transaction identifier. A number between 0 and
//!                                 `TXN_ID_MASK` (2^30-1).
//!
//! # Message ID Segment
//!
//! Used as a hint to the decoder for which protocol message is contained in the following message
//! body segment. The message ID segment is variable length indicated by the `message_id_len` field
//! on the header segment. It contains a string representation of the message ID.
//!
//! # Message Body Segment
//!
//! Contains the actual payload of the message encoded using Google
//! [Protobuf 2](https://developers.google.com/protocol-buffers/docs/reference/proto2-spec).

use std::fmt;
use std::io::{self, Cursor};
use std::str;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures;
use prost::{self, Message};
use tokio::net::TcpStream;
use tokio_codec::{Decoder, Encoder, Framed};

use message::MessageStatic;
use net::{NetErr, NetResult};

const BODY_LEN_MASK: u32 = 0xFFFFF;
const HEADER_LEN: usize = 4;
const MESSAGE_ID_MASK: u32 = 0x3F;
const MESSAGE_ID_OFFSET: u32 = 20;
const TXN_LEN: usize = 4;
const TXN_OFFSET: u32 = 31;

const TXN_ID_MASK: u32 = 0x3FFFFFFF;
const RESPONSE_OFFSET: u32 = 31;
const RESPONSE_MASK: u32 = 0x1;
const COMPLETE_OFFSET: u32 = 30;
const COMPLETE_MASK: u32 = 0x1;

/// A `TcpStream` framed with `SrvCodec`. This is the base socket connection that the CtlGateway
/// client and server speak.
pub type SrvStream = Framed<TcpStream, SrvCodec>;

/// Sending half of `SrvStream`.
pub type SrvSink = futures::stream::SplitSink<SrvStream>;

/// An unsigned 32-bit integer packed with transaction information which is present if a request
/// should receive a response from the destination.
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SrvTxn(u32);

impl SrvTxn {
    /// The contained transaction ID.
    pub fn id(&self) -> u32 {
        self.0 & TXN_ID_MASK
    }

    /// Update the transaction ID to the next valid value.
    pub fn increment(&mut self) {
        self.0 += 1;
        if self.0 >= TXN_ID_MASK || self.0 == 0 {
            self.0 = 1;
        }
    }

    /// Check if this transaction represents the last message in a transaction.
    pub fn is_complete(&self) -> bool {
        ((self.0 >> COMPLETE_OFFSET) & COMPLETE_MASK) == 1
    }

    /// Check if this transaction represents a reply to a request.
    pub fn is_response(&self) -> bool {
        ((self.0 >> RESPONSE_OFFSET) & RESPONSE_MASK) == 1
    }

    /// Set the completion bit indicating that the message this transaction is associated with is
    /// the last reply to a transactional request.
    pub fn set_complete(&mut self) {
        self.0 = self.0 | (1 << COMPLETE_OFFSET);
    }

    /// Set the response bit indicating that the message this transaction is associated with is
    /// a response to transactional request.
    pub fn set_response(&mut self) {
        self.0 = self.0 | (1 << RESPONSE_OFFSET);
    }
}

impl From<u32> for SrvTxn {
    fn from(value: u32) -> Self {
        SrvTxn(value)
    }
}

impl fmt::Debug for SrvTxn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SrvTxn[id: {}, is_complete: {}, is_response: {}]",
            self.id(),
            self.is_complete(),
            self.is_response(),
        )
    }
}

/// An unsigned 32-bit integer packed with information for a socket and decoder to know how any
/// frames to read and how long each variable frame is.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SrvHeader(u32);

impl SrvHeader {
    pub fn new(body_len: u32, message_id_len: u32, is_txn: bool) -> Self {
        assert!(
            message_id_len <= MESSAGE_ID_MASK,
            "cannot construct message with message-id length larger than MESSAGE_ID_MASK"
        );
        assert!(
            body_len <= BODY_LEN_MASK,
            "cannot construct message with body length larger than BODY_LEN_MASK"
        );
        let txn_value = if is_txn { 1 } else { 0 };
        let value = (txn_value << TXN_OFFSET) | (message_id_len << MESSAGE_ID_OFFSET) | body_len;
        SrvHeader(value)
    }

    #[inline]
    pub fn body_len(&self) -> usize {
        (self.0 & BODY_LEN_MASK) as usize
    }

    #[inline]
    pub fn message_id_len(&self) -> usize {
        ((self.0 >> MESSAGE_ID_OFFSET) & MESSAGE_ID_MASK) as usize
    }

    #[inline]
    pub fn is_transaction(&self) -> bool {
        match (self.0 >> TXN_OFFSET) & 1 {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }

    /// Set the presence of the transaction frame of this message.
    #[inline]
    pub fn set_is_transaction(&mut self) {
        self.0 = self.0 | (1 << TXN_OFFSET);
    }
}

impl From<u32> for SrvHeader {
    fn from(value: u32) -> Self {
        SrvHeader(value)
    }
}

impl fmt::Debug for SrvHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SrvHeader[body_len: {}, message_id_len: {}, is_txn: {}]",
            self.body_len(),
            self.message_id_len(),
            self.is_transaction()
        )
    }
}

/// The payload of a `SrvProtocol` message. This is the unit to send and receive between any
/// socket speaking `SrvProtocol`.
#[derive(Clone)]
pub struct SrvMessage {
    header: SrvHeader,
    transaction: Option<SrvTxn>,
    message_id: String,
    body: Bytes,
}

impl SrvMessage {
    /// Returns a reference to the encoded bytes of the protocol message.
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Returns the header frame of the protocol message.
    pub fn header(&self) -> SrvHeader {
        self.header
    }

    /// Returns true if the message is non-transactional or if the message is transactional and
    /// if this message is the last in a message stream. Returns false if this is not the last
    /// message in a transaction stream.
    pub fn is_complete(&self) -> bool {
        match self.transaction {
            Some(txn) => txn.is_complete(),
            None => true,
        }
    }

    /// Returns true if the message is a response to a transactional request and false otherwise.
    pub fn is_response(&self) -> bool {
        match self.transaction {
            Some(txn) => txn.is_response(),
            None => false,
        }
    }

    /// Returns true if the message is transactional.
    pub fn is_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    /// Returns a reference to the message ID of the encoded protobuf for this protocol message.
    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    /// Attempts to parse the message as the given type `T`. You can use `message_id()` as a hint
    /// to which type to use as type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// # use habitat_sup_protocol::message::MessageStatic;
    /// # use habitat_sup_protocol::codec::SrvMessage;
    /// # use habitat_sup_protocol::net;
    /// # let m = SrvMessage::from(net::NetErr::default());
    /// if m.message_id() == net::NetErr::MESSAGE_ID {
    ///     let msg = m.parse::<net::NetErr>().unwrap();
    /// }
    /// ```
    pub fn parse<T>(&self) -> Result<T, prost::DecodeError>
    where
        T: Message + MessageStatic + Default,
    {
        T::decode(&self.body)
    }

    /// Update the message as a reply for the given transaction. The `complete` argument will
    /// additionally note if this is the final message in the message stream or not.
    pub fn reply_for(&mut self, mut txn: SrvTxn, complete: bool) {
        txn.set_response();
        if complete {
            txn.set_complete();
        }
        self.set_transaction(txn);
    }

    /// The size of this message - including message segments - in bytes.
    pub fn size(&self) -> usize {
        let mut size = HEADER_LEN;
        if self.transaction.is_some() {
            size += TXN_LEN;
        }
        size += self.message_id().len();
        size += self.body().len();
        size
    }

    /// Returns the transaction.
    pub fn transaction(&self) -> Option<SrvTxn> {
        self.transaction
    }

    /// Set a transaction to the given message.
    pub fn set_transaction(&mut self, txn: SrvTxn) {
        self.header.set_is_transaction();
        self.transaction = Some(txn);
    }

    /// Returns `Ok(())` if the message contains anything other than a `NetErr` and `Err(NetErr)`
    /// if the message contains a net error. This is useful in combinators when you want to quickly
    /// fail out if the received message contains an error.
    pub fn try_ok(&self) -> NetResult<()> {
        if self.message_id() == NetErr::MESSAGE_ID {
            let err = NetErr::decode(self.body()).expect("try_ok bad NetErr");
            return Err(err);
        }
        Ok(())
    }
}

impl fmt::Debug for SrvMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}",
            self.header, self.transaction, self.message_id
        )
    }
}

impl<T> From<T> for SrvMessage
where
    T: Message + MessageStatic,
{
    fn from(msg: T) -> Self {
        let mut buf = BytesMut::with_capacity(msg.encoded_len());
        msg.encode(&mut buf).unwrap();
        let body = buf.freeze();
        let message_id = T::MESSAGE_ID.to_string();
        SrvMessage {
            header: SrvHeader::new(body.len() as u32, message_id.len() as u32, false),
            transaction: None,
            message_id: message_id,
            body: body,
        }
    }
}

/// Binary encoder decoder for the `SrvProtocol` binary protocol.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SrvCodec {
    recv_buf: Vec<u8>,
}

impl SrvCodec {
    /// Creates a new `SrvCodec` for shipping around `SrvMessage`s.
    pub fn new() -> SrvCodec {
        SrvCodec {
            recv_buf: vec![0; BODY_LEN_MASK as usize],
        }
    }
}

impl Decoder for SrvCodec {
    type Item = SrvMessage;
    type Error = io::Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        if bytes.len() < HEADER_LEN {
            return Ok(None);
        }
        trace!("Decoding SrvMessage\n  -> Bytes: {:?}", bytes);
        let mut buf = Cursor::new(bytes);
        let header = SrvHeader(buf.get_u32_be());
        trace!("  -> SrvHeader: {:?}", header);
        let mut txn: Option<SrvTxn> = None;
        if header.is_transaction() {
            if buf.remaining() < TXN_LEN {
                return Ok(None);
            }
            let t = SrvTxn(buf.get_u32_be());
            trace!("  -> SrvTxn: {:?}", t);
            txn = Some(t);
        }
        if buf.remaining() < (header.message_id_len() + header.body_len()) {
            // Not enough bytes to read message_id and body
            return Ok(None);
        }
        buf.copy_to_slice(&mut self.recv_buf[0..header.message_id_len()]);
        let message_id = str::from_utf8(&self.recv_buf[0..header.message_id_len()])
            .unwrap()
            .to_string();
        buf.copy_to_slice(&mut self.recv_buf[0..header.body_len()]);
        let position = buf.position() as usize;
        let bytes = buf.into_inner();
        bytes.split_to(position);
        Ok(Some(SrvMessage {
            header: header,
            transaction: txn,
            message_id: message_id,
            body: Bytes::from(&self.recv_buf[0..header.body_len()]),
        }))
    }
}

impl Encoder for SrvCodec {
    type Item = SrvMessage;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.reserve(msg.size());
        buf.put_u32_be(msg.header().0);
        if let Some(txn) = msg.transaction {
            buf.put_u32_be(txn.0);
        }
        buf.put_slice(msg.message_id().as_bytes());
        buf.put_slice(msg.body());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use net;

    #[test]
    fn test_header_pack_unpack() {
        let body_value = 305888;
        let message_id_value = 40;
        let header = SrvHeader::new(body_value, message_id_value, true);
        assert_eq!(header.body_len(), body_value as usize);
        assert_eq!(header.message_id_len(), message_id_value as usize);
        assert_eq!(header.is_transaction(), true);
    }

    #[test]
    fn test_txn_increment() {
        let mut txn = SrvTxn::default();
        txn.increment();
        assert_eq!(1, txn.id());
        let mut txn = SrvTxn(TXN_ID_MASK);
        txn.increment();
        assert_eq!(1, txn.id());
        let mut txn = SrvTxn(TXN_ID_MASK + 1);
        txn.increment();
        assert_eq!(1, txn.id());
    }

    #[test]
    fn test_txn_set_complete() {
        let mut header = SrvHeader::new(0, 0, false);
        assert_eq!(header.is_transaction(), false);
        header.set_is_transaction();
        assert_eq!(header.is_transaction(), true);
    }

    #[test]
    fn test_txn_pack_unpack() {
        let mut txn = SrvTxn(TXN_ID_MASK);
        assert_eq!(txn.is_complete(), false);
        assert_eq!(txn.is_response(), false);

        txn.set_complete();
        assert_eq!(txn.is_complete(), true);
        assert_eq!(txn.is_response(), false);

        txn.set_response();
        assert_eq!(txn.is_complete(), true);
        assert_eq!(txn.is_response(), true);
    }

    #[test]
    #[should_panic]
    fn test_body_len_overflow() {
        SrvHeader::new(BODY_LEN_MASK + 1, 0, true);
    }

    #[test]
    #[should_panic]
    fn test_message_id_len_overflow() {
        SrvHeader::new(0, MESSAGE_ID_MASK + 1, true);
    }

    #[test]
    fn test_codec() {
        let mut codec = SrvCodec::new();
        let mut inner = net::NetErr::default();
        inner.code = net::ErrCode::NotFound as i32;
        inner.msg = "this".to_string();
        let msg = SrvMessage::from(inner);
        let mut buf = BytesMut::new();
        codec.encode(msg.clone(), &mut buf).unwrap();
        let decoded = codec.decode(&mut buf).unwrap().unwrap();

        assert_eq!(decoded.header(), msg.header());
        assert_eq!(decoded.message_id(), msg.message_id());
        assert_eq!(decoded.transaction(), msg.transaction());
        assert_eq!(decoded.body(), msg.body());
    }
}
