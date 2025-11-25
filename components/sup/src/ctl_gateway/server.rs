//! Listening server for receiving client connections which speak SrvProtocol.
//!
//! The server runs in a separate thread and dispatches operational commands to the main thread
//! over an unbounded mpsc channel, `MgrSender`, to `MgrReceiver`. These commands are wrapped in
//! a [`ctl_gateway.CtlRequest`] if they are transactional.
//!
//! Replies to transactional messages are sent back to the CtlGateway thread over an unbounded
//! mpsc channel, [`CtlSender`], to [`CtlReceiver`]. A new mpsc pair is created for each
//! transactional request where the sending half is given to a [`ctl_gateway.CtlRequest`].

use super::{CtlRequest,
            REQ_TIMEOUT};
use crate::manager::{ManagerState,
                     action::ActionSender,
                     commands};
use futures::{channel::mpsc,
              executor,
              prelude::*,
              ready,
              task::{Context,
                     Poll}};
use habitat_core::{crypto,
                   tls::rustls_wrapper::TcpOrTlsStream};
use habitat_sup_protocol::{self as protocol,
                           codec::{SrvCodec,
                                   SrvMessage,
                                   SrvStream,
                                   SrvTxn},
                           net::{self,
                                 ErrCode,
                                 NetErr,
                                 NetResult}};
use lazy_static::lazy_static;
use log::{debug,
          error,
          trace,
          warn};
use pin_project::pin_project;
use prometheus::{HistogramTimer,
                 HistogramVec,
                 IntCounterVec,
                 register_histogram_vec,
                 register_int_counter_vec};

use rustls::{self,
             RootCertStore,
             ServerConfig as TlsServerConfig,
             pki_types::{CertificateDer,
                         PrivateKeyDer,
                         PrivatePkcs8KeyDer},
             server::WebPkiClientVerifier};
use std::{error,
          fmt,
          io,
          net::SocketAddr,
          pin::Pin,
          sync::{Arc,
                 Mutex},
          time::Duration};
use tokio::{io::AsyncWrite,
            net::TcpListener,
            task,
            time};
use tokio_util::codec::Decoder;

lazy_static! {
    static ref RPC_CALLS: IntCounterVec = register_int_counter_vec!("hab_sup_rpc_call_total",
                                                                    "Total number of RPC calls",
                                                                    &["name"]).unwrap();
    static ref RPC_CALL_DURATION: HistogramVec =
        register_histogram_vec!("hab_sup_rpc_call_request_duration_seconds",
                                "The latency for RPC calls",
                                &["name"]).unwrap();
}

/// Sending half of an mpsc unbounded channel used for sending replies for a transactional message
/// from the main thread back to the CtlGateway. This half is stored in a
/// [`ctl_gateway.CtlRequest`] in the main thread.
pub type CtlSender = mpsc::UnboundedSender<SrvMessage>;
/// Receiving half of an mpsc unbounded channel used for sending replies for a transactional
/// message from the main thread back to the CtlGateway. This half is stored in the CtlGateway on
/// it's thread.
pub type CtlReceiver = mpsc::UnboundedReceiver<SrvMessage>;
/// Sender from the CtlGateway to the Manager to dispatch control commands for clients.
pub type MgrSender = mpsc::UnboundedSender<CtlCommand>;
/// Receiver on the Manager for the sender on the CtlGateway to receive control commands.
pub type MgrReceiver = mpsc::UnboundedReceiver<CtlCommand>;

#[derive(Debug)]
pub enum HandlerError {
    Decode(prost::DecodeError),
    Io(io::Error),
    NetErr(NetErr),
    SendError(mpsc::SendError),
}

impl error::Error for HandlerError {}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match *self {
            HandlerError::Decode(ref err) => format!("{}", err),
            HandlerError::Io(ref err) => format!("{}", err),
            HandlerError::NetErr(ref err) => format!("{}", err),
            HandlerError::SendError(ref err) => format!("{}", err),
        };
        write!(f, "{}", content)
    }
}

impl From<NetErr> for HandlerError {
    fn from(err: NetErr) -> Self { HandlerError::NetErr(err) }
}

impl From<io::Error> for HandlerError {
    fn from(err: io::Error) -> Self { HandlerError::Io(err) }
}

impl From<prost::DecodeError> for HandlerError {
    fn from(err: prost::DecodeError) -> Self { HandlerError::Decode(err) }
}

impl From<mpsc::SendError> for HandlerError {
    fn from(err: mpsc::SendError) -> Self { HandlerError::SendError(err) }
}

