//! Client for connecting and communicating with a server listener which speaks SrvProtocol.
//!
//! Currently all functions will block and wait for a response from the remote. This is likely to
//! change to a futures based implementation as our needs increase.
//!
//! # RPC Call Example
//!
//! ```
//! # use futures::future::Future as _;
//! # use futures::stream::Stream as _;
//! # use habitat_sup_client::SrvClient;
//! # use habitat_sup_protocol as protocols;
//! # use habitat_common::types::ListenCtlAddr;
//! # let listen_addr = ListenCtlAddr::default();
//! # let secret_key = "seekrit";
//! let conn = SrvClient::connect(&listen_addr, secret_key);
//! let msg = protocols::ctl::SvcGetDefaultCfg::default();
//! conn.and_then(|conn| {
//!         conn.call(msg).for_each(|reply| {
//!                           match reply.message_id() {
//!                               "ServiceCfg" => {
//!                                   let m =
//!                                       reply.parse::<protocols::types::ServiceCfg>().unwrap();
//!                                   println!("{}", m.default.unwrap_or_default());
//!                               }
//!                               "NetErr" => {
//!                                   let m = reply.parse::<protocols::net::NetErr>().unwrap();
//!                                   println!("{}", m);
//!                               }
//!                               _ => (),
//!                           }
//!                           Ok(())
//!                       })
//!     });
//! ```

#[macro_use]
extern crate futures;
use habitat_sup_protocol as protocol;
#[macro_use]
extern crate log;
use crate::{common::types::ListenCtlAddr,
            protocol::{codec::*,
                       net::NetErr}};
use futures::{prelude::*,
              sink};
use habitat_common as common;
use std::{error,
          fmt,
          io,
          path::PathBuf};
use tokio::net::TcpStream;
use tokio_codec::Framed;

pub type SrvSend = sink::Send<SrvStream>;

/// Error types returned by a [`SrvClient`].
#[derive(Debug)]
pub enum SrvClientError {
    /// Connection refused
    ConnectionRefused,
    /// The remote server unexpectedly closed the connection.
    ConnectionClosed,
    /// Unable to locate a secret key on disk.
    CtlSecretNotFound(PathBuf),
    /// Decoding a message from the remote failed.
    Decode(prost::DecodeError),
    /// An Os level IO error occurred.
    Io(io::Error),
    /// An RPC call to the remote was received but failed.
    NetErr(NetErr),
    /// A parse error from an Invalid Color string
    ParseColor(termcolor::ParseColorError),
}

impl error::Error for SrvClientError {
    fn description(&self) -> &str {
        match *self {
            SrvClientError::ConnectionClosed => "Connection closed",
            SrvClientError::ConnectionRefused => "Connection refused",
            SrvClientError::CtlSecretNotFound(_) => "Ctl secret key not found",
            SrvClientError::Decode(ref err) => err.description(),
            SrvClientError::Io(ref err) => err.description(),
            SrvClientError::NetErr(ref err) => err.description(),
            SrvClientError::ParseColor(ref err) => err.description(),
        }
    }
}

impl fmt::Display for SrvClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match *self {
            SrvClientError::ConnectionClosed => {
                "The connection was unexpectedly closed.\n\nThis may be because the given \
                 Supervisor is in the middle of an orderly shutdown,\nand is no longer processing \
                 command requests."
                                   .to_string()
            }
            SrvClientError::ConnectionRefused => {
                "Unable to contact the Supervisor.\n\nIf the Supervisor you are contacting is \
                 local, this probably means it is not running. You can run a Supervisor in the \
                 foreground with:\n\nhab sup run\n\nOr try restarting the Supervisor through your \
                 operating system's init process or Windows service."
                                                                     .to_string()
            }
            SrvClientError::CtlSecretNotFound(ref path) => {
                format!("No Supervisor CtlGateway secret set in `cli.toml` or found at {}. Run \
                         `hab setup` or run the Supervisor for the first time before attempting \
                         to command the Supervisor.",
                        path.display())
            }
            SrvClientError::Decode(ref err) => format!("{}", err),
            SrvClientError::Io(ref err) => format!("{}", err),
            SrvClientError::NetErr(ref err) => format!("{}", err),
            SrvClientError::ParseColor(ref err) => format!("{}", err),
        };
        write!(f, "{}", content)
    }
}

impl From<NetErr> for SrvClientError {
    fn from(err: NetErr) -> Self { SrvClientError::NetErr(err) }
}

impl From<io::Error> for SrvClientError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::ConnectionRefused => SrvClientError::ConnectionRefused,
            _ => SrvClientError::Io(err),
        }
    }
}

impl From<prost::DecodeError> for SrvClientError {
    fn from(err: prost::DecodeError) -> Self { SrvClientError::Decode(err) }
}

