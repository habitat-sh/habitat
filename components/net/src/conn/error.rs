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

use std::error;
use std::fmt;
use std::string::FromUtf8Error;

use protocol;
use zmq;

#[derive(Debug)]
pub enum ConnErr {
    BadIdentity(FromUtf8Error),
    BadHeader(protocol::ProtocolError),
    BadRouteInfo(protocol::ProtocolError),
    BadTxn(protocol::ProtocolError),
    HostUnreachable,
    MultipleSender,
    NoBody,
    NoIdentity,
    NoHeader,
    NoRouteInfo,
    NoTxn,
    Protocol(protocol::ProtocolError),
    Shutdown(zmq::Error),
    Socket(zmq::Error),
    Timeout,
    TxnNotComplete,
}

impl fmt::Display for ConnErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnErr::BadIdentity(ref e) => {
                write!(f, "Unable to parse identity message part, {}", e)
            }
            ConnErr::BadHeader(ref e) => write!(f, "Unable to parse header message part, {}", e),
            ConnErr::BadRouteInfo(ref e) => {
                write!(f, "Unable to parse route-info message part, {}", e)
            }
            ConnErr::BadTxn(ref e) => write!(f, "Unable to parse transaction message part, {}", e),
            ConnErr::HostUnreachable => write!(f, "Unable to route message to destination"),
            ConnErr::MultipleSender => write!(f, "Message header contained multiple senders"),
            ConnErr::NoBody => write!(f, "Message missing body message part"),
            ConnErr::NoIdentity => write!(f, "Message missing identity message parts"),
            ConnErr::NoHeader => {
                write!(
                    f,
                    "Unable to send or receive message without a `Header` message part"
                )
            }
            ConnErr::NoRouteInfo => write!(f, "Expected to receive `RouteInfo` message part"),
            ConnErr::NoTxn => write!(f, "Expected to receive `Txn` message part"),
            ConnErr::Protocol(ref e) => write!(f, "{}", e),
            ConnErr::Shutdown(ref e) => write!(f, "Received shutdown signal, {}", e),
            ConnErr::Socket(ref e) => write!(f, "Connection send/recv error, {}", e),
            ConnErr::Timeout => write!(f, "Connection recv timeout"),
            ConnErr::TxnNotComplete => {
                write!(
                    f,
                    "Attempted to send transaction reply to an incomplete message"
                )
            }
        }
    }
}

impl error::Error for ConnErr {
    fn description(&self) -> &str {
        match *self {
            ConnErr::BadIdentity(_) => "Unable to parse identity message part",
            ConnErr::BadHeader(_) => "Unable to parse header message part",
            ConnErr::BadRouteInfo(_) => "Unable to parse route-info message part",
            ConnErr::BadTxn(_) => "Unable to parse transaction message part",
            ConnErr::HostUnreachable => "Unable to route message to destination",
            ConnErr::MultipleSender => "Message header contained multiple senders",
            ConnErr::NoBody => "Message missing body message part",
            ConnErr::NoHeader => "Unable to route message without a `Header` message part",
            ConnErr::NoIdentity => "Message missing identity message parts",
            ConnErr::NoRouteInfo => "Expected to receive `RouteInfo` message part",
            ConnErr::NoTxn => "Expected to receive `Txn` message part",
            ConnErr::Protocol(ref e) => e.description(),
            ConnErr::Shutdown(_) => "Received shutdown signal",
            ConnErr::Socket(ref e) => e.description(),
            ConnErr::Timeout => "Connection recv timeout",
            ConnErr::TxnNotComplete => {
                "Attempted to send transaction reply to an incomplete message"
            }
        }
    }
}

impl From<zmq::Error> for ConnErr {
    fn from(err: zmq::Error) -> Self {
        match err {
            zmq::Error::EHOSTUNREACH => ConnErr::HostUnreachable,
            _ => ConnErr::Socket(err),
        }
    }
}

impl From<protocol::ProtocolError> for ConnErr {
    fn from(err: protocol::ProtocolError) -> Self {
        ConnErr::Protocol(err)
    }
}
