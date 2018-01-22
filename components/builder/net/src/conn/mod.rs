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

//! Contains types and functions for sending and receiving messages to and from a message broker
//! connected to one or more `RouteSrv`. All messages are routed through a `RouteSrv` and forwarded
//! to the appropriate receiver of a message.

mod error;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use protobuf;
use protocol::Routable;
use protocol::message::{Header, Message, RouteInfo, Txn};
use zmq;

pub use self::error::ConnErr;
use error::{ErrCode, NetError, NetResult};
use socket::DEFAULT_CONTEXT;

/// Time to wait before timing out a message receive for a `RouteConn`.
pub const RECV_TIMEOUT_MS: i32 = 15_000;
/// Time to wait before timing out a message send for a `RouteBroker` to a router.
pub const SEND_TIMEOUT_MS: i32 = 15_000;

static TXN_ID: AtomicUsize = ATOMIC_USIZE_INIT;

/// Message events signaling activity on the server listener.
pub enum ConnEvent {
    /// Occurs when a new connection was made to the listener.
    OnConnect,
    /// Occurs when the listener receives a new, well formed, message.
    OnMessage,
}

/// Client connection for sending and receiving messages to and from the service cluster through
/// a running `RouteBroker`.
pub struct RouteClient {
    socket: zmq::Socket,
    msg_buf: Message,
    recv_buf: zmq::Message,
}

impl RouteClient {
    /// Create a new `RouteClient`
    ///
    /// # Errors
    ///
    /// * Socket(s) could not be created
    pub fn new() -> Result<Self, ConnErr> {
        let socket = (**DEFAULT_CONTEXT).as_mut().socket(zmq::REQ)?;
        socket.set_rcvtimeo(RECV_TIMEOUT_MS)?;
        socket.set_sndtimeo(SEND_TIMEOUT_MS)?;
        socket.set_immediate(true)?;
        Ok(RouteClient {
            socket: socket,
            msg_buf: Message::default(),
            recv_buf: zmq::Message::new()?,
        })
    }

    pub fn connect<T>(&self, queue: T) -> Result<(), ConnErr>
    where
        T: AsRef<str>,
    {
        self.socket.connect(queue.as_ref())?;
        Ok(())
    }

    /// Routes a message to the connected broker, through a router, and to appropriate service,
    /// waits for a response, and then returns the response.
    pub fn route<M, T>(&mut self, msg: &M) -> NetResult<T>
    where
        M: Routable,
        T: protobuf::MessageStatic,
    {
        self.msg_buf.reset();
        if let Err(e) = self.msg_buf.populate(msg) {
            let err = NetError::new(ErrCode::BUG, "net:route:1");
            error!("{}, {}", err, e);
            return Err(err);
        }
        let txn_id = next_txn_id();
        self.msg_buf.txn_mut().unwrap().set_id(txn_id);
        if let Err(e) = route(&self.socket, &self.msg_buf) {
            let err = NetError::new(ErrCode::from(&e), "net:route:2");
            error!("{}, {}", err, e);
            return Err(err);
        }
        self.msg_buf.reset();
        if let Err(e) = read_header(&self.socket, &mut self.msg_buf, &mut self.recv_buf) {
            let err = NetError::new(ErrCode::from(&e), "net:route:3");
            error!("{}, {}", err, e);
            return Err(err);
        }
        if self.msg_buf.header().has_route_info() {
            if let Err(e) = try_read_route_info(
                &self.socket,
                &mut self.msg_buf,
                &mut self.recv_buf,
            )
            {
                let err = NetError::new(ErrCode::from(&e), "net:route:4");
                error!("{}, {}", err, e);
                return Err(err);
            }
        }
        if self.msg_buf.header().has_txn() {
            if let Err(e) = try_read_txn(&self.socket, &mut self.msg_buf, &mut self.recv_buf) {
                let err = NetError::new(ErrCode::from(&e), "net:route:5");
                error!("{}, {}", err, e);
                return Err(err);
            }
        }
        if let Err(e) = try_read_body(&self.socket, &mut self.msg_buf, &mut self.recv_buf) {
            let err = NetError::new(ErrCode::from(&e), "net:route:6");
            error!("{}, {}", err, e);
            return Err(err);
        }
        // JW TODO: Due to the way ZMQ sockets work, we should never receive a message out of order
        // or a message intended for another socket. We check the transaction ID as a corruption
        // check. This message means that we have a bug in how we have connected the ZMQ sockets
        // together and should be addressed immediately.
        if self.msg_buf.txn().unwrap().id() != txn_id {
            read_until_end(&self.socket, &mut self.recv_buf);
            let err = NetError::new(ErrCode::BUG, "net:route:37");
            error!("{}", err);
            return Err(err);
        }
        if self.msg_buf.message_id() == NetError::message_id() {
            match NetError::parse(&self.msg_buf) {
                Ok(err) => return Err(err),
                Err(err) => panic!("{}", err),
            }
        }
        match self.msg_buf.parse::<T>() {
            Ok(reply) => Ok(reply),
            Err(e) => {
                let err = NetError::new(ErrCode::BUG, "net:route:7");
                error!("{}, {}", err, e);
                Err(err)
            }
        }
    }
}

