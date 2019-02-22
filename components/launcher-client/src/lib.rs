// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

#[macro_use]
extern crate log;

mod client;
pub mod error;

pub use habitat_launcher_protocol::{ERR_NO_RETRY_EXCODE,
                                    LAUNCHER_LOCK_CLEAN_ENV,
                                    LAUNCHER_PID_ENV,
                                    OK_NO_RETRY_EXCODE};

pub use crate::{client::LauncherCli,
                error::Error};

pub fn env_pipe() -> Option<String> {
    habitat_core::env::var(habitat_launcher_protocol::LAUNCHER_PIPE_ENV).ok()
}
