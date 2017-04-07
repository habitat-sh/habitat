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

use uuid::Uuid;
use bodyparser;
use dbcache::{self, BasicSet};
use hab_core::package::{Identifiable, FromArchive, PackageArchive, PackageTarget};
use hab_core::crypto::keys::{self, PairType};
use hab_core::crypto::SigKeyPair;
use hab_core::event::*;
use bld_core::metrics::*;
use hab_net::config::RouteAddrs;
use hab_net::http::controller::*;
use hab_net::privilege;
use hab_net::routing::{Broker, RouteResult};
use hab_net::server::NetIdent;
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
use protocol::{depotsrv, Routable};
use protocol::net::{ErrCode, NetError};
use protocol::sessionsrv::{Account, AccountGet};
use protocol::scheduler::{Group, GroupCreate, GroupGet};
use protocol::originsrv::*;
use regex::Regex;
use router::{Params, Router};
use serde::Serialize;
use serde_json;
use url;
use urlencoded::UrlEncodedQuery;

use super::Depot;
use config::Config;
use error::{Error, Result};

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
                Ok(parse_from_bytes::<M>(msg).expect(&format!("Unable to parse {:?} message",
                                                              msg_type)))
            }
            None => Err(Error::MessageTypeNotFound),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct OriginCreateReq {
    name: String,
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

fn route_message<M: Routable, R: protobuf::MessageStatic>(req: &mut Request,
                                                          msg: &M)
                                                          -> RouteResult<R> {
    if let Some(broker) = req.extensions.get_mut::<TestableBroker>() {
        return broker.route::<M, R>(msg);
    }

    Broker::connect().unwrap().route::<M, R>(msg)
}

fn package_results_json<T: Serialize>(packages: &Vec<T>,
                                      count: isize,
                                      start: isize,
                                      end: isize)
                                      -> String {
    let results = PackageResults {
        range_start: start,
        range_end: end,
        total_count: count,
        package_list: packages,
    };

    serde_json::to_string(&results).unwrap()
}

pub fn origin_create(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginCreate::new();
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        request.set_owner_id(session.get_id());
        request.set_owner_name(session.get_name().to_string());
    }
    match req.get::<bodyparser::Struct<OriginCreateReq>>() {
        Ok(Some(body)) => request.set_name(body.name),
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    };

    if !keys::is_valid_origin_name(request.get_name()) {
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
        _ => return Ok(Response::with(status::BadRequest)),
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

pub fn get_origin<T: ToString>(req: &mut Request, origin: T) -> IronResult<Option<Origin>> {
    let mut request = OriginGet::new();
    request.set_name(origin.to_string());
    match route_message::<OriginGet, Origin>(req, &request) {
        Ok(origin) => Ok(Some(origin)),
        Err(err) => {
            if err.get_code() == ErrCode::ENTITY_NOT_FOUND {
                Ok(None)
            } else {
                let body = serde_json::to_string(&err).unwrap();
                let status = net_err_to_http(err.get_code());
                Err(IronError::new(err, (body, status)))
            }
        }
    }
}

pub fn check_origin_access<T: ToString>(req: &mut Request,
                                        account_id: u64,
                                        origin: T)
                                        -> IronResult<bool> {
    let mut request = CheckOriginAccessRequest::new();
    request.set_account_id(account_id);
    request.set_origin_name(origin.to_string());
    match route_message::<CheckOriginAccessRequest, CheckOriginAccessResponse>(req, &request) {
        Ok(response) => Ok(response.get_has_access()),
        Err(err) => {
            let body = serde_json::to_string(&err).unwrap();
            let status = net_err_to_http(err.get_code());
            Err(IronError::new(err, (body, status)))
        }
    }
}

pub fn invite_to_origin(req: &mut Request) -> IronResult<Response> {
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions
        .get::<Authenticated>()
        .unwrap()
        .clone();
    let params = req.extensions
        .get::<Router>()
        .unwrap()
        .clone();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let user_to_invite = match params.find("username") {
        Some(username) => username,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = Broker::connect().unwrap();
    debug!("Creating invitation for user {} origin {}",
           &user_to_invite,
           &origin);
    if !try!(check_origin_access(req, session.get_id(), &origin)) {
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
    match try!(get_origin(req, &origin)) {
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
            log_event!(req,
                       Event::OriginInvitationSend {
                           origin: origin.to_string(),
                           user: user_to_invite.to_string(),
                           id: invitation.get_id().to_string(),
                           account: session.get_id().to_string(),
                       });
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
    if !try!(check_origin_access(req, session_id, &origin_name)) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginInvitationListRequest::new();
    match try!(get_origin(req, origin_name.as_str())) {
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

    if !try!(check_origin_access(req, session_id, &origin_name)) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginMemberListRequest::new();
    match try!(get_origin(req, origin_name.as_str())) {
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
    let file = try!(File::create(&filename));
    let mut writer = BufWriter::new(file);
    let mut written: i64 = 0;
    let mut buf = [0u8; 100000]; // Our byte buffer
    loop {
        let len = try!(body.read(&mut buf)); // Raise IO errors
        match len {
            0 => {
                // 0 == EOF, so stop writing and finish progress
                break;
            }
            _ => {
                // Write the buffer to the BufWriter on the Heap
                let bytes_written = try!(writer.write(&buf[0..len]));
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
    let session = req.extensions
        .get::<Authenticated>()
        .unwrap()
        .clone();
    let params = req.extensions
        .get::<Router>()
        .unwrap()
        .clone();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginPublicKeyCreate::new();
    request.set_owner_id(session.get_id());

    let origin = match params.find("origin") {
        Some(origin) => {
            if !try!(check_origin_access(req, session.get_id(), origin)) {
                return Ok(Response::with(status::Forbidden));
            }
            match get_origin(req, origin)? {
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
            log_event!(req,
                       Event::OriginKeyUpload {
                           origin: origin.to_string(),
                           version: request.get_revision().to_string(),
                           account: session.get_id().to_string(),
                       });
            let mut response =
                Response::with((status::Created,
                                format!("/origins/{}/keys/{}", &origin, &request.get_revision())));
            let mut base_url: url::Url = req.url.clone().into();
            base_url.set_path(&format!("key/{}-{}", &origin, &request.get_revision()));
            response.headers.set(headers::Location(format!("{}", base_url)));
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
    match try!(get_origin(req, origin)) {
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
    let session = req.extensions
        .get::<Authenticated>()
        .unwrap()
        .clone();
    let params = req.extensions
        .get::<Router>()
        .unwrap()
        .clone();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginSecretKeyCreate::new();
    request.set_owner_id(session.get_id());

    let origin = match params.find("origin") {
        Some(origin) => {
            if !try!(check_origin_access(req, session.get_id(), origin)) {
                return Ok(Response::with(status::Forbidden));
            }
            match try!(get_origin(req, origin)) {
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
            log_event!(req,
                       Event::OriginSecretKeyUpload {
                           origin: origin.to_string(),
                           version: request.take_revision(),
                           account: session.get_id().to_string(),
                       });
            Ok(Response::with(status::Created))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn upload_package(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
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

    debug!("UPLOADING checksum={}, ident={}",
           checksum_from_param,
           ident);

    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions
        .get::<Authenticated>()
        .unwrap()
        .clone();
    if !depot.config.insecure {
        if !try!(check_origin_access(req, session.get_id(), &ident.get_origin())) {
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

    let mut archive = try!(write_archive(&temp_path, &mut req.body));
    debug!("Package Archive: {:#?}", archive);

    let target_from_artifact = match archive.target() {
        Ok(target) => target,
        Err(e) => {
            info!("Could not read the target for {:#?}: {:#?}", archive, e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };

    if !depot.config.supported_targets.contains(&target_from_artifact) {
        debug!("Unsupported package platform or architecture {}.",
               target_from_artifact);
        return Ok(Response::with(status::NotImplemented));
    };

    match depot.datastore.packages.find(&ident) {
        Ok(_) |
        Err(dbcache::Error::EntityNotFound) => {
            if depot.archive(&ident, &target_from_artifact).is_some() {
                return Ok(Response::with((status::Conflict)));
            }
        }
        Err(e) => {
            error!("upload_package:1, err={:?}", e);
            return Ok(Response::with(status::InternalServerError));
        }
    }

    let checksum_from_artifact = match archive.checksum() {
        Ok(cksum) => cksum,
        Err(e) => {
            info!("Could not compute a checksum for {:#?}: {:#?}", archive, e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };
    if checksum_from_param != checksum_from_artifact {
        info!("Checksums did not match: from_param={:?}, from_artifact={:?}",
              checksum_from_param,
              checksum_from_artifact);
        return Ok(Response::with(status::UnprocessableEntity));
    }

    let filename = depot.archive_path(&ident, &target_from_artifact);

    match fs::rename(&temp_path, &filename) {
        Ok(_) => {}
        Err(e) => {
            error!("Unable to rename temp archive {:?} to {:?}, err={:?}",
                   temp_path,
                   filename,
                   e);
            return Ok(Response::with(status::InternalServerError));
        }
    }

    info!("File added to Depot at {}", filename.to_string_lossy());
    let mut archive = PackageArchive::new(filename);
    let object = match depotsrv::Package::from_archive(&mut archive) {
        Ok(object) => object,
        Err(e) => {
            info!("Error building package from archive: {:#?}", e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };
    if ident.satisfies(object.get_ident()) {
        depot.datastore
            .packages
            .write(&object)
            .unwrap();

        log_event!(req,
                   Event::PackageUpload {
                       origin: ident.get_origin().to_string(),
                       package: ident.get_name().to_string(),
                       version: ident.get_version().to_string(),
                       release: ident.get_release().to_string(),
                       target: target_from_artifact.to_string(),
                       account: session.get_id().to_string(),
                   });

        // Schedule re-build of dependent packages (if requested)
        // Don't schedule builds if the upload is being done by the builder
        // Currently, we only do dep builds of 'core' packages
        if depot.config.builds_enabled && ident.get_origin() == "core" &&
           !match extract_query_value("builder", req) {
                Some(_) => true,
                None => false,
            } {
            let mut conn = Broker::connect().unwrap();

            let mut request = GroupCreate::new();
            request.set_origin(ident.get_origin().to_string());
            request.set_package(ident.get_name().to_string());
            request.set_deps_only(true);

            match conn.route::<GroupCreate, Group>(&request) {
                Ok(group) => {
                    debug!("Scheduled reverse dependecy build, group id: {}",
                           group.get_id())
                }
                Err(err) => error!("Unable to schedule build, err: {:?}", err),
            }
        }

        let mut response = Response::with((status::Created,
                                           format!("/pkgs/{}/download", object.get_ident())));
        let mut base_url: url::Url = req.url.clone().into();
        base_url.set_path(&format!("pkgs/{}/download", object.get_ident()));
        response.headers.set(headers::Location(format!("{}", base_url)));
        Ok(response)
    } else {
        info!("Ident mismatch, expected={:?}, got={:?}",
              ident,
              object.get_ident());
        Ok(Response::with(status::UnprocessableEntity))
    }
}

fn schedule(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions
        .get::<Router>()
        .unwrap()
        .clone();
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

fn download_origin_key(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions
        .get::<Authenticated>()
        .unwrap()
        .clone();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginPublicKeyGet::new();
    request.set_owner_id(session.get_id());
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
    response.headers.set(ContentDisposition(format!("attachment; filename=\"{}\"", xfilename)));
    response.headers.set(XFileName(xfilename));
    do_cache_response(&mut response);
    Ok(response)
}

fn download_latest_origin_key(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    // TODO: SA - Eliminate need to clone the session and params
    let session = req.extensions
        .get::<Authenticated>()
        .unwrap()
        .clone();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginPublicKeyLatestGet::new();
    request.set_owner_id(session.get_id());
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
    response.headers.set(ContentDisposition(format!("attachment; filename=\"{}\"", xfilename)));
    response.headers.set(XFileName(xfilename));
    dont_cache_response(&mut response);
    Ok(response)
}



fn download_package(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let depot = lock.read().expect("depot read lock is poisoned");
    let params = req.extensions.get::<Router>().unwrap();
    let ident = ident_from_params(params);
    let agent_target = target_from_headers(&req.headers.get::<UserAgent>().unwrap()).unwrap();
    if !depot.config.supported_targets.contains(&agent_target) {
        error!("Unsupported client platform ({}) for this depot.",
               agent_target);
        return Ok(Response::with(status::NotImplemented));
    }

    match depot.datastore.packages.find(&ident) {
        Ok(ident) => {
            if let Some(archive) = depot.archive(&ident, &agent_target) {
                match fs::metadata(&archive.path) {
                    Ok(_) => {
                        let mut response = Response::with((status::Ok, archive.path.clone()));
                        do_cache_response(&mut response);
                        response.headers
                            .set(ContentDisposition(format!("attachment; filename=\"{}\"",
                                                            archive.file_name())));
                        response.headers.set(XFileName(archive.file_name()));
                        Ok(response)
                    }
                    Err(_) => Ok(Response::with(status::NotFound)),
                }
            } else {
                // This should never happen. Writing the package to disk and recording it's
                // existence in the metadata is a transactional operation and one cannot exist
                // without the other.
                panic!("Inconsistent package metadata! Exit and run `hab-depot repair` to fix \
                        data integrity.");
            }
        }
        Err(dbcache::Error::EntityNotFound) => Ok(Response::with((status::NotFound))),
        Err(e) => {
            error!("download_package:1, err={:?}", e);
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn list_origin_keys(req: &mut Request) -> IronResult<Response> {
    let session_id: u64;
    let origin_name: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();

        let params = req.extensions.get::<Router>().unwrap();
        origin_name = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    if !try!(check_origin_access(req, session_id, &origin_name)) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginPublicKeyListRequest::new();
    match try!(get_origin(req, origin_name.as_str())) {
        Some(origin) => request.set_origin_id(origin.get_id()),
        None => return Ok(Response::with(status::NotFound)),
    };
    match route_message::<OriginPublicKeyListRequest, OriginPublicKeyListResponse>(req, &request) {
        Ok(list) => {
            let list: Vec<depotsrv::OriginKeyIdent> = list.get_keys()
                .iter()
                .map(|key| {
                    let mut ident = depotsrv::OriginKeyIdent::new();
                    ident.set_location(format!("/origins/{}/keys/{}",
                                               &key.get_name(),
                                               &key.get_revision()));
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
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let depot = lock.read().expect("depot read lock is poisoned");
    let (start, stop) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let ident: String = match params.find("origin") {
        Some(origin) => origin.to_string(),
        None => return Ok(Response::with(status::BadRequest)),
    };

    match depot.datastore
              .packages
              .index
              .unique(&ident, start, stop) {
        Ok(packages) => {
            let count = depot.datastore
                .packages
                .index
                .count_unique(&ident)
                .unwrap();
            debug!("list_unique_packages start: {}, stop: {}, total count: {}",
                   start,
                   stop,
                   count);
            let body = package_results_json(&packages, count as isize, start, stop);

            let mut response = if count as isize > (stop + 1) {
                Response::with((status::PartialContent, body))
            } else {
                Response::with((status::Ok, body))
            };

            response.headers.set(ContentType(Mime(TopLevel::Application,
                                                  SubLevel::Json,
                                                  vec![(Attr::Charset, Value::Utf8)])));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
            Ok(Response::with((status::NotFound)))
        }
        Err(e) => {
            error!("list_packages:2, err={:?}", e);
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn list_packages(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let mut depot = lock.write().expect("depot read lock is poisoned");
    let (start, stop) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };

    let (origin, ident, channel) = {
        let params = req.extensions.get::<Router>().unwrap();

        let origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let ident: String = if params.find("pkg").is_none() {
            origin.clone()
        } else {
            ident_from_params(params).to_string()
        };

        let channel = match params.find("channel") {
            Some(ch) => Some(ch.to_string()),
            None => None,
        };

        (origin, ident, channel)
    };

    // let's make sure this origin actually exists
    if try!(get_origin(req, &origin)).is_none() {
        return Ok(Response::with(status::NotFound));
    }

    match channel {
        Some(channel) => {
            // let's make sure this channel actually exists
            if !depot.datastore.channels.channel_exists(&origin, &channel) {
                return Ok(Response::with(status::NotFound));
            }

            let packages =
                depot.datastore.channels.all_packages(&origin, &channel, &ident, start, stop);
            let count = packages.len();
            let body = package_results_json(&packages, count as isize, start, stop);

            let mut response = if count as isize > (stop + 1) {
                Response::with((status::PartialContent, body))
            } else {
                Response::with((status::Ok, body))
            };

            response.headers.set(ContentType(Mime(TopLevel::Application,
                                                  SubLevel::Json,
                                                  vec![(Attr::Charset, Value::Utf8)])));

            dont_cache_response(&mut response);
            Ok(response)
        }
        None => {
            match depot.datastore
                      .packages
                      .index
                      .list(&ident, start, stop) {
                Ok(packages) => {
                    let count = depot.datastore
                        .packages
                        .index
                        .count(&ident)
                        .unwrap();
                    debug!("list_packages start: {}, stop: {}, total count: {}",
                           start,
                           stop,
                           count);
                    let body = package_results_json(&packages, count as isize, start, stop);

                    let mut response = if count as isize > (stop + 1) {
                        Response::with((status::PartialContent, body))
                    } else {
                        Response::with((status::Ok, body))
                    };

                    response.headers.set(ContentType(Mime(TopLevel::Application,
                                                          SubLevel::Json,
                                                          vec![(Attr::Charset, Value::Utf8)])));
                    dont_cache_response(&mut response);
                    Ok(response)
                }
                Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                    Ok(Response::with((status::NotFound)))
                }
                Err(e) => {
                    error!("list_packages:2, err={:?}", e);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

fn list_channels(req: &mut Request) -> IronResult<Response> {
    let session_id: u64;
    let origin_name: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();

        let params = req.extensions.get::<Router>().unwrap();
        origin_name = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    };

    if !try!(check_origin_access(req, session_id, &origin_name)) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginChannelListRequest::new();
    match try!(get_origin(req, origin_name.as_str())) {
        Some(origin) => request.set_origin_id(origin.get_id()),
        None => return Ok(Response::with(status::NotFound)),
    };

    match route_message::<OriginChannelListRequest, OriginChannelListResponse>(req, &request) {
        Ok(list) => {
            let list: Vec<depotsrv::OriginChannelIdent> = list.get_channels()
                .iter()
                .map(|channel| {
                    let mut ident = depotsrv::OriginChannelIdent::new();
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

    //let mut response = Response::with((status::Ok));

    //Ok(response)
}

fn create_channel(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let mut depot = lock.write().expect("depot write lock is poisoned");

    let session_id: u64;
    let origin: String;
    let channel: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();
        let params = req.extensions.get::<Router>().unwrap();
        origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };
        channel = match params.find("channel") {
            Some(channel) => channel.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };
    }

    // let's make sure this origin actually exists
    if try!(get_origin(req, &origin)).is_none() {
        return Ok(Response::with(status::NotFound));
    }

    // make sure the person trying to create the channel has access to do so
    if !try!(check_origin_access(req, session_id, &origin)) {
        return Ok(Response::with(status::Forbidden));
    }

    match depot.datastore.channels.create(origin, channel) {
        Ok(_) => Ok(Response::with(status::Created)),
        Err(Error::ChannelAlreadyExists(_)) => Ok(Response::with(status::Conflict)),
        Err(_) => Ok(Response::with(status::InternalServerError)),
    }
}

fn delete_channel(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let mut depot = lock.write().expect("depot write lock is poisoned");

    let session_id: u64;
    let origin: String;
    let channel: String;
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        session_id = session.get_id();
        let params = req.extensions.get::<Router>().unwrap();
        origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };
        channel = match params.find("channel") {
            Some(channel) => channel.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };
    }

    // let's make sure this origin actually exists
    if try!(get_origin(req, &origin)).is_none() {
        return Ok(Response::with(status::NotFound));
    }

    // make sure the person trying to create the channel has access to do so
    if !try!(check_origin_access(req, session_id, &origin)) {
        return Ok(Response::with(status::Forbidden));
    }

    // stable and unstable can't be deleted
    if channel == "stable" || channel == "unstable" {
        return Ok(Response::with(status::Forbidden));
    }

    depot.datastore.channels.remove(&origin, &channel);
    Ok(Response::with(status::Ok))
}

fn show_package(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let mut depot = lock.write().expect("depot read lock is poisoned");

    let (origin, mut ident, channel) = {
        let params = req.extensions.get::<Router>().unwrap();

        let origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let ident = ident_from_params(params);

        let channel = match params.find("channel") {
            Some(ch) => Some(ch.to_owned()),
            None => None,
        };

        (origin, ident, channel)
    };

    // let's make sure this origin actually exists
    if try!(get_origin(req, &origin)).is_none() {
        return Ok(Response::with(status::NotFound));
    }

    if let Some(channel) = channel {
        // let's make sure this channel actually exists
        if !depot.datastore.channels.channel_exists(&origin, &channel) {
            return Ok(Response::with(status::NotFound));
        }

        if !ident.fully_qualified() {
            match depot.datastore.channels.latest(&origin, channel.as_str(), &ident.to_string()) {
                Some(ident) => {
                    match depot.datastore.packages.find(&ident) {
                        Ok(pkg) => render_package(&pkg, false),
                        Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
                        Err(e) => {
                            error!("show_package:1, err={:?}", e);
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
                }
                None => Ok(Response::with(status::NotFound)),
            }
        } else {
            let key = format!("{}/{}", &origin, &channel);
            if depot.datastore.channels.package_exists(&key, &ident) {
                match depot.datastore.packages.find(&ident) {
                    Ok(pkg) => render_package(&pkg, false),
                    Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
                    Err(e) => {
                        error!("show_package:3, err={:?}", e);
                        Ok(Response::with(status::InternalServerError))
                    }
                }
            } else {
                Ok(Response::with(status::NotFound))
            }
        }
    } else {
        if !ident.fully_qualified() {
            let agent_target = target_from_headers(&req.headers.get::<UserAgent>().unwrap())
                .unwrap();
            match depot.datastore
                      .packages
                      .index
                      .latest(&ident, &agent_target.to_string()) {
                Ok(id) => ident = id.into(),
                Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                    return Ok(Response::with(status::NotFound));
                }
                Err(e) => {
                    error!("show_package:5, err={:?}", e);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }

        match depot.datastore.packages.find(&ident) {
            Ok(pkg) => {
                // If the request was for a fully qualified ident, cache the response, otherwise do
                // not cache
                if ident.fully_qualified() {
                    render_package(&pkg, true)
                } else {
                    render_package(&pkg, false)
                }
            }
            Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
            Err(e) => {
                error!("show_package:6, err={:?}", e);
                Ok(Response::with(status::InternalServerError))
            }
        }
    }
}

fn search_packages(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let depot = lock.read().expect("depot read lock is poisoned");
    let (start, stop) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let partial = params.find("query").unwrap();

    debug!("search_packages called with: {}", partial);
    Counter::SearchPackages.increment();
    Gauge::PackageCount.set(depot.datastore.key_count().unwrap() as f64);

    // Note: the search call takes offset and count values
    let (packages, total_count) = depot.datastore
        .packages
        .index
        .search(partial, start, stop - start + 1)
        .unwrap();

    debug!("search_packages offset: {}, count: {}, packages len: {}, total_count: {}",
           start,
           stop - start + 1,
           packages.len(),
           total_count);

    let body = package_results_json(&packages, total_count, start, stop);

    let mut response = if total_count > (stop + 1) {
        Response::with((status::PartialContent, body))
    } else {
        Response::with((status::Ok, body))
    };

    response.headers.set(ContentType(Mime(TopLevel::Application,
                                          SubLevel::Json,
                                          vec![(Attr::Charset, Value::Utf8)])));

    dont_cache_response(&mut response);
    Ok(response)
}

fn render_package(pkg: &depotsrv::Package, should_cache: bool) -> IronResult<Response> {
    let body = serde_json::to_string(&pkg).unwrap();
    let mut response = Response::with((status::Ok, body));
    response.headers.set(ETag(pkg.get_checksum().to_string()));
    response.headers.set(ContentType(Mime(TopLevel::Application,
                                          SubLevel::Json,
                                          vec![(Attr::Charset, Value::Utf8)])));
    if should_cache {
        do_cache_response(&mut response);
    } else {
        dont_cache_response(&mut response);
    }
    Ok(response)
}

fn promote_package(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<Depot>>().expect("depot not found");
    let mut depot = lock.write().expect("depot write lock is poisoned");
    let (channel, origin, mut ident, session_id) = {
        let session = req.extensions.get::<Authenticated>().unwrap();
        let session_id = session.get_id();

        let params = req.extensions.get::<Router>().unwrap();
        let origin = match params.find("origin") {
            Some(o) => o.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let channel = match params.find("channel") {
            Some(c) => c.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let pkg = match params.find("pkg") {
            Some(p) => p.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let version = match params.find("version") {
            Some(v) => v.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let release = match params.find("release") {
            Some(r) => r.to_string(),
            _ => return Ok(Response::with(status::BadRequest)),
        };

        let mut ident = depotsrv::PackageIdent::new();
        ident.set_name(pkg);
        ident.set_version(version);
        ident.set_release(release);

        (channel, origin, ident, session_id)
    };

    if try!(get_origin(req, &origin)).is_none() {
        return Ok(Response::with(status::NotFound));
    }
    ident.set_origin(origin);

    match depot.datastore.channels.channel_exists(&ident.get_origin(), &channel) {
        true => {
            if !try!(check_origin_access(req, session_id, &ident.get_origin())) {
                return Ok(Response::with(status::Forbidden));
            }

            match depot.datastore.packages.find(&ident) {
                Ok(package) => {
                    depot.datastore
                        .channels
                        .associate(&channel, &package)
                        .unwrap();
                    Ok(Response::with(status::Ok))
                }
                Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
                Err(e) => {
                    error!("promote:2, err={:?}", e);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }
        false => Ok(Response::with(status::NotFound)),
    }
}

fn ident_from_params(params: &Params) -> depotsrv::PackageIdent {
    let mut ident = depotsrv::PackageIdent::new();
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

    let user_agent_regex = Regex::new(r"(?P<client>\.*)\s\((?P<target>\w+-\w+); (?P<kernel>.*)\)")
        .unwrap();
    let user_agent_capture =
        user_agent_regex.captures(user_agent).expect("Invalid user agent supplied.");
    match PackageTarget::from_str(&user_agent_capture["target"]) {
        Ok(target) => Ok(target),
        Err(_) => Err(Response::with(status::BadRequest)),
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

    debug!("extract_pagination range: (start, end): ({}, {})",
           offset,
           (offset + PAGINATION_RANGE_MAX - 1));
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
    response.headers.set(CacheControl(format!("public, max-age={}", ONE_YEAR_IN_SECS)));
}

fn dont_cache_response(response: &mut Response) {
    response.headers.set(CacheControl(format!("private, no-cache, no-store")));
}

pub fn routes<M: BeforeMiddleware + Clone>(insecure: bool, basic: M, worker: M) -> Router {
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
        package_pkg_latest: get "/pkgs/:origin/:pkg/latest" => show_package,
        packages_version: get "/pkgs/:origin/:pkg/:version" => list_packages,
        package_version_latest: get "/pkgs/:origin/:pkg/:version/latest" => show_package,
        package: get "/pkgs/:origin/:pkg/:version/:release" => show_package,

        package_download: get "/pkgs/:origin/:pkg/:version/:release/download" => {
            download_package
        },
        package_upload: post "/pkgs/:origin/:pkg/:version/:release" => {
            if insecure {
                XHandler::new(upload_package)
            } else {
                XHandler::new(upload_package).before(basic.clone())
            }
        },
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
        origin: get "/origins/:origin" => origin_show,

        //origin_channels: get "/origins/:origin/channels" => list_channels,

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
        origin_invitation_create: post "/origins/:origin/users/:username/invitations" => {
            XHandler::new(invite_to_origin).before(basic.clone())
        },
        origin_invitations: get "/origins/:origin/invitations" => {
            XHandler::new(list_origin_invitations).before(basic.clone())
        },
        origin_users: get "/origins/:origin/users" => {
            XHandler::new(list_origin_members).before(basic.clone())
        }
    )
}

pub fn router(depot: Depot) -> Result<Chain> {
    let basic = Authenticated::new(&depot.config);
    let worker = Authenticated::new(&depot.config).require(privilege::BUILD_WORKER);
    let router = routes(depot.config.insecure, basic, worker);
    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<EventLog>::both(EventLogger::new(&depot.config.log_dir,
                                                                   depot.config.events_enabled)));
    chain.link(persistent::State::<Depot>::both(depot));
    chain.link_after(Cors);
    Ok(chain)
}

pub fn run(config: Config) -> Result<()> {
    let listen_addr = config.listen_addr.clone();
    let depot = try!(Depot::new(config.clone()));
    let v1 = try!(router(depot));
    let broker = Broker::run(Depot::net_ident(), &config.route_addrs().clone());

    let mut mount = Mount::new();
    mount.mount("/v1", v1);
    Iron::new(mount).http(listen_addr).unwrap();
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

    use iron::{self, method, Handler, Headers, Request, Url};
    use iron::middleware::BeforeMiddleware;
    use iron::prelude::*;
    use iron_test::mock_stream::MockStream;
    use iron_test::response;
    use hyper;
    use hyper::net::NetworkStream;
    use hyper::buffer::BufReader;
    use protocol::sessionsrv::Session;
    use protobuf;

    use std::io::Cursor;

    use super::*;
    use super::super::DepotUtil;

    #[derive(Clone)]
    pub struct AuthenticatedTest;

    impl BeforeMiddleware for AuthenticatedTest {
        fn before(&self, req: &mut Request) -> IronResult<()> {
            Ok(())
        }
    }

    pub fn hart_file(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name)
    }

    fn iron_request(method: method::Method,
                    path: &str,
                    body: &mut Vec<u8>,
                    headers: Headers,
                    broker: TestableBroker)
                    -> (IronResult<Response>, RoutedMessages) {
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

        let depot = DepotUtil::new(Config::default());
        req.extensions.insert::<Authenticated>(Session::new());
        req.extensions.insert::<TestableBroker>(broker);

        let basic = AuthenticatedTest;
        let worker = AuthenticatedTest;
        let router = routes(true, basic, worker);
        let mut chain = Chain::new(router);
        chain.link(persistent::State::<DepotUtil>::both(depot));
        chain.link(persistent::Read::<EventLog>::both(EventLogger::new("", false)));
        let resp = chain.handle(&mut req);
        let req_broker = req.extensions.get::<TestableBroker>().unwrap();
        let msgs = req_broker.routed_messages();
        (resp, msgs)
    }

    #[test]
    fn list_public_keys() {
        let mut broker: TestableBroker = Default::default();

        let mut access_res = CheckOriginAccessResponse::new();
        access_res.set_has_access(true);
        broker.setup::<CheckOriginAccessRequest, CheckOriginAccessResponse>(&access_res);

        let mut origin_res = Origin::new();
        origin_res.set_id(5000);
        broker.setup::<OriginGet, Origin>(&origin_res);

        let mut key_res = OriginPublicKeyListResponse::new();
        let mut keys = protobuf::RepeatedField::new();

        let mut key = OriginPublicKey::new();
        key.set_name("my_name".to_string());
        key.set_revision("my_rev".to_string());
        keys.push(key);

        let mut key2 = OriginPublicKey::new();
        key2.set_name("my_name2".to_string());
        key2.set_revision("my_rev2".to_string());
        keys.push(key2);

        key_res.set_keys(keys);
        broker.setup::<OriginPublicKeyListRequest, OriginPublicKeyListResponse>(&key_res);

        let (response, _) = iron_request(method::Get,
                                         "http://localhost/origins/org/keys",
                                         &mut Vec::new(),
                                         Headers::new(),
                                         broker);
        let result_body = response::extract_body_to_string(response.unwrap());

        assert_eq!(result_body,
                   "[\
            {\
                \"origin\":\"my_name\",\
                \"revision\":\"my_rev\",\
                \"location\":\"/origins/my_name/keys/my_rev\"\
            },\
            {\
                \"origin\":\"my_name2\",\
                \"revision\":\"my_rev2\",\
                \"location\":\"/origins/my_name2/keys/my_rev2\"\
            }\
        ]");
    }

    #[test]
    fn list_channels() {
        let mut broker: TestableBroker = Default::default();

        let mut access_res = CheckOriginAccessResponse::new();
        access_res.set_has_access(true);
        broker.setup::<CheckOriginAccessRequest, CheckOriginAccessResponse>(&access_res);

        let mut origin_res = Origin::new();
        origin_res.set_id(5000);
        broker.setup::<OriginGet, Origin>(&origin_res);

        let mut channel_res = OriginChannelListResponse::new();
        let mut channels = protobuf::RepeatedField::new();

        let mut channel = OriginChannel::new();
        channel.set_name("my_channel".to_string());
        channels.push(channel);

        let mut channel2 = OriginChannel::new();
        channel2.set_name("my_channel2".to_string());
        channels.push(channel2);

        channel_res.set_channels(channels);

        broker.setup::<OriginChannelListRequest, OriginChannelListResponse>(&channel_res);

        let (response, _) = iron_request(method::Get,
                                         "http://localhost/channels/org",
                                         &mut Vec::new(),
                                         Headers::new(),
                                         broker);
        let result_body = response::extract_body_to_string(response.unwrap());

        assert_eq!(result_body,
                   "[\
            {\
                \"name\":\"my_channel\"\
            },\
            {\
                \"name\":\"my_channel2\"\
            }\
        ]");
    }

//    #[test]
//    fn create_channel() {
//        let mut broker: TestableBroker = Default::default();

//        let mut access_res = CheckOriginAccessResponse::new();
//        access_res.set_has_access(true);
//        broker.setup::<CheckOriginAccessRequest, CheckOriginAccessResponse>(&access_res);

//        let mut origin_res = Origin::new();
//        origin_res.set_id(5000);
//        broker.setup::<OriginGet, Origin>(&origin_res);

//        let mut request = OriginChannelCreate::new();
//        broker.setup::<OriginChannelCreate, OriginChannel>(&request);
//    }
}
