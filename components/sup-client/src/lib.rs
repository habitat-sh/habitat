#![allow(clippy::needless_doctest_main)]
//! Client for connecting and communicating with a server listener which speaks SrvProtocol.
//!
//! # RPC Call Example
//!
//! ```rust no_run
//! use habitat_common::types::ResolvedListenCtlAddr;
//! use habitat_sup_client::SrvClient;
//! use habitat_sup_protocol as protocols;
//! use futures::stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let listen_addr = ResolvedListenCtlAddr::default();
//!     let msg = protocols::ctl::SvcGetDefaultCfg::default();
//!     let mut response = SrvClient::request(&listen_addr, msg).await.unwrap();
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
use log::{debug,
          trace};

use crate::{common::types::ResolvedListenCtlAddr,
            protocol::{codec::*,
                       net::NetErr}};
use futures::{sink::SinkExt,
              stream::{Stream,
                       StreamExt}};
use habitat_common::{self as common,
                     cli::CTL_SECRET_ENVVAR,
                     cli_config::{CliConfig,
                                  Error as CliConfigError}};
use habitat_core::{env as henv,
                   tls::rustls_wrapper::TcpOrTlsStream};
use rustls::Error as RustlsError;
use std::{error,
          fmt,
          io,
          sync::Arc,
          time::Duration};
use tokio::{net::TcpStream,
            time};
use tokio_util::codec::Framed;

/// Time to wait in milliseconds for a client connection to timeout.
pub const REQ_TIMEOUT: u64 = 10_000;

/// Error types returned by a [`SrvClient`].
#[derive(Debug)]
pub enum SrvClientError {
    /// Connection refused
    ConnectionRefused,
    /// The remote server unexpectedly closed the connection.
    ConnectionClosed,
    CliConfigError(CliConfigError),
    /// Decoding a message from the remote failed.
    Decode(prost::DecodeError),
    /// An Os level IO error occurred.
    Io(io::Error),
    /// An RPC call to the remote was received but failed.
    NetErr(NetErr),
    /// A parse error from an Invalid Color string
    ParseColor(termcolor::ParseColorError),
    RustlsError(RustlsError),
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
            SrvClientError::CliConfigError(ref err) => format!("{}", err),
            SrvClientError::Decode(ref err) => format!("{}", err),
            SrvClientError::Io(ref err) => format!("{}", err),
            SrvClientError::NetErr(ref err) => format!("{}", err),
            SrvClientError::ParseColor(ref err) => format!("{}", err),
            SrvClientError::RustlsError(ref err) => {
                format!("failed to establish TLS connection, err: {}", err)
            }
        };
        write!(f, "{}", content)
    }
}

impl From<CliConfigError> for SrvClientError {
    fn from(err: CliConfigError) -> Self { SrvClientError::CliConfigError(err) }
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

impl From<RustlsError> for SrvClientError {
    fn from(err: RustlsError) -> Self { SrvClientError::RustlsError(err) }
}

/// Client for connecting and communicating with a server speaking SrvProtocol.
///
/// See module doc for usage.
pub struct SrvClient;

impl SrvClient {
    /// Connect to the remote server with the given secret_key and make a request.
    ///
    /// Returns a stream of `SrvMessage`'s representing the server response.
    pub async fn request(
        addr: &ResolvedListenCtlAddr,
        request: impl Into<SrvMessage> + fmt::Debug)
        -> Result<impl Stream<Item = Result<SrvMessage, io::Error>>, SrvClientError> {
        let tcp_stream = TcpStream::connect(addr.addr()).await?;

        // Upgrade to a TLS connection if necessary
        let config = CliConfig::load()?;
        let server_name_indication = config.ctl_server_name_indication.clone();
        let tcp_stream = match config.maybe_tls_client_config()?.map(Arc::new) {
            Some(tls_config) => {
                let domain = server_name_indication.unwrap_or_else(|| addr.domain().to_string());
                debug!("Upgrading ctl-gateway to TLS with domain '{}'", domain);
                TcpOrTlsStream::new_tls_client(tcp_stream, tls_config, &domain).await
                                                                               .map_err(|e| e.0)?
            }
            _ => TcpOrTlsStream::new(tcp_stream),
        };

        let mut tcp_stream = Framed::new(tcp_stream, SrvCodec::new());
        let mut current_transaction = SrvTxn::default();

        // Send the handshake message to the server
        let handshake = protocol::ctl::Handshake { secret_key: Some(Self::ctl_secret_key()?), };
        let mut message = SrvMessage::from(handshake);
        message.set_transaction(current_transaction);
        tcp_stream.send(message).await?;

        // Verify the handshake response. There are three kinds of errors we could encounter:
        // 1. The handshake timedout
        // 2. The `tcp_stream.next()` call returns `None` indicating the connection was unexpectedly
        // closed by the server
        // 3. Any other socket io error
        let handshake_result =
            time::timeout(Duration::from_millis(REQ_TIMEOUT), tcp_stream.next()).await;
        let handshake_reply = handshake_result.map_err(|_| {
                                                  io::Error::new(io::ErrorKind::TimedOut,
                                                                 "client timed out")
                                              })?
                                              .ok_or(SrvClientError::ConnectionClosed)??;
        handshake_reply.try_ok()?;

        // Send the actual request message
        current_transaction.increment();
        let mut message = request.into();
        message.set_transaction(current_transaction);
        trace!("Sending SrvMessage -> {:?}", message);
        tcp_stream.send(message).await?;

        // Return the tcp_stream for use as a Stream of responses
        Ok(tcp_stream)
    }

    /// Check if the `HAB_CTL_SECRET` env var is set. If not, check the CLI config to see if there
    /// is a ctl secret set. If not, read CTL_SECRET
    fn ctl_secret_key() -> Result<String, SrvClientError> {
        match henv::var(CTL_SECRET_ENVVAR) {
            Ok(v) => Ok(v),
            Err(_) => {
                let config = CliConfig::load()?;
                match config.ctl_secret {
                    Some(v) => Ok(v),
                    None => SrvClient::ctl_secret_key_from_file(),
                }
            }
        }
    }

    fn ctl_secret_key_from_file() -> Result<String, SrvClientError> {
        let mut buf = String::new();
        protocol::read_secret_key(protocol::sup_root(None), &mut buf)?;
        Ok(buf)
    }
}
