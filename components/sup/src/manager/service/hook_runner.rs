//! Runs a service lifecycle hook on a separate thread, and wraps the
//! whole execution in a future.
//!
//! Ideally, we'd want to use something like
//! [tokio_process](https://github.com/alexcrichton/tokio-process),
//! but we're not able to use that based on how our Windows hooks get
//! executed. If that were to be re-cast in terms of Rust's
//! `std::process::Command`, we could consider it. In the meantime,
//! this seems to do the trick.

use super::{hook_timer,
            Pkg};
use crate::error::Error;
use futures::{future::{self,
                       FutureResult,
                       Lazy},
              IntoFuture};
use habitat_common::templating::hooks::Hook;
use habitat_core::service::ServiceGroup;
use std::{sync::Arc,
          time::{Duration,
                 Instant}};

pub struct HookRunner<H: Hook> {
    hook:          Arc<H>,
    service_group: ServiceGroup,
    pkg:           Pkg,
    passwd:        Option<String>,
}

impl<H> HookRunner<H> where H: Hook
{
    pub fn new(hook: Arc<H>,
               service_group: ServiceGroup,
               pkg: Pkg,
               passwd: Option<String>)
               -> HookRunner<H> {
        HookRunner { hook,
                     service_group,
                     pkg,
                     passwd }
    }
}

type LazyClosure<I, E> = Lazy<Box<dyn FnOnce() -> FutureResult<I, E> + Send>, FutureResult<I, E>>;

impl<H: Hook + 'static> IntoFuture for HookRunner<H> {
    type Error = Error;
    type Future = LazyClosure<Self::Item, Self::Error>;
    type Item = (H::ExitValue, Duration);

    fn into_future(self) -> Self::Future {
        // TODO (CM): May want to consider adding a configurable
        // timeout to how long this hook is allowed to run.
        future::lazy(Box::new(move || {
                         // _timer is for Prometheus metrics, but we also want
                         // the runtime for other purposes. Unfortunately,
                         // we're not able to use the same timer for both :(
                         let _timer = hook_timer(H::file_name());
                         let start = Instant::now();
                         let exit_value =
                             self.hook
                                 .run(&self.service_group, &self.pkg, self.passwd.as_ref());
                         let run_time = start.elapsed();
                         future::ok((exit_value, run_time))
                     }))
    }
}
