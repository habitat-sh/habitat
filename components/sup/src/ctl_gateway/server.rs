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
use crate::manager::{action::ActionSender,
                     commands,
                     ManagerState};
use futures::{channel::mpsc,
              prelude::*,
              ready,
              task::{Context,
                     Poll}};
use habitat_common::liveliness_checker;
use habitat_core::crypto;
use habitat_sup_protocol::{self as protocol,
                           codec::{SrvCodec,
                                   SrvMessage,
                                   SrvStream,
                                   SrvTxn},
                           net::{self,
                                 ErrCode,
                                 NetErr,
                                 NetResult}};
use prometheus::{HistogramTimer,
                 HistogramVec,
                 IntCounterVec};
use prost;
use std::{error,
          fmt,
          io,
          net::SocketAddr,
          pin::Pin,
          sync::{Arc,
                 Mutex},
          time::Duration};
use tokio::{net::TcpListener,
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
    // This is now possible see https://github.com/habitat-sh/habitat/issues/6832
    // We held off on making the change to reduce the risk of a regression and to lump it in with
    // more general Future refactoring.
    fun: Box<dyn Fn(&ManagerState, &mut CtlRequest, ActionSender) -> NetResult<()> + Send>,
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
        let secret_key = self.state
                             .lock()
                             .expect("SrvState mutex poisoned")
                             .secret_key
                             .to_string();
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
                    let decoded_key = decoded.secret_key.unwrap_or_default();
                    crypto::secure_eq(decoded_key, secret_key)
                }
                Err(err) => {
                    warn!("Handshake error, {:?}", err);
                    return Err(HandlerError::from(io::Error::from(io::ErrorKind::ConnectionAborted)));
                }
            }
        };
        let mut reply = if success {
            SrvMessage::from(net::ok())
        } else {
            SrvMessage::from(net::err(ErrCode::Unauthorized, "secret key mismatch"))
        };
        reply.reply_for(message.transaction().unwrap(), true);
        socket.send(reply).await?;
        Ok(())
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvMessage` replies.
#[must_use = "futures do nothing unless polled"]
struct SrvHandler {
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
    fn command_from_message_gsr_msr(msg: &SrvMessage,
                                    ctl_sender: CtlSender)
                                    -> std::result::Result<CtlCommand, HandlerError> {
        match msg.message_id() {
            "SvcGetDefaultCfg" => {
                let m = msg.parse::<protocol::ctl::SvcGetDefaultCfg>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_cfg_msr(state, req, m.clone())
                                   }))
            }
            "SvcFilePut" => {
                let m = msg.parse::<protocol::ctl::SvcFilePut>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_file_put(state, req, m.clone())
                                   }))
            }
            "SvcSetCfg" => {
                let m = msg.parse::<protocol::ctl::SvcSetCfg>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_cfg_set(state, req, m.clone())
                                   }))
            }
            "SvcValidateCfg" => {
                let m = msg.parse::<protocol::ctl::SvcValidateCfg>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_cfg_validate(state, req, m.clone())
                                   }))
            }
            "SvcLoad" => {
                let m = msg.parse::<protocol::ctl::SvcLoad>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_load(state, req, m.clone())
                                   }))
            }
            "SvcUnload" => {
                let m = msg.parse::<protocol::ctl::SvcUnload>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, action_sender| {
                                       commands::service_unload(state,
                                                                req,
                                                                m.clone(),
                                                                &action_sender)
                                   }))
            }
            "SvcStart" => {
                let m = msg.parse::<protocol::ctl::SvcStart>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_start(state, req, m.clone())
                                   }))
            }
            "SvcStop" => {
                let m = msg.parse::<protocol::ctl::SvcStop>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, action_sender| {
                                       commands::service_stop(state, req, m.clone(), &action_sender)
                                   }))
            }
            "SvcStatus" => {
                let m = msg.parse::<protocol::ctl::SvcStatus>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::service_status_gsr(state, req, m.clone())
                                   }))
            }
            "SupDepart" => {
                let m = msg.parse::<protocol::ctl::SupDepart>()
                           .map_err(HandlerError::from)?;
                Ok(CtlCommand::new(ctl_sender,
                                   msg.transaction(),
                                   move |state, req, _action_sender| {
                                       commands::supervisor_depart(state, req, m.clone())
                                   }))
            }
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
        let _: liveliness_checker::ThreadUnregistered = loop {
            let checked_thread = liveliness_checker::mark_thread_alive();
            match self.state {
                SrvHandlerState::Receiving => {
                    match ready!(self.io.poll_next_unpin(cx)) {
                        None => {
                            break checked_thread.unregister(Ok(()));
                        }
                        Some(Ok(msg)) => {
                            self.start_timer(&msg.message_id());
                            trace!("OnMessage, {}", msg.message_id());

                            let cmd = match Self::command_from_message_gsr_msr(&msg,
                                                                               self.ctl_sender
                                                                                   .clone())
                            {
                                Ok(cmd) => cmd,
                                Err(_) => {
                                    break checked_thread.unregister(Ok(()));
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
                            if let Err(err) = futures::ready!(Pin::new(&mut self.io).poll_ready(cx))
                            {
                                return Poll::Ready(Err(HandlerError::from(err)));
                            }
                            match Pin::new(&mut self.io).start_send(msg) {
                                Ok(()) => {
                                    if let Err(err) =
                                        futures::ready!(Pin::new(&mut self.io).poll_flush(cx))
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
                    trace!("OnMessage complete");
                    break checked_thread.unregister(Ok(()));
                }
            }
        };
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

/// Start a new thread which will run the CtlGateway server.
///
/// New connections will be authenticated using `secret_key`. Messages from the main thread
/// will be sent over the channel `mgr_sender`.
pub async fn run(listen_addr: SocketAddr, secret_key: String, mgr_sender: MgrSender) {
    let state = SrvState { secret_key,
                           mgr_sender };
    let state = Arc::new(Mutex::new(state));
    let mut listner =
        TcpListener::bind(&listen_addr).await
                                       .expect("Could not bind ctl gateway listen address!");
    let mut incoming = listner.incoming();
    while let Some(tcp_stream) = incoming.next().await {
        match tcp_stream {
            Ok(tcp_stream) => {
                let addr = tcp_stream.peer_addr().expect("Couldn't get peer address!");
                let io = SrvCodec::new().framed(tcp_stream);
                let client = Client { state: Arc::clone(&state), };
                tokio::spawn(async move {
                    let res = client.serve(io).await;
                    debug!("DISCONNECTED from {:?} with result {:?}", addr, res);
                });
            }
            Err(e) => error!("SrvHandler failed to connect, err: {}", e),
        }
    }
}