/// A wrapper around a [`ctl_gateway.CtlRequest`] and a closure for the main thread to execute.
pub struct CtlCommand {
    pub req: CtlRequest,
    // JW: This needs to be an `FnOnce<Box>` and not an `Fn<Box>` but right now there is no support
    // for boxing an FnOnce in stable Rust. There is a new type called `FnBox` which exists only on
    // nightly right now which accomplishes this but it won't stabilize because the Rust core team
    // feels that they should just get `Box<FnOnce>` working. We'll need to clone the `CtlRequest`
    // argument passed to this closure until `FnOnce<Box>` stabilizes.
    //
    // https://github.com/rust-lang/rust/issues/28796
    //
    // TODO: This is now possible see https://github.com/habitat-sh/habitat/issues/6832
    // We held off on making the change to reduce the risk of a regression and to lump it in with
    // more general Future refactoring.
    #[allow(clippy::type_complexity)]
    fun:     Box<dyn Fn(&ManagerState, &mut CtlRequest, ActionSender) -> NetResult<()> + Send>,
}

impl CtlCommand {
    /// Create a new CtlCommand from the given CtlSender, transaction, and closure to execute.
    pub fn new<F>(tx: CtlSender, txn: Option<SrvTxn>, fun: F) -> Self
        where F: Fn(&ManagerState, &mut CtlRequest, ActionSender) -> NetResult<()> + Send + 'static
    {
        CtlCommand { fun: Box::new(fun),
                     req: CtlRequest::new(tx, txn), }
    }

    /// Run the contained closure with the given [`manager.ManagerState`].
    pub fn run(&mut self, state: &ManagerState, action_sender: ActionSender) -> NetResult<()> {
        (self.fun)(state, &mut self.req, action_sender)
    }
}

/// Server's client representation. Each new connection will allocate a new Client.
struct Client {
    state: Arc<Mutex<SrvState>>,
}

impl Client {
    /// Serve the client from the given framed socket stream.
    pub async fn serve(self, mut socket: SrvStream) -> Result<(), HandlerError> {
        let mgr_sender = self.state
                             .lock()
                             .expect("SrvState mutex poisoned")
                             .mgr_sender
                             .clone();
        let handshake_with_timeout = time::timeout(Duration::from_millis(REQ_TIMEOUT),
                                                   self.handshake(&mut socket));
        handshake_with_timeout.await
                              .map_err(|_| {
                                  io::Error::new(io::ErrorKind::TimedOut, "client timed out")
                              })??;
        SrvHandler::new(socket, mgr_sender).await
    }

    /// Initiate a handshake with the connected client before allowing future requests. A failed
    /// handshake will close the connection.
    async fn handshake(&self, socket: &mut SrvStream) -> Result<(), HandlerError> {
        let message = socket.next()
                            .await
                            .ok_or_else(|| io::Error::from(io::ErrorKind::UnexpectedEof))??;
        let success = if message.message_id() != "Handshake" {
            debug!("No handshake");
            return Err(HandlerError::from(io::Error::from(io::ErrorKind::ConnectionAborted)));
        } else if !message.is_transaction() {
            return Err(HandlerError::from(io::Error::from(io::ErrorKind::ConnectionAborted)));
        } else {
            match message.parse::<protocol::ctl::Handshake>() {
                Ok(decoded) => {
                    trace!("Received handshake, {:?}", decoded);
                    let secret_key = self.state
                                         .lock()
                                         .expect("SrvState mutex poisoned")
                                         .secret_key
                                         .to_string();
                    let decoded_key = decoded.secret_key.unwrap_or_default();
                    crypto::secure_eq(decoded_key, secret_key)
                }
                Err(err) => {
                    warn!("Handshake error, {:?}", err);
                    return Err(HandlerError::from(io::Error::from(
                        io::ErrorKind::ConnectionAborted,
                    )));
                }
            }
        };
        let (mut reply, result) = if success {
            (SrvMessage::from(net::ok()), Ok(()))
        } else {
            (SrvMessage::from(net::err(ErrCode::Unauthorized, "secret key mismatch")),
             Err(HandlerError::from(io::Error::new(io::ErrorKind::ConnectionAborted,
                                                   "handshake failed"))))
        };
        reply.reply_for(message.transaction().unwrap(), true);
        socket.send(reply).await?;
        result
    }
}

