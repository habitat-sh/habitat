//! Custom futures used throughout the Supervisor.

use futures::{future::Either,
              sync::oneshot,
              Future,
              IntoFuture};

/// A handle attached to a future generated using
/// `cancelable_future`. That future will be dropped when this handle
/// is dropped.
#[derive(Debug)]
pub struct FutureHandle(oneshot::Sender<()>);

impl FutureHandle {
    /// A more explicit way to cancel a future instead of relying on
    /// dropping the handle.
    pub fn terminate(self) { let _ = self.0.send(()); }
}

// TODO (CM): I'm not super thrilled with the error type being
// Option<E> here, but it should be "fine" for now.
/// Wrap a future in another future which allows the entire thing to
/// be dropped if a `Handle` to the future is dropped.
///
/// This is useful for retaining control of long-running futures.
pub fn cancelable_future<F, I, E>(fut: F)
                                  -> (FutureHandle, impl Future<Item = I, Error = Option<E>>)
    where F: IntoFuture<Item = I, Error = E>
{
    let (tx, rx) = oneshot::channel();
    let cancelable =
        fut.into_future().select2(rx).then(|res| {
                                         match res {
                                             Ok(Either::A((f_ok, _rx))) => Ok(f_ok),
                                             Err(Either::A((f_err, _rx))) => Err(Some(f_err)),
                                             Ok(Either::B((..))) => {
                                                 trace!("Handle signalled; canceling future");
                                                 Err(None)
                                             }
                                             Err(Either::B((..))) => {
                                                 trace!("Handle dropped; canceling future");
                                                 Err(None)
                                             }
                                         }
                                     });

    (FutureHandle(tx), cancelable)
}
