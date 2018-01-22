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

use std::fmt;
use std::str::FromStr;

use protobuf::core::ProtobufEnum;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub use message::{ErrCode, Message, NetError, NetOk, Protocol, RouteInfo, Txn};
use error::ProtocolError;

pub fn err<T>(code: ErrCode, msg: T) -> NetError
where
    T: ToString,
{
    let mut err = NetError::new();
    err.set_code(code);
    err.set_msg(msg.to_string());
    err
}

impl Serialize for ErrCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.value() as u64)
    }
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[err: {:?}, msg: {}]", self.get_code(), self.get_msg())
    }
}

impl Serialize for NetError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("error", 2)?;
        strukt.serialize_field("code", &self.get_code())?;
        strukt.serialize_field("msg", self.get_msg())?;
        strukt.end()
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If you add a new value here, you *must* update the `FromStr` implementation for
        // `Protocol` below.
        let value = match *self {
            Protocol::JobSrv => "jobsrv",
            Protocol::Net => "net",
            Protocol::RouteSrv => "routesrv",
            Protocol::SessionSrv => "sessionsrv",
            Protocol::OriginSrv => "originsrv",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Protocol {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "jobsrv" => Ok(Protocol::JobSrv),
            "net" => Ok(Protocol::Net),
            "routesrv" => Ok(Protocol::RouteSrv),
            "sessionsrv" => Ok(Protocol::SessionSrv),
            "originsrv" => Ok(Protocol::OriginSrv),
            protocol_id => Err(ProtocolError::NoProtocol(protocol_id.to_string())),
        }
    }
}
