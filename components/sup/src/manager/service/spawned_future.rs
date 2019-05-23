//! Encapsulate a pattern where we spawn a task on a separate thread,
//! but need to represent the failure to spawn that thread, as well as
//! the return value of that thread's computation as a single Future.
//!
//! It's essentially a tailored version `futures::FutureResult`,
//! except that it polls its `Ok` variant.

use crate::error::Error;
use futures::{sync::oneshot,
              Future,
              Poll};
use std::{io,
          result};

pub struct SpawnedFuture<T>(result::Result<oneshot::Receiver<T>, Option<io::Error>>);

impl<T> From<oneshot::Receiver<T>> for SpawnedFuture<T> {
    fn from(r: oneshot::Receiver<T>) -> SpawnedFuture<T> { SpawnedFuture(Ok(r)) }
}

impl<T> From<io::Error> for SpawnedFuture<T> {
    fn from(e: io::Error) -> SpawnedFuture<T> { SpawnedFuture(Err(Some(e))) }
}

impl<T> Future for SpawnedFuture<T> {
    type Error = Error;
    type Item = T;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            SpawnedFuture(Ok(r)) => r.poll().map_err(Into::into),
            SpawnedFuture(Err(e)) => {
                Err(e.take()
                     .expect("Cannot poll SpawnedFuture::Err twice")
                     .into())
            }
        }
    }
}