/// Helpers for creating `CtlCommand`s in a `SrvHandler` for a given
/// Supervisor protocol message.
///
/// This is only intended for reducing current code redundancies. At
/// some point the entire architecture of this interaction should be
/// revisited (it feels like there are too many layers of indirection
/// at play here).
mod util {
    use super::{CtlCommand,
                CtlSender,
                HandlerError};
    use crate::{ctl_gateway::CtlRequest,
                manager::{ManagerState,
                          action::ActionSender}};
    use habitat_sup_protocol::{codec::SrvMessage,
                               message::MessageStatic,
                               net::NetResult};
    use prost::Message;

    /// Helper function to capture the creation of a CtlCommand for an
    /// action that communicates with the Supervisor via an
    /// `ActionSender`.
    pub(super) fn to_supervisor_command<T, F>(msg: &SrvMessage,
                                              ctl_sender: CtlSender,
                                              callback: F)
                                              -> std::result::Result<CtlCommand, HandlerError>
        where T: Message + MessageStatic + Default + Clone + 'static,
              F: Fn(&ManagerState, &mut CtlRequest, T, &ActionSender) -> NetResult<()>
                  + Send
                  + 'static
    {
        let m = msg.parse::<T>().map_err(HandlerError::from)?;
        Ok(CtlCommand::new(ctl_sender,
                           msg.transaction(),
                           move |state, req, action_sender| {
                               callback(state, req, m.clone(), &action_sender)
                           }))
    }

