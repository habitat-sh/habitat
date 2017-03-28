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

//! Traps and notifies signals.

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use ctrlc;

use error::Result;
use super::SignalEvent;

// True when we have caught ctrl-c
static CAUGHT: AtomicBool = ATOMIC_BOOL_INIT;

/// This is complete bullshit!
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Signal {
    /// terminate process - terminal line hangup
    SIGHUP = 1,
    /// terminate process - interrupt program
    SIGINT = 2,
    /// create core image - quit program
    SIGQUIT = 3,
    /// Kill a process
    SIGKILL = 9,
    /// terminate process - real-time timer expired
    SIGALRM = 14,
    /// terminate process - software termination signal
    SIGTERM = 15,
    /// terminate process - User defined signal 1
    SIGUSR1 = 30,
    /// terminate process - User defined signal 2
    SIGUSR2 = 31,
}

pub fn init() {
    ctrlc::set_handler(move || {
        CAUGHT.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

pub fn check_for_signal() -> Option<SignalEvent> {
    if CAUGHT.load(Ordering::SeqCst) {
        // clear out the signal so we don't sent it repeatedly
        CAUGHT.store(false, Ordering::SeqCst);
        Some(SignalEvent::Shutdown)
    } else {
        None
    }
}

/// send a signal to a pid
pub fn send_signal(pid: u32, sig: u32) -> Result<()> {
    debug!("sending no-op(windows) signal {} to pid {}", sig, pid);
    Ok(())
}
