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

use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

use protobuf::RepeatedField;
use zmq;

use dbcache::{self, ExpiringSet, IndexSet, InstaSet};
use hab_net::server::{Application, Envelope, NetIdent, RouteConn, Service, Supervisor,
                      Supervisable};
use protocol::net::{self, ErrCode};
use protocol::vault as proto;

use config::Config;
use data_store::DataStore;
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

pub struct Worker {
    config: Arc<RwLock<Config>>,
    sock: zmq::Socket,
    datastore: Option<DataStore>,
}

impl Worker {
    fn datastore(&self) -> &DataStore {
        self.datastore.as_ref().unwrap()
    }

    fn dispatch(&mut self, req: &mut Envelope) -> Result<()> {
        match req.message_id() {
            "AccountInvitationListRequest" => {
                let msg: proto::AccountInvitationListRequest = try!(req.parse_msg());
                let invites = try!(self.datastore()
                    .origins
                    .invites
                    .get_by_account_id(msg.get_account_id()));
                debug!("Got invites for account {} ", &msg.get_account_id());
                let mut resp = proto::AccountInvitationListResponse::new();
                resp.set_account_id(msg.get_account_id());

                let mut r_invites = RepeatedField::new();
                for invite in invites {
                    r_invites.push(invite);
                }
                resp.set_invitations(r_invites);
                try!(req.reply_complete(&mut self.sock, &resp));
            }
            "CheckOriginAccessRequest" => {
                // !!!NOTE!!!
                // !!!NOTE!!!
                // only account_id and origin_name are implemented
                // !!!NOTE!!!
                // !!!NOTE!!!
                let msg: proto::CheckOriginAccessRequest = try!(req.parse_msg());
                let is_member = try!(self.datastore()
                    .origins
                    .is_origin_member(msg.get_account_id(), msg.get_origin_name()));
                let mut resp = proto::CheckOriginAccessResponse::new();
                resp.set_has_access(is_member);
                try!(req.reply_complete(&mut self.sock, &resp));
            }
            "OriginCreate" => {
                let msg: proto::OriginCreate = try!(req.parse_msg());
                let mut origin = proto::Origin::new();
                origin.set_name(msg.get_name().to_string());
                origin.set_owner_id(msg.get_owner_id());
                // if the origin already exists, then return
                if let Ok(_origin) = self.datastore()
                    .origins
                    .name_idx
                    .find(&msg.get_name().to_string()) {
                    let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:0");
                    try!(req.reply_complete(&mut self.sock, &err));
                }

                try!(self.datastore().origins.write(&mut origin));

                // after the origin is written and has an id, add the owner
                // to the list of members
                debug!("Adding owner as origin member: {}", &msg.get_owner_name());
                try!(self.datastore()
                    .origins
                    .add_origin_member(msg.get_owner_id(), msg.get_owner_name(), msg.get_name()));
                try!(req.reply_complete(&mut self.sock, &origin));
            }
            "OriginGet" => {
                let mut msg: proto::OriginGet = try!(req.parse_msg());
                match self.datastore().origins.name_idx.find(&msg.take_name()) {
                    Ok(origin_id) => {
                        let origin = self.datastore().origins.find(&origin_id).unwrap();
                        try!(req.reply_complete(&mut self.sock, &origin));
                    }
                    Err(dbcache::Error::EntityNotFound) => {
                        let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-get:1");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e) => {
                        error!("OriginGet, err={:?}", e);
                        let err = net::err(ErrCode::BUG, "vt:origin-get:0");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                }
            }
            "OriginInvitationAcceptRequest" => {
                let msg: proto::OriginInvitationAcceptRequest = try!(req.parse_msg());

                // we might not have an invite here if it's already been accepted
                match self.datastore().origins.invites.find(&msg.get_invite_id()) {
                    Ok(invite) => {
                        debug!("REQ    {:?}", &msg);
                        debug!("INVITE {:?}", &invite);
                        if msg.get_account_accepting_request() != invite.get_account_id() {
                            let err = net::err(ErrCode::ACCESS_DENIED, "vt:origin-invite-accept:0");
                            try!(req.reply_complete(&mut self.sock, &err));
                        }

                        match self.datastore().origins.modify_invite(&invite, msg.get_ignore()) {
                            Ok(()) => (),
                            Err(e) => {
                                debug!("Error accepting invite: {}", e);
                            }
                        };
                    }
                    Err(e) => {
                        debug!("Error accepting invite, maybe it's already been accepted? {}",
                               e);
                    }
                };

                let resp = proto::OriginInvitationAcceptResponse::new();
                try!(req.reply_complete(&mut self.sock, &resp));
            }
            "OriginInvitationCreate" => {
                let msg: proto::OriginInvitationCreate = try!(req.parse_msg());
                let mut invitation = proto::OriginInvitation::new();
                if !try!(self.datastore()
                    .origins
                    .is_origin_member(msg.get_account_id(), msg.get_origin_name())) {
                    debug!("Can't invite to this org unless your already a member");
                    let err = net::err(ErrCode::ACCESS_DENIED, "vt:origin-create:0");
                    try!(req.reply_complete(&mut self.sock, &err));
                }

                let existing_invites = try!(self.datastore()
                    .origins
                    .invites
                    .get_by_account_id(msg.get_account_id()));

                for invite in &existing_invites {
                    if invite.get_origin_name() == msg.get_origin_name() {
                        debug!("Invite for origin {} for user {} already exists",
                               &msg.get_origin_name(),
                               &msg.get_account_name());
                        let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:1");
                        try!(req.reply_complete(&mut self.sock, &err));
                        return Ok(())
                    }
                }

                invitation.set_account_id(msg.get_account_id());
                invitation.set_account_name(msg.get_account_name().to_string());
                invitation.set_origin_id(msg.get_origin_id());
                invitation.set_origin_name(msg.get_origin_name().to_string());

                invitation.set_owner_id(msg.get_owner_id());
                try!(self.datastore().origins.invites.write(&mut invitation));
                try!(req.reply_complete(&mut self.sock, &invitation));
            }
            "OriginInvitationListRequest" => {
                let msg: proto::OriginInvitationListRequest = try!(req.parse_msg());
                let invites = try!(self.datastore()
                    .origins
                    .invites
                    .get_by_origin_id(msg.get_origin_id()));
                let mut resp = proto::OriginInvitationListResponse::new();
                resp.set_origin_id(msg.get_origin_id());

                let mut r_invites = RepeatedField::new();
                for invite in invites {
                    r_invites.push(invite);
                }
                resp.set_invitations(r_invites);
                try!(req.reply_complete(&mut self.sock, &resp));
            }
            "OriginList" => {
                let origin1 = proto::Origin::new();
                let origin2 = proto::Origin::new();
                let origins = vec![origin1, origin2];
                for (i, origin) in origins.iter().enumerate() {
                    if i == origins.len() {
                        try!(req.reply_complete(&mut self.sock, origin));
                    } else {
                        try!(req.reply(&mut self.sock, origin));
                    }
                }
            }
            "OriginMemberListRequest" => {
                let msg: proto::OriginMemberListRequest = try!(req.parse_msg());
                let members =
                    try!(self.datastore().origins.list_origin_members(msg.get_origin_id()));
                let mut r_members = RepeatedField::new();
                for member in members {
                    r_members.push(member);
                }
                let mut resp = proto::OriginMemberListResponse::new();
                resp.set_origin_id(msg.get_origin_id());
                resp.set_members(r_members);
                try!(req.reply_complete(&mut self.sock, &resp));
            }
            "AccountOriginListRequest" => {
                let msg: proto::AccountOriginListRequest = try!(req.parse_msg());
                let origins =
                    try!(self.datastore().origins.list_account_origins(msg.get_account_id()));
                let mut r_origins = RepeatedField::new();
                for origin in origins {
                    r_origins.push(origin);
                }
                let mut resp = proto::AccountOriginListResponse::new();
                resp.set_account_id(msg.get_account_id());
                resp.set_origins(r_origins);
                try!(req.reply_complete(&mut self.sock, &resp));
            }
            "OriginSecretKeyCreate" => {
                let msg: proto::OriginSecretKeyCreate = try!(req.parse_msg());
                let mut pk = proto::OriginSecretKey::new();
                pk.set_name(msg.get_name().to_string());
                pk.set_revision(msg.get_revision().to_string());
                pk.set_origin_id(msg.get_origin_id());
                pk.set_owner_id(msg.get_owner_id());
                pk.set_body(msg.get_body().to_vec());
                // DP TODO: handle db errors
                try!(self.datastore().origins.origin_secret_keys.write(&mut pk));
                try!(req.reply_complete(&mut self.sock, &pk));
            }
            _ => panic!("unexpected message: {}", req.message_id()),
        }
        Ok(())
    }
}

impl Supervisable for Worker {
    type Config = Config;
    type Error = Error;