    /// Helper function to capture the creation of a CtlCommand for an
    /// action that DOES NOT communicate with the Supervisor via an
    /// `ActionSender`.
    pub(super) fn to_command<T, F>(msg: &SrvMessage,
                                   ctl_sender: CtlSender,
                                   callback: F)
                                   -> std::result::Result<CtlCommand, HandlerError>
        where T: Message + MessageStatic + Default + Clone + 'static,
              F: Fn(&ManagerState, &mut CtlRequest, T) -> NetResult<()> + Send + 'static
    {
        let m = msg.parse::<T>().map_err(HandlerError::from)?;
        Ok(CtlCommand::new(ctl_sender,
                           msg.transaction(),
                           move |state, req, _action_sender| {
                               callback(state, req, m.clone())
                           }))
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvMessage` replies.
#[must_use = "futures do nothing unless polled"]
#[pin_project]
struct SrvHandler {
    #[pin]
    io:           SrvStream,
    state:        SrvHandlerState,
    mgr_sender:   MgrSender,
    ctl_receiver: CtlReceiver,
    ctl_sender:   CtlSender,
    timer:        Option<HistogramTimer>,
}

impl SrvHandler {
    fn new(io: SrvStream, mgr_sender: MgrSender) -> Self {
        let (ctl_sender, ctl_receiver) = mpsc::unbounded();

        SrvHandler { io,
                     state: SrvHandlerState::Receiving,
                     mgr_sender,
                     ctl_receiver,
                     ctl_sender,
                     timer: None }
    }

    /// # Locking (see locking.md)
    /// * `GatewayState::inner` (read)
    /// * `ManagerServices::inner` (read)
    async fn command_from_message_gsr_msr(msg: &SrvMessage,
                                          ctl_sender: CtlSender)
                                          -> std::result::Result<CtlCommand, HandlerError> {
        match msg.message_id() {
            "SvcGetDefaultCfg" => util::to_command(msg, ctl_sender, commands::service_cfg_msr),
            "SvcFilePut" => util::to_command(msg, ctl_sender, commands::service_file_put),
            "SvcSetCfg" => util::to_command(msg, ctl_sender, commands::service_cfg_set),
            "SvcValidateCfg" => util::to_command(msg, ctl_sender, commands::service_cfg_validate),
            "SvcLoad" => {
                // This arm doesn't use a `util` module helper because
                // it's currently the only thing that behaves like
                // this.
                let m = msg.parse::<protocol::ctl::SvcLoad>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       // To avoid significant architecture changes to `CtlCommand,`
                                       // block on the load service future because futures cannot
                                       // be awaited in a closure. It is safe to use
                                       // `block_in_place` here because it is called within a
                                       // spawned future.
                                       task::block_in_place(|| {
                                           executor::block_on(commands::service_load(state,
                                                                                     req,
                                                                                     m.clone()))
                                       })
                                   }))
            }
            "SvcUpdate" => util::to_supervisor_command(msg, ctl_sender, commands::service_update),
            "SvcUnload" => util::to_supervisor_command(msg, ctl_sender, commands::service_unload),
            "SvcStart" => util::to_command(msg, ctl_sender, commands::service_start),
            "SvcStop" => util::to_supervisor_command(msg, ctl_sender, commands::service_stop),
            "SvcStatus" => util::to_command(msg, ctl_sender, commands::service_status_gsr),
            "SupDepart" => util::to_command(msg, ctl_sender, commands::supervisor_depart),
            "SupRestart" => util::to_command(msg, ctl_sender, commands::supervisor_restart),
            _ => {
                warn!("Unhandled message, {}", msg.message_id());
                Err(HandlerError::from(io::Error::from(io::ErrorKind::InvalidData)))
            }
        }
    }

    fn start_timer(&mut self, label: &str) {
        let label_values = &[label];
        RPC_CALLS.with_label_values(label_values).inc();
        let timer = RPC_CALL_DURATION.with_label_values(label_values)
                                     .start_timer();
        self.timer = Some(timer);
    }
}

impl Future for SrvHandler {
    type Output = Result<(), HandlerError>;

    /// # Locking (see locking.md)
    /// * `GatewayState::inner` (read)
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match self.state {
                SrvHandlerState::Receiving => {
                    match ready!(self.io.poll_next_unpin(cx)) {
                        None => {
                            break;
                        }
                        Some(Ok(msg)) => {
                            self.start_timer(msg.message_id());
                            trace!("OnMessage, {}", msg.message_id());

                            let fut =
                                Self::command_from_message_gsr_msr(&msg, self.ctl_sender.clone());
                            tokio::pin!(fut);
                            let cmd = match futures::ready!(fut.poll_unpin(cx)) {
                                Ok(cmd) => cmd,
                                Err(_) => {
                                    break;
                                }
                            };
                            if let Err(err) = futures::ready!(self.mgr_sender.poll_ready(cx)) {
                                return Poll::Ready(Err(HandlerError::from(err)));
                            }
                            match self.mgr_sender.start_send(cmd) {
                                Ok(()) => {
                                    self.state = SrvHandlerState::Sending;
                                    continue;
                                }
                                Err(err) => {
                                    // An error here means that the
                                    // receiving end of this channel went
                                    // away.
                                    //
                                    // Most often, this will be because
                                    // we're in the middle of an orderly
                                    // shutdown and no longer wish to
                                    // process incoming commands.
                                    warn!("ManagerReceiver err: {}", err);
                                    return Poll::Ready(Err(HandlerError::from(err)));
                                }
                            }
                        }
                        Some(Err(err)) => {
                            error!("SrvHandler failed to receive message, err: {}", err);
                            return Poll::Ready(Err(HandlerError::from(err)));
                        }
                    }
                }
                SrvHandlerState::Sending => {
                    match futures::ready!(self.ctl_receiver.poll_next_unpin(cx)) {
                        Some(msg) => {
                            trace!("MgrSender -> SrvHandler, {:?}", msg);
                            if msg.is_complete() {
                                self.state = SrvHandlerState::Sent;
                            }
                            if let Err(err) =
                                futures::ready!(self.as_mut().project().io.poll_ready(cx))
                            {
                                return Poll::Ready(Err(HandlerError::from(err)));
                            }
                            match self.as_mut().project().io.start_send(msg) {
                                Ok(()) => {
                                    if let Err(err) =
                                        futures::ready!(self.as_mut().project().io.poll_flush(cx))
                                    {
                                        return Poll::Ready(Err(HandlerError::from(err)));
                                    }
                                    continue;
                                }
                                Err(e) if e.kind() == ::std::io::ErrorKind::WouldBlock => {
                                    return Poll::Pending;
                                }
                                Err(err) => return Poll::Ready(Err(HandlerError::from(err))),
                            }
                        }
                        None => self.state = SrvHandlerState::Sent,
                    }
                }
                SrvHandlerState::Sent => {
                    if let Some(timer) = self.timer.take() {
                        timer.observe_duration();
                    }
                    if let Err(err) = futures::ready!(Pin::new(self.io.get_mut()).poll_shutdown(cx))
                    {
                        return Poll::Ready(Err(HandlerError::from(err)));
                    }
                    trace!("OnMessage complete");
                    break;
                }
            }
        }
        Poll::Ready(Ok(()))
    }
}

enum SrvHandlerState {
    /// Handler is Receiving/Waiting for message from client.
    Receiving,
    /// Handler has sent a request to the Manager and is streaming replies back to the client
    /// socket.
    Sending,
    /// All messages have been sent to the client and the Handler is now flushing the connection.
    Sent,
}

struct SrvState {
    secret_key: String,
    mgr_sender: MgrSender,
}

pub(crate) struct CtlGatewayServer {
    pub(crate) listen_addr:         SocketAddr,
    pub(crate) secret_key:          String,
    pub(crate) mgr_sender:          MgrSender,
    pub(crate) server_certificates: Option<Vec<CertificateDer<'static>>>,
    pub(crate) server_key:          Option<PrivatePkcs8KeyDer<'static>>,
    pub(crate) client_certificates: Option<RootCertStore>,
}

impl CtlGatewayServer {
    /// Start a new thread which will run the CtlGateway server.
    ///
    /// New connections will be authenticated using `secret_key`. Messages from the main thread
    /// will be sent over the channel `mgr_sender`.
    pub async fn run(self) {
        let Self { listen_addr,
                   secret_key,
                   mgr_sender,
                   server_certificates,
                   server_key,
                   client_certificates, } = self;

        let state = SrvState { secret_key,
                               mgr_sender };
        let state = Arc::new(Mutex::new(state));
        let listener =
            TcpListener::bind(&listen_addr).await
                                           .expect("Could not bind ctl gateway listen address!");

        let maybe_tls_config = Self::maybe_tls_config(server_certificates,
                                                      server_key,
                                                      client_certificates).map(Arc::new);
        loop {
            let tcp_stream = listener.accept().await;
            match tcp_stream {
                Ok((tcp_stream, _)) => {
                    let addr = match tcp_stream.peer_addr() {
                        Ok(addr) => addr,
                        Err(e) => {
                            error!("Client peer address not available from socket, err {}", e);
                            continue;
                        }
                    };

                    // Upgrade to a TLS connection if necessary
                    let tcp_stream = if let Some(tls_config) = &maybe_tls_config {
                        match TcpOrTlsStream::new_tls_server(tcp_stream, Arc::clone(tls_config))
                            .await
                        {
                            Ok(tcp_stream) => tcp_stream,
                            Err((e, tcp_stream)) => {
                                error!("Failed to accept TLS client connection, err {}", e);
                                // If the client sent a corrupt TLS message it is a good indicator that
                                // they did not upgrade to TLS. In this case send back an error response.
                                // We do not always send back an error response because it can lead to
                                // confusing error messages on the client.
                                #[allow(clippy::redundant_closure_for_method_calls)]
                                if let Some(&rustls::Error::InvalidMessage(_)) =
                                    e.get_ref().and_then(|e| e.downcast_ref())
                                {
                                    let mut srv_codec = SrvCodec::new().framed(tcp_stream);
                                    let net_err = net::err(
                                        ErrCode::TlsHandshakeFailed,
                                        format!("TLS handshake failed, err: {}", e),
                                    );
                                    if let Err(e) = srv_codec.send(SrvMessage::from(net_err)).await
                                    {
                                        error!(
                                            "Failed to send TLS failure message to client, err {}",
                                            e
                                        );
                                    }
                                }
                                continue;
                            }
                        }
                    } else {
                        TcpOrTlsStream::new(tcp_stream)
                    };

                    let srv_codec = SrvCodec::new().framed(tcp_stream);
                    let client = Client { state: Arc::clone(&state), };
                    tokio::spawn(async move {
                        let res = client.serve(srv_codec).await;
                        debug!("DISCONNECTED from {:?} with result {:?}", addr, res);
                    });
                }
                Err(e) => error!("SrvHandler failed to connect, err: {}", e),
            }
        }
    }

    fn maybe_tls_config(server_certificates: Option<Vec<CertificateDer<'static>>>,
                        server_key: Option<PrivatePkcs8KeyDer<'static>>,
                        client_certificates: Option<RootCertStore>)
                        -> Option<TlsServerConfig> {
        if let Some(server_key) = server_key {
            let client_auth = if let Some(client_certificates) = client_certificates {
                debug!("Upgrading ctl-gateway to TLS with client authentication");
                WebPkiClientVerifier::builder(client_certificates.into()).build()
                                                                         .unwrap()
            } else {
                debug!("Upgrading ctl-gateway to TLS");
                WebPkiClientVerifier::no_client_auth()
            };
            let tls_config =
                TlsServerConfig::builder().with_client_cert_verifier(client_auth)
                                          .with_single_cert(server_certificates.unwrap_or_default(),
                                                            PrivateKeyDer::Pkcs8(server_key))
                                          .expect("Could not set certificate for ctl gateway!");

            Some(tls_config)
        } else {
            None
        }
    }
}
