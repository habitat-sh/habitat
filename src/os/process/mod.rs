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
#[path = "windows.rs"]
mod imp;

#[cfg(not(windows))]
#[path = "linux.rs"]
mod imp;

pub use self::imp::*;

pub trait OsSignal {
    fn os_signal(&self) -> SignalCode;
    fn from_signal_code(_: SignalCode) -> Option<Signal>;
}

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub enum Signal {
    INT,
    ILL,
    ABRT,
    FPE,
    KILL,
    SEGV,
    TERM,
    HUP,
    QUIT,
    ALRM,
    USR1,
    USR2,
    CHLD,
}

impl From<i32> for Signal {
    fn from(val: i32) -> Signal {
        match val {
            1 => Signal::HUP,
            2 => Signal::INT,
            3 => Signal::QUIT,
            4 => Signal::ILL,
            6 => Signal::ABRT,
            8 => Signal::FPE,
            9 => Signal::KILL,
            10 => Signal::USR1,
            11 => Signal::SEGV,
            12 => Signal::USR2,
            14 => Signal::ALRM,
            15 => Signal::TERM,
            17 => Signal::CHLD,
            _ => Signal::KILL,
        }
    }
}

impl From<Signal> for i32 {
    fn from(value: Signal) -> i32 {
        match value {
            Signal::HUP => 1,
            Signal::INT => 2,
            Signal::QUIT => 3,
            Signal::ILL => 4,
            Signal::ABRT => 6,
            Signal::FPE => 8,
            Signal::KILL => 9,
            Signal::USR1 => 10,
            Signal::SEGV => 11,
            Signal::USR2 => 12,
            Signal::ALRM => 14,
            Signal::TERM => 15,
            Signal::CHLD => 17,
        }
    }
}
