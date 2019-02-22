// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

use crate::types::NetErr;
use prost;
use std::{fmt,
          result};

#[derive(Debug)]
pub enum Error {
    Deserialize(prost::DecodeError),
    NetErr(NetErr),
    ProtocolMismatch(&'static str),
    Serialize(prost::EncodeError),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::Deserialize(ref e) => format!("Unable to deserialize message: {}", e),
            Error::NetErr(ref e) => format!("Net error: {}", e),
            Error::ProtocolMismatch(ref field) => format!(
                "Received an unsupported or bad protocol message. Missing field: {}",
                field
            ),
            Error::Serialize(ref e) => format!("Unable to serialize message: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Error { Error::Deserialize(err) }
}

impl From<prost::EncodeError> for Error {
    fn from(err: prost::EncodeError) -> Error { Error::Serialize(err) }
}
