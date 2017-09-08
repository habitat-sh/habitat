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

use std::any::TypeId;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{Read, Write, BufWriter};
use std::result;
use std::str::FromStr;
use base64;

use uuid::Uuid;
use bld_core;
use bld_core::api::{channels_for_package_ident, platforms_for_package_ident};
use bodyparser;
use hab_core::package::{Identifiable, FromArchive, PackageArchive, PackageIdent, PackageTarget,
                        ident};
use hab_core::crypto::keys::PairType;
use hab_core::crypto::{BoxKeyPair, SigKeyPair};
use hab_core::crypto::PUBLIC_BOX_KEY_VERSION;
use hab_core::event::*;
use hab_net::config::RouterCfg;
use hab_net::http::controller::*;
use hab_net::privilege;
use hab_net::routing::{Broker, RouteResult};
use hab_net::server::NetIdent;
use hyper::header::{Charset, ContentDisposition, DispositionType, DispositionParam};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::{status, headers, typemap};
use iron::headers::{ContentType, UserAgent};
use iron::middleware::BeforeMiddleware;
use iron::prelude::*;
use iron::request::Body;
use iron::typemap::Key;
use mount::Mount;
use persistent;
use protobuf::{self, parse_from_bytes};
use protocol::net::{NetOk, ErrCode, NetError};
use protocol::originsrv::*;
use protocol::Routable;
use protocol::scheduler::{Group, GroupCreate, GroupGet, PackageStatsGet, PackageStats,
                          PackagePreCreate};
use protocol::sessionsrv::{Account, AccountGet};
use regex::Regex;
use router::{Params, Router};
use serde::Serialize;
use serde_json;
use url;
use urlencoded::UrlEncodedQuery;

use super::DepotUtil;
use config::Config;
use error::{Error, Result};
use handlers;

define_event_log!();

#[derive(Default)]
pub struct TestableBroker {
    message_map: HashMap<TypeId, Vec<u8>>,
    error_map: HashMap<TypeId, NetError>,
    cached_messages: HashMap<TypeId, Vec<u8>>,
}

impl TestableBroker {
    pub fn setup<M: Routable, R: protobuf::MessageStatic>(&mut self, response: &R) {
        let bytes = response.write_to_bytes().unwrap();
        self.message_map.insert(TypeId::of::<M>(), bytes);
    }

    pub fn setup_error<M: Routable>(&mut self, error: NetError) {
        self.error_map.insert(TypeId::of::<M>(), error);
    }

    pub fn routed_messages(&self) -> RoutedMessages {
        RoutedMessages(self.cached_messages.clone())
    }

    pub fn route<M: Routable, R: protobuf::MessageStatic>(&mut self, msg: &M) -> RouteResult<R> {
        let bytes = msg.write_to_bytes().unwrap();
        self.cached_messages.insert(TypeId::of::<M>(), bytes);
        let msg_type = &TypeId::of::<M>();
        match self.message_map.get(msg_type) {
            Some(message) => Ok(parse_from_bytes::<R>(message).unwrap()),
            None => {
                match self.error_map.get(msg_type) {
                    Some(error) => Err(error.clone()),
                    None => panic!("Unable to find message of given type"),
                }
            }
        }
    }
}

impl Key for TestableBroker {
    type Value = Self;
}

pub struct RoutedMessages(HashMap<TypeId, Vec<u8>>);