/// Underlying connection struct for sending and receiving messages to and from a RouteSrv.
pub struct RouteConn {
    rep_sock: zmq::Socket,
    recv_buf: zmq::Message,
    req_queue: Arc<String>,
}

impl RouteConn {
    pub fn new(req_queue: Arc<String>) -> Result<Self, ConnErr> {
        let rep_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        Ok(RouteConn {
            rep_sock: rep_sock,
            recv_buf: zmq::Message::new()?,
            req_queue: req_queue,
        })
    }

    pub fn connect<T>(&self, rep_queue: T) -> Result<(), ConnErr>
    where
        T: AsRef<str>,
    {
        self.rep_sock.connect(rep_queue.as_ref())?;
        Ok(())
    }

    pub fn route<M, T>(&mut self, message: &M) -> NetResult<T>
    where
        M: Routable,
        T: protobuf::MessageStatic,
    {
        let mut conn = RouteClient::new().unwrap();
        conn.connect(&*self.req_queue).unwrap();
        conn.route(message)
    }

    pub fn route_reply<T>(&self, message: &mut Message, reply: &T) -> Result<(), ConnErr>
    where
        T: protobuf::Message,
    {
        route_reply(&self.rep_sock, message, reply)
    }

    pub fn wait_recv(&mut self, message: &mut Message, timeout: i64) -> Result<ConnEvent, ConnErr> {
        wait_recv(&self.rep_sock, message, &mut self.recv_buf, timeout)
    }
}

/// Send a message directly to the given receiver ignoring all identities in the given message.
pub fn route(socket: &zmq::Socket, message: &Message) -> Result<(), ConnErr> {
    for identity in message.identities.iter() {
        socket.send(identity, zmq::SNDMORE)?;
        trace!("route: {:?}", identity);
    }
    socket.send(&[], zmq::SNDMORE)?;
    trace!("route: {}", message);
    send_header(socket, message)?;
    if message.header().has_route_info() {
        send_route_info(socket, message)?;
    }
    if message.header().has_txn() {
        send_txn(socket, message)?;
    }
    send_body(socket, message)
}

pub fn route_reply<T>(socket: &zmq::Socket, message: &mut Message, reply: &T) -> Result<(), ConnErr>
where
    T: protobuf::Message,
{
    message.populate_reply(reply)?;
    route(socket, message)
}

pub fn send_to(socket: &zmq::Socket, message: &Message, dest: &[u8]) -> Result<(), ConnErr> {
    socket.send(dest, zmq::SNDMORE)?;
    socket.send(&[], zmq::SNDMORE)?;
    send_header(socket, message)?;
    if message.header().has_route_info() {
        send_route_info(socket, message)?;
    }
    if message.header().has_txn() {
        send_txn(socket, message)?;
    }
    send_body(socket, message)
}

pub fn socket_poll(items: &mut [zmq::PollItem], timeout: i64) -> Result<u32, ConnErr> {
    match zmq::poll(items, timeout)? {
        count if count < 0 => unreachable!("zmq::poll, returned a negative count"),
        count if count == 0 => Err(ConnErr::Timeout),
        count => Ok(count as u32),
    }
}

