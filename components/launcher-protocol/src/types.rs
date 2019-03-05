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

use crate::{error::{Error,
                    Result},
            generated};
use prost::{self,
            Message};
use std::{collections::HashMap,
          fmt};

pub trait MessageStatic {
    const MESSAGE_ID: &'static str;
}

pub trait FromProto<T: prost::Message + Default>: Sized {
    fn from_proto(value: T) -> Result<Self>;
}

pub trait LauncherMessage
    where Self: Clone + fmt::Debug
{
    type Generated: prost::Message + Default + From<Self>;
    const MESSAGE_ID: &'static str;

    fn from_proto(value: Self::Generated) -> Result<Self>;

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let decoded = Self::Generated::decode(bytes)?;
        Self::from_proto(decoded)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        let envelope = Self::Generated::from(self.clone());
        let mut buf = bytes::BytesMut::with_capacity(envelope.encoded_len());
        envelope.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

pub use generated::{ErrCode,
                    ShutdownMethod};

// Now we're going to define our own set of structs to use internally, as well as conversion
// facilities to and from the corresponding protobuf types. It'd be rad if there was a way to
// simplify or eliminate a lot of this boilerplate, but I'm not sure if there is or not.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetErr {
    pub code: generated::ErrCode,
    pub msg:  String,
}

impl LauncherMessage for NetErr {
    type Generated = generated::NetErr;

    const MESSAGE_ID: &'static str = "NetErr";

    fn from_proto(proto: generated::NetErr) -> Result<Self> {
        Ok(NetErr {
            code: generated::ErrCode::from_i32(proto.code.ok_or(Error::ProtocolMismatch("code"))?)
                .ok_or(Error::ProtocolMismatch("code"))?,
            msg: proto.msg.ok_or(Error::ProtocolMismatch("msg"))?,
        })
    }
}

impl From<NetErr> for generated::NetErr {
    fn from(value: NetErr) -> Self {
        generated::NetErr { code: Some(value.code as i32),
                            msg:  Some(value.msg), }
    }
}

impl fmt::Display for NetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.code, self.msg)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetOk {}

impl LauncherMessage for NetOk {
    type Generated = generated::NetOk;

    const MESSAGE_ID: &'static str = "NetOk";

    fn from_proto(_proto: generated::NetOk) -> Result<Self> { Ok(NetOk {}) }
}

impl From<NetOk> for generated::NetOk {
    fn from(_value: NetOk) -> Self { generated::NetOk {} }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Register {
    pub pipe: String,
}

impl Register {
    pub fn new(pipe: String) -> Self { Register { pipe } }
}

impl LauncherMessage for Register {
    type Generated = generated::Register;

    const MESSAGE_ID: &'static str = "Register";

    fn from_proto(proto: generated::Register) -> Result<Self> {
        Ok(Register { pipe: proto.pipe.ok_or(Error::ProtocolMismatch("pipe"))?, })
    }
}

impl From<Register> for generated::Register {
    fn from(value: Register) -> Self { generated::Register { pipe: Some(value.pipe), } }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Restart {
    pub pid: i64,
}

impl LauncherMessage for Restart {
    type Generated = generated::Restart;

    const MESSAGE_ID: &'static str = "Restart";

    fn from_proto(proto: generated::Restart) -> Result<Self> {
        Ok(Restart { pid: proto.pid.ok_or(Error::ProtocolMismatch("pid"))?, })
    }
}

impl From<Restart> for generated::Restart {
    fn from(value: Restart) -> Self { generated::Restart { pid: Some(value.pid), } }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Spawn {
    pub id:           String,
    pub binary:       String,
    pub svc_user:     Option<String>,
    pub svc_group:    Option<String>,
    pub svc_password: Option<String>,
    pub env:          HashMap<String, String>,
    pub svc_user_id:  Option<u32>,
    pub svc_group_id: Option<u32>,
}

impl LauncherMessage for Spawn {
    type Generated = generated::Spawn;

    const MESSAGE_ID: &'static str = "Spawn";

