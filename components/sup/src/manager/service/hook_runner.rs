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

//! Runs a service lifecycle hook on a separate thread, and wraps the
//! whole execution in a future.
//!
//! Ideally, we'd want to use something like
//! [tokio_process](https://github.com/alexcrichton/tokio-process),
//! but we're not able to use that based on how our Windows hooks get
//! executed. If that were to be re-cast in terms of Rust's
//! `std::process::Command`, we could consider it. In the meantime,
//! this seems to do the trick.

use super::{hook_timer, spawned_future::SpawnedFuture, Pkg};
use crate::error::SupError;
use futures::{sync::oneshot, IntoFuture};
use habitat_common::templating::hooks::Hook;
use habitat_core::service::ServiceGroup;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct HookRunner<H: Hook> {
    hook: Arc<Mutex<H>>,
    sg: ServiceGroup,
    pkg: Pkg,
    passwd: Option<String>,
}

impl<H: Hook> HookRunner<H> {
    pub fn new(
        hook: Arc<Mutex<H>>,
        sg: ServiceGroup,
        pkg: Pkg,
        passwd: Option<String>,
    ) -> HookRunner<H> {
        HookRunner {
            hook: hook,
            sg: sg,
            pkg: pkg,
            passwd: passwd,
        }
    }
}
impl<H: Hook + 'static> IntoFuture for HookRunner<H> {
    type Item = H::ExitValue;
    type Error = SupError;
    type Future = SpawnedFuture<Self::Item>;

    fn into_future(self) -> Self::Future {
        let (tx, rx) = oneshot::channel();

        // TODO (CM): Consider using a short abbreviation for the hook
        // name in the thread name (e.g. "HC" for "health_check", "I"
        // for "init", etc.

        // TODO (CM): May want to consider adding a configurable
        // timeout to how long this hook is allowed to run.
        let handle_result = thread::Builder::new()
            .name(format!("{}-{}", H::file_name(), self.sg))
            .spawn(move || {
                let hook = self.hook.lock().expect("Hook lock poisoned");
                let _timer = hook_timer(H::file_name());
                let exit_value = hook.run(&self.sg, &self.pkg, self.passwd.as_ref());
                tx.send(exit_value)
                    .expect("Couldn't send oneshot signal from HookRunner: receiver went away");
            });

        match handle_result {
            Ok(_handle) => rx.into(),
            Err(io_err) => io_err.into(),
        }
    }
}
