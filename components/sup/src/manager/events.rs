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
use std::sync::mpsc::{self, Receiver, RecvError, Sender, SyncSender};
use std::thread;

use crate::eventsrv_client::message::{
    EventEnvelope, EventEnvelope_Type, PackageIdent as PackageIdentProto,
    ServiceUpdate as ServiceUpdateProto, SysInfo as SysInfoProto,
};
use crate::eventsrv_client::{EventSrvAddr, EventSrvClient};
use crate::hcore::service::ServiceGroup;
use byteorder::{ByteOrder, LittleEndian};
use protobuf::Message;
use toml;

use crate::census::{CensusMember, CensusRing};
use crate::manager::service::Service;
use crate::PRODUCT;

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

    pub fn send_service(&self, member: &CensusMember, service: &Service) {
        let mut payload_buf: Vec<u8> = vec![];
        let mut proto_size = vec![0; 8];
        let mut bytes = build_service_update(member, service)
            .write_to_bytes()
            .unwrap();
        LittleEndian::write_u64(&mut proto_size, bytes.len() as u64);
        payload_buf.append(&mut proto_size);
        payload_buf.append(&mut bytes);
        let mut event = EventEnvelope::new();
        event.set_field_type(EventEnvelope_Type::ProtoBuf);
        event.set_payload(payload_buf);
        event.set_member_id(service.sys.member_id.clone());
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
    addr.host = IpAddr::from_str(&member.sys.ip).unwrap();
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

fn build_service_update(member: &CensusMember, service: &Service) -> ServiceUpdateProto {
    let mut sep = ServiceUpdateProto::new();
    sep.set_member_id(service.sys.member_id.clone());

    match service.service_group.application_environment() {
        Some(appenv) => {
            sep.set_application(appenv.application().to_string());
            sep.set_environment(appenv.environment().to_string());
        }
        None => {
            sep.set_application(String::new());
            sep.set_environment(String::new());
        }
    }
    sep.set_service(service.service_group.service().to_string());
    sep.set_group(service.service_group.group().to_string());
    sep.set_org(
        service
            .service_group
            .org()
            .unwrap_or(&"".to_string())
            .to_string(),
    );
    sep.set_bldr_url(service.bldr_url.clone());
    sep.set_channel(service.channel.clone());
    sep.set_topology(service.topology.to_string());
    sep.set_update_strategy(service.update_strategy.to_string());

    // JW TODO: We need to leverage `swim.SysInfo` inside of the EventSrv protobufs. That
    // will alleviate this translation and make things more re-usable.
    let mut sys_info_proto = SysInfoProto::new();
    let sys_info = service.sys.as_sys_info();
    sys_info_proto.set_ip(sys_info.ip.to_string());
    sys_info_proto.set_hostname(sys_info.hostname.to_string());
    sys_info_proto.set_gossip_ip(sys_info.gossip_ip.to_string());
    sys_info_proto.set_gossip_port(sys_info.gossip_port.to_string());
    sys_info_proto.set_http_gateway_ip(sys_info.http_gateway_ip.to_string());
    sys_info_proto.set_http_gateway_port(sys_info.http_gateway_port.to_string());
    sep.set_sys(sys_info_proto);
    let pkg = service.pkg.clone();
    let mut pkg_ident = PackageIdentProto::new();
    pkg_ident.set_origin(pkg.origin);
    pkg_ident.set_name(pkg.name);
    pkg_ident.set_version(pkg.version);
    pkg_ident.set_release(pkg.release);
    sep.set_pkg(pkg_ident);
    sep.set_initialized(service.initialized);

    let cfg_str = toml::to_string(&service.cfg).unwrap();
    sep.set_cfg(cfg_str.into_bytes());

    sep.set_leader(member.leader);
    sep.set_follower(member.follower);
    sep.set_update_leader(member.update_leader);
    sep.set_update_follower(member.update_follower);
    sep.set_election_is_running(member.election_is_running);
    sep.set_election_is_no_quorum(member.election_is_no_quorum);
    sep.set_election_is_finished(member.election_is_finished);
    sep.set_update_election_is_running(member.update_election_is_running);
    sep.set_update_election_is_no_quorum(member.update_election_is_no_quorum);
    sep.set_update_election_is_finished(member.update_election_is_finished);
    sep
}
