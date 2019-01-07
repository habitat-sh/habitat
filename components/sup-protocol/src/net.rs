// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Messages for signaling or controlling networked applications. All types defined in this
//! module are building blocks for sockets speaking SrvProtocol.
//!
//! Note: See `protocols/net.proto` for type level documentation for generated types.

include!("generated/sup.net.rs");
include!("generated/sup.net.impl.rs");

use std::error;
use std::fmt;
use std::io;

use crate::core;

pub type NetResult<T> = Result<T, NetErr>;

/// Helper function for quickly generating a `NetErr` from an `ErrCode` and message.
pub fn err<T>(code: ErrCode, msg: T) -> NetErr
where
    T: fmt::Display,
{
    NetErr {
        code: code as i32,
        msg: msg.to_string(),
    }
}

/// Helper function for quickly generating a `NetOk` message.
pub fn ok() -> NetOk {
    NetOk::default()
}

impl error::Error for NetErr {
    fn description(&self) -> &str {
        match ErrCode::from_i32(self.code).unwrap_or_default() {
            ErrCode::Internal => "Internal error",
            ErrCode::Io => "IO error",
            ErrCode::NotFound => "Entity not found",
            ErrCode::Unauthorized => "Client failed authorization with server",
            ErrCode::Conflict => "Entity exists or is unable to update with given parameters",
            ErrCode::NotSupported => {
                "Request contained a valid payload but a detail of the request was not supported by the remote"
            }
            ErrCode::BadPayload => {
                "Request contained a bad or unreadable value for one or more fields of one or more messages"
            }
            ErrCode::InvalidPayload => {
                "Request contained a well-formed payload but it was rejected as invalid by the remote"
            }
            ErrCode::EntityTooLarge => {
                "Requestor sent a well-formed payload but it exceeded an allowed limit."
            }
            ErrCode::UpdateClient => {
                "Requestor sent a message which the server cannot process. The requestor should update their client before making the same request again."
            }
        }
    }
}

impl fmt::Display for NetErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Err: {}] {}", self.code, self.msg)
    }
}

impl From<io::Error> for NetErr {
    fn from(other: io::Error) -> Self {
        err(ErrCode::Io, other)
    }
}

impl From<core::Error> for NetErr {
    fn from(other: core::Error) -> Self {
        err(ErrCode::Internal, other)
    }
}
