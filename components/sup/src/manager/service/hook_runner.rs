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
            spawned_future::SpawnedFuture,
            Pkg};
use crate::{error::Error,
            sup_futures::{self,
                          FutureHandle}};
use futures::{future::{self,
                       Loop},
              sync::oneshot,
              Future,
              IntoFuture};
use habitat_common::templating::hooks::Hook;
use habitat_core::service::ServiceGroup;
use std::{clone::Clone,
          sync::Arc,
          thread,
          time::{Duration,
                 Instant}};

pub struct HookRunner<H: Hook + Sync> {
    hook:          Arc<H>,
    service_group: ServiceGroup,
    pkg:           Pkg,
    passwd:        Option<String>,
}

// We cannot use `#[derive(Clone)]` here because it unnecessarily requires `H` to be
// `Clone`. See https://github.com/rust-lang/rust/issues/44151.
impl<H: Hook + Sync> Clone for HookRunner<H> {
    fn clone(&self) -> Self {
        Self { hook:          self.hook.clone(),
               service_group: self.service_group.clone(),
               pkg:           self.pkg.clone(),
               passwd:        self.passwd.clone(), }
    }
}

impl<H> HookRunner<H> where H: Hook + Sync + 'static
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

    pub fn retryable_future(&self) -> (FutureHandle, impl Future<Item = (), Error = ()>) {
        let f = future::loop_fn(self.clone(), |hook_runner| {
            hook_runner.clone()
                       .into_future()
                       .map(move |(maybe_exit_value, _duration)| {
                           // If we did not get an exit value always retry
                           if maybe_exit_value.as_ref()
                                              .map(H::should_retry)
                                              .unwrap_or(true)
                           {
                               debug!("retrying the '{}' hook", H::file_name());
                               Loop::Continue(hook_runner)
                           } else {
                               Loop::Break(())
                           }
                       })
                       .map_err(|_| ())
        });
        let (handle, f) = sup_futures::cancelable_future(f);
        (handle, f.map_err(|_| ()))
    }
}

impl<H: Hook + Sync + 'static> IntoFuture for HookRunner<H> {
    type Error = Error;
    type Future = SpawnedFuture<Self::Item>;
    type Item = (Option<H::ExitValue>, Duration);

    fn into_future(self) -> Self::Future {
        let (tx, rx) = oneshot::channel();

        // TODO (CM): Consider using a short abbreviation for the hook
        // name in the thread name (e.g. "HC" for "health_check", "I"
        // for "init", etc.

        // TODO (CM): May want to consider adding a configurable
        // timeout to how long this hook is allowed to run.
        let handle_result =
            thread::Builder::new().name(format!("{}-{}", H::file_name(), self.service_group))
                                  .spawn(move || {
                                      // _timer is for Prometheus metrics, but we also want
                                      // the runtime for other purposes. Unfortunately,
                                      // we're not able to use the same timer for both :(
                                      let _timer = hook_timer(H::file_name());
                                      let start = Instant::now();
                                      let exit_value = self.hook.run(&self.service_group,
                                                                     &self.pkg,
                                                                     self.passwd.as_ref());
                                      let run_time = start.elapsed();
                                      tx.send((exit_value, run_time))
                                        .expect("Couldn't send oneshot signal from HookRunner: \
                                                 receiver went away");
                                  });

        match handle_result {
            Ok(_handle) => rx.into(),
            Err(io_err) => io_err.into(),
        }
    }
}
