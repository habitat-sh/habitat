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
extern crate prost_derive;

#[macro_use]
extern crate serde_derive;

mod error;
mod generated;
mod types;

use crate::error::Result;
pub use crate::{error::Error, types::*};

pub const LAUNCHER_PIPE_ENV: &str = "HAB_LAUNCHER_PIPE";
pub const LAUNCHER_PID_ENV: &str = "HAB_LAUNCHER_PID";
// Set to instruct the Supervisor to clean the Launcher's process LOCK on startup. This is useful
// when restarting a Supervisor which terminated normally.
pub const LAUNCHER_LOCK_CLEAN_ENV: &str = "HAB_LAUNCHER_LOCK_CLEAN";
/// Process exit code from Supervisor which indicates to Launcher that the Supervisor
/// ran to completion with a successful result. The Launcher should not attempt to restart
/// the Supervisor and should exit immediately with a successful exit code.
pub const OK_NO_RETRY_EXCODE: i32 = 84;
/// Same as `OK_NO_RETRY_EXCODE` except the Supervisor ran to completion with an unsuccessful
/// exit code. The Launcher should exit immediately with a non-zero exit code.
pub const ERR_NO_RETRY_EXCODE: i32 = 86;

#[derive(Debug)]
pub struct NetTxn(Envelope);

impl NetTxn {
    pub fn build<T>(message: &T) -> Result<Self>
    where
        T: LauncherMessage,
    {
        let env = Envelope {
            message_id: T::MESSAGE_ID.to_string(),
            payload: message.to_bytes()?,
        };
        Ok(NetTxn(env))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let env = Envelope::from_bytes(bytes)?;
        Ok(NetTxn(env))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        self.0.clone().to_bytes()
    }

    pub fn decode<T>(&self) -> Result<T>
    where
        T: LauncherMessage,
    {
        T::from_bytes(&self.0.payload)
    }

    pub fn message_id(&self) -> &str {
        &self.0.message_id
    }
}

pub fn error<T>(err: T) -> NetErr
where
    T: ToString + Into<ErrCode>,
{
    NetErr {
        msg: err.to_string(),
        code: err.into(),
    }
}
