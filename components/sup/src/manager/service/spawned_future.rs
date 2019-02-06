// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Encapsulate a pattern where we spawn a task on a separate thread,
//! but need to represent the failure to spawn that thread, as well as
//! the return value of that thread's computation as a single Future.
//!
//! It's essentially a tailored version `futures::FutureResult`,
//! except that it polls its `Ok` variant.

use crate::error::SupError;
use futures::{sync::oneshot, Future, Poll};
use std::io;

pub enum SpawnedFuture<T> {
    Ok(oneshot::Receiver<T>),
    Err(Option<io::Error>),
}

impl<T> From<oneshot::Receiver<T>> for SpawnedFuture<T> {
    fn from(r: oneshot::Receiver<T>) -> SpawnedFuture<T> {
        SpawnedFuture::Ok(r)
    }
}

impl<T> From<io::Error> for SpawnedFuture<T> {
    fn from(e: io::Error) -> SpawnedFuture<T> {
        SpawnedFuture::Err(Some(e))
    }
}

impl<T> Future for SpawnedFuture<T> {
    type Item = T;
    type Error = SupError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            SpawnedFuture::Ok(r) => r.poll().map_err(Into::into),
            SpawnedFuture::Err(e) => Err(e
                .take()
                .expect("Cannot poll SpawnedFuture::Err twice")
                .into()),
        }
    }
}
