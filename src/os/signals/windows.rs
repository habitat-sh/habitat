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

use ctrlc;
use std::sync::atomic::{AtomicBool,
                        Ordering};

/// True when we have caught `ctrl-c`
static CAUGHT: AtomicBool = AtomicBool::new(false);

pub fn init() {
    ctrlc::set_handler(move || {
        CAUGHT.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

/// Returns `true` if we have received a signal to shut down.
pub fn check_for_shutdown() -> bool { CAUGHT.compare_and_swap(true, false, Ordering::SeqCst) }
