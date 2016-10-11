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

use std::borrow::Cow;
use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::PathBuf;
use std::result;

use bodyparser;
use dbcache::{self, BasicSet};
use hab_core::package::{Identifiable, FromArchive, PackageArchive};
use hab_core::crypto::keys::{self, PairType};
use hab_core::crypto::SigKeyPair;
use hab_net::config::RouteAddrs;
use hab_net::http::controller::*;
use hab_net::routing::{Broker, BrokerConn};
use hab_net::server::NetIdent;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::{ContentType, Vary};
use iron::prelude::*;
use iron::{status, headers};
use iron::request::Body;
use mount::Mount;
use persistent;
use protocol::depotsrv;
use protocol::net::ErrCode;
use protocol::sessionsrv::{Account, AccountGet};
use protocol::vault::*;
use router::{Params, Router};
use rustc_serialize::json::{self, ToJson};
use unicase::UniCase;
use urlencoded::UrlEncodedQuery;

use super::Depot;
use config::Config;
use error::{Error, Result};

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

const PAGINATION_RANGE_DEFAULT: isize = 0;
const PAGINATION_RANGE_MAX: isize = 50;
const ONE_YEAR_IN_SECS: usize = 31536000;

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

pub fn get_origin(conn: &mut BrokerConn, origin: &str) -> IronResult<Option<Origin>> {
    let mut request = OriginGet::new();
    request.set_name(origin.to_string());
    match conn.route::<OriginGet, Origin>(&request) {
        Ok(origin) => Ok(Some(origin)),
        Err(err) => {
            if err.get_code() == ErrCode::ENTITY_NOT_FOUND {
                Ok(None)
            } else {
                let body = json::encode(&err.to_json()).unwrap();
                let status = net_err_to_http(err.get_code());
                Err(IronError::new(err, (body, status)))
            }
        }
    }
}

pub fn check_origin_access<T: ToString>(conn: &mut BrokerConn,
                                        account_id: u64,
                                        origin: T)
                                        -> IronResult<bool> {
    let mut request = CheckOriginAccessRequest::new();
    request.set_account_id(account_id);
    request.set_origin_name(origin.to_string());
    match conn.route::<CheckOriginAccessRequest, CheckOriginAccessResponse>(&request) {
        Ok(response) => Ok(response.get_has_access()),
        Err(err) => {
            let body = json::encode(&err.to_json()).unwrap();
            let status = net_err_to_http(err.get_code());
            Err(IronError::new(err, (body, status)))
        }
    }
}

