//! Client for connecting and communicating with a server listener which speaks SrvProtocol.
//!
//! # RPC Call Example
//!
//! ```rust no_run
//! use habitat_common::types::ListenCtlAddr;
//! use habitat_sup_client::SrvClient;
//! use habitat_sup_protocol as protocols;
//! use futures::stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let listen_addr = ListenCtlAddr::default();
//!     let secret_key = "seekrit";
//!     let msg = protocols::ctl::SvcGetDefaultCfg::default();
//!     let mut response = SrvClient::request(&listen_addr, secret_key, msg).await.unwrap();
//!     while let Some(message_result) = response.next().await {
//!         let reply = message_result.unwrap();
//!         match reply.message_id() {
//!             "ServiceCfg" => {
//!                 let m = reply.parse::<protocols::types::ServiceCfg>().unwrap();
//!                 println!("{}", m.default.unwrap_or_default());
//!             }
//!             "NetErr" => {
//!                 let m = reply.parse::<protocols::net::NetErr>().unwrap();
//!                 println!("{}", m);
//!             }
//!             _ => (),
//!         }
//!     }
//! }
//! ```

use habitat_sup_protocol as protocol;
#[macro_use]
extern crate log;
use crate::{common::types::ListenCtlAddr,
            protocol::{codec::*,
                       net::NetErr}};
use futures::{sink::SinkExt,
              stream::{Stream,
                       StreamExt}};
use habitat_common as common;
use std::{error,
          fmt,
          io,
          path::PathBuf};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

/// Error types returned by a [`SrvClient`].
#[derive(Debug)]
pub enum SrvClientError {
    /// Connection refused
    ConnectionRefused,
    /// The remote server unexpectedly closed the connection.
    ConnectionClosed,
    /// The client is disconnected and no further requests can be sent.
    Disconnected,
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

impl error::Error for SrvClientError {}

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
            SrvClientError::Disconnected => {
                "The client is disconnected and no further requests can be sent.".to_string()
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

/// Client for connecting and communicating with a server speaking SrvProtocol.
///
/// See module doc for usage.
#[derive(Clone)]
pub struct SrvClient;

impl SrvClient {
    /// Connect to the remote server with the given secret_key and make a request.
    ///
    /// Returns a stream of `SrvMessage`'s representing the server response.
    pub async fn request(
        address: &ListenCtlAddr,
        secret_key: &str,
        request: impl Into<SrvMessage> + fmt::Debug)
        -> Result<impl Stream<Item = Result<SrvMessage, io::Error>>, SrvClientError> {
        println!("IN NEW request");

        let socket = TcpStream::connect(address.as_ref()).await?;
        let mut socket = Framed::new(socket, SrvCodec::new());
        let mut current_transaction = SrvTxn::default();

        // Send the handshake message to the server
        let mut handshake = protocol::ctl::Handshake::default();
        handshake.secret_key = Some(String::from(secret_key));
        let mut message = SrvMessage::from(handshake);
        message.set_transaction(current_transaction);
        socket.send(message).await?;

        // Verify the handshake response
        socket.next()
              .await
              .ok_or(SrvClientError::ConnectionClosed)??;

        // Send the actual request message
        current_transaction.increment();
        let mut message = request.into();
        message.set_transaction(current_transaction);
        trace!("Sending SrvMessage -> {:?}", message);
        socket.send(message).await?;

        // Return the socket for use as a Stream of responses
        Ok(socket)
    }

    pub fn read_secret_key() -> Result<String, SrvClientError> {
        let mut buf = String::new();
        protocol::read_secret_key(protocol::sup_root(None), &mut buf)
            .map_err(SrvClientError::from)?;
        Ok(buf)
    }
}
