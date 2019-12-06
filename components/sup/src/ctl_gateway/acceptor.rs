//! Future for mediating the processing of commands received from the
//! CtlGateway in the Supervisor.

use super::handler::CtlHandler;
use crate::{ctl_gateway::server::MgrReceiver,
            manager::{action::ActionSender,
                      ManagerState}};
use futures::{channel::oneshot,
              future::FutureExt,
              stream::{Stream,
                       StreamExt},
              task::{Context,
                     Poll}};
use std::{pin::Pin,
          sync::Arc};

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
    type Item = CtlHandler;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.shutdown_trigger.poll_unpin(cx) {
            Poll::Ready(Ok(())) => {
                info!("Signal received; stopping CtlAcceptor");
                Poll::Ready(None)
            }
            Poll::Ready(Err(e)) => {
                error!("Error polling CtlAcceptor shutdown trigger: {}", e);
                Poll::Ready(None)
            }
            Poll::Pending => {
                match futures::ready!(self.mgr_receiver.poll_next_unpin(cx)) {
                    Some(cmd) => {
                        let task =
                            CtlHandler::new(cmd, self.state.clone(), self.action_sender.clone());
                        Poll::Ready(Some(task))
                    }
                    None => Poll::Ready(None),
                }
            }
        }
    }
}
