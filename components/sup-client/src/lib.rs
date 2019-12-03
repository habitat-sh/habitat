//! Client for connecting and communicating with a server listener which speaks SrvProtocol.
//!
//! # RPC Call Example
//!
//! ```rust no_run
//! use habitat_common::types::ListenCtlAddr;
//! use habitat_sup_client::SrvClient;
//! use habitat_sup_protocol as protocols;
//!
//! #[tokio::main]
//! async fn main() {
//!     let listen_addr = ListenCtlAddr::default();
//!     let secret_key = "seekrit";
//!     let mut client = SrvClient::connect(&listen_addr, secret_key).await.unwrap();
//!     let msg = protocols::ctl::SvcGetDefaultCfg::default();
//!     let mut response = client.request(msg).await.unwrap();
//!     while let Some(message_result) = response.recv().await {
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
use futures::{future::{self,
                       Either},
              lock::Mutex,
              sink::SinkExt,
              stream::StreamExt};
use habitat_common as common;
use std::{collections::HashMap,
          error,
          fmt,
          io,
          path::PathBuf,
          sync::Arc};
use tokio::{io::{self as tokio_io,
                 ReadHalf,
                 WriteHalf},
            net::TcpStream,
            sync::{mpsc::{self,
                          UnboundedReceiver,
                          UnboundedSender},
                   oneshot::{self,
                             Receiver,
                             Sender}}};
use tokio_util::codec::{FramedRead,
                        FramedWrite};

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

enum SrvClientState {
    Disconnected,
    Connected,
}

/// Client for connecting and communicating with a server speaking SrvProtocol.
///
/// See module doc for usage.
#[derive(Clone)]
pub struct SrvClient {
    sync: Arc<Mutex<SyncSrvClient>>,
}

impl SrvClient {
    /// Connect to the given remote server and authenticate with the given secret_key.
    ///
    /// Disconnect by simply dropping all instances of the `SrvClient`.
    ///
    /// *Note:* The client does not contain any reconnection logic. If the socket becomes
    /// disconnected all future requests will fail.
    pub async fn connect(address: &ListenCtlAddr,
                         secret_key: &str)
                         -> Result<SrvClient, SrvClientError> {
        let socket = TcpStream::connect(address.as_ref()).await?;
        let (reader, writer) = tokio_io::split(socket);
        let reader = FramedRead::new(reader, SrvCodec::new());
        let writer = FramedWrite::new(writer, SrvCodec::new());
        let (disconnect_tx, disconnect_rx) = oneshot::channel();
        let sync = SyncSrvClient { writer,
                                   current_transaction: SrvTxn::default(),
                                   transaction_lookup: HashMap::new(),
                                   // Default to connected. This instance will only be returned
                                   // if the connect is successfull.
                                   state: SrvClientState::Connected,
                                   disconnect_tx: Some(disconnect_tx) };
        let mut client = SrvClient { sync: Arc::new(Mutex::new(sync)), };

        tokio::spawn(Self::handle_messages(Self::clone(&client), reader, disconnect_rx));

        // Handshake with the server
        let mut request = protocol::ctl::Handshake::default();
        request.secret_key = Some(String::from(secret_key));
        let mut response = client.request(request).await?;
        // Verify there was only one response message and it was not an error
        response.recv()
                .await
                .ok_or(SrvClientError::ConnectionClosed)??;
        debug_assert!(response.recv().await.is_none());

        Ok(client)
    }

    pub fn read_secret_key() -> Result<String, SrvClientError> {
        let mut buf = String::new();
        protocol::read_secret_key(protocol::sup_root(None), &mut buf)
            .map_err(SrvClientError::from)?;
        Ok(buf)
    }

    /// Send a transactional request. A stream of response messages is returned.
    pub async fn request(
        &mut self,
        request: impl Into<SrvMessage> + fmt::Debug)
        -> Result<UnboundedReceiver<Result<SrvMessage, SrvClientError>>, SrvClientError> {
        let mut client = self.sync.lock().await;
        client.request(request).await
    }

    // A task for receiving messages and sending them to the correct transaction receiver
    async fn handle_messages(self,
                             mut reader: FramedRead<ReadHalf<TcpStream>, SrvCodec>,
                             mut disconnect_rx: Receiver<()>) {
        loop {
            // Select between the next message and disconnecting
            let message_result = match future::select(reader.next(), disconnect_rx).await {
                Either::Left((Some(message), unresolved_disconnect_rx)) => {
                    disconnect_rx = unresolved_disconnect_rx;
                    message
                }
                Either::Left((None, _)) => {
                    // The TCP socket was disconnected
                    break;
                }
                Either::Right((..)) => {
                    // The client has been dropped and we should disconnect
                    break;
                }
            };

            match message_result {
                Ok(message) => {
                    if let Some(transaction) = message.transaction() {
                        let mut client = self.sync.lock().await;
                        let transaction_id = transaction.id();
                        // Lookup the sender that maps to this transaction
                        match client.transaction_lookup.get_mut(&transaction_id) {
                            Some(tx) => {
                                let complete = message.is_complete();
                                // Try and send the message to the receiver.
                                // If the send fails, it is because the receiver was closed.
                                // There is no further action necessary.
                                tx.send(Ok(message)).ok();
                                // If the message is complete, the transaction is over and the
                                // sender can be removed.
                                if complete {
                                    client.transaction_lookup.remove(&transaction_id);
                                }
                            }
                            None => {
                                error!("SrvClient was unable to find received transaction {:?}",
                                       transaction)
                            }
                        }
                    } else {
                        // Do nothing with messages that are not part of a transaction
                    }
                }
                Err(e) => {
                    error!("SrvClient errored when reading next server message, err: {}",
                           e)
                }
            }
        }
        // For every outstanding transaction, send an error indicating that the connection was
        // closed
        let mut client = self.sync.lock().await;
        for (_, tx) in client.transaction_lookup.drain() {
            tx.send(Err(SrvClientError::ConnectionClosed)).ok();
        }
        client.state = SrvClientState::Disconnected;
    }
}

pub struct SyncSrvClient {
    writer:              FramedWrite<WriteHalf<TcpStream>, SrvCodec>,
    current_transaction: SrvTxn,
    transaction_lookup:  HashMap<TxnId, UnboundedSender<Result<SrvMessage, SrvClientError>>>,
    state:               SrvClientState,
    disconnect_tx:       Option<Sender<()>>,
}

impl SyncSrvClient {
    pub async fn request(
        &mut self,
        request: impl Into<SrvMessage> + fmt::Debug)
        -> Result<UnboundedReceiver<Result<SrvMessage, SrvClientError>>, SrvClientError> {
        if let SrvClientState::Disconnected = self.state {
            return Err(SrvClientError::Disconnected);
        }
        // Get the next transaction number
        self.current_transaction.increment();
        let mut message = request.into();
        message.set_transaction(self.current_transaction);

        // Create the new channel for this transaction
        let (tx, rx) = mpsc::unbounded_channel();
        self.transaction_lookup
            .insert(self.current_transaction.id(), tx);

        trace!("Sending SrvMessage -> {:?}", message);
        self.writer.send(message).await?;

        Ok(rx)
    }
}

impl Drop for SyncSrvClient {
    fn drop(&mut self) {
        // This will break out of the task receiving messages
        self.disconnect_tx
            .take()
            .expect("SvcClient disconnect_tx to be Some")
            .send(())
            .ok();
    }
}
