// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

#[cfg(windows)]
pub mod windows_child;

#[allow(unused_variables)]
#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub(crate) use self::unix::SignalCode;
#[cfg(unix)]
pub use self::unix::{become_command,
                     current_pid,
                     is_alive,
                     signal,
                     Pid,
                     ShutdownSignal,
                     Signal};
#[cfg(windows)]
pub use self::windows::{become_command,
                        current_pid,
                        handle_from_pid,
                        is_alive,
                        Pid};
use crate::error::Error;
use std::{fmt,
          str::FromStr};
use time::Duration;

/// This type encapsulates the number of seconds we should wait after
/// send a shutdown signal to a process before we kill it.
///
/// (Another nice thing it does is hide the whole
/// `std::time::Duration` / `time::Duration` mess from the rest of the
/// code. Rather than having to juggle the two `Duration` types
/// throughout our code, which can be confusing, we can just pass this
/// around, and turn it into a `time::Duration` at the last possible
/// moment.)
#[derive(Debug, Clone)]
pub struct ShutdownTimeout(u32);

impl Default for ShutdownTimeout {
    /// Unless otherwise specified, the Supervisor will wait 8 seconds
    /// for a service to finish shutting down before killing it.
    fn default() -> Self { 8.into() }
}

impl FromStr for ShutdownTimeout {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(ShutdownTimeout(s.parse()?)) }
}

impl fmt::Display for ShutdownTimeout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<u32> for ShutdownTimeout {
    fn from(seconds: u32) -> Self { ShutdownTimeout(seconds) }
}

impl From<ShutdownTimeout> for u32 {
    fn from(timeout: ShutdownTimeout) -> Self { timeout.0 }
}

impl From<ShutdownTimeout> for Duration {
    fn from(timeout: ShutdownTimeout) -> Self { Duration::seconds(timeout.0.into()) }
}
