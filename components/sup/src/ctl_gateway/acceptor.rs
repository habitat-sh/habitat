//! Future for mediating the processing of commands received from the
//! CtlGateway in the Supervisor.

use super::handler::CtlHandler;
use crate::{ctl_gateway::server::MgrReceiver,
            manager::{action::ActionSender,
                      ManagerState}};
use futures::{future::Future,
              sync::oneshot,
              Async,
              Poll,
              Stream};
use std::sync::Arc;

pub struct CtlAcceptor {
    /// Communication channel from the control gateway server. User
    /// interactions are received there and then sent here into the
    /// `CtlAcceptor` future for further processing.
    mgr_receiver: MgrReceiver,
    /// Reference to the Supervisor's main state. This is passed into
    /// handlers that need to access, e.g., what services are running,
    /// etc.
    state: Arc<ManagerState>,
    /// Signaling channel for the intention to shut down. A message
    /// received on this channel will cause the `CtlAcceptor` future
    /// stream to terminate.
    shutdown_trigger: oneshot::Receiver<()>,
    /// Communication channel back into the main Supervisor loop. This
    /// is passed into any generated command handlers as a way to
    /// send actions into the Supervisor.
    action_sender: ActionSender,
}

impl CtlAcceptor {
    pub fn new(state: Arc<ManagerState>,
               mgr_receiver: MgrReceiver,
               shutdown_trigger: oneshot::Receiver<()>,
               action_sender: ActionSender)
               -> Self {
        CtlAcceptor { state,
                      mgr_receiver,
                      shutdown_trigger,
                      action_sender }
    }
}

impl Stream for CtlAcceptor {
    type Error = ();
    type Item = CtlHandler;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.shutdown_trigger.poll() {
            Ok(Async::Ready(())) => {
                info!("Signal received; stopping CtlAcceptor");
                Ok(Async::Ready(None))
            }
            Err(e) => {
                error!("Error polling CtlAcceptor shutdown trigger: {:?}", e);
                Ok(Async::Ready(None))
            }
            Ok(Async::NotReady) => {
                match self.mgr_receiver.poll() {
                    Ok(Async::Ready(Some(cmd))) => {
                        let task =
                            CtlHandler::new(cmd, self.state.clone(), self.action_sender.clone());
                        Ok(Async::Ready(Some(task)))
                    }
                    Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    Err(e) => {
                        debug!("CtlAcceptor error, {:?}", e);
                        Err(())
                    }
                }
            }
        }
    }
}
