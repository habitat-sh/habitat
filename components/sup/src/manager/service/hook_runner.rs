//! Runs a service lifecycle hook on a separate thread, and wraps the
//! whole execution in a future.
//!
//! Ideally, we'd want to use something like
//! [tokio_process](https://github.com/alexcrichton/tokio-process),
//! but we're not able to use that based on how our Windows hooks get
//! executed. If that were to be re-cast in terms of Rust's
//! `std::process::Command`, we could consider it. In the meantime,
//! this seems to do the trick.

use super::{Pkg,
            hook_timer};
use crate::error::{Error,
                   Result};
use habitat_common::templating::hooks::Hook;
use habitat_core::service::ServiceGroup;
use log::{debug,
          error};
use std::{clone::Clone,
          sync::Arc,
          time::{Duration,
                 Instant}};
use tokio::task;

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

    pub async fn retryable_future(self) {
        loop {
            match self.clone().into_future().await {
                Ok((exit_value, _duration)) => {
                    if H::should_retry(&exit_value) {
                        debug!("Retrying the '{}' hook", H::FILE_NAME);
                    } else {
                        break;
                    }
                }
                Err(e) => error!("Error running the '{}' hook: {:?}", H::FILE_NAME, e),
            }
        }
    }

    pub async fn into_future(self) -> Result<(H::ExitValue, Duration)> {
        // TODO (CM): May want to consider adding a configurable
        // timeout to how long this hook is allowed to run.
        task::spawn_blocking(move || {
            // _timer is for Prometheus metrics, but we also want
            // the runtime for other purposes. Unfortunately,
            // we're not able to use the same timer for both :(
            let _timer = hook_timer(H::FILE_NAME);
            let start = Instant::now();
            let result = self.hook
                             .run(&self.service_group, &self.pkg, self.passwd.as_ref());
            let run_time = start.elapsed();
            let exit_value = result.map_err(|e| Error::from(e).with_duration(run_time))?;
            Ok((exit_value, run_time))
        }).await?
    }
}
