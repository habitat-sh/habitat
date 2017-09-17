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

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;
use std::ops::Deref;

pub use protocol::net::{ErrCode, NetOk};
use hyper;
use protobuf::{self, MessageStatic};
use protocol::{self, net};
use serde_json;
use zmq;

use conn;
use oauth;

pub type LibResult<T> = Result<T, LibError>;
pub type NetResult<T> = Result<T, NetError>;

#[derive(Debug)]
pub enum LibError {
    Auth(oauth::github::AuthErr),
    Connection(conn::ConnErr),
    GitHubAPI(hyper::status::StatusCode, HashMap<String, String>),
    HttpClient(hyper::Error),
    HttpClientParse(hyper::error::ParseError),
    HttpResponse(hyper::status::StatusCode),
    IO(io::Error),
    Json(serde_json::Error),
    NetError(NetError),
    Protobuf(protobuf::ProtobufError),
    Protocol(protocol::ProtocolError),
    RequiredConfigField(&'static str),
    Sys,
    Zmq(zmq::Error),
}

impl LibError {
    pub fn net_err<T>(code: ErrCode, msg: T) -> LibError
    where
        T: ToString,
    {
        LibError::NetError(NetError::new(code, msg))
    }
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            LibError::Auth(ref e) => format!("GitHub Authentication error, {}", e),
            LibError::Connection(ref e) => format!("{}", e),
            LibError::GitHubAPI(ref c, ref m) => format!("[{}] {:?}", c, m),
            LibError::HttpClient(ref e) => format!("{}", e),
            LibError::HttpClientParse(ref e) => format!("{}", e),
            LibError::HttpResponse(ref e) => format!("{}", e),
            LibError::IO(ref e) => format!("{}", e),
            LibError::Json(ref e) => format!("{}", e),
            LibError::NetError(ref e) => format!("{}", e),
            LibError::Protobuf(ref e) => format!("{}", e),
            LibError::Protocol(ref e) => format!("{}", e),
            LibError::RequiredConfigField(ref e) => {
                format!("Missing required field in configuration, {}", e)
            }
            LibError::Sys => format!("Internal system error"),
            LibError::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for LibError {
    fn description(&self) -> &str {
        match *self {
            LibError::Auth(_) => "GitHub authorization error.",
            LibError::Connection(ref err) => err.description(),
            LibError::GitHubAPI(_, _) => "GitHub API error.",
            LibError::HttpClient(ref err) => err.description(),
            LibError::HttpClientParse(ref err) => err.description(),
            LibError::HttpResponse(_) => "Non-200 HTTP response.",
            LibError::IO(ref err) => err.description(),
            LibError::Json(ref err) => err.description(),
            LibError::NetError(ref err) => err.description(),
            LibError::Protobuf(ref err) => err.description(),
            LibError::Protocol(ref err) => err.description(),
            LibError::RequiredConfigField(_) => "Missing required field in configuration.",
            LibError::Sys => "Internal system error",
            LibError::Zmq(ref err) => err.description(),
        }
    }
}

impl From<conn::ConnErr> for LibError {
    fn from(err: conn::ConnErr) -> LibError {
        LibError::Connection(err)
    }
}

impl From<io::Error> for LibError {
    fn from(err: io::Error) -> LibError {
        LibError::IO(err)
    }
}

impl From<hyper::Error> for LibError {
    fn from(err: hyper::Error) -> LibError {
        LibError::HttpClient(err)
    }
}

impl From<oauth::github::AuthErr> for LibError {
    fn from(err: oauth::github::AuthErr) -> Self {
        LibError::Auth(err)
    }
}

impl From<protobuf::ProtobufError> for LibError {
    fn from(err: protobuf::ProtobufError) -> LibError {
        LibError::Protobuf(err)
    }
}

impl From<protocol::ProtocolError> for LibError {
    fn from(err: protocol::ProtocolError) -> LibError {
        LibError::Protocol(err)
    }
}

impl From<serde_json::Error> for LibError {
    fn from(err: serde_json::Error) -> LibError {
        LibError::Json(err)
    }
}

impl From<zmq::Error> for LibError {
    fn from(err: zmq::Error) -> LibError {
        LibError::Zmq(err)
    }
}

#[derive(Debug, Serialize)]
pub struct NetError(net::NetError);

impl NetError {
    pub fn message_id() -> &'static str {
        net::NetError::descriptor_static(None).name()
    }

    pub fn new<T>(code: ErrCode, msg: T) -> NetError
    where
        T: ToString,
    {
        NetError(net::err(code, msg))
    }

    pub fn parse(msg: &protocol::Message) -> Result<NetError, LibError> {
        let err = protocol::message::decode::<net::NetError>(&msg.body)
            .map_err(LibError::Protocol)?;
        Ok(NetError(err))
    }

    pub fn code(&self) -> ErrCode {
        self.0.get_code()
    }

    pub fn msg(&self) -> &str {
        self.0.get_msg()
    }

    pub fn take_err(self) -> net::NetError {
        self.0
    }
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for NetError {
    fn description(&self) -> &str {
        match self.code() {
            ErrCode::BUG => "An unexpected error occurred.",
            ErrCode::SYS => "Internal error: See the server or client's log output for details.",
            ErrCode::TIMEOUT => "Network timeout.",
            ErrCode::REMOTE_REJECTED => "Remote server rejected request.",
            ErrCode::BAD_REMOTE_REPLY => "Remote server returned a bad response.",
            ErrCode::ENTITY_NOT_FOUND => "Entity not found in datastore.",
            ErrCode::NO_SHARD => "Shard not available.",
            ErrCode::ACCESS_DENIED => "Operation not allowed by authenticated.",
            ErrCode::SESSION_EXPIRED => "Session expired, user should re-authenticate.",
            ErrCode::ENTITY_CONFLICT => "Entity already exists in datastore.",
            ErrCode::SOCK => "Network error.",
            ErrCode::DATA_STORE => "Database error.",
            ErrCode::AUTH_SCOPE => "Additional authorization scope(s) required for action.",
            ErrCode::WORKSPACE_SETUP => "Worker runner unable to setup build workspace.",
            ErrCode::SECRET_KEY_FETCH => "Worker runner unable to fetch secret key for origin.",
            ErrCode::SECRET_KEY_IMPORT => "Worker runner unable to import secret key for origin.",
            ErrCode::VCS_CLONE => "Worker runner unable to retrieve project source to build.",
            ErrCode::BUILD => "Worker runner failed to build project.",
            ErrCode::POST_PROCESSOR => "One or more post processing step failed in Worker runner.",
            ErrCode::REG_CONFLICT => {
                "Service registration rejected by RouteSrv. Conflicting registration."
            }
            ErrCode::REG_NOT_FOUND => "RouteSrv was unable to find a registration for Service.",
            ErrCode::REMOTE_UNAVAILABLE => "Remote server not respnoding.",
            ErrCode::GROUP_NOT_COMPLETE => "Scheduler Job Group incomplete.",
            ErrCode::PARTIAL_JOB_GROUP_PROMOTE => {
                "Some packages failed to promote to the specified channel."
            }
        }
    }
}

impl Deref for NetError {
    type Target = net::NetError;

    fn deref(&self) -> &net::NetError {
        &self.0
    }
}