pub fn invite_to_origin(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
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
    if !try!(check_origin_access(&mut conn, session.get_id(), &origin)) {
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
    match try!(get_origin(&mut conn, &origin)) {
        Some(mut origin) => {
            invite_request.set_origin_id(origin.get_id());
            invite_request.set_origin_name(origin.take_name());
        }
        None => return Ok(Response::with(status::NotFound)),
    };
    invite_request.set_owner_id(session.get_id());
    // store invitations in the vault
    match conn.route::<OriginInvitationCreate, OriginInvitation>(&invite_request) {
        Ok(invitation) => Ok(render_json(status::Created, &invitation)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn list_origin_invitations(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let origin_name = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut conn = Broker::connect().unwrap();
    if !try!(check_origin_access(&mut conn, session.get_id(), &origin_name)) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginInvitationListRequest::new();
    match try!(get_origin(&mut conn, origin_name)) {
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
    let session = req.extensions.get::<Authenticated>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let origin_name = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let mut conn = Broker::connect().unwrap();

    if !try!(check_origin_access(&mut conn, session.get_id(), &origin_name)) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginMemberListRequest::new();
    match try!(get_origin(&mut conn, origin_name)) {
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

fn write_string_to_file(filename: &PathBuf, body: String) -> Result<bool> {
    let path = filename.parent().unwrap();
    try!(fs::create_dir_all(path));
    let tempfile = format!("{}.tmp", filename.to_string_lossy());
    let f = try!(File::create(&tempfile));
    let mut writer = BufWriter::new(&f);
    try!(writer.write_all(body.as_bytes()));
    info!("File added to Depot at {}", filename.to_string_lossy());
    try!(fs::rename(&tempfile, &filename));
    Ok(true)
}

fn write_file(filename: &PathBuf, body: &mut Body) -> Result<bool> {
    let path = filename.parent().unwrap();
    try!(fs::create_dir_all(path));
    let tempfile = format!("{}.tmp", filename.to_string_lossy());
    let f = try!(File::create(&tempfile));
    let mut writer = BufWriter::new(&f);
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
    info!("File added to Depot at {}", filename.to_string_lossy());
    try!(fs::rename(&tempfile, &filename));
    Ok(true)
}

fn upload_origin_key(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let origin = params.find("origin").unwrap();
    let revision = params.find("revision").unwrap();
    if !depot.config.insecure {
        let session = req.extensions.get::<Authenticated>().unwrap();
        let mut conn = Broker::connect().unwrap();
        if !try!(check_origin_access(&mut conn, session.get_id(), origin)) {
            return Ok(Response::with(status::Forbidden));
        }
    }

    let mut content = String::new();
    if let Err(e) = req.body.read_to_string(&mut content) {
        debug!("Can't read public key upload content: {}", e);
        return Ok(Response::with(status::NotAcceptable));
    }

    match SigKeyPair::parse_key_str(&content) {
        Ok((PairType::Public, _, _)) => (),
        Ok(_) => {
            return Ok(Response::with((status::NotAcceptable,
                                      format!("Received a secret key instead of a public key"))));
        }
        Err(e) => {
            return Ok(Response::with((status::NotAcceptable,
                                      format!("Invalid public key content: {}", e))));
        }
    }

    let origin_keyfile = depot.key_path(origin, revision);
    // TODO: We can't just check if the keyfile exists on disk since this operation also requires
    // an entry in the database. This operation should instead check the database since that is
    // the last step of writing an origin key and represents the success.
    if origin_keyfile.is_file() {
        return Ok(Response::with(status::Conflict));
    }
    debug!("Writing key file {}", origin_keyfile.to_string_lossy());
    try!(write_string_to_file(&origin_keyfile, content));
    try!(depot.datastore.origin_keys.write(&origin, &revision));

    let mut response = Response::with((status::Created,
                                       format!("/origins/{}/keys/{}", &origin, &revision)));
    let mut base_url = req.url.clone().into_generic_url();
    base_url.set_path(&format!("key/{}-{}", &origin, &revision));
    response.headers.set(headers::Location(format!("{}", base_url)));
    Ok(response)
}

fn upload_origin_secret_key(req: &mut Request) -> IronResult<Response> {
    debug!("Upload Origin Secret Key {:?}", req);
    let session = req.extensions.get::<Authenticated>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginSecretKeyCreate::new();
    request.set_owner_id(session.get_id());

    match params.find("origin") {
        Some(origin) => {
            if !try!(check_origin_access(&mut conn, session.get_id(), origin)) {
                return Ok(Response::with(status::Forbidden));
            }
            match try!(get_origin(&mut conn, origin)) {
                Some(mut origin) => {
                    request.set_name(origin.take_name());
                    request.set_origin_id(origin.get_id());
                }
                None => return Ok(Response::with(status::NotFound)),
            };
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
        Ok(_) => Ok(Response::with(status::Created)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn upload_package(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
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

    let mut conn = Broker::connect().unwrap();

    debug!("UPLOADING checksum={}, ident={}",
           checksum_from_param,
           ident);

    if !depot.config.insecure {
        let session = req.extensions.get::<Authenticated>().unwrap();
        if !try!(check_origin_access(&mut conn, session.get_id(), &ident.get_origin())) {
            return Ok(Response::with(status::Forbidden));
        }
        if !ident.fully_qualified() {
            return Ok(Response::with(status::BadRequest));
        }
    }

    match depot.datastore.packages.find(&ident) {
        Ok(_) |
        Err(dbcache::Error::EntityNotFound) => {
            if let Some(_) = depot.archive(&ident) {
                return Ok(Response::with((status::Conflict)));
            }
        }
        Err(e) => {
            error!("upload_package:1, err={:?}", e);
            return Ok(Response::with(status::InternalServerError));
        }
    }

    let filename = depot.archive_path(&ident);
    try!(write_file(&filename, &mut req.body));
    let mut archive = PackageArchive::new(filename);
    debug!("Package Archive: {:#?}", archive);
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
    let object = match depotsrv::Package::from_archive(&mut archive) {
        Ok(object) => object,
        Err(e) => {
            info!("Error building package from archive: {:#?}", e);
            return Ok(Response::with(status::UnprocessableEntity));
        }
    };
    if ident.satisfies(object.get_ident()) {
        depot.datastore.packages.write(&object).unwrap();
        let mut response = Response::with((status::Created,
                                           format!("/pkgs/{}/download", object.get_ident())));
        let mut base_url = req.url.clone().into_generic_url();
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

fn download_origin_key(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let revision = match params.find("revision") {
        Some(revision) => revision,
        None => return Ok(Response::with(status::BadRequest)),
    };
    debug!("Trying to retreive origin key {}-{}", &origin, &revision);
    let origin_keyfile = depot.key_path(&origin, &revision);
    debug!("Looking for {}", &origin_keyfile.to_string_lossy());
    match origin_keyfile.metadata() {
        Ok(md) => {
            if !md.is_file() {
                return Ok(Response::with(status::NotFound));
            };
        }
        Err(e) => {
            println!("Can't read key file {}: {}",
                     &origin_keyfile.to_string_lossy(),
                     e);
            return Ok(Response::with(status::NotFound));
        }
    };

    let xfilename = origin_keyfile.file_name().unwrap().to_string_lossy().into_owned();
    let mut response = Response::with((status::Ok, origin_keyfile));
    response.headers.set(ContentDisposition(format!("attachment; filename=\"{}\"", xfilename)));
    response.headers.set(XFileName(xfilename));
    do_cache_response(&mut response);
    Ok(response)
}

fn download_latest_origin_key(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    debug!("Trying to retreive latest origin key for {}", &origin);
    let latest_rev = depot.datastore.origin_keys.latest(&origin).unwrap();
    let origin_keyfile = depot.key_path(&origin, &latest_rev);
    debug!("Looking for {}", &origin_keyfile.to_string_lossy());
    match origin_keyfile.metadata() {
        Ok(md) => {
            if !md.is_file() {
                return Ok(Response::with(status::NotFound));
            };
        }
        Err(e) => {
            println!("Can't read key file {}: {}",
                     &origin_keyfile.to_string_lossy(),
                     e);
            return Ok(Response::with(status::NotFound));
        }
    };

    let xfilename = origin_keyfile.file_name().unwrap().to_string_lossy().into_owned();
    let mut response = Response::with((status::Ok, origin_keyfile));
    response.headers.set(ContentDisposition(format!("attachment; filename=\"{}\"", xfilename)));
    response.headers.set(XFileName(xfilename));
    dont_cache_response(&mut response);
    Ok(response)
}

fn download_package(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let ident = ident_from_params(params);

    match depot.datastore.packages.find(&ident) {
        Ok(ident) => {
            if let Some(archive) = depot.archive(&ident) {
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
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let origin = params.find("origin").unwrap();
    match depot.datastore.origin_keys.all(origin) {
        Ok(revisions) => {
            let body = json::encode(&revisions.to_json()).unwrap();
            let mut response = Response::with((status::Ok, body));
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(e) => {
            error!("list_origin_keys:1, err={:?}", e);
            Ok(Response::with(status::InternalServerError))
        }
    }
}

fn list_packages(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let (offset, num) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let ident: String = if params.find("pkg").is_none() {
        match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        }
    } else {
        ident_from_params(params).to_string()
    };

    if let Some(channel) = params.find("channel") {
        match depot.datastore.channels.channel_pkg_idx.all(channel, &ident) {
            Ok(packages) => {
                let count = depot.datastore.packages.index.count(&ident).unwrap();
                let body = json::encode(&packages).unwrap();
                let next_range = num + 1;
                let mut response = if count as isize >= next_range {
                    let mut response = Response::with((status::PartialContent, body));
                    response.headers.set(NextRange(next_range));
                    response
                } else {
                    Response::with((status::Ok, body))
                };
                let range = format!("{}..{}; count={}", offset, num, count);
                response.headers.set(ContentRange(range.clone()));
                // We set both of these because Fastly was stripping the
                // Content-Range, so we use both until we can get that fixed.
                response.headers.set(XContentRange(range));
                response.headers.set(ContentType(Mime(TopLevel::Application,
                                                      SubLevel::Json,
                                                      vec![(Attr::Charset, Value::Utf8)])));
                response.headers.set(Vary::Items(vec![UniCase("range".to_owned())]));
                dont_cache_response(&mut response);
                Ok(response)
            }
            Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                Ok(Response::with((status::NotFound)))
            }
            Err(e) => {
                error!("list_packages:1, err={:?}", e);
                Ok(Response::with(status::InternalServerError))
            }
        }
    } else {
        match depot.datastore.packages.index.list(&ident, offset, num) {
            Ok(packages) => {
                let count = depot.datastore.packages.index.count(&ident).unwrap();
                let body = json::encode(&packages).unwrap();
                let next_range = num + 1;
                let mut response = if count as isize >= next_range {
                    let mut response = Response::with((status::PartialContent, body));
                    response.headers.set(NextRange(next_range));
                    response
                } else {
                    Response::with((status::Ok, body))
                };
                let range = format!("{}..{}; count={}", offset, num, count);
                response.headers.set(ContentRange(range.clone()));
                // We set both of these because Fastly was stripping the
                // Content-Range, so we use both until we can get that fixed.
                response.headers.set(XContentRange(range));
                response.headers.set(ContentType(Mime(TopLevel::Application,
                                                      SubLevel::Json,
                                                      vec![(Attr::Charset, Value::Utf8)])));
                response.headers.set(Vary::Items(vec![UniCase("range".to_owned())]));
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

fn list_channels(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let channels = try!(depot.datastore.channels.all());
    let body = json::encode(&channels).unwrap();

    let mut response = Response::with((status::Ok, body));
    dont_cache_response(&mut response);
    Ok(response)
}

fn show_package(req: &mut Request) -> IronResult<Response> {
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let mut ident = ident_from_params(params);

    if let Some(channel) = params.find("channel") {
        if !ident.fully_qualified() {
            match depot.datastore.channels.channel_pkg_idx.latest(channel, &ident.to_string()) {
                Ok(ident) => {
                    match depot.datastore.packages.find(&ident) {
                        Ok(pkg) => render_package(&pkg, false),
                        Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
                        Err(e) => {
                            error!("show_package:1, err={:?}", e);
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
                }
                Err(Error::DataStore(dbcache::Error::EntityNotFound)) => {
                    Ok(Response::with(status::NotFound))
                }
                Err(e) => {
                    error!("show_package:2, err={:?}", e);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        } else {
            match depot.datastore.channels.channel_pkg_idx.is_member(channel, &ident) {
                Ok(true) => {
                    match depot.datastore.packages.find(&ident) {
                        Ok(pkg) => render_package(&pkg, false),
                        Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
                        Err(e) => {
                            error!("show_package:3, err={:?}", e);
                            Ok(Response::with(status::InternalServerError))
                        }
                    }
                }
                Ok(false) => Ok(Response::with(status::NotFound)),
                Err(e) => {
                    error!("show_package:4, err={:?}", e);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    } else {
        if !ident.fully_qualified() {
            match depot.datastore.packages.index.latest(&ident) {
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
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let (offset, num) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let partial = params.find("query").unwrap();
    let packages = depot.datastore.packages.index.search(partial, offset, num).unwrap();
    let body = json::encode(&packages).unwrap();
    let mut response = if packages.len() as isize >= (num - offset) {
        let mut response = Response::with((status::PartialContent, body));
        response.headers.set(NextRange(num + 1));
        response
    } else {
        Response::with((status::Ok, body))
    };
    let range = format!("{}..{}", offset, num);
    response.headers.set(ContentRange(range.clone()));
    // We set both of these because Fastly was stripping the
    // Content-Range, so we use both until we can get that fixed.
    response.headers.set(XContentRange(range));
    response.headers.set(ContentType(Mime(TopLevel::Application,
                                          SubLevel::Json,
                                          vec![(Attr::Charset, Value::Utf8)])));
    response.headers.set(Vary::Items(vec![UniCase("range".to_owned())]));
    dont_cache_response(&mut response);
    Ok(response)
}

fn render_package(pkg: &depotsrv::Package, should_cache: bool) -> IronResult<Response> {
    let body = json::encode(&pkg.to_json()).unwrap();
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
    let depot = req.get::<persistent::Read<Depot>>().unwrap();
    let session = req.extensions.get::<Authenticated>().unwrap();
    let (channel, ident) = {
        let params = req.extensions.get::<Router>().unwrap();
        let channel = params.find("channel").unwrap();
        let ident = ident_from_params(params);
        (channel, ident)
    };
    let mut conn = Broker::connect().unwrap();
    match depot.datastore.channels.is_member(channel) {
        Ok(true) => {
            if !try!(check_origin_access(&mut conn, session.get_id(), &ident.get_origin())) {
                return Ok(Response::with(status::Forbidden));
            }
            match depot.datastore.packages.find(&ident) {
                Ok(package) => {
                    depot.datastore.channels.associate(channel, &package).unwrap();
                    Ok(Response::with(status::Ok))
                }
                Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
                Err(e) => {
                    error!("promote:2, err={:?}", e);
                    return Ok(Response::with(status::InternalServerError));
                }
            }
        }
        Ok(false) => Ok(Response::with(status::NotFound)),
        Err(e) => {
            error!("promote:1, err={:?}", e);
            return Ok(Response::with(status::InternalServerError));
        }
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

// Returns a tuple representing the from and to values representing a paginated set.
//
// These values can be passed to a sorted set in Redis to return a paginated list.
fn extract_pagination(req: &mut Request) -> result::Result<(isize, isize), Response> {
    let offset = {
        match req.headers.get_raw("range") {
            Some(bytes) if bytes.len() > 0 => {
                let header = Cow::Borrowed(&bytes[0]);
                match String::from_utf8(header.into_owned()) {
                    Ok(raw) => {
                        match raw.parse::<usize>() {
                            Ok(range) if range > 0 => (range - 1) as isize,
                            Ok(range) => range as isize,
                            Err(_) => return Err(Response::with(status::BadRequest)),
                        }
                    }
                    Err(_) => return Err(Response::with(status::BadRequest)),
                }
            }
            _ => PAGINATION_RANGE_DEFAULT,
        }
    };
    Ok((offset, offset + PAGINATION_RANGE_MAX))
}

fn extract_query_value(key: &str, req: &mut Request) -> Option<String> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(map) => {
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

pub fn router(depot: Depot) -> Result<Chain> {
    let basic = Authenticated::new(&depot.config);
    let router = router!(
        channels: get "/channels" => list_channels,
        channel_packages: get "/channels/:channel/pkgs/:origin" => list_packages,
        channel_packages_pkg: get "/channels/:channel/pkgs/:origin/:pkg" => list_packages,
        channel_package_latest: get "/channels/:channel/pkgs/:origin/:pkg/latest" => show_package,
        channel_packages_version:
            get "/channels/:channel/pkgs/:origin/:pkg/:version" => list_packages,
        channel_package_version_latest:
            get "/channels/:channel/pkgs/:origin/:pkg/:version/latest" => show_package,
        channel_package: get "/channels/:channel/pkgs/:origin/:pkg/:version/:release" => {
            show_package
        },
        channel_package_promote:
            post "/channels/:channel/pkgs/:origin/:pkg/:version/:release/promote" => {
                XHandler::new(promote_package).before(basic.clone())
        },
        channel_package_download:
            get "/channels/:channel/pkgs/:origin/:pkg/:version/:release/download" => {
                download_package
        },

        channel_origin_keys: get "/channels/:channel/origins/:origin/keys" => list_origin_keys,
        channel_origin_key_latest: get "/channels/:channel/origins/:origin/keys/latest" => {
            download_latest_origin_key
        },
        channel_origin_key: get "/channels/:channel/origins/:origin/keys/:revision" => {
            download_origin_key
        },

        package_search: get "/pkgs/search/:query" => search_packages,
        packages: get "/pkgs/:origin" => list_packages,
        packages_pkg: get "/pkgs/:origin/:pkg" => list_packages,
        package_pkg_latest: get "/pkgs/:origin/:pkg/latest" => show_package,
        packages_version: get "/pkgs/:origin/:pkg/:version" => list_packages,
        package_version_latest: get "/pkgs/:origin/:pkg/:version/latest" => show_package,
        package: get "/pkgs/:origin/:pkg/:version/:release" => show_package,

        package_download: get "/pkgs/:origin/:pkg/:version/:release/download" => {
            download_package
        },
        package_upload: post "/pkgs/:origin/:pkg/:version/:release" => {
            if depot.config.insecure {
                XHandler::new(upload_package)
            } else {
                XHandler::new(upload_package).before(basic.clone())
            }
        },

        origin_create: post "/origins" => {
            XHandler::new(origin_create).before(basic.clone())
        },
        origin: get "/origins/:origin" => origin_show,

        origin_keys: get "/origins/:origin/keys" => list_origin_keys,
        origin_key_latest: get "/origins/:origin/keys/latest" => download_latest_origin_key,
        origin_key: get "/origins/:origin/keys/:revision" => download_origin_key,
        origin_key_create: post "/origins/:origin/keys/:revision" => {
            if depot.config.insecure {
                XHandler::new(upload_origin_key)
            } else {
                XHandler::new(upload_origin_key).before(basic.clone())
            }
        },
        origin_secret_key_create: post "/origins/:origin/secret_keys/:revision" => {
            XHandler::new(upload_origin_secret_key).before(basic.clone())
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
    );
    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<Depot>::both(depot));
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
