// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::sync::{Once, ONCE_INIT};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};

use error::{Error, Result, SupError};
use super::SignalEvent;

static LOGKEY: &'static str = "SI";

static INIT: Once = ONCE_INIT;
// True when we have caught a signal
static CAUGHT: AtomicBool = ATOMIC_BOOL_INIT;
// Stores the value of the signal we caught
static SIGNAL: AtomicUsize = ATOMIC_USIZE_INIT;

// Functions from POSIX libc.
extern "C" {
    fn signal(sig: u32, cb: unsafe extern "C" fn(u32)) -> unsafe extern "C" fn(u32);
    fn kill(pid: i32, sig: u32) -> u32;
}

unsafe extern "C" fn handle_signal(signal: u32) {
    CAUGHT.store(true, Ordering::SeqCst);
    SIGNAL.store(signal as usize, Ordering::SeqCst);
}

/// `i32` representation of each Unix Signal of interest.
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
    INIT.call_once(|| {
        self::set_signal_handlers();
        CAUGHT.store(false, Ordering::SeqCst);
        SIGNAL.store(0 as usize, Ordering::SeqCst);
    });
}

pub fn check_for_signal() -> Option<SignalEvent> {
    if CAUGHT.load(Ordering::SeqCst) {
        let result = match SIGNAL.load(Ordering::SeqCst) {
            signal if signal == Signal::SIGHUP as usize => {
                SignalEvent::Passthrough(Signal::SIGHUP as u32)
            }
            signal if signal == Signal::SIGINT as usize => SignalEvent::Shutdown,
            signal if signal == Signal::SIGQUIT as usize => {
                SignalEvent::Passthrough(Signal::SIGQUIT as u32)
            }
            signal if signal == Signal::SIGALRM as usize => {
                SignalEvent::Passthrough(Signal::SIGALRM as u32)
            }
            signal if signal == Signal::SIGTERM as usize => SignalEvent::Shutdown,
            signal if signal == Signal::SIGUSR1 as usize => {
                SignalEvent::Passthrough(Signal::SIGUSR1 as u32)
            }
            signal if signal == Signal::SIGUSR2 as usize => {
                SignalEvent::Passthrough(Signal::SIGUSR2 as u32)
            }
            signal => {
                outputln!("Received invalid signal: #{}", signal);
                return None;
            }
        };
        // clear out the signal so we don't sent it repeatedly
        CAUGHT.store(false, Ordering::SeqCst);
        SIGNAL.store(0 as usize, Ordering::SeqCst);
        Some(result)
    } else {
        None
    }
}

fn set_signal_handlers() {
    unsafe {
        signal(Signal::SIGHUP as u32, handle_signal);
        signal(Signal::SIGINT as u32, handle_signal);
        signal(Signal::SIGQUIT as u32, handle_signal);
        signal(Signal::SIGALRM as u32, handle_signal);
        signal(Signal::SIGTERM as u32, handle_signal);
        signal(Signal::SIGUSR1 as u32, handle_signal);
        signal(Signal::SIGUSR2 as u32, handle_signal);
    }
}

/// send a Unix signal to a pid
pub fn send_signal(pid: u32, sig: u32) -> Result<()> {
    debug!("sending signal {} to pid {}", sig, pid);
    unsafe {
        let result = kill(pid as i32, sig);
        match result {
            0 => Ok(()),
            _ => return Err(sup_error!(Error::SignalFailed)),
        }
    }
}
