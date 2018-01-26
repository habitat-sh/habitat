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

extern crate protobuf;

mod message;

use std::fmt;

use protobuf::Message;

pub use message::error::*;
pub use message::launcher::*;
pub use message::supervisor::*;
use message::net::*;

pub const LAUNCHER_PIPE_ENV: &'static str = "HAB_LAUNCHER_PIPE";
pub const LAUNCHER_PID_ENV: &'static str = "HAB_LAUNCHER_PID";
// Set to instruct the Supervisor to clean the Launcher's process LOCK on startup. This is useful
// when restarting a Supervisor which terminated normally.
pub const LAUNCHER_LOCK_CLEAN_ENV: &'static str = "HAB_LAUNCHER_LOCK_CLEAN";
/// Process exit code from Supervisor which indicates to Launcher that the Supervisor
/// ran to completion with a successful result. The Launcher should not attempt to restart
/// the Supervisor and should exit immediately with a successful exit code.
pub const OK_NO_RETRY_EXCODE: i32 = 84;
/// Same as `OK_NO_RETRY_EXCODE` except the Supervisor ran to completion with an unsuccessful
/// exit code. The Launcher should exit immediately with a non-zero exit code.
pub const ERR_NO_RETRY_EXCODE: i32 = 86;

pub struct NetTxn(Envelope);

impl NetTxn {
    pub fn build<T>(message: &T) -> Result<Self, protobuf::ProtobufError>
    where
        T: protobuf::MessageStatic,
    {
        let mut env = Envelope::new();
        env.set_message_id(message.descriptor().name().to_string());
        env.set_payload(message.write_to_bytes()?);
        Ok(NetTxn(env))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, protobuf::ProtobufError> {
        let env = protobuf::parse_from_bytes::<Envelope>(bytes)?;
        Ok(NetTxn(env))
    }

    pub fn build_reply<T>(&self, message: &T) -> Result<Self, protobuf::ProtobufError>
    where
        T: protobuf::MessageStatic,
    {
        let mut env = Self::build(message)?;
        env.0.set_txn_id(self.0.get_txn_id());
        Ok(env)
    }

    pub fn decode<T>(&self) -> Result<T, protobuf::ProtobufError>
    where
        T: protobuf::MessageStatic,
    {
        let msg = protobuf::parse_from_bytes::<T>(self.0.get_payload())?;
        Ok(msg)
    }

    pub fn message_id(&self) -> &str {
        self.0.get_message_id()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, protobuf::ProtobufError> {
        self.0.write_to_bytes()
    }
}

pub fn error<T>(err: T) -> NetErr
where
    T: ToString + Into<ErrCode>,
{
    let mut message = NetErr::new();
    message.set_msg(err.to_string());
    message.set_code(err.into());
    message
}

impl fmt::Display for NetErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.get_code(), self.get_msg())
    }
}

impl fmt::Display for ShutdownMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ShutdownMethod::AlreadyExited => "Already Exited",
            ShutdownMethod::GracefulTermination => "Graceful Termination",
            ShutdownMethod::Killed => "Killed",
        };
        write!(f, "{}", printable)
    }
}