impl From<termcolor::ParseColorError> for SrvClientError {
    fn from(err: termcolor::ParseColorError) -> Self { SrvClientError::ParseColor(err) }
}

/// Client for connecting and communicating with a server listener which speaks SrvProtocol.
///
/// See module doc for usage.
pub struct SrvClient {
    /// Sending and receiving framed socket pair for connecting and communicating to a remote
    /// server.
    socket: SrvStream,
    /// Transaction ID counter.
    ///
    /// Useful for determining the next transaction ID in sequence embed in
    /// the next transactional message sent.
    current_txn: SrvTxn,
}

impl SrvClient {
    /// Connect to the given remote server and authenticate with the given secret_key.
    pub fn connect(addr: &ListenCtlAddr,
                   secret_key: &str)
                   -> Box<dyn Future<Item = SrvClient, Error = SrvClientError> + 'static> {
        let secret_key = secret_key.to_string();
        let conn = TcpStream::connect(addr.as_ref()).map_err(SrvClientError::from)
                                                    .and_then(move |socket| {
                                                        let client = Self::new(socket, None);
                                                        let mut request =
                                                            protocol::ctl::Handshake::default();
                                                        request.secret_key = Some(secret_key);
                                                        client.call(request)
                                                              .into_future()
                                                              .map_err(|(err, _)| err)
                                                              .and_then(move |(m, io)| {
                                                                  m.map_or_else(
                            || Err(SrvClientError::ConnectionClosed),
                            move |m| {
                                m.try_ok()
                                    .map_err(SrvClientError::from)
                                    .and_then(|()| Ok(io.into_inner()))
                            },
                        )
                                                              })
                                                    });
        Box::new(conn)
    }

    pub fn read_secret_key() -> Result<String, SrvClientError> {
        let mut buf = String::new();
        protocol::read_secret_key(protocol::sup_root(None), &mut buf)
            .map_err(SrvClientError::from)?;
        Ok(buf)
    }

    fn new(socket: TcpStream, current_txn: Option<SrvTxn>) -> Self {
        SrvClient { socket:      Framed::new(socket, SrvCodec::new()),
                    current_txn: current_txn.unwrap_or_default(), }
    }

    /// Send a transactional request to the connected server. The returned `SrvReply` is a Stream
    /// containing one or more `SrvMessage` responses for the given request.
    pub fn call<T>(mut self, request: T) -> SrvReply
        where T: Into<SrvMessage> + fmt::Debug
    {
        self.current_txn.increment();
        let mut msg: SrvMessage = request.into();
        msg.set_transaction(self.current_txn);
        trace!("Sending SrvMessage -> {:?}", msg);
        SrvReply::new(self.socket.send(msg), self.current_txn)
    }

    /// Send a non-transactional request to the connected server.
    pub fn cast<T>(self, request: T) -> SrvSend
        where T: Into<SrvMessage> + fmt::Debug
    {
        let message: SrvMessage = request.into();
        self.socket.send(message)
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvMessage` replies.
#[must_use = "futures do nothing unless polled"]
pub struct SrvReply {
    io:     sink::Send<SrvStream>,
    state:  SrvReplyState,
    txn_id: SrvTxn,
}

impl SrvReply {
    fn new(io: sink::Send<SrvStream>, txn_id: SrvTxn) -> Self {
        SrvReply { io,
                   state: SrvReplyState::Sending,
                   txn_id }
    }

    /// Consume the `SrvReply` and return the contained `SrvClient`.
    pub fn into_inner(self) -> SrvClient {
        match self.state {
            SrvReplyState::Receiving(io, true) => {
                SrvClient::new(io.into_inner(), Some(self.txn_id))
            }
            _ => panic!("into_inner called before complete"),
        }
    }
}

impl Stream for SrvReply {
    type Error = SrvClientError;
    type Item = SrvMessage;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.state {
                SrvReplyState::Sending => {
                    let io = try_ready!(self.io.poll());
                    self.state = SrvReplyState::Receiving(io, false);
                    continue;
                }
                SrvReplyState::Receiving(_, true) => return Ok(Async::Ready(None)),
                SrvReplyState::Receiving(ref mut io, ref mut complete) => {
                    match try_ready!(io.poll()) {
                        Some(msg) => {
                            *complete = msg.is_complete();
                            return Ok(Async::Ready(Some(msg)));
                        }
                        None => return Err(SrvClientError::ConnectionClosed),
                    }
                }
            }
        }
    }
}

enum SrvReplyState {
    /// Request is sending.
    Sending,
    /// Request is sent and awaiting message(s). Receiving is complete when the bool at `self.1`
    /// is true.
    Receiving(SrvStream, bool),
}