    fn from_proto(proto: generated::Spawn) -> Result<Self> {
        Ok(Spawn { id:           proto.id.ok_or(Error::ProtocolMismatch("id"))?,
                   binary:       proto.binary.ok_or(Error::ProtocolMismatch("binary"))?,
                   svc_user:     proto.svc_user,
                   svc_group:    proto.svc_group,
                   svc_password: proto.svc_password,
                   env:          proto.env,
                   svc_user_id:  proto.svc_user_id,
                   svc_group_id: proto.svc_group_id, })
    }
}

impl From<Spawn> for generated::Spawn {
    fn from(value: Spawn) -> Self {
        generated::Spawn { id:           Some(value.id),
                           binary:       Some(value.binary),
                           svc_user:     value.svc_user,
                           svc_group:    value.svc_group,
                           svc_password: value.svc_password,
                           env:          value.env,
                           svc_user_id:  value.svc_user_id,
                           svc_group_id: value.svc_group_id, }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpawnOk {
    pub pid: i64,
}

impl LauncherMessage for SpawnOk {
    type Generated = generated::SpawnOk;

    const MESSAGE_ID: &'static str = "SpawnOk";

    fn from_proto(proto: generated::SpawnOk) -> Result<Self> {
        Ok(SpawnOk { pid: proto.pid.ok_or(Error::ProtocolMismatch("pid"))?, })
    }
}

impl From<SpawnOk> for generated::SpawnOk {
    fn from(value: SpawnOk) -> Self { generated::SpawnOk { pid: Some(value.pid), } }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Terminate {
    pub pid: i64,
}

impl LauncherMessage for Terminate {
    type Generated = generated::Terminate;

    const MESSAGE_ID: &'static str = "Terminate";

    fn from_proto(proto: generated::Terminate) -> Result<Self> {
        Ok(Terminate { pid: proto.pid.ok_or(Error::ProtocolMismatch("pid"))?, })
    }
}

impl From<Terminate> for generated::Terminate {
    fn from(value: Terminate) -> Self { generated::Terminate { pid: Some(value.pid), } }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TerminateOk {
    pub exit_code:       i32,
    pub shutdown_method: generated::ShutdownMethod,
}

impl LauncherMessage for TerminateOk {
    type Generated = generated::TerminateOk;

    const MESSAGE_ID: &'static str = "TerminateOk";

    fn from_proto(proto: generated::TerminateOk) -> Result<Self> {
        Ok(TerminateOk {
            exit_code: proto
                .exit_code
                .ok_or(Error::ProtocolMismatch("exit_code"))?,
            shutdown_method: generated::ShutdownMethod::from_i32(
                proto
                    .shutdown_method
                    .ok_or(Error::ProtocolMismatch("shutdown_method"))?,
            )
            .ok_or(Error::ProtocolMismatch("shutdown_method"))?,
        })
    }
}

impl From<TerminateOk> for generated::TerminateOk {
    fn from(value: TerminateOk) -> Self {
        generated::TerminateOk { exit_code:       Some(value.exit_code),
                                 shutdown_method: Some(value.shutdown_method as i32), }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Envelope {
    pub message_id: String,
    pub payload:    Vec<u8>,
}

impl LauncherMessage for Envelope {
    type Generated = generated::Envelope;

    const MESSAGE_ID: &'static str = "Envelope";

    fn from_proto(proto: generated::Envelope) -> Result<Self> {
        Ok(Envelope { message_id: proto.message_id
                                       .ok_or(Error::ProtocolMismatch("message_id"))?,
                      payload:    proto.payload.ok_or(Error::ProtocolMismatch("payload"))?, })
    }
}

impl From<Envelope> for generated::Envelope {
    fn from(value: Envelope) -> Self {
        generated::Envelope { message_id: Some(value.message_id),
                              payload:    Some(value.payload), }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Shutdown {}

impl LauncherMessage for Shutdown {
    type Generated = generated::Shutdown;

    const MESSAGE_ID: &'static str = "Shutdown";

    fn from_proto(_proto: generated::Shutdown) -> Result<Self> { Ok(Shutdown {}) }
}

impl From<Shutdown> for generated::Shutdown {
    fn from(_value: Shutdown) -> Self { generated::Shutdown {} }
}
