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

//! Contains the cross-platform signal behavior.
// If signal handling ever becomes part of the rust stdlib, consider removing
// our homespun implementation. Check for status of that here:
// https://github.com/rust-lang/rfcs/issues/1368

use std::sync::atomic::{AtomicBool,
                        Ordering};

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::{check_for_signal,
                     init,
                     SignalEvent};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

// NOTE: The Unix implementation of `init` also establishes a similar
// handler for shutdown signals, but also does some other stuff, as
// well. Seems best for now to keep all those implementation details
// in the `unix` module.
#[cfg(windows)]
pub fn init() {
    ctrlc::set_handler(move || {
        SHUTDOWN.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

/// Returns `true` if we have received a signal to shut down.
pub fn check_for_shutdown() -> bool { SHUTDOWN.compare_and_swap(true, false, Ordering::SeqCst) }
