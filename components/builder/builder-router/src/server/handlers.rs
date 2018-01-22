// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use hab_net::{ErrCode, NetError};
use protocol::routesrv;
use protocol::message::Message;

use super::ServerMap;
use conn::SrvConn;
use error::Result;

pub fn on_disconnect(_: &SrvConn, message: &mut Message, servers: &mut ServerMap) -> Result<()> {
    debug!("OnDisconnect, {:?}", message.sender_str().unwrap());
    servers.drop(message.sender().unwrap());
    Ok(())
}

pub fn on_heartbeat(conn: &SrvConn, message: &mut Message, servers: &mut ServerMap) -> Result<()> {
    debug!("OnHeartbeat, {:?}", message.sender_str().unwrap());
    if !servers.renew(message.sender().unwrap()) {
        let err = NetError::new(ErrCode::REG_NOT_FOUND, "rt:heartbeat:1");
        warn!("{}", err);
        conn.route_reply(message, &*err)?;
    }
    Ok(())
}

pub fn on_registration(
    conn: &SrvConn,
    message: &mut Message,
    servers: &mut ServerMap,
) -> Result<()> {
    let mut body = message.parse::<routesrv::Registration>()?;
    debug!("OnRegistration, {:?}", body);
    let protocol = body.get_protocol();
    let shards = body.take_shards();
    if !servers.add(protocol, message.sender().unwrap().to_vec(), shards) {
        let err = NetError::new(ErrCode::REG_CONFLICT, "rt:register:1");
        warn!("{}", err);
        conn.route_reply(message, &*err)?;
    }
    Ok(())
}
