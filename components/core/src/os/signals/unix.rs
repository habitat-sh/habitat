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

//! Traps and notifies UNIX signals.

use crate::os::process::{Signal,
                         SignalCode};
use std::{collections::VecDeque,
          sync::{atomic::Ordering,
                 Mutex,
                 Once,
                 ONCE_INIT}};

static INIT: Once = ONCE_INIT;

lazy_static::lazy_static! {
    static ref CAUGHT_SIGNALS: Mutex<VecDeque<SignalCode>> = Mutex::new(VecDeque::new());
}

// Functions from POSIX libc.
extern "C" {
    fn signal(sig: SignalCode,
              cb: unsafe extern "C" fn(SignalCode))
              -> unsafe extern "C" fn(SignalCode);
}

unsafe extern "C" fn handle_signal(signal: SignalCode) {
    CAUGHT_SIGNALS.lock()
                  .expect("Signal mutex poisoned")
                  .push_back(signal);
}

unsafe extern "C" fn handle_shutdown_signal(_signal: SignalCode) {
    super::SHUTDOWN.store(true, Ordering::SeqCst);
}

pub fn init() {
    INIT.call_once(|| {
            self::set_signal_handlers();
        });
}

pub enum SignalEvent {
    WaitForChild,
    Passthrough(Signal),
}

/// Consumers should call this function fairly frequently and since the vast
/// majority of the time there is at most one signal event waiting, we return
/// at most one. If multiple signals have been received since the last call,
/// they will be returned, one per call in the order they were received.
pub fn check_for_signal() -> Option<SignalEvent> {
    let mut signals = CAUGHT_SIGNALS.lock().expect("Signal mutex poisoned");

    if let Some(code) = signals.pop_front() {
        match from_signal_code(code) {
            Some(Signal::CHLD) => Some(SignalEvent::WaitForChild),
            Some(signal) => Some(SignalEvent::Passthrough(signal)),
            None => {
                println!("Received invalid signal: #{}", code);
                None
            }
        }
    } else {
        None
    }
}

fn set_signal_handlers() {
    unsafe {
        signal(libc::SIGINT, handle_shutdown_signal);
        signal(libc::SIGTERM, handle_shutdown_signal);

        signal(libc::SIGHUP, handle_signal);
        signal(libc::SIGQUIT, handle_signal);
        signal(libc::SIGALRM, handle_signal);
        signal(libc::SIGUSR1, handle_signal);
        signal(libc::SIGUSR2, handle_signal);
        signal(libc::SIGCHLD, handle_signal);
    }
}

/// These are the signals that we can eventually translate into
/// some kind of event
fn from_signal_code(code: SignalCode) -> Option<Signal> {
    match code {
        libc::SIGHUP => Some(Signal::HUP),
        libc::SIGCHLD => Some(Signal::CHLD),
        _ => None,
    }
}
