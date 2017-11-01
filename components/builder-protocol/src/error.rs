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

use std::error;
use std::fmt;
use std::result;
use std::string::FromUtf8Error;

use protobuf;

#[derive(Debug)]
pub enum ProtocolError {
    BadJobGroupProjectState(String),
    BadJobGroupState(String),
    BadJobState(String),
    BadSearchEntity(String),
    BadSearchKey(String),
    Decode(protobuf::ProtobufError),
    Encode(protobuf::ProtobufError),
    IdentityDecode(FromUtf8Error),
    MsgNotInitialized,
    NoControlFrame(String),
    NoProtocol(String),
    NoTxn,
}

pub type ProtocolResult<T> = result::Result<T, ProtocolError>;

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            ProtocolError::BadJobGroupProjectState(ref e) => {
                format!("Bad Job Group Project State {}", e)
            }
            ProtocolError::BadJobGroupState(ref e) => format!("Bad Job Group State {}", e),
            ProtocolError::BadJobState(ref e) => format!("Bad Job State {}", e),
            ProtocolError::BadSearchEntity(ref e) => {
                format!("Search not implemented for entity, {}", e)
            }
            ProtocolError::BadSearchKey(ref e) => {
                format!("Search not implemented for entity with key, {}", e)
            }
            ProtocolError::Decode(ref e) => format!("Unable to decode protocol message, {}", e),
            ProtocolError::Encode(ref e) => format!("Unable to encode protocol message, {}", e),
            ProtocolError::IdentityDecode(ref e) => {
                format!("Unable to decode identity message part, {}", e)
            }
            ProtocolError::MsgNotInitialized => {
                format!("Message not ready for transport, is it missing it's header?")
            }
            ProtocolError::NoControlFrame(ref e) => {
                format!(
                    "No `routesrv::ControlFrame` matches the given string, {}",
                    e
                )
            }
            ProtocolError::NoProtocol(ref e) => {
                format!("No `net::Protocol` matching given string, {}", e)
            }
            ProtocolError::NoTxn => format!("Message is not transactional"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for ProtocolError {
    fn description(&self) -> &str {
        match *self {
            ProtocolError::BadJobGroupProjectState(_) => "Job Group Project state cannot be parsed",
            ProtocolError::BadJobGroupState(_) => "Job Group state cannot be parsed",
            ProtocolError::BadJobState(_) => "Job state cannot be parsed",
            ProtocolError::BadSearchEntity(_) => "Search not implemented for entity.",
            ProtocolError::BadSearchKey(_) => "Entity not indexed by the given key.",
            ProtocolError::Decode(_) => "Unable to decode protocol message",
            ProtocolError::Encode(_) => "Unable to encode protocol message",
            ProtocolError::IdentityDecode(_) => "Unable to decode identity message part",
            ProtocolError::MsgNotInitialized => {
                "Message not ready for transport, is it missing it's header?"
            }
            ProtocolError::NoControlFrame(_) => {
                "No `routesrv::ControlFrame` matches the given string"
            }
            ProtocolError::NoProtocol(_) => "No `net::Protocol` matches the given string",
            ProtocolError::NoTxn => "Message is not transactional",
        }
    }
}