    fn new(context: &mut zmq::Context, config: Arc<RwLock<Config>>) -> Self {
        let sock = context.socket(zmq::DEALER).unwrap();
        Worker {
            config: config,
            sock: sock,
            datastore: None,
        }
    }

    fn init(&mut self) -> Result<()> {
        loop {
            let result = {
                let cfg = self.config.read().unwrap();
                DataStore::open(cfg.deref())
            };
            match result {
                Ok(datastore) => {
                    self.datastore = Some(datastore);
                    break;
                }
                Err(e) => {
                    error!("{}", e);
                    thread::sleep(Duration::from_millis(5000));
                }
            }
        }
        Ok(())
    }

    fn on_message(&mut self, req: &mut Envelope) -> Result<()> {
        self.dispatch(req)
    }

    fn socket(&mut self) -> &mut zmq::Socket {
        &mut self.sock
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sock.close().unwrap();
    }
}

pub struct Server {
    config: Arc<RwLock<Config>>,
    ctx: Arc<RwLock<zmq::Context>>,
    router: RouteConn,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let mut ctx = zmq::Context::new();
        let router = try!(RouteConn::new(Self::net_ident(), &mut ctx));
        let be = try!(ctx.socket(zmq::DEALER));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            ctx: Arc::new(RwLock::new(ctx)),
            router: router,
            be_sock: be,
        })
    }

    pub fn reconfigure(&self, config: Config) -> Result<()> {
        {
            let mut cfg = self.config.write().unwrap();
            *cfg = config;
        }
        // obtain lock and replace our config
        // notify datastore to refresh it's connection if it needs to
        // notify sockets to reconnect if changes
        Ok(())
    }
}

impl Application for Server {
    type Error = Error;

    fn run(&mut self) -> Result<()> {
        try!(self.be_sock.bind(BE_LISTEN_ADDR));
        let ctx = self.ctx.clone();
        let cfg = self.config.clone();
        let sup: Supervisor<Worker> = Supervisor::new(ctx, cfg);
        {
            let cfg = self.config.read().unwrap();
            try!(sup.start(BE_LISTEN_ADDR, cfg.worker_threads));
        }
        try!(self.connect());
        try!(zmq::proxy(&mut self.router.socket, &mut self.be_sock));
        Ok(())
    }
}

impl Service for Server {
    type Application = Self;
    type Config = Config;
    type Error = Error;

    fn protocol() -> net::Protocol {
        net::Protocol::VaultSrv
    }

    fn config(&self) -> &Arc<RwLock<Self::Config>> {
        &self.config
    }

    fn conn(&self) -> &RouteConn {
        &self.router
    }

    fn conn_mut(&mut self) -> &mut RouteConn {
        &mut self.router
    }
}

impl NetIdent for Server {}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
