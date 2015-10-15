//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Traps and notifies UNIX signals.
//!
//! Start's another thread which you can subscribe to which traps UNIX signals
//! sent to the running process and notifies the receiver channel of a caught
//! `signals::Signal`.
//!
//! # Examples
//!
//! ```
//! use util::signals::SignalNotifier;
//!
//! let handler = SignalNotifier::start();
//!
//! match handler.receiver.try_recv() {
//!     Ok(signals::Signal::SIGHUP) => {
//!         println!("Got SIGHUP!");
//!     },
//!     Ok(sig) => {
//!         println!("Got unhandled - {:?}!", sig);
//!     },
//!     Err(TryRecvError::Empty) => {},
//!     Err(TryRecvError::Disconnected) => {
//!         panic!("signal handler crashed!");
//!     },
//! }

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, ATOMIC_USIZE_INIT, ATOMIC_BOOL_INIT};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::sync::mpsc;
use std::thread;

use error::BldrResult;

// Has a value when we have caught a signal
static CAUGHT_SIGNAL: AtomicBool = ATOMIC_BOOL_INIT;
// Stores the specific value of the signal we caught
static WHICH_SIGNAL: AtomicUsize = ATOMIC_USIZE_INIT;

// Functions from POSIX libc.
extern "C" {
    fn signal(sig: u32, cb: extern fn(u32)) -> extern fn(u32);
}

extern fn handle_signal(sig: u32) {
    CAUGHT_SIGNAL.store(true, Ordering::SeqCst);
    WHICH_SIGNAL.store(sig as usize, Ordering::SeqCst);
}

/// `i32` representation of each Unix Signal of interest.
pub enum Signal {
    /// terminate process - terminal line hangup
    SIGHUP = 1,
    /// terminate process - interrupt program
    SIGINT = 2,
    /// create core image - quit program
    SIGQUIT = 3,
    /// terminate process - real-time timer expired
    SIGALRM = 14,
    /// terminate process - software termination signal
    SIGTERM = 15,
    /// terminate process - User defined signal 1
    SIGUSR1 = 30,
    /// terminate process - User defined signal 2
    SIGUSR2 = 31,
}

/// Thread worker that traps UNIX signals and sends a `Signal` down the receiver
/// channel representing the trapped UNIX signal.
pub struct SignalNotifier {
    pub sender: Sender<i32>,
    pub receiver: Receiver<Signal>,
    pub worker: thread::JoinHandle<BldrResult<()>>,
}

impl SignalNotifier {
    /// Create a new handler struct
    pub fn new(sender: Sender<i32>, receiver: Receiver<Signal>, worker: thread::JoinHandle<BldrResult<()>>) -> SignalNotifier {
        SignalNotifier {
            sender: sender,
            receiver: receiver,
            worker: worker,
        }
    }

    /// Start a SignalNotifier thread and return a SignalNotifier struct with the sending and receiving
    /// channel to-and-from the started thread.
    pub fn start() -> SignalNotifier {
        let (otx, orx): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        let (itx, irx) = mpsc::channel();
        let handle = thread::Builder::new().name(String::from("signal_handler")).spawn(move || {
            SignalNotifier::init(otx, irx)
        }).unwrap();
        SignalNotifier::new(itx, orx, handle)
    }

    fn init(tx: Sender<Signal>, rx: Receiver<i32>) -> BldrResult<()> {
        SignalNotifier::set_signal_handlers();
        loop {
            if CAUGHT_SIGNAL.load(Ordering::SeqCst) {
                match WHICH_SIGNAL.load(Ordering::SeqCst) {
                    1 => tx.send(Signal::SIGHUP).unwrap(),
                    2 => tx.send(Signal::SIGINT).unwrap(),
                    3 => tx.send(Signal::SIGQUIT).unwrap(),
                    14 => tx.send(Signal::SIGALRM).unwrap(),
                    15 => tx.send(Signal::SIGTERM).unwrap(),
                    30 => tx.send(Signal::SIGUSR1).unwrap(),
                    31 => tx.send(Signal::SIGUSR2).unwrap(),
                    _ => unreachable!(),
                }
                // Reset the signal handler flags
                CAUGHT_SIGNAL.store(false, Ordering::SeqCst);
                WHICH_SIGNAL.store(0 as usize, Ordering::SeqCst);
            }
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => { break; },
                Err(TryRecvError::Empty) => {},
            }
        }
        Ok(())
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
}