impl RoutedMessages {
    pub fn get<M: Routable>(&self) -> Result<M> {
        let msg_type = &TypeId::of::<M>();
        match self.0.get(msg_type) {
            Some(msg) => {
                Ok(parse_from_bytes::<M>(msg).expect(&format!(
                    "Unable to parse {:?} message",
                    msg_type
                )))
            }
            None => Err(Error::MessageTypeNotFound),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct OriginCreateReq {
    name: String,
    default_package_visibility: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct OriginUpdateReq {
    default_package_visibility: String,
}

#[derive(Serialize)]
struct PackageResults<'a, T: 'a> {
    range_start: isize,
    range_end: isize,
    total_count: isize,
    package_list: &'a Vec<T>,
}

const PAGINATION_RANGE_DEFAULT: isize = 0;
const PAGINATION_RANGE_MAX: isize = 50;
const ONE_YEAR_IN_SECS: usize = 31536000;

pub fn route_message<M: Routable, R: protobuf::MessageStatic>(
    req: &mut Request,
    msg: &M,
) -> RouteResult<R> {
    if let Some(broker) = req.extensions.get_mut::<TestableBroker>() {
        return broker.route::<M, R>(msg);
    }

    Broker::connect().unwrap().route::<M, R>(msg)
}

fn package_results_json<T: Serialize>(
    packages: &Vec<T>,
    count: isize,
    start: isize,
    end: isize,
) -> String {
    let results = PackageResults {
        range_start: start,
        range_end: end,
        total_count: count,
        package_list: packages,
    };

    serde_json::to_string(&results).unwrap()
}

pub fn origin_update(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginUpdate::new();

    let origin = {
        let params = req.extensions.get::<Router>().unwrap();

        match params.find("name") {
            Some(o) => o.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    match req.get::<bodyparser::Struct<OriginUpdateReq>>() {
        Ok(Some(body)) => {
            let dpv = match body.default_package_visibility
                .parse::<OriginPackageVisibility>() {
                Ok(x) => x,
                Err(_) => return Ok(Response::with(status::UnprocessableEntity)),
            };
            request.set_name(origin.clone());
            request.set_default_package_visibility(dpv);
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    }

    let mut conn = Broker::connect().unwrap();
    let mut origin_get = OriginGet::new();
    origin_get.set_name(origin);

    let origin_id = match conn.route::<OriginGet, Origin>(&origin_get) {
        Ok(o) => o.get_id(),
        Err(err) => return Ok(render_net_error(&err)),
    };

    request.set_id(origin_id);
    match conn.route::<OriginUpdate, NetOk>(&request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn origin_create(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginCreate::new();
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        request.set_owner_id(session.get_id());
        request.set_owner_name(session.get_name().to_string());
    }

    match req.get::<bodyparser::Struct<OriginCreateReq>>() {
        Ok(Some(body)) => {
            let dpv = match body.default_package_visibility
                .parse::<OriginPackageVisibility>() {
                Ok(x) => x,
                Err(_) => return Ok(Response::with(status::UnprocessableEntity)),
            };
            request.set_name(body.name);
            request.set_default_package_visibility(dpv);
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    }

    if !ident::is_valid_origin_name(request.get_name()) {
        return Ok(Response::with(status::UnprocessableEntity));
    }

    let mut conn = Broker::connect().unwrap();
    match conn.route::<OriginCreate, Origin>(&request) {
        Ok(origin) => Ok(render_json(status::Created, &origin)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn origin_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let origin = match params.find("origin") {
        Some(origin) => origin.to_string(),
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut conn = Broker::connect().unwrap();
    let mut request = OriginGet::new();
    request.set_name(origin);
    match conn.route::<OriginGet, Origin>(&request) {
        Ok(origin) => {
            let mut response = render_json(status::Ok, &origin);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn get_origin<T: ToString>(origin: T) -> IronResult<Option<Origin>> {
    match bld_core::api::get_origin(origin) {
        Ok(o) => Ok(o),
        Err(err) => {
            let body = serde_json::to_string(&err).unwrap();
            let status = net_err_to_http(err.get_code());
            Err(IronError::new(err, (body, status)))
        }
    }
}

pub fn check_origin_access<T: ToString>(account_id: u64, origin: T) -> IronResult<bool> {
    match bld_core::api::check_origin_access(account_id, origin) {
        Ok(b) => Ok(b),
        Err(err) => {
            let body = serde_json::to_string(&err).unwrap();
            let status = net_err_to_http(err.get_code());
            Err(IronError::new(err, (body, status)))
        }
    }
}

pub fn accept_invitation(req: &mut Request) -> IronResult<Response> {
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let params = req.extensions.get::<Router>().unwrap().clone();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let invitation = match params.find("invitation_id") {
        Some(invitation) => invitation,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let invitation_id = match invitation.parse::<u64>() {
        Ok(invitation_id) => invitation_id,
        Err(e) => {
            error!("Bad request; invitation ID did not parse into a u64, {}", e);
            return Ok(Response::with(status::BadRequest));
        }
    };

    let mut conn = Broker::connect().unwrap();
    debug!(
        "Accepting invitation for user {} origin {}",
        &session.get_id(),
        &origin
    );

    let mut request = OriginInvitationAcceptRequest::new();
    request.set_account_id(session.get_id());
    request.set_invite_id(invitation_id);
    request.set_origin_name(origin.to_string());
    request.set_ignore(false);

    match conn.route::<OriginInvitationAcceptRequest, NetOk>(&request) {
        Ok(_) => {
            log_event!(
                req,
                Event::OriginInvitationAccept {
                    id: request.get_invite_id().to_string(),
                    account: session.get_id().to_string(),
                }
            );
            Ok(Response::with(status::NoContent))
        }
        Err(err) => {
            error!("Error accepting invitation, {}", err);
            Ok(render_net_error(&err))
        }
    }
}

pub fn invite_to_origin(req: &mut Request) -> IronResult<Response> {
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let params = req.extensions.get::<Router>().unwrap().clone();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let user_to_invite = match params.find("username") {
        Some(username) => username,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = Broker::connect().unwrap();
    debug!(
        "Creating invitation for user {} origin {}",
        &user_to_invite,
        &origin
    );
    if !check_origin_access(session.get_id(), &origin)? {
        return Ok(Response::with(status::Forbidden));
    }
    let mut request = AccountGet::new();
    let mut invite_request = OriginInvitationCreate::new();
    request.set_name(user_to_invite.to_string());
    // Lookup the users account_id
    match conn.route::<AccountGet, Account>(&request) {
        Ok(mut account) => {
            invite_request.set_account_id(account.get_id());
            invite_request.set_account_name(account.take_name());
        }
        Err(err) => return Ok(render_net_error(&err)),
    };
    match get_origin(&origin)? {
        Some(mut origin) => {
            invite_request.set_origin_id(origin.get_id());
            invite_request.set_origin_name(origin.take_name());
        }
        None => return Ok(Response::with(status::NotFound)),
    };
    invite_request.set_owner_id(session.get_id());

    // store invitations in the originsrv
    match conn.route::<OriginInvitationCreate, OriginInvitation>(&invite_request) {
        Ok(invitation) => {
            log_event!(
                req,
                Event::OriginInvitationSend {
                    origin: origin.to_string(),
                    user: user_to_invite.to_string(),
                    id: invitation.get_id().to_string(),
                    account: session.get_id().to_string(),
                }
            );
            Ok(render_json(status::Created, &invitation))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn list_origin_invitations(req: &mut Request) -> IronResult<Response> {
    let session_id: u64;
    let origin_name: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();
        let params = req.extensions.get::<Router>().unwrap();
        origin_name = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };
    }

    let mut conn = Broker::connect().unwrap();
    if !check_origin_access(session_id, &origin_name)? {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginInvitationListRequest::new();
    match get_origin(origin_name.as_str())? {
        Some(origin) => request.set_origin_id(origin.get_id()),
        None => return Ok(Response::with(status::NotFound)),
    };

    match conn.route::<OriginInvitationListRequest, OriginInvitationListResponse>(&request) {
        Ok(list) => {
            let mut response = render_json(status::Ok, &list);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn list_origin_members(req: &mut Request) -> IronResult<Response> {
    let session_id: u64;
    let origin_name: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();
        let params = req.extensions.get::<Router>().unwrap();
        origin_name = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };
    }

    let mut conn = Broker::connect().unwrap();

    if !check_origin_access(session_id, &origin_name)? {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginMemberListRequest::new();
    match get_origin(origin_name.as_str())? {
        Some(origin) => request.set_origin_id(origin.get_id()),
        None => return Ok(Response::with(status::NotFound)),
    };
    match conn.route::<OriginMemberListRequest, OriginMemberListResponse>(&request) {
        Ok(list) => {
            let mut response = render_json(status::Ok, &list);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn write_archive(filename: &PathBuf, body: &mut Body) -> Result<PackageArchive> {
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);
    let mut written: i64 = 0;
    let mut buf = [0u8; 100000]; // Our byte buffer
    loop {
        let len = body.read(&mut buf)?; // Raise IO errors
        match len {
            0 => {
                // 0 == EOF, so stop writing and finish progress
                break;
            }
            _ => {
                // Write the buffer to the BufWriter on the Heap
                let bytes_written = writer.write(&buf[0..len])?;
                if bytes_written == 0 {
                    return Err(Error::WriteSyncFailed);
                }
                written = written + (bytes_written as i64);
            }
        };
    }
    Ok(PackageArchive::new(filename))
}

fn upload_origin_key(req: &mut Request) -> IronResult<Response> {
    debug!("Upload Origin Public Key {:?}", req);
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let params = req.extensions.get::<Router>().unwrap().clone();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginPublicKeyCreate::new();
    request.set_owner_id(session.get_id());

    let origin = match params.find("origin") {
        Some(origin) => {
            if !check_origin_access(session.get_id(), origin)? {
                return Ok(Response::with(status::Forbidden));
            }
            match get_origin(origin)? {
                Some(mut origin) => {
                    request.set_name(origin.take_name());
                    request.set_origin_id(origin.get_id());
                }
                None => return Ok(Response::with(status::NotFound)),
            }
            origin
        }
        None => return Ok(Response::with(status::BadRequest)),
    };
    match params.find("revision") {
        Some(revision) => request.set_revision(revision.to_string()),
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut key_content = Vec::new();
    if let Err(e) = req.body.read_to_end(&mut key_content) {
        debug!("Can't read key content {}", e);
        return Ok(Response::with(status::BadRequest));
    }

    match String::from_utf8(key_content.clone()) {
        Ok(content) => {
            match SigKeyPair::parse_key_str(&content) {
                Ok((PairType::Public, _, _)) => {
                    debug!("Received a valid public key");
                }
                Ok(_) => {
                    debug!("Received a secret key instead of a public key");
                    return Ok(Response::with(status::BadRequest));
                }
                Err(e) => {
                    debug!("Invalid public key content: {}", e);
                    return Ok(Response::with(status::BadRequest));
                }
            }
        }
        Err(e) => {
            debug!("Can't parse public key upload content: {}", e);
            return Ok(Response::with(status::BadRequest));
        }
    }

    request.set_body(key_content);
    request.set_owner_id(0);

    match conn.route::<OriginPublicKeyCreate, OriginPublicKey>(&request) {
        Ok(_) => {
            log_event!(
                req,
                Event::OriginKeyUpload {
                    origin: origin.to_string(),
                    version: request.get_revision().to_string(),
                    account: session.get_id().to_string(),
                }
            );
            let mut response = Response::with((
                status::Created,
                format!(
                    "/origins/{}/keys/{}",
                    &origin,
                    &request.get_revision()
                ),
            ));
            let mut base_url: url::Url = req.url.clone().into();
            base_url.set_path(&format!("key/{}-{}", &origin, &request.get_revision()));
            response.headers.set(
                headers::Location(format!("{}", base_url)),
            );
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn download_latest_origin_secret_key(req: &mut Request) -> IronResult<Response> {
    let origin = {
        let params = req.extensions.get::<Router>().unwrap();
        params.find("origin").unwrap().to_owned()
    };
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginSecretKeyGet::new();
    match get_origin(origin)? {
        Some(mut origin) => {
            request.set_owner_id(origin.get_owner_id());
            request.set_origin(origin.take_name());
        }
        None => return Ok(Response::with(status::NotFound)),
    }
    match conn.route::<OriginSecretKeyGet, OriginSecretKey>(&request) {
        Ok(ref key) => Ok(render_json(status::Ok, key)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn upload_origin_secret_key(req: &mut Request) -> IronResult<Response> {
    debug!("Upload Origin Secret Key {:?}", req);
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let params = req.extensions.get::<Router>().unwrap().clone();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginSecretKeyCreate::new();
    request.set_owner_id(session.get_id());

    let origin = match params.find("origin") {
        Some(origin) => {
            if !check_origin_access(session.get_id(), origin)? {
                return Ok(Response::with(status::Forbidden));
            }
            match get_origin(origin)? {
                Some(mut origin) => {
                    request.set_name(origin.take_name());
                    request.set_origin_id(origin.get_id());
                }
                None => return Ok(Response::with(status::NotFound)),
            }
            origin
        }
        None => return Ok(Response::with(status::BadRequest)),
    };
    match params.find("revision") {
        Some(revision) => request.set_revision(revision.to_string()),
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut key_content = Vec::new();
    if let Err(e) = req.body.read_to_end(&mut key_content) {
        debug!("Can't read key content {}", e);
        return Ok(Response::with(status::BadRequest));
    }

    match String::from_utf8(key_content.clone()) {
        Ok(content) => {
            match SigKeyPair::parse_key_str(&content) {
                Ok((PairType::Secret, _, _)) => {
                    debug!("Received a valid secret key");
                }
                Ok(_) => {
                    debug!("Received a public key instead of a secret key");
                    return Ok(Response::with(status::BadRequest));
                }
                Err(e) => {
                    debug!("Invalid secret key content: {}", e);
                    return Ok(Response::with(status::BadRequest));
                }
            }
        }
        Err(e) => {
            debug!("Can't parse secret key upload content: {}", e);
            return Ok(Response::with(status::BadRequest));
        }
    }

    request.set_body(key_content);
    request.set_owner_id(0);

    match conn.route::<OriginSecretKeyCreate, OriginSecretKey>(&request) {
        Ok(_) => {
            log_event!(
                req,
                Event::OriginSecretKeyUpload {
                    origin: origin.to_string(),
                    version: request.take_revision(),
                    account: session.get_id().to_string(),
                }
            );
            Ok(Response::with(status::Created))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn upload_package(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");
    let checksum_from_param = match extract_query_value("checksum", req) {
        Some(checksum) => checksum,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let ident = {
        let params = req.extensions.get::<Router>().unwrap();
        ident_from_params(params)
    };

    if !ident.valid() {
        info!("Invalid package identifier: {}", ident);
        return Ok(Response::with(status::BadRequest));
    }

    debug!(
        "UPLOADING checksum={}, ident={}",
        checksum_from_param,
        ident
    );

    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    if !depot.config.insecure {
        if !check_origin_access(session.get_id(), &ident.get_origin())? {
            debug!(
                "Failed origin access check, session: {}, ident: {}",
                session.get_id(),
                ident
            );
            return Ok(Response::with(status::Forbidden));
        }
        if !ident.fully_qualified() {
            return Ok(Response::with(status::BadRequest));
        }
    }

    // Find the path to folder where archive should be created, and
    // create the folder if necessary
    let parent_path = depot.archive_parent(&ident);

    match fs::create_dir_all(parent_path.clone()) {
        Ok(_) => {}
        Err(e) => {
            error!("Unable to create archive directory, err={:?}", e);
            return Ok(Response::with(status::InternalServerError));
        }
    };

    // Create a temp file at the archive location
    let temp_name = format!("{}.tmp", Uuid::new_v4());
    let temp_path = parent_path.join(temp_name);

    let mut archive = write_archive(&temp_path, &mut req.body)?;
    debug!("Package Archive: {:#?}", archive);

    let target_from_artifact = match archive.target() {
        Ok(target) => target,
        Err(e) => {
            info!("Could not read the target for {:#?}: {:#?}", archive, e);
            return Ok(Response::with((
                status::UnprocessableEntity,
                format!("ds:up:1, err={:?}", e),
            )));
        }
    };

    if !depot.config.targets.contains(&target_from_artifact) {
        debug!(
            "Unsupported package platform or architecture {}.",
            target_from_artifact
        );
        return Ok(Response::with(status::NotImplemented));
    };

    let mut ident_req = OriginPackageGet::new();
    ident_req.set_ident(ident.clone());

    match route_message::<OriginPackageGet, OriginPackage>(req, &ident_req) {
        Ok(_) => return Ok(Response::with((status::Conflict))),
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => {
                    if depot.archive(&ident, &target_from_artifact).is_some() {
                        return Ok(Response::with((status::Conflict)));
                    }
                }
                _ => {
                    error!("upload_package:1, err={:?}", err);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }
    }

    let checksum_from_artifact = match archive.checksum() {
        Ok(cksum) => cksum,
        Err(e) => {
            info!("Could not compute a checksum for {:#?}: {:#?}", archive, e);
            return Ok(Response::with((status::UnprocessableEntity, "ds:up:2")));
        }
    };
    if checksum_from_param != checksum_from_artifact {
        info!(
            "Checksums did not match: from_param={:?}, from_artifact={:?}",
            checksum_from_param,
            checksum_from_artifact
        );
        return Ok(Response::with((status::UnprocessableEntity, "ds:up:3")));
    }

    // Check with scheduler to ensure we don't have circular deps
    let mut pcr_req = PackagePreCreate::new();
    pcr_req.set_ident(format!("{}", ident));

    let mut pcr_deps = protobuf::RepeatedField::new();
    let deps_from_artifact = match archive.deps() {
        Ok(deps) => deps,
        Err(e) => {
            info!("Could not get deps from {:#?}: {:#?}", archive, e);
            return Ok(Response::with((status::UnprocessableEntity, "ds:up:4")));
        }
    };

    for ident in deps_from_artifact {
        let dep_str = format!("{}", ident);
        pcr_deps.push(dep_str);
    }
    pcr_req.set_deps(pcr_deps);

    match route_message::<PackagePreCreate, NetOk>(req, &pcr_req) {
        Ok(_) => (),
        Err(e) => {
            if e.get_code() == ErrCode::ENTITY_CONFLICT {
                warn!(
                    "Failed package circular dependency check: {:?}, err: {:?}",
                    ident,
                    e
                );
                return Ok(Response::with(status::FailedDependency));
            } else {
                error!(
                    "Unable to check package dependency: {:?}, err: {:?}",
                    ident,
                    e
                );
                return Ok(Response::with(status::InternalServerError));
            }
        }
    }

    let filename = depot.archive_path(&ident, &target_from_artifact);

    match fs::rename(&temp_path, &filename) {
        Ok(_) => {}
        Err(e) => {
            error!(
                "Unable to rename temp archive {:?} to {:?}, err={:?}",
                temp_path,
                filename,
                e
            );
            return Ok(Response::with(status::InternalServerError));
        }
    }

    info!("File added to Depot at {}", filename.to_string_lossy());
    let mut archive = PackageArchive::new(filename);
    let mut package = match OriginPackageCreate::from_archive(&mut archive) {
        Ok(package) => package,
        Err(e) => {
            info!("Error building package from archive: {:#?}", e);
            return Ok(Response::with((status::UnprocessableEntity, "ds:up:5")));
        }
    };
    if ident.satisfies(package.get_ident()) {
        package.set_owner_id(session.get_id());

        // let's make sure this origin actually exists
        match get_origin(&ident.get_origin())? {
            Some(origin) => {
                package.set_origin_id(origin.get_id());
            }
            None => return Ok(Response::with(status::NotFound)),
        };

        match route_message::<OriginPackageCreate, OriginPackage>(req, &package) {
            Ok(_) => (),
            Err(err) => {
                error!(
                    "Unable to create origin package for {:?}, err={:?}",
                    ident,
                    err
                );
                return Ok(Response::with(status::InternalServerError));
            }
        }

        log_event!(
            req,
            Event::PackageUpload {
                origin: ident.get_origin().to_string(),
                package: ident.get_name().to_string(),
                version: ident.get_version().to_string(),
                release: ident.get_release().to_string(),
                target: target_from_artifact.to_string(),
                account: session.get_id().to_string(),
            }
        );

        // Schedule re-build of dependent packages (if requested)
        // Don't schedule builds if the upload is being done by the builder
        // Currently, we only do dep builds of 'core' packages
        if depot.config.builds_enabled && ident.get_origin() == "core" &&
            !match extract_query_value("builder", req) {
                Some(_) => true,
                None => false,
            }
        {
            let mut conn = Broker::connect().unwrap();

            let mut request = GroupCreate::new();
            request.set_origin(ident.get_origin().to_string());
            request.set_package(ident.get_name().to_string());
            request.set_deps_only(true);

            match conn.route::<GroupCreate, Group>(&request) {
                Ok(group) => {
                    debug!(
                        "Scheduled reverse dependecy build, group id: {}",
                        group.get_id()
                    )
                }
                Err(err) => error!("Unable to schedule build, err: {:?}", err),
            }
        }

        let mut response = Response::with((
            status::Created,
            format!("/pkgs/{}/download", package.get_ident()),
        ));
        let mut base_url: url::Url = req.url.clone().into();
        base_url.set_path(&format!("pkgs/{}/download", package.get_ident()));
        response.headers.set(
            headers::Location(format!("{}", base_url)),
        );
        Ok(response)
    } else {
        info!(
            "Ident mismatch, expected={:?}, got={:?}",
            ident,
            package.get_ident()
        );
        Ok(Response::with((status::UnprocessableEntity, "ds:up:6")))
    }
}

fn package_stats(req: &mut Request) -> IronResult<Response> {
    let origin = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("origin") {
            Some(s) => s,
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    let mut conn = Broker::connect().unwrap();

    let mut request = PackageStatsGet::new();
    request.set_origin(String::from(origin));

    match conn.route::<PackageStatsGet, PackageStats>(&request) {
        Ok(stats) => {
            let mut response = render_json(status::Ok, &stats);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn schedule(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap().clone();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let package = match params.find("pkg") {
        Some(pkg) => pkg,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut conn = Broker::connect().unwrap();

    let mut request = GroupCreate::new();
    request.set_origin(String::from(origin));
    request.set_package(String::from(package));

    match conn.route::<GroupCreate, Group>(&request) {
        Ok(group) => {
            let mut response = render_json(status::Ok, &group);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn get_schedule(req: &mut Request) -> IronResult<Response> {
    let group_id = {
        let params = req.extensions.get::<Router>().unwrap();
        let group_id_str = match params.find("groupid") {
            Some(s) => s,
            None => return Ok(Response::with(status::BadRequest)),
        };

        match group_id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let mut conn = Broker::connect().unwrap();

    let mut request = GroupGet::new();
    request.set_group_id(group_id);

    match conn.route::<GroupGet, Group>(&request) {
        Ok(group) => {
            let mut response = render_json(status::Ok, &group);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

// This function should not require authentication (session/auth token)
fn download_origin_key(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginPublicKeyGet::new();

    match params.find("origin") {
        Some(origin) => request.set_origin(origin.to_string()),
        None => return Ok(Response::with(status::BadRequest)),
    };
    match params.find("revision") {
        Some(revision) => request.set_revision(revision.to_string()),
        None => return Ok(Response::with(status::BadRequest)),
    };

    let key = match conn.route::<OriginPublicKeyGet, OriginPublicKey>(&request) {
        Ok(key) => key,
        Err(err) => {
            error!("Can't retrieve key file: {}", err);
            return Ok(Response::with(status::NotFound));
        }
    };

    let xfilename = format!("{}-{}.pub", key.get_name(), key.get_revision());
    let mut response = Response::with((status::Ok, key.get_body()));
    response.headers.set(ContentDisposition(
        format!("attachment; filename=\"{}\"", xfilename),
    ));
    response.headers.set(XFileName(xfilename));
    do_cache_response(&mut response);
    Ok(response)
}

// This function should not require authentication (session/auth token)
fn download_latest_origin_key(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginPublicKeyLatestGet::new();

    match params.find("origin") {
        Some(origin) => request.set_origin(origin.to_string()),
        None => return Ok(Response::with(status::BadRequest)),
    };

    let key = match conn.route::<OriginPublicKeyLatestGet, OriginPublicKey>(&request) {
        Ok(key) => key,
        Err(err) => {
            error!("Can't retrieve key file: {}", err);
            return Ok(Response::with(status::NotFound));
        }
    };

    let xfilename = format!("{}-{}.pub", key.get_name(), key.get_revision());
    let mut response = Response::with((status::Ok, key.get_body()));
    response.headers.set(ContentDisposition(
        format!("attachment; filename=\"{}\"", xfilename),
    ));
    response.headers.set(XFileName(xfilename));
    dont_cache_response(&mut response);
    Ok(response)
}

fn package_channels(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let ident = ident_from_params(params);

    if !ident.fully_qualified() {
        error!("package_channels:1");
        return Ok(Response::with(status::BadRequest));
    }

    let mut request = OriginPackageChannelListRequest::new();
    request.set_ident(ident);

    match conn.route::<OriginPackageChannelListRequest, OriginPackageChannelListResponse>(
        &request,
    ) {
        Ok(channels) => {
            let list: Vec<String> = channels
                .get_channels()
                .iter()
                .map(|channel| channel.get_name().to_string())
                .collect();
            let body = serde_json::to_string(&list).unwrap();
            let mut response = Response::with((status::Ok, body));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(e) => Ok(render_net_error(&e)),
    }
}

fn download_package(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");
    let mut ident_req = OriginPackageGet::new();
    {
        let params = req.extensions.get::<Router>().unwrap();
        ident_req.set_ident(ident_from_params(params));
    };
    let agent_target = target_from_headers(&req.headers.get::<UserAgent>().unwrap()).unwrap();
    if !depot.config.targets.contains(&agent_target) {
        error!(
            "Unsupported client platform ({}) for this depot.",
            agent_target
        );
        return Ok(Response::with(status::NotImplemented));
    }

    match route_message::<OriginPackageGet, OriginPackage>(req, &ident_req) {
        Ok(package) => {
            if let Some(archive) = depot.archive(package.get_ident(), &agent_target) {
                match fs::metadata(&archive.path) {
                    Ok(_) => {
                        let mut response = Response::with((status::Ok, archive.path.clone()));
                        do_cache_response(&mut response);
                        let disp = ContentDisposition {
                            disposition: DispositionType::Attachment,
                            parameters: vec![
                                DispositionParam::Filename(
                                    Charset::Iso_8859_1,
                                    None,
                                    archive.file_name().as_bytes().to_vec()
                                ),
                            ],
                        };
                        response.headers.set(disp);
                        response.headers.set(XFileName(archive.file_name()));
                        Ok(response)
                    }
                    Err(_) => Ok(Response::with(status::NotFound)),
                }
            } else {
                // This can happen if the package is not found in the file system for some reason
                error!("package not found - inconsistentcy between metadata and filesystem. download_package:2",);
                Ok(Response::with(status::InternalServerError))
            }
        }
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("download_package:1, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

fn list_origin_keys(req: &mut Request) -> IronResult<Response> {
    let origin_name: String;
    {
        let params = req.extensions.get::<Router>().unwrap();
        origin_name = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    let mut request = OriginPublicKeyListRequest::new();
    match get_origin(origin_name.as_str())? {
        Some(origin) => request.set_origin_id(origin.get_id()),
        None => return Ok(Response::with(status::NotFound)),
    };
    match route_message::<OriginPublicKeyListRequest, OriginPublicKeyListResponse>(req, &request) {
        Ok(list) => {
            let list: Vec<OriginKeyIdent> = list.get_keys()
                .iter()
                .map(|key| {
                    let mut ident = OriginKeyIdent::new();
                    ident.set_location(format!(
                        "/origins/{}/keys/{}",
                        &key.get_name(),
                        &key.get_revision()
                    ));
                    ident.set_origin(key.get_name().to_string());
                    ident.set_revision(key.get_revision().to_string());
                    ident
                })
                .collect();
            let body = serde_json::to_string(&list).unwrap();
            let mut response = Response::with((status::Ok, body));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn list_unique_packages(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginPackageUniqueListRequest::new();
    let (start, stop) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    request.set_start(start as u64);
    request.set_stop(stop as u64);
    {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("origin") {
            Some(origin) => request.set_origin(origin.to_string()),
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    match route_message::<OriginPackageUniqueListRequest, OriginPackageUniqueListResponse>(
        req,
        &request,
    ) {
        Ok(packages) => {
            debug!(
                "list_unique_packages start: {}, stop: {}, total count: {}",
                packages.get_start(),
                packages.get_stop(),
                packages.get_count()
            );
            let body = package_results_json(
                &packages.get_idents().to_vec(),
                packages.get_count() as isize,
                packages.get_start() as isize,
                packages.get_stop() as isize,
            );

            let mut response =
                if packages.get_count() as isize > (packages.get_stop() as isize + 1) {
                    Response::with((status::PartialContent, body))
                } else {
                    Response::with((status::Ok, body))
                };

            response.headers.set(ContentType(Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)],
            )));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("list_unique_packages:2, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

fn list_package_versions(req: &mut Request) -> IronResult<Response> {
    let (origin, name) = {
        let params = req.extensions.get::<Router>().unwrap();

        let origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let name = match params.find("pkg") {
            Some(pkg) => pkg.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        (origin, name)
    };

    let packages: RouteResult<OriginPackageVersionListResponse>;

    let mut request = OriginPackageVersionListRequest::new();
    request.set_origin(origin);
    request.set_name(name);
    packages = route_message::<OriginPackageVersionListRequest, OriginPackageVersionListResponse>(
        req,
        &request,
    );

    match packages {
        Ok(packages) => {
            debug!("packages = {:?}", &packages);
            let body = serde_json::to_string(&packages.get_versions().to_vec()).unwrap();
            let mut response = Response::with((status::Ok, body));

            response.headers.set(ContentType(Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)],
            )));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("list_package_versions:1, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

fn list_packages(req: &mut Request) -> IronResult<Response> {
    let mut distinct = false;
    let (start, stop) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };

    let (ident, channel) = {
        let params = req.extensions.get::<Router>().unwrap();

        let origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let ident: String = if params.find("pkg").is_none() {
            origin
        } else {
            ident_from_params(params).to_string()
        };

        let channel = match params.find("channel") {
            Some(ch) => Some(ch.to_string()),
            None => None,
        };

        (ident, channel)
    };

    let packages: RouteResult<OriginPackageListResponse>;
    match channel {
        Some(channel) => {
            let mut request = OriginChannelPackageListRequest::new();
            request.set_name(channel);
            request.set_start(start as u64);
            request.set_stop(stop as u64);
            request.set_ident(OriginPackageIdent::from_str(ident.as_str()).expect(
                "invalid package identifier",
            ));
            packages =
                route_message::<OriginChannelPackageListRequest, OriginPackageListResponse>(
                    req,
                    &request,
                );
        }
        None => {
            let mut request = OriginPackageListRequest::new();
            request.set_start(start as u64);
            request.set_stop(stop as u64);

            // only set this if "distinct" is present as a URL parameter, e.g. ?distinct=true
            if extract_query_value("distinct", req).is_some() {
                distinct = true;
                request.set_distinct(true);
            }

            request.set_ident(OriginPackageIdent::from_str(ident.as_str()).expect(
                "invalid package identifier",
            ));
            packages =
                route_message::<OriginPackageListRequest, OriginPackageListResponse>(req, &request);
        }
    }

    match packages {
        Ok(packages) => {
            debug!(
                "list_packages start: {}, stop: {}, total count: {}",
                packages.get_start(),
                packages.get_stop(),
                packages.get_count()
            );

            let mut results = Vec::new();

            // The idea here is for every package we get back, pull its channels using the zmq API
            // and accumulate those results. This avoids the N+1 HTTP requests that would be
            // required to fetch channels for a list of packages in the UI. However, if our request
            // has been marked as "distinct" then skip this step because it doesn't make sense in
            // that case. Let's get platforms at the same time.
            for package in packages.get_idents().to_vec() {
                let mut channels: Option<Vec<String>> = None;
                let mut platforms: Option<Vec<String>> = None;

                if !distinct {
                    channels = channels_for_package_ident(&package);
                    platforms = platforms_for_package_ident(&package);
                }

                let mut pkg_json = serde_json::to_value(package).unwrap();

                if channels.is_some() {
                    pkg_json["channels"] = json!(channels);
                }

                if platforms.is_some() {
                    pkg_json["platforms"] = json!(platforms);
                }

                results.push(pkg_json);
            }

            let body = package_results_json(
                &results,
                packages.get_count() as isize,
                packages.get_start() as isize,
                packages.get_stop() as isize,
            );

            let mut response =
                if packages.get_count() as isize > (packages.get_stop() as isize + 1) {
                    Response::with((status::PartialContent, body))
                } else {
                    Response::with((status::Ok, body))
                };

            response.headers.set(ContentType(Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)],
            )));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("list_packages:2, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

fn list_channels(req: &mut Request) -> IronResult<Response> {
    let origin_name = {
        let params = req.extensions.get::<Router>().unwrap();

        match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    let mut request = OriginChannelListRequest::new();
    match get_origin(origin_name.as_str())? {
        Some(origin) => request.set_origin_id(origin.get_id()),
        None => return Ok(Response::with(status::NotFound)),
    };

    match route_message::<OriginChannelListRequest, OriginChannelListResponse>(req, &request) {
        Ok(list) => {
            let list: Vec<OriginChannelIdent> = list.get_channels()
                .iter()
                .map(|channel| {
                    let mut ident = OriginChannelIdent::new();
                    ident.set_name(channel.get_name().to_string());
                    ident
                })
                .collect();
            let body = serde_json::to_string(&list).unwrap();
            let mut response = Response::with((status::Ok, body));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn create_channel(req: &mut Request) -> IronResult<Response> {
    let session_id: u64;
    let origin: String;
    let channel: String;

    // JB TODO: eliminate the need to clone the session and params.  HI SALIM =)
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    session_id = session.get_id();

    let params = req.extensions.get::<Router>().unwrap().clone();
    origin = match params.find("origin") {
        Some(origin) => origin.to_string(),
        None => return Ok(Response::with(status::BadRequest)),
    };
    channel = match params.find("channel") {
        Some(channel) => channel.to_string(),
        None => return Ok(Response::with(status::BadRequest)),
    };

    do_channel_creation(&origin, &channel, session_id)
}

fn delete_channel(req: &mut Request) -> IronResult<Response> {
    let session_id: u64;
    let origin: String;
    let channel: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();
        let params = req.extensions.get::<Router>().unwrap();
        origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };
        channel = match params.find("channel") {
            Some(channel) => channel.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };
    }

    let mut channel_req = OriginChannelGet::new();
    channel_req.set_origin_name(origin.clone());
    channel_req.set_name(channel.clone());
    match route_message::<OriginChannelGet, OriginChannel>(req, &channel_req) {
        Ok(origin_channel) => {
            // make sure the person trying to create the channel has access to do so
            if !check_origin_access(session_id, &origin)? {
                return Ok(Response::with(status::Forbidden));
            }

            // stable and unstable can't be deleted
            if channel == "stable" || channel == "unstable" {
                return Ok(Response::with(status::Forbidden));
            }

            let mut delete = OriginChannelDelete::new();
            delete.set_id(origin_channel.get_id());
            delete.set_origin_id(origin_channel.get_origin_id());
            match route_message::<OriginChannelDelete, NetOk>(req, &delete) {
                Ok(_) => Ok(Response::with(status::Ok)),
                Err(err) => {
                    error!("Error deleting channel, {}", err);
                    Ok(render_net_error(&err))
                }
            }
        }
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("delete_channel:1, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

fn show_package(req: &mut Request) -> IronResult<Response> {
    let (mut ident, channel) = {
        let params = req.extensions.get::<Router>().unwrap();

        if params.find("origin").is_none() {
            return Ok(Response::with(status::BadRequest));
        }

        let ident = ident_from_params(params);

        let channel = match params.find("channel") {
            Some(ch) => Some(ch.to_owned()),
            None => None,
        };

        (ident, channel)
    };

    let qualified = ident.fully_qualified();

    if let Some(channel) = channel {
        if !qualified {
            let target = target_from_headers(&req.headers.get::<UserAgent>().unwrap())
                .unwrap()
                .to_string();
            let mut request = OriginChannelPackageLatestGet::new();
            request.set_name(channel.clone());
            request.set_ident(ident);
            request.set_target(target);
            match route_message::<OriginChannelPackageLatestGet, OriginPackageIdent>(
                req,
                &request,
            ) {
                Ok(id) => ident = id.into(),
                Err(err) => {
                    match err.get_code() {
                        ErrCode::ENTITY_NOT_FOUND => return Ok(Response::with((status::NotFound))),
                        _ => {
                            error!("show_package:2, err={:?}", err);
                            return Ok(Response::with(status::InternalServerError));
                        }
                    }
                }
            }
        }

        let mut request = OriginChannelPackageGet::new();
        request.set_name(channel);
        request.set_ident(ident);
        match route_message::<OriginChannelPackageGet, OriginPackage>(req, &request) {
            Ok(pkg) => render_package(&pkg, false),
            Err(err) => {
                match err.get_code() {
                    ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                    _ => {
                        error!("show_package:3, err={:?}", err);
                        Ok(Response::with(status::InternalServerError))
                    }
                }
            }
        }
    } else {
        if !qualified {
            let target = target_from_headers(&req.headers.get::<UserAgent>().unwrap())
                .unwrap()
                .to_string();
            let mut request = OriginPackageLatestGet::new();
            request.set_ident(ident);
            request.set_target(target);
            match route_message::<OriginPackageLatestGet, OriginPackageIdent>(req, &request) {
                Ok(id) => ident = id.into(),
                Err(err) => {
                    match err.get_code() {
                        ErrCode::ENTITY_NOT_FOUND => return Ok(Response::with((status::NotFound))),
                        _ => {
                            error!("show_package:5, err={:?}", err);
                            return Ok(Response::with(status::InternalServerError));
                        }
                    }
                }
            }
        }

        let mut request = OriginPackageGet::new();
        request.set_ident(ident);
        match route_message::<OriginPackageGet, OriginPackage>(req, &request) {
            Ok(pkg) => {
                // If the request was for a fully qualified ident, cache the response, otherwise do
                // not cache
                if qualified {
                    render_package(&pkg, true)
                } else {
                    render_package(&pkg, false)
                }
            }
            Err(err) => {
                match err.get_code() {
                    ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                    _ => {
                        error!("show_package:6, err={:?}", err);
                        Ok(Response::with(status::InternalServerError))
                    }
                }
            }
        }
    }
}

fn search_packages(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginPackageSearchRequest::new();
    let (start, stop) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    request.set_start(start as u64);
    request.set_stop(stop as u64);


    // First, try to parse the query like it's a PackageIdent, since it seems reasonable to expect
    // that many people will try searching using that kind of string, e.g. core/redis.  If that
    // works, set the origin appropriately and do a regular search.  If that doesn't work, do a
    // search across all origins, similar to how the "distinct" search works now, but returning all
    // the details instead of just names.

    {
        let params = req.extensions.get::<Router>().unwrap();
        let query = params.find("query").unwrap();

        let decoded_query = match url::percent_encoding::percent_decode(query.as_bytes())
            .decode_utf8() {
            Ok(q) => q.to_string(),
            Err(_) => return Ok(Response::with(status::BadRequest)),
        };

        match PackageIdent::from_str(decoded_query.as_ref()) {
            Ok(ident) => {
                request.set_origin(ident.origin().to_string());
                request.set_query(ident.name().to_string());
            }
            Err(_) => {
                request.set_query(decoded_query);
            }
        }
    };

    debug!("search_packages called with: {}", request.get_query());

    // Not sure if we need this
    // Counter::SearchPackages.increment();
    // Gauge::PackageCount.set(depot.datastore.key_count().unwrap() as f64);

    // Setting distinct to true makes this query ignore any origin set, because it's going to
    // search both the origin name and the package name for the query string provided. This is
    // likely sub-optimal for performance but it makes things work right now and we should probably
    // switch to some kind of full-text search engine in the future anyway.
    // Also, to get this behavior, you need to ensure that "distinct" is a URL parameter in your
    // request, e.g. blah?distinct=true
    if extract_query_value("distinct", req).is_some() {
        request.set_distinct(true);
    }

    match route_message::<OriginPackageSearchRequest, OriginPackageListResponse>(req, &request) {
        Ok(packages) => {
            debug!(
                "search_packages start: {}, stop: {}, total count: {}",
                packages.get_start(),
                packages.get_stop(),
                packages.get_count()
            );
            let body = package_results_json(
                &packages.get_idents().to_vec(),
                packages.get_count() as isize,
                packages.get_start() as isize,
                packages.get_stop() as isize,
            );

            let mut response =
                if packages.get_count() as isize > (packages.get_stop() as isize + 1) {
                    Response::with((status::PartialContent, body))
                } else {
                    Response::with((status::Ok, body))
                };

            response.headers.set(ContentType(Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)],
            )));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => {
            error!("search_packages:2, err={:?}", err);
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn render_package(pkg: &OriginPackage, should_cache: bool) -> IronResult<Response> {
    let mut pkg_json = serde_json::to_value(pkg.clone()).unwrap();
    let channels = channels_for_package_ident(pkg.get_ident());
    pkg_json["channels"] = json!(channels);

    let body = serde_json::to_string(&pkg_json).unwrap();
    let mut response = Response::with((status::Ok, body));
    response.headers.set(ETag(pkg.get_checksum().to_string()));
    response.headers.set(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![(Attr::Charset, Value::Utf8)],
    )));
    if should_cache {
        do_cache_response(&mut response);
    } else {
        dont_cache_response(&mut response);
    }
    Ok(response)
}

fn promote_package(req: &mut Request) -> IronResult<Response> {
    let (channel, ident, session_id) = {
        let session = req.extensions.get::<Authenticated>().unwrap().clone();
        let session_id = session.get_id();

        let params = req.extensions.get::<Router>().unwrap().clone();
        let origin = match params.find("origin") {
            Some(o) => o.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let channel = match params.find("channel") {
            Some(c) => c.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let pkg = match params.find("pkg") {
            Some(p) => p.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let version = match params.find("version") {
            Some(v) => v.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let release = match params.find("release") {
            Some(r) => r.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let mut ident = OriginPackageIdent::new();
        ident.set_origin(origin);
        ident.set_name(pkg);
        ident.set_version(version);
        ident.set_release(release);

        (channel, ident, session_id)
    };

    do_promotion(&ident, &channel, session_id)
}

fn demote_package(req: &mut Request) -> IronResult<Response> {
    let (channel, ident, session_id) = {
        let session = req.extensions.get::<Authenticated>().unwrap();
        let session_id = session.get_id();

        let params = req.extensions.get::<Router>().unwrap();
        let origin = match params.find("origin") {
            Some(o) => o.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let channel = match params.find("channel") {
            Some(c) => c.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let pkg = match params.find("pkg") {
            Some(p) => p.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let version = match params.find("version") {
            Some(v) => v.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let release = match params.find("release") {
            Some(r) => r.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let mut ident = OriginPackageIdent::new();
        ident.set_origin(origin);
        ident.set_name(pkg);
        ident.set_version(version);
        ident.set_release(release);

        (channel, ident, session_id)
    };

    // you can't demote from "unstable"
    if channel == "unstable" {
        return Ok(Response::with(status::Forbidden));
    }

    let mut channel_req = OriginChannelGet::new();
    channel_req.set_origin_name(ident.get_origin().to_string());
    channel_req.set_name(channel);
    match route_message::<OriginChannelGet, OriginChannel>(req, &channel_req) {
        Ok(origin_channel) => {
            if !check_origin_access(session_id, &ident.get_origin())? {
                return Ok(Response::with(status::Forbidden));
            }

            let mut request = OriginPackageGet::new();
            request.set_ident(ident.clone());
            match route_message::<OriginPackageGet, OriginPackage>(req, &request) {
                Ok(package) => {
                    let mut demote = OriginPackageDemote::new();
                    demote.set_channel_id(origin_channel.get_id());
                    demote.set_package_id(package.get_id());
                    demote.set_ident(ident);
                    match route_message::<OriginPackageDemote, NetOk>(req, &demote) {
                        Ok(_) => Ok(Response::with(status::Ok)),
                        Err(err) => {
                            error!("Error demoting package, {}", err);
                            Ok(render_net_error(&err))
                        }
                    }
                }
                Err(err) => {
                    match err.get_code() {
                        ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                        _ => {
                            error!("demote:2, err={:?}", err);
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
                }
            }
        }
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("demote_package:1, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

pub fn download_latest_builder_key(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");

    // The builder key pair is expected to be found at the key_dir config.
    // It is not currently persisted in the DB. Instead, it will be
    // propagated via a 'hab file upload' to the depot service group.
    let kp = match BoxKeyPair::get_latest_pair_for(
        bld_core::keys::BUILDER_KEY_NAME,
        &depot.config.key_dir,
    ) {
        Ok(p) => p,
        Err(_) => {
            error!("Can't find bldr key pair at {:?}", depot.config.key_dir);
            return Ok(Response::with(status::NotFound));
        }
    };

    let key = match kp.public() {
        Ok(k) => k,
        Err(_) => {
            error!("Can't find public key in key pair: {}", kp.name_with_rev());
            return Ok(Response::with(status::NotFound));
        }
    };

    let xfilename = format!("{}-{}.pub", kp.name, kp.rev);
    let body = base64::encode(&key[..]);

    let output = format!(
        "{}\n{}\n\n{}",
        PUBLIC_BOX_KEY_VERSION,
        kp.name_with_rev(),
        body
    );

    let mut response = Response::with((status::Ok, output));
    response.headers.set(ContentDisposition(
        format!("attachment; filename=\"{}\"", xfilename),
    ));
    response.headers.set(XFileName(xfilename));
    dont_cache_response(&mut response);
    Ok(response)
}

fn ident_from_params(params: &Params) -> OriginPackageIdent {
    let mut ident = OriginPackageIdent::new();
    ident.set_origin(params.find("origin").unwrap().to_string());
    ident.set_name(params.find("pkg").unwrap().to_string());
    if let Some(ver) = params.find("version") {
        ident.set_version(ver.to_string());
    }
    if let Some(rel) = params.find("release") {
        ident.set_release(rel.to_string());
    }
    ident
}

fn target_from_headers(user_agent_header: &UserAgent) -> result::Result<PackageTarget, Response> {
    let user_agent = user_agent_header.as_str();
    debug!("Headers = {}", &user_agent);

    let user_agent_regex = Regex::new(
        r"(?P<client>[^\s]+)\s?(\((?P<target>\w+-\w+); (?P<kernel>.*)\))?",
    ).unwrap();
    let user_agent_capture = user_agent_regex.captures(user_agent).expect(
        "Invalid user agent supplied.",
    );

    // All of our tooling that depends on this function to return a target will have a user
    // agent that includes the platform. Therefore, if we can't find a target, it's safe to
    // assume that some other kind of HTTP tool is being used, e.g. curl. For those kinds
    // of clients, the target platform isn't important, so let's default it to linux
    // instead of returning a bad request.
    let target = if let Some(target_match) = user_agent_capture.name("target") {
        target_match.as_str()
    } else {
        "x86_64-linux"
    };

    match PackageTarget::from_str(target) {
        Ok(t) => Ok(t),
        Err(_) => Err(Response::with(status::BadRequest)),
    }
}

pub fn do_channel_creation(origin: &str, channel: &str, session_id: u64) -> IronResult<Response> {
    let origin_channel = match bld_core::api::create_channel(origin, channel, session_id) {
        Ok(c) => c,
        Err(bld_core::Error::OriginNotFound(_)) => return Ok(Response::with(status::NotFound)),
        Err(bld_core::Error::NetError(e)) => return Ok(render_net_error(&e)),
        Err(e) => {
            error!("channel_create:1, err={:?}", &e);
            return Ok(Response::with(status::InternalServerError));
        }
    };

    Ok(render_json(status::Created, &origin_channel))
}

pub fn do_promotion(
    ident: &OriginPackageIdent,
    channel: &str,
    session_id: u64,
) -> IronResult<Response> {
    match bld_core::api::promote_package_to_channel(ident, channel, session_id) {
        Ok(_) => Ok(Response::with(status::Ok)),
        Err(bld_core::Error::OriginAccessDenied) => Ok(Response::with(status::Forbidden)),
        Err(bld_core::Error::NetError(err)) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with(status::NotFound)),
                _ => {
                    error!("promote:1, err={:?}", &err);
                    Ok(render_net_error(&err))
                }
            }
        }
        Err(e) => {
            error!("promote:2, err={:?}", &e);
            Ok(Response::with(status::InternalServerError))
        }
    }
}

// Returns a tuple representing the from and to values representing a paginated set.
// The range (start, stop) values are zero-based.
//
// These values can be passed to a sorted set in Redis to return a paginated list.
fn extract_pagination(req: &mut Request) -> result::Result<(isize, isize), Response> {
    let range_from_param = match extract_query_value("range", req) {
        Some(range) => range,
        None => PAGINATION_RANGE_DEFAULT.to_string(),
    };

    let offset = {
        match range_from_param.parse::<usize>() {
            Ok(range) => range as isize,
            Err(_) => return Err(Response::with(status::BadRequest)),
        }
    };

    debug!(
        "extract_pagination range: (start, end): ({}, {})",
        offset,
        (offset + PAGINATION_RANGE_MAX - 1)
    );
    Ok((offset, offset + PAGINATION_RANGE_MAX - 1))
}

fn extract_query_value(key: &str, req: &mut Request) -> Option<String> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref map) => {
            for (k, v) in map.iter() {
                if key == *k {
                    if v.len() < 1 {
                        return None;
                    }
                    return Some(v[0].clone());
                }
            }
            None
        }
        Err(_) => None,
    }
}

fn do_cache_response(response: &mut Response) {
    response.headers.set(CacheControl(
        format!("public, max-age={}", ONE_YEAR_IN_SECS),
    ));
}

fn dont_cache_response(response: &mut Response) {
    response.headers.set(CacheControl(
        format!("private, no-cache, no-store"),
    ));
}

pub fn routes<M>(insecure: bool, basic: M, worker: M) -> Router
where
    M: BeforeMiddleware + Clone,
{
    router!(
        channels: get "/channels/:origin" => list_channels,
        channel_packages: get "/channels/:origin/:channel/pkgs" => list_packages,
        channel_packages_pkg: get "/channels/:origin/:channel/pkgs/:pkg" => list_packages,
        channel_package_latest: get "/channels/:origin/:channel/pkgs/:pkg/latest" => show_package,
        channel_packages_version: get
            "/channels/:origin/:channel/pkgs/:pkg/:version" => list_packages,
        channel_packages_version_latest: get
            "/channels/:origin/:channel/pkgs/:pkg/:version/latest" => show_package,
        channel_package_release: get
            "/channels/:origin/:channel/pkgs/:pkg/:version/:release" => show_package,
        channel_package_promote: put
            "/channels/:origin/:channel/pkgs/:pkg/:version/:release/promote" => {
            XHandler::new(promote_package).before(basic.clone())
        },
        channel_package_demote: put
            "/channels/:origin/:channel/pkgs/:pkg/:version/:release/demote" => {
            XHandler::new(demote_package).before(basic.clone())
        },
        channel_create: post "/channels/:origin/:channel" => {
            XHandler::new(create_channel).before(basic.clone())
        },
        channel_delete: delete "/channels/:origin/:channel" => {
            XHandler::new(delete_channel).before(basic.clone())
        },
        package_search: get "/pkgs/search/:query" => search_packages,
        packages: get "/pkgs/:origin" => list_packages,
        packages_unique: get "/:origin/pkgs" => list_unique_packages,
        packages_pkg: get "/pkgs/:origin/:pkg" => list_packages,
        package_pkg_versions: get "/pkgs/:origin/:pkg/versions" => list_package_versions,
        package_pkg_latest: get "/pkgs/:origin/:pkg/latest" => show_package,
        packages_version: get "/pkgs/:origin/:pkg/:version" => list_packages,
        package_version_latest: get "/pkgs/:origin/:pkg/:version/latest" => show_package,
        package: get "/pkgs/:origin/:pkg/:version/:release" => show_package,
        package_channels: get "/pkgs/:origin/:pkg/:version/:release/channels" => package_channels,
        package_download: get "/pkgs/:origin/:pkg/:version/:release/download" => download_package,
        package_upload: post "/pkgs/:origin/:pkg/:version/:release" => {
            if insecure {
                XHandler::new(upload_package)
            } else {
                XHandler::new(upload_package).before(basic.clone())
            }
        },
        packages_stats: get "/pkgs/origins/:origin/stats" => package_stats,
        schedule: post "/pkgs/schedule/:origin/:pkg" => {
            if insecure {
                XHandler::new(schedule)
            } else {
                XHandler::new(schedule).before(basic.clone())
            }
        },
        schedule_get: get "/pkgs/schedule/:groupid" => get_schedule,

        origin_create: post "/origins" => {
            XHandler::new(origin_create).before(basic.clone())
        },
        origin_update: put "/origins/:name" => {
            XHandler::new(origin_update).before(basic.clone())
        },
        origin: get "/origins/:origin" => origin_show,

        origin_keys: get "/origins/:origin/keys" => list_origin_keys,
        origin_key_latest: get "/origins/:origin/keys/latest" => download_latest_origin_key,
        origin_key: get "/origins/:origin/keys/:revision" => download_origin_key,
        origin_key_create: post "/origins/:origin/keys/:revision" => {
            if insecure {
                XHandler::new(upload_origin_key)
            } else {
                XHandler::new(upload_origin_key).before(basic.clone())
            }
        },
        origin_secret_key_create: post "/origins/:origin/secret_keys/:revision" => {
            XHandler::new(upload_origin_secret_key).before(basic.clone())
        },
        origin_secret_key_latest: get "/origins/:origin/secret_keys/latest" => {
            if insecure {
                XHandler::new(download_latest_origin_secret_key)
            } else {
                XHandler::new(download_latest_origin_secret_key).before(worker.clone())
            }
        },

        builder_key_latest: get "/builder/keys/latest" => download_latest_builder_key,

        origin_integration_get_names: get "/origins/:origin/integrations/:integration/names" => {
            XHandler::new(handlers::integrations::fetch_origin_integration_names).before(basic.clone())
        },
        origin_integration_put: put "/origins/:origin/integrations/:integration/:name" => {
            XHandler::new(handlers::integrations::create_origin_integration).before(basic.clone())
        },
        origin_integration_delete: delete "/origins/:origin/integrations/:integration/:name" => {
            XHandler::new(handlers::integrations::delete_origin_integration).before(basic.clone())
        },

        // All of this API feels wrong to me.
        origin_invitation_create: post "/origins/:origin/users/:username/invitations" => {
            XHandler::new(invite_to_origin).before(basic.clone())
        },
        origin_invitation_accept: put "/origins/:origin/invitations/:invitation_id" => {
            XHandler::new(accept_invitation).before(basic.clone())
        },
        //origin_invitation_ignore: delete "/origins/:origin/invitations/:invitation_id" => {
        //    XHandler::new(ignore_invitation).before(basic.clone())
        //},
        origin_invitations: get "/origins/:origin/invitations" => {
            XHandler::new(list_origin_invitations).before(basic.clone())
        },
        origin_users: get "/origins/:origin/users" => {
            XHandler::new(list_origin_members).before(basic.clone())
        }
    )
}

pub fn router(depot: DepotUtil) -> Result<Chain> {
    let basic = Authenticated::new(&depot.config);
    let worker = Authenticated::new(&depot.config).require(privilege::BUILD_WORKER);
    let router = routes(depot.config.insecure, basic, worker);
    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<EventLog>::both(EventLogger::new(
        &depot.config.log_dir,
        depot.config.events_enabled,
    )));
    chain.link(persistent::State::<DepotUtil>::both(depot));

    chain.link_after(Cors);
    Ok(chain)
}

pub fn run(config: Config) -> Result<()> {
    let depot = DepotUtil::new(config.clone());
    let v1 = router(depot)?;
    let broker = Broker::run(DepotUtil::net_ident(), &config.route_addrs().clone());

    let mut mount = Mount::new();
    mount.mount("/v1", v1);
    Iron::new(mount).http(&config.http).expect(
        "Unable to start HTTP listener",
    );
    broker.join().unwrap();
    Ok(())
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        IronError {
            error: Box::new(err),
            response: Response::with((status::InternalServerError, "Internal Habitat error")),
        }
    }
}

#[cfg(test)]
mod test {
    use iron::{self, method, Handler, Headers, Request, status, Url};
    use iron::middleware::BeforeMiddleware;
    use iron::prelude::*;
    use iron_test::mock_stream::MockStream;
    use iron_test::response;
    use hyper;
    use hyper::net::NetworkStream;
    use hyper::buffer::BufReader;

    use protocol::sessionsrv::Session;

    use std::env;
    use std::io::Cursor;
    use std::path::PathBuf;

    use super::*;
    use super::super::DepotUtil;

    #[derive(Clone)]
    pub struct AuthenticatedTest;

    impl BeforeMiddleware for AuthenticatedTest {
        fn before(&self, _: &mut Request) -> IronResult<()> {
            Ok(())
        }
    }

    fn iron_request(
        method: method::Method,
        path: &str,
        body: &mut Vec<u8>,
        headers: Headers,
        broker: TestableBroker,
    ) -> (IronResult<Response>, RoutedMessages) {
        let url = Url::parse(path).unwrap();
        let mut buffer = String::new();
        buffer.push_str(&format!("{} {} HTTP/1.1\r\n", &method, url));
        buffer.push_str(&format!("Content-Length: {}\r\n", body.len() as u64));
        for header in headers.iter() {
            buffer.push_str(&format!("{}: {}\r\n", header.name(), header.value_string()));
        }
        buffer.push_str("\r\n");
        let mut bytes = buffer.as_bytes().to_vec();
        bytes.append(body);
        let mut stream = MockStream::new(Cursor::new(bytes));
        let mut buf_reader = BufReader::new(&mut stream as &mut NetworkStream);

        let addr = "127.0.0.1:3000".parse().unwrap();
        let http_request = hyper::server::Request::new(&mut buf_reader, addr).unwrap();
        let mut req = Request::from_http(http_request, addr, &iron::Protocol::http()).unwrap();

        let mut config = Config::default();
        config.path = PathBuf::from(
            env::temp_dir()
                .join("depot-tests")
                .to_string_lossy()
                .into_owned(),
        );
        let depot = DepotUtil::new(config);
        req.extensions.insert::<Authenticated>(Session::new());
        req.extensions.insert::<TestableBroker>(broker);

        let basic = AuthenticatedTest;
        let worker = AuthenticatedTest;
        let router = routes(true, basic, worker);
        let mut chain = Chain::new(router);
        chain.link(persistent::State::<DepotUtil>::both(depot));
        chain.link(persistent::Read::<EventLog>::both(
            EventLogger::new("", false),
        ));
        let resp = chain.handle(&mut req);
        let req_broker = req.extensions.get::<TestableBroker>().unwrap();
        let msgs = req_broker.routed_messages();
        (resp, msgs)
    }

    #[test]
    fn list_unique_packages() {
        let mut broker: TestableBroker = Default::default();

        let mut pkg_res = OriginPackageUniqueListResponse::new();
        pkg_res.set_start(0);
        pkg_res.set_stop(1);
        pkg_res.set_count(2);
        let mut idents = protobuf::RepeatedField::new();

        let mut ident1 = OriginPackageIdent::new();
        ident1.set_origin("org".to_string());
        ident1.set_name("name1".to_string());
        idents.push(ident1);

        let mut ident2 = OriginPackageIdent::new();
        ident2.set_origin("org".to_string());
        ident2.set_name("name2".to_string());
        idents.push(ident2);

        pkg_res.set_idents(idents);
        broker.setup::<OriginPackageUniqueListRequest, OriginPackageUniqueListResponse>(&pkg_res);

        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/org/pkgs?range=2",
            &mut Vec::new(),
            Headers::new(),
            broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);

        assert_eq!(
            result_body,
            "{\
            \"range_start\":0,\
            \"range_end\":1,\
            \"total_count\":2,\
            \"package_list\":[\
                {\
                    \"origin\":\"org\",\
                    \"name\":\"name1\"\
                },\
                {\
                    \"origin\":\"org\",\
                    \"name\":\"name2\"\
                }\
            ]\
        }"
        );

        //assert we sent the corect range to postgres
        let package_req = msgs.get::<OriginPackageUniqueListRequest>().unwrap();
        assert_eq!(package_req.get_start(), 2);
        assert_eq!(package_req.get_stop(), 51);
    }

    #[test]
    fn list_packages() {
        let mut broker: TestableBroker = Default::default();

        let mut pkg_res = OriginPackageListResponse::new();
        pkg_res.set_start(0);
        pkg_res.set_stop(1);
        pkg_res.set_count(2);
        let mut packages = protobuf::RepeatedField::new();

        let mut ident1 = OriginPackageIdent::new();
        ident1.set_origin("org".to_string());
        ident1.set_name("name1".to_string());
        ident1.set_version("1.1.1".to_string());
        ident1.set_release("20170101010101".to_string());
        packages.push(ident1);

        let mut ident2 = OriginPackageIdent::new();
        ident2.set_origin("org".to_string());
        ident2.set_name("name2".to_string());
        ident2.set_version("2.2.2".to_string());
        ident2.set_release("20170202020202".to_string());
        packages.push(ident2);

        pkg_res.set_idents(packages);
        broker.setup::<OriginPackageListRequest, OriginPackageListResponse>(&pkg_res);

        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/pkgs/org?range=2",
            &mut Vec::new(),
            Headers::new(),
            broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);
        let json_body: serde_json::Value = serde_json::from_str(&result_body).unwrap();
        let expected_json = json!({
            "range_start":0,
            "range_end":1,
            "total_count":2,
            "package_list":[
                {
                    "origin":"org",
                    "name":"name1",
                    "version":"1.1.1",
                    "release":"20170101010101",
                },
                {
                    "origin":"org",
                    "name":"name2",
                    "version":"2.2.2",
                    "release":"20170202020202",
                }
            ]
        });

        assert_eq!(json_body, expected_json);

        //assert we sent the corect range to postgres
        let package_req = msgs.get::<OriginPackageListRequest>().unwrap();
        assert_eq!(package_req.get_start(), 2);
        assert_eq!(package_req.get_stop(), 51);
        assert_eq!(package_req.get_ident().to_string(), "org/".to_string());
    }

    #[test]
    fn list_channel_packages() {
        let mut broker: TestableBroker = Default::default();

        let mut pkg_res = OriginPackageListResponse::new();
        pkg_res.set_start(0);
        pkg_res.set_stop(1);
        pkg_res.set_count(2);
        let mut packages = protobuf::RepeatedField::new();

        let mut ident1 = OriginPackageIdent::new();
        ident1.set_origin("org".to_string());
        ident1.set_name("name1".to_string());
        ident1.set_version("1.1.1".to_string());
        ident1.set_release("20170101010101".to_string());
        packages.push(ident1);

        let mut ident2 = OriginPackageIdent::new();
        ident2.set_origin("org".to_string());
        ident2.set_name("name2".to_string());
        ident2.set_version("2.2.2".to_string());
        ident2.set_release("20170202020202".to_string());
        packages.push(ident2);

        pkg_res.set_idents(packages);
        broker.setup::<OriginChannelPackageListRequest, OriginPackageListResponse>(&pkg_res);

        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/channels/org/channel/pkgs?range=2",
            &mut Vec::new(),
            Headers::new(),
            broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);
        let json_body: serde_json::Value = serde_json::from_str(&result_body).unwrap();
        let expected_json = json!({
            "range_start":0,
            "range_end":1,
            "total_count":2,
            "package_list":[
                {
                    "origin":"org",
                    "name":"name1",
                    "version":"1.1.1",
                    "release":"20170101010101",
                },
                {
                    "origin":"org",
                    "name":"name2",
                    "version":"2.2.2",
                    "release":"20170202020202",
                }
            ]
        });

        assert_eq!(json_body, expected_json);

        //assert we sent the corect range to postgres
        let package_req = msgs.get::<OriginChannelPackageListRequest>().unwrap();
        assert_eq!(package_req.get_start(), 2);
        assert_eq!(package_req.get_stop(), 51);
        assert_eq!(package_req.get_ident().to_string(), "org/".to_string());
        assert_eq!(package_req.get_name().to_string(), "channel".to_string());
    }

    #[test]
    fn show_package_fully_qualified() {
        let mut show_broker: TestableBroker = Default::default();

        //setup our package db request
        let mut ident = OriginPackageIdent::new();
        ident.set_origin("org".to_string());
        ident.set_name("name".to_string());
        ident.set_version("1.1.1".to_string());
        ident.set_release("20170101010101".to_string());

        let mut dep_idents = protobuf::RepeatedField::new();
        let mut dep_ident = OriginPackageIdent::new();
        dep_ident.set_origin("dep_org".to_string());
        dep_ident.set_name("dep_name".to_string());
        dep_ident.set_version("1.1.1-dep".to_string());
        dep_ident.set_release("20170101010102".to_string());
        dep_idents.push(dep_ident);

        let mut tdep_idents = protobuf::RepeatedField::new();
        let mut tdep_ident = OriginPackageIdent::new();
        tdep_ident.set_origin("tdep_org".to_string());
        tdep_ident.set_name("tdep_name".to_string());
        tdep_ident.set_version("1.1.1-tdep".to_string());
        tdep_ident.set_release("20170101010103".to_string());
        tdep_idents.push(tdep_ident);

        let mut package = OriginPackage::new();
        package.set_ident(ident.clone());
        package.set_checksum("checksum".to_string());
        package.set_manifest("manifest".to_string());
        package.set_deps(dep_idents);
        package.set_tdeps(tdep_idents);
        package.set_config("config".to_string());
        package.set_target("x86_64-linux".to_string());

        show_broker.setup::<OriginPackageGet, OriginPackage>(&package);

        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/pkgs/org/name/1.1.1/20170101010101",
            &mut Vec::new(),
            Headers::new(),
            show_broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);
        let json_body: serde_json::Value = serde_json::from_str(&result_body).unwrap();
        let expected_json = json!({
            "channels": null,
            "ident":{
                "origin":"org",
                "name":"name",
                "version":"1.1.1",
                "release":"20170101010101"
            },
            "checksum":"checksum",
            "manifest":"manifest",
            "target":"x86_64-linux",
            "deps":[{
                "origin":"dep_org",
                "name":"dep_name",
                "version":"1.1.1-dep",
                "release":"20170101010102"
            }],
            "tdeps":[{
                "origin":"tdep_org",
                "name":"tdep_name",
                "version":"1.1.1-tdep",
                "release":"20170101010103"
            }],
            "exposes":[],
            "config":"config"
        });

        assert_eq!(json_body, expected_json);

        //assert we sent the corect range to postgres
        let package_req = msgs.get::<OriginPackageGet>().unwrap();
        assert_eq!(package_req.get_ident().to_string(), ident.to_string());
    }

    #[test]
    fn show_package_fully_qualified_with_channel() {
        let mut show_broker: TestableBroker = Default::default();

        //setup our package db request
        let mut ident = OriginPackageIdent::new();
        ident.set_origin("org".to_string());
        ident.set_name("name".to_string());
        ident.set_version("1.1.1".to_string());
        ident.set_release("20170101010101".to_string());

        let mut dep_idents = protobuf::RepeatedField::new();
        let mut dep_ident = OriginPackageIdent::new();
        dep_ident.set_origin("dep_org".to_string());
        dep_ident.set_name("dep_name".to_string());
        dep_ident.set_version("1.1.1-dep".to_string());
        dep_ident.set_release("20170101010102".to_string());
        dep_idents.push(dep_ident);

        let mut tdep_idents = protobuf::RepeatedField::new();
        let mut tdep_ident = OriginPackageIdent::new();
        tdep_ident.set_origin("tdep_org".to_string());
        tdep_ident.set_name("tdep_name".to_string());
        tdep_ident.set_version("1.1.1-tdep".to_string());
        tdep_ident.set_release("20170101010103".to_string());
        tdep_idents.push(tdep_ident);

        let mut package = OriginPackage::new();
        package.set_ident(ident.clone());
        package.set_checksum("checksum".to_string());
        package.set_manifest("manifest".to_string());
        package.set_deps(dep_idents);
        package.set_tdeps(tdep_idents);
        package.set_config("config".to_string());
        package.set_target("x86_64-linux".to_string());

        show_broker.setup::<OriginChannelPackageGet, OriginPackage>(&package);

        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/channels/org/channel/pkgs/name/1.1.1/20170101010101",
            &mut Vec::new(),
            Headers::new(),
            show_broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);
        let json_body: serde_json::Value = serde_json::from_str(&result_body).unwrap();
        let expected_json = json!({
            "channels": null,
            "ident":{
                "origin":"org",
                "name":"name",
                "version":"1.1.1",
                "release":"20170101010101"
            },
            "checksum":"checksum",
            "manifest":"manifest",
            "target":"x86_64-linux",
            "deps":[{
                "origin":"dep_org",
                "name":"dep_name",
                "version":"1.1.1-dep",
                "release":"20170101010102"
            }],
            "tdeps":[{
                "origin":"tdep_org",
                "name":"tdep_name",
                "version":"1.1.1-tdep",
                "release":"20170101010103"
            }],
            "exposes":[],
            "config":"config"
        });

        assert_eq!(json_body, expected_json);

        //assert we sent the corect range to postgres
        let package_req = msgs.get::<OriginChannelPackageGet>().unwrap();
        assert_eq!(package_req.get_ident().to_string(), ident.to_string());
        assert_eq!(package_req.get_name().to_string(), "channel".to_string());
    }

    #[test]
    fn show_package_latest() {
        let mut show_broker: TestableBroker = Default::default();

        //setup our full package
        let mut ident = OriginPackageIdent::new();
        ident.set_origin("org".to_string());
        ident.set_name("name".to_string());
        ident.set_version("1.1.1".to_string());
        ident.set_release("20170101010101".to_string());

        let mut dep_idents = protobuf::RepeatedField::new();
        let mut dep_ident = OriginPackageIdent::new();
        dep_ident.set_origin("dep_org".to_string());
        dep_ident.set_name("dep_name".to_string());
        dep_ident.set_version("1.1.1-dep".to_string());
        dep_ident.set_release("20170101010102".to_string());
        dep_idents.push(dep_ident);

        let mut tdep_idents = protobuf::RepeatedField::new();
        let mut tdep_ident = OriginPackageIdent::new();
        tdep_ident.set_origin("tdep_org".to_string());
        tdep_ident.set_name("tdep_name".to_string());
        tdep_ident.set_version("1.1.1-tdep".to_string());
        tdep_ident.set_release("20170101010103".to_string());
        tdep_idents.push(tdep_ident);

        let mut package = OriginPackage::new();
        package.set_ident(ident.clone());
        package.set_checksum("checksum".to_string());
        package.set_manifest("manifest".to_string());
        package.set_deps(dep_idents);
        package.set_tdeps(tdep_idents);
        package.set_config("config".to_string());
        package.set_target("x86_64-linux".to_string());
        show_broker.setup::<OriginPackageGet, OriginPackage>(&package);

        //setup query for the latest ident
        let mut latest_ident = OriginPackageIdent::new();
        latest_ident.set_origin("org".to_string());
        latest_ident.set_name("name".to_string());
        latest_ident.set_version("1.1.1".to_string());
        latest_ident.set_release("20170101010101".to_string());
        show_broker.setup::<OriginPackageLatestGet, OriginPackageIdent>(&latest_ident);

        //set the user agent to look like a linux download
        let mut headers = Headers::new();
        headers.set(UserAgent(
            "hab/0.20.0-dev/20170326090935 (x86_64-linux; 9.9.9)"
                .to_string(),
        ));
        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/pkgs/org/name/latest",
            &mut Vec::new(),
            headers,
            show_broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);
        let json_body: serde_json::Value = serde_json::from_str(&result_body).unwrap();
        let expected_json = json!({
            "ident":{
                "origin":"org",
                "name":"name",
                "version":"1.1.1",
                "release":"20170101010101"
            },
            "channels":null,
            "checksum":"checksum",
            "manifest":"manifest",
            "target":"x86_64-linux",
            "deps":[{
                "origin":"dep_org",
                "name":"dep_name",
                "version":"1.1.1-dep",
                "release":"20170101010102"
            }],
            "tdeps":[{
                "origin":"tdep_org",
                "name":"tdep_name",
                "version":"1.1.1-tdep",
                "release":"20170101010103"
            }],
            "exposes":[],
            "config":"config"
        });

        assert_eq!(json_body, expected_json);

        //assert we sent the corect requests to postgres
        let latest_req = msgs.get::<OriginPackageLatestGet>().unwrap();
        assert_eq!(latest_req.get_ident().to_string(), "org/name".to_string());
        assert_eq!(
            latest_req.get_target().to_string(),
            "x86_64-linux".to_string()
        );
        let package_req = msgs.get::<OriginPackageGet>().unwrap();
        assert_eq!(package_req.get_ident().to_string(), ident.to_string());
    }

    #[test]
    fn show_package_latest_with_channel() {
        let mut show_broker: TestableBroker = Default::default();

        //setup our full package
        let mut ident = OriginPackageIdent::new();
        ident.set_origin("org".to_string());
        ident.set_name("name".to_string());
        ident.set_version("1.1.1".to_string());
        ident.set_release("20170101010101".to_string());

        let mut dep_idents = protobuf::RepeatedField::new();
        let mut dep_ident = OriginPackageIdent::new();
        dep_ident.set_origin("dep_org".to_string());
        dep_ident.set_name("dep_name".to_string());
        dep_ident.set_version("1.1.1-dep".to_string());
        dep_ident.set_release("20170101010102".to_string());
        dep_idents.push(dep_ident);

        let mut tdep_idents = protobuf::RepeatedField::new();
        let mut tdep_ident = OriginPackageIdent::new();
        tdep_ident.set_origin("tdep_org".to_string());
        tdep_ident.set_name("tdep_name".to_string());
        tdep_ident.set_version("1.1.1-tdep".to_string());
        tdep_ident.set_release("20170101010103".to_string());
        tdep_idents.push(tdep_ident);

        let mut package = OriginPackage::new();
        package.set_ident(ident.clone());
        package.set_checksum("checksum".to_string());
        package.set_manifest("manifest".to_string());
        package.set_deps(dep_idents);
        package.set_tdeps(tdep_idents);
        package.set_config("config".to_string());
        package.set_target("x86_64-linux".to_string());
        show_broker.setup::<OriginChannelPackageGet, OriginPackage>(&package);

        //setup query for the latest ident
        let mut latest_ident = OriginPackageIdent::new();
        latest_ident.set_origin("org".to_string());
        latest_ident.set_name("name".to_string());
        latest_ident.set_version("1.1.1".to_string());
        latest_ident.set_release("20170101010101".to_string());
        show_broker.setup::<OriginChannelPackageLatestGet, OriginPackageIdent>(&latest_ident);

        //set the user agent to look like a linux download
        let mut headers = Headers::new();
        headers.set(UserAgent(
            "hab/0.20.0-dev/20170326090935 (x86_64-linux; 9.9.9)"
                .to_string(),
        ));
        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/channels/org/channel/pkgs/name/latest",
            &mut Vec::new(),
            headers,
            show_broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);
        let json_body: serde_json::Value = serde_json::from_str(&result_body).unwrap();
        let expected_json = json!({
            "ident":{
                "origin":"org",
                "name":"name",
                "version":"1.1.1",
                "release":"20170101010101"
            },
            "channels":null,
            "checksum":"checksum",
            "manifest":"manifest",
            "target":"x86_64-linux",
            "deps":[{
                "origin":"dep_org",
                "name":"dep_name",
                "version":"1.1.1-dep",
                "release":"20170101010102"
            }],
            "tdeps":[{
                "origin":"tdep_org",
                "name":"tdep_name",
                "version":"1.1.1-tdep",
                "release":"20170101010103"
            }],
            "exposes":[],
            "config":"config"
        });

        assert_eq!(json_body, expected_json);

        //assert we sent the corect requests to postgres
        let latest_req = msgs.get::<OriginChannelPackageLatestGet>().unwrap();
        assert_eq!(latest_req.get_ident().to_string(), "org/name".to_string());
        assert_eq!(
            latest_req.get_target().to_string(),
            "x86_64-linux".to_string()
        );
        assert_eq!(latest_req.get_name().to_string(), "channel".to_string());
        let package_req = msgs.get::<OriginChannelPackageGet>().unwrap();
        assert_eq!(package_req.get_ident().to_string(), ident.to_string());
        assert_eq!(package_req.get_name().to_string(), "channel".to_string());
    }

    #[test]
    fn search_packages() {
        let mut broker: TestableBroker = Default::default();

        let mut pkg_res = OriginPackageListResponse::new();
        pkg_res.set_start(0);
        pkg_res.set_stop(1);
        pkg_res.set_count(2);
        let mut packages = protobuf::RepeatedField::new();

        let mut ident1 = OriginPackageIdent::new();
        ident1.set_origin("org".to_string());
        ident1.set_name("name1".to_string());
        ident1.set_version("1.1.1".to_string());
        ident1.set_release("20170101010101".to_string());
        packages.push(ident1);

        let mut ident2 = OriginPackageIdent::new();
        ident2.set_origin("org".to_string());
        ident2.set_name("name2".to_string());
        ident2.set_version("2.2.2".to_string());
        ident2.set_release("20170202020202".to_string());
        packages.push(ident2);

        pkg_res.set_idents(packages);
        broker.setup::<OriginPackageSearchRequest, OriginPackageListResponse>(&pkg_res);

        let (response, msgs) = iron_request(
            method::Get,
            "http://localhost/pkgs/search/org%2Fname?range=2",
            &mut Vec::new(),
            Headers::new(),
            broker,
        );

        let response = response.unwrap();
        assert_eq!(response.status, Some(status::Ok));

        let result_body = response::extract_body_to_string(response);

        assert_eq!(
            result_body,
            "{\
            \"range_start\":0,\
            \"range_end\":1,\
            \"total_count\":2,\
            \"package_list\":[\
                {\
                    \"origin\":\"org\",\
                    \"name\":\"name1\",\
                    \"version\":\"1.1.1\",\
                    \"release\":\"20170101010101\"\
                },\
                {\
                    \"origin\":\"org\",\
                    \"name\":\"name2\",\
                    \"version\":\"2.2.2\",\
                    \"release\":\"20170202020202\"\
                }\
            ]\
        }"
        );

        //assert we sent the corect range to postgres
        let package_req = msgs.get::<OriginPackageSearchRequest>().unwrap();
        assert_eq!(package_req.get_start(), 2);
        assert_eq!(package_req.get_stop(), 51);
        assert_eq!(package_req.get_origin(), "org".to_string());
        assert_eq!(package_req.get_query(), "name".to_string());
    }
}