pub fn socket_read(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<ConnEvent, ConnErr> {
    match read_into(socket, message, buf) {
        Ok(event) => Ok(event),
        Err(err) => {
            read_until_end(socket, buf);
            Err(err)
        }
    }
}

/// Attempts to wait for a value on this receiver, returning an error if the corresponding
/// connection has shutdown, or if it waits more than timeout.
///
/// This function will always block the current thread if there is no data available. Once
/// a message is sent to the corresponding connection, the thread will wake up write the
/// the contents into `message`.
pub fn wait_recv(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
    timeout: i64,
) -> Result<ConnEvent, ConnErr> {
    match socket_poll(&mut [socket.as_poll_item(zmq::POLLIN)], timeout) {
        Ok(count) => {
            trace!("wait-recv, received '{}' POLLIN events", count);
            socket_read(socket, message, buf)
        }
        Err(err) => Err(err),
    }
}

fn next_txn_id() -> u64 {
    let mut id = 0;
    while id == 0 {
        id = TXN_ID.fetch_add(1, Ordering::Relaxed);
    }
    id as u64
}

fn read_into(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<ConnEvent, ConnErr> {
    read_identity(socket, message, buf)?;
    if try_read_header(socket, message, buf).is_err() {
        return Ok(ConnEvent::OnConnect);
    }
    if message.header().has_route_info() {
        try_read_route_info(socket, message, buf)?;
    }
    if message.header().has_txn() {
        try_read_txn(socket, message, buf)?;
    }
    try_read_body(socket, message, buf)?;
    if buf.get_more() {
        warn!("received message with additional message parts");
        read_until_end(socket, buf);
    }
    Ok(ConnEvent::OnMessage)
}

fn read_identity(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    let mut first = true;
    loop {
        socket.recv(buf, 0)?;
        trace!("recv: identity, {:?}", buf);
        if buf.len() == 0 && first {
            return Err(ConnErr::NoIdentity);
        }
        if buf.len() == 0 {
            break;
        }
        message.identities.push(buf.to_vec());
        first = false;
    }
    Ok(())
}

fn read_header(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    loop {
        socket.recv(buf, 0)?;
        if buf.len() == 0 {
            continue;
        }
        let header = Header::from_bytes(&*buf).map_err(ConnErr::BadHeader)?;
        trace!("recv: header, {:?}", header);
        message.set_header(header);
        break;
    }
    Ok(())
}

fn read_route_info(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    socket.recv(buf, 0)?;
    let route_info = RouteInfo::from_bytes(&*buf).map_err(ConnErr::BadRouteInfo)?;
    trace!("recv: route-info, {}", route_info);
    message.set_route_info(route_info);
    Ok(())
}

fn read_txn(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    socket.recv(buf, 0)?;
    let txn = Txn::from_bytes(&*buf).map_err(ConnErr::BadTxn)?;
    trace!("recv: txn, {}", txn);
    message.set_txn(txn);
    Ok(())
}

fn read_body(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    socket.recv(buf, 0)?;
    trace!("recv: body, {:?}", buf);
    message.body = buf.to_vec();
    Ok(())
}

fn read_until_end(socket: &zmq::Socket, buf: &mut zmq::Message) {
    loop {
        if !buf.get_more() {
            break;
        }
        if let Err(err) = socket.recv(buf, 0) {
            error!("error while reading to end of message, {}", err);
            return;
        }
        trace!("recv: overflow, {:?}", buf);
    }
}

fn send_body(socket: &zmq::Socket, message: &Message) -> Result<(), ConnErr> {
    socket.send(&*message.body, 0)?;
    Ok(())
}

fn send_header(socket: &zmq::Socket, message: &Message) -> Result<(), ConnErr> {
    let bytes = message.header().to_bytes()?;
    socket.send(&bytes, zmq::SNDMORE)?;
    Ok(())
}

fn send_route_info(socket: &zmq::Socket, message: &Message) -> Result<(), ConnErr> {
    let bytes = message.route_info().as_ref().unwrap().to_bytes()?;
    socket.send(&bytes, zmq::SNDMORE)?;
    Ok(())
}

fn send_txn(socket: &zmq::Socket, message: &Message) -> Result<(), ConnErr> {
    if let Some(txn) = message.txn() {
        let bytes = txn.to_bytes()?;
        socket.send(&bytes, zmq::SNDMORE)?;
    }
    Ok(())
}

fn try_read_header(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    if !buf.get_more() {
        return Err(ConnErr::NoHeader);
    }
    read_header(socket, message, buf)
}

fn try_read_route_info(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    if !buf.get_more() {
        return Err(ConnErr::NoRouteInfo);
    }
    read_route_info(socket, message, buf)
}

fn try_read_txn(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    if !buf.get_more() {
        return Err(ConnErr::NoTxn);
    }
    read_txn(socket, message, buf)
}

fn try_read_body(
    socket: &zmq::Socket,
    message: &mut Message,
    buf: &mut zmq::Message,
) -> Result<(), ConnErr> {
    if !buf.get_more() {
        return Err(ConnErr::NoBody);
    }
    read_body(socket, message, buf)
}
