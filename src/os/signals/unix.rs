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

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_BOOL_INIT, ATOMIC_USIZE_INIT};
use std::sync::{Once, ONCE_INIT};

use os::process::{OsSignal, Signal, SignalCode};

use super::SignalEvent;

static INIT: Once = ONCE_INIT;
// True when we have caught a signal
static CAUGHT: AtomicBool = ATOMIC_BOOL_INIT;
// Stores the value of the signal we caught
static SIGNAL: AtomicUsize = ATOMIC_USIZE_INIT;

// Functions from POSIX libc.
extern "C" {
    fn signal(
        sig: SignalCode,
        cb: unsafe extern "C" fn(SignalCode),
    ) -> unsafe extern "C" fn(SignalCode);
}

unsafe extern "C" fn handle_signal(signal: SignalCode) {
    CAUGHT.store(true, Ordering::SeqCst);
    SIGNAL.store(signal as usize, Ordering::SeqCst);
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
        let code = SIGNAL.load(Ordering::SeqCst) as SignalCode;
        let event = match Signal::from_signal_code(code) {
            Some(Signal::INT) | Some(Signal::TERM) => Some(SignalEvent::Shutdown),
            Some(Signal::CHLD) => Some(SignalEvent::WaitForChild),
            Some(signal) => Some(SignalEvent::Passthrough(signal)),
            None => {
                println!("Received invalid signal: #{}", code);
                None
            }
        };
        // Clear out a signal that has been caught so we don't end up
        // processing it again.
        CAUGHT.store(false, Ordering::SeqCst);
        SIGNAL.store(0 as usize, Ordering::SeqCst);
        event
    } else {
        None
    }
}

fn set_signal_handlers() {
    unsafe {
        signal(Signal::HUP.os_signal(), handle_signal);
        signal(Signal::INT.os_signal(), handle_signal);
        signal(Signal::QUIT.os_signal(), handle_signal);
        signal(Signal::ALRM.os_signal(), handle_signal);
        signal(Signal::TERM.os_signal(), handle_signal);
        signal(Signal::USR1.os_signal(), handle_signal);
        signal(Signal::USR2.os_signal(), handle_signal);
        signal(Signal::CHLD.os_signal(), handle_signal);
    }
}
