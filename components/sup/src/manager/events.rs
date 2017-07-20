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

use std::net::IpAddr;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender, RecvError};
use std::thread;

use byteorder::{ByteOrder, LittleEndian};
use eventsrv_client::{EventSrvAddr, EventSrvClient};
use eventsrv_client::message::{EventEnvelope, EventEnvelope_Type};
use hcore::service::ServiceGroup;
use protobuf::Message;

use PRODUCT;
use census::{CensusMember, CensusRing};

enum Command {
    SendEvent(EventEnvelope),
    TryConnect(Vec<EventSrvAddr>),
}

pub struct EventsCli {
    group: ServiceGroup,
    tx: Sender<Command>,
}

impl EventsCli {
    fn new(group: ServiceGroup, tx: Sender<Command>) -> Self {
        EventsCli {
            group: group,
            tx: tx,
        }
    }

    pub fn send_census(&self, member: &CensusMember) {
        let mut payload_buf: Vec<u8> = vec![];
        let mut proto_size = vec![0; 8];
        let mut bytes = member.as_protobuf().write_to_bytes().unwrap();
        LittleEndian::write_u64(&mut proto_size, bytes.len() as u64);
        payload_buf.append(&mut proto_size);
        payload_buf.append(&mut bytes);
        let mut event = EventEnvelope::new();
        event.set_field_type(EventEnvelope_Type::ProtoBuf);
        event.set_payload(payload_buf);
        event.set_member_id(member.member_id.clone());
        event.set_service(PRODUCT.to_string());
        self.tx.send(Command::SendEvent(event)).unwrap();
    }

    pub fn try_connect(&self, census: &CensusRing) {
        if let Some(cg) = census.census_group_for(&self.group) {
            // JW TODO: We're over allocating here. We should determine who we are already
            // connected to before we generate an addr list.
            let addrs = cg.members().iter().map(|m| eventsrv_addr(&m)).collect();
            self.tx.send(Command::TryConnect(addrs)).unwrap();
        }
    }
}

pub struct EventsMgr {
    client: EventSrvClient,
    rx: Receiver<Command>,
}

impl EventsMgr {
    pub fn start(group: ServiceGroup) -> EventsCli {
        let (tx, rx) = mpsc::channel::<Command>();
        let (rztx, rzrx) = mpsc::sync_channel(1);
        let client = EventsCli::new(group, tx);
        thread::Builder::new()
            .name("eventsrv-manager".to_string())
            .spawn(move || Self::new(rx).run(rztx))
            .unwrap();
        match rzrx.recv() {
            Ok(()) => client,
            Err(e) => panic!("eventsrv-manager thread startup error, err={}", e),
        }
    }

    fn new(rx: Receiver<Command>) -> Self {
        EventsMgr {
            client: EventSrvClient::new(),
            rx: rx,
        }
    }

    fn run(&mut self, rz: SyncSender<()>) {
        rz.send(()).unwrap();
        loop {
            match self.rx.recv() {
                Ok(Command::TryConnect(addrs)) => {
                    for addr in addrs {
                        debug!("Connecting to eventsrv, {:?}", addr);
                        self.client.connect(&addr)
                    }
                }
                Ok(Command::SendEvent(mut event)) => {
                    debug!("Sending event, {:?}", event);
                    self.client.send(&mut event);
                }
                Err(RecvError) => break,
            }
        }
    }
}

fn eventsrv_addr(member: &CensusMember) -> EventSrvAddr {
    let mut addr = EventSrvAddr::default();
    addr.host = IpAddr::from_str(member.sys.get_ip()).unwrap();
    addr.consumer_port = member
        .cfg
        .get("consumer_port")
        .unwrap()
        .as_integer()
        .unwrap() as u16;
    addr.producer_port = member
        .cfg
        .get("producer_port")
        .unwrap()
        .as_integer()
        .unwrap() as u16;
    addr
}
