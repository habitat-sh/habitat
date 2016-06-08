// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::borrow::Cow;
use std::fs::{self, File};
use std::io::{Read, Write, BufWriter};
use std::path::PathBuf;
use std::result;
use std::sync::{Arc, Mutex};

use bodyparser;
use dbcache::{self, BasicSet, IndexSet};
use hab_core::package::{Identifiable, FromArchive, PackageArchive};
use hab_core::crypto::keys;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::prelude::*;
use iron::{status, headers, AfterMiddleware};
use iron::headers::{Authorization, Bearer};
use iron::request::Body;
use mount::Mount;
use protobuf;
use protocol::depotsrv;
use protocol::net::{NetError, ErrCode};
use protocol::sessionsrv::{Account, AccountGet, Session, SessionGet};
use protocol::vault::*;
use router::{Params, Router};
use rustc_serialize::json::{self, ToJson};
use unicase::UniCase;
use urlencoded::UrlEncodedQuery;
use zmq;

use super::Depot;
use config::Config;
use error::{Error, Result};
use hab_net::routing::Broker;
use hab_net::server::NetIdent;

pub fn authenticate(req: &mut Request,
                    ctx: &Arc<Mutex<zmq::Context>>)
                    -> result::Result<Session, Response> {
    match req.headers.get::<Authorization<Bearer>>() {
        Some(&Authorization(Bearer { ref token })) => {
            let mut conn = Broker::connect(&ctx).unwrap();
            let mut request = SessionGet::new();
            request.set_token(token.to_string());
            conn.route(&request).unwrap();
            match conn.recv() {
                Ok(rep) => {
                    match rep.get_message_id() {
                        "Session" => {
                            let session = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                            Ok(session)
                        }
                        "NetError" => {
                            let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                            Err(render_net_error(&err))
                        }
                        _ => unreachable!("unexpected msg: {:?}", rep),
                    }
                }
                Err(e) => {
                    error!("session get, err={:?}", e);
                    Err(Response::with(status::InternalServerError))
                }
            }
        }
        _ => Err(Response::with(status::Unauthorized)),
    }
}

/// Return an IronResult containing the body of a NetError and the appropriate HTTP response status
/// for the corresponding NetError.
///
/// For example, a NetError::ENTITY_NOT_FOUND will result in an HTTP response containing the body
/// of the NetError with an HTTP status of 404.
///
/// # Panics
///
/// * The given encoded message was not a NetError
/// * The given messsage could not be decoded
/// * The NetError could not be encoded to JSON
fn render_net_error(err: &NetError) -> Response {
    let encoded = json::encode(&err.to_json()).unwrap();
    let status = match err.get_code() {
        ErrCode::ENTITY_NOT_FOUND => status::NotFound,
        ErrCode::NO_SHARD => status::ServiceUnavailable,
        ErrCode::TIMEOUT => status::RequestTimeout,
        ErrCode::BAD_REMOTE_REPLY => status::BadGateway,
        _ => status::InternalServerError,
    };
    Response::with((status, encoded))
}

pub fn origin_create(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };
    let mut request = OriginCreate::new();
    request.set_owner_id(session.get_id());
    request.set_owner_name(session.get_name().to_string());
    match req.get::<bodyparser::Json>() {
        Ok(Some(body)) => {
            match body.find("name") {
                Some(origin) => request.set_name(origin.as_string().unwrap().to_owned()),
                _ => return Ok(Response::with(status::BadRequest)),
            }
        }
        _ => return Ok(Response::with(status::BadRequest)),
    };

    if !keys::is_valid_origin_name(request.get_name()) {
        return Ok(Response::with(status::UnprocessableEntity));
    }

    let mut conn = Broker::connect(&ctx).unwrap();
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "Origin" => {
                    let origin: Origin = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&origin.to_json()).unwrap();
                    Ok(Response::with((status::Created, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    debug!("origin_create error: {:?}", err);
                    Ok(Response::with(status::Conflict))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

pub fn origin_show(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    let params = req.extensions.get::<Router>().unwrap();
    let origin = match params.find("origin") {
        Some(origin) => origin.to_string(),
        _ => return Ok(Response::with(status::BadRequest)),
    };

    let mut conn = Broker::connect(&ctx).unwrap();
    let mut request = OriginGet::new();
    request.set_name(origin);
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "Origin" => {
                    let origin: Origin = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&origin.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}


/// Return the origin IFF it exists
pub fn get_origin(depot: &Depot, origin: &str) -> Result<Option<Origin>> {
    let ctx = &depot.context;
    let mut conn = Broker::connect(&ctx).unwrap();
    let mut request = OriginGet::new();
    request.set_name(origin.to_string());
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "Origin" => {
                    let origin: Origin = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(Some(origin))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    debug!("get_origin error: {:?}", err);
                    Ok(None)
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(Error::from(e))
        }
    }
}


pub fn check_origin_access(depot: &Depot, account_id: u64, origin_name: &str) -> bool {
    let ctx = &depot.context;
    let mut conn = Broker::connect(&ctx).unwrap();

    let mut request = CheckOriginAccessRequest::new();
    // !!!NOTE!!!
    // only account_id and origin_name are implemented in
    // CheckOriginAccessRequest
    // !!!NOTE!!!
    request.set_account_id(account_id);
    request.set_origin_name(origin_name.to_string());

    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "CheckOriginAccessResponse" => {
                    let response: CheckOriginAccessResponse =
                        protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    response.get_has_access()
                }
                "NetError" => false,
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            debug!("Error checking origin access: {}", e);
            false
        }
    }
}

pub fn invite_to_origin(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };

    let params = req.extensions.get::<Router>().unwrap();

    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let user_to_invite = match params.find("username") {
        Some(username) => username,
        None => return Ok(Response::with(status::BadRequest)),
    };
    debug!("Creating invitation for user {} origin {}",
           &user_to_invite,
           &origin);

    if !check_origin_access(&depot, session.get_id(), &origin) {
        return Ok(Response::with(status::Forbidden));
    }

    // Lookup the users account_id
    let mut conn = Broker::connect(&ctx).unwrap();
    let mut request = AccountGet::new();
    request.set_name(user_to_invite.to_string());
    conn.route(&request).unwrap();

    let acct_obj = match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "Account" => {
                    let account: Account = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    account
                }
                "NetError" => {
                    return Ok(Response::with(status::NotFound));
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            debug!("Error inviting to origin: {}", e);
            return Ok(Response::with(status::NotFound));
        }
    };

    let origin_obj = match try!(get_origin(&depot, &origin)) {
        Some(origin) => origin,
        None => {
            debug!("Origin {} not found", &origin);
            return Ok(Response::with(status::NotFound));
        }
    };

    // store invitations in the vault
    let mut invite_request = OriginInvitationCreate::new();
    invite_request.set_account_id(acct_obj.get_id());
    invite_request.set_account_name(acct_obj.get_name().to_string());
    invite_request.set_origin_id(origin_obj.get_id());
    invite_request.set_origin_name(origin_obj.get_name().to_string());
    invite_request.set_owner_id(session.get_id());

    conn.route(&invite_request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "OriginInvitation" => {
                    // we don't do anything with the invitation, but we could
                    // if we want to!
                    let _invite: OriginInvitation = protobuf::parse_from_bytes(rep.get_body())
                        .unwrap();
                    let encoded = json::encode(&origin.to_json()).unwrap();
                    Ok(Response::with((status::Created, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),

            }
        }
        Err(e) => {
            debug!("Error: {}", &e);
            return Ok(Response::with(status::InternalServerError));
        }
    }
}

pub fn list_origin_invitations(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    debug!("list_origin_invitations");

    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();

    let origin_name = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(&depot, session.get_id(), &origin_name) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut conn = Broker::connect(&ctx).unwrap();
    let mut request = OriginInvitationListRequest::new();

    let origin = match try!(get_origin(&depot, origin_name)) {
        Some(o) => o,
        None => return Ok(Response::with(status::NotFound)),
    };

    request.set_origin_id(origin.get_id());
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "OriginInvitationListResponse" => {
                    let invites: OriginInvitationListResponse =
                        protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&invites.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}

pub fn list_origin_members(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    debug!("list_origin_members");

    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();

    let origin_name = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(&depot, session.get_id(), &origin_name) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut conn = Broker::connect(&ctx).unwrap();
    let mut request = OriginMemberListRequest::new();

    let origin = match try!(get_origin(&depot, origin_name)) {
        Some(o) => o,
        None => return Ok(Response::with(status::NotFound)),
    };

    request.set_origin_id(origin.get_id());
    conn.route(&request).unwrap();
    match conn.recv() {
        Ok(rep) => {
            match rep.get_message_id() {
                "OriginMemberListResponse" => {
                    let members: OriginMemberListResponse =
                        protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    let encoded = json::encode(&members.to_json()).unwrap();
                    Ok(Response::with((status::Ok, encoded)))
                }
                "NetError" => {
                    let err: NetError = protobuf::parse_from_bytes(rep.get_body()).unwrap();
                    Ok(render_net_error(&err))
                }
                _ => unreachable!("unexpected msg: {:?}", rep),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Ok(Response::with(status::ServiceUnavailable))
        }
    }
}


const PAGINATION_RANGE_DEFAULT: isize = 0;
const PAGINATION_RANGE_MAX: isize = 50;

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

fn upload_origin_key(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };

    debug!("Upload Origin Key {:?}", req);
    let params = req.extensions.get::<Router>().unwrap();

    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let revision = match params.find("revision") {
        Some(revision) => revision,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(&depot, session.get_id(), &origin) {
        return Ok(Response::with(status::Forbidden));
    }

    let origin_keyfile = depot.key_path(&origin, &revision);
    debug!("Writing key file {}", origin_keyfile.to_string_lossy());
    if origin_keyfile.is_file() {
        return Ok(Response::with(status::Conflict));
    }

    depot.datastore.origin_keys.write(&origin, &revision).unwrap();

    try!(write_file(&origin_keyfile, &mut req.body));

    let mut response = Response::with((status::Created,
                                       format!("/origins/{}/keys/{}", &origin, &revision)));

    let mut base_url = req.url.clone();
    base_url.path = vec![String::from("key"), format!("{}-{}", &origin, &revision)];
    response.headers.set(headers::Location(format!("{}", base_url)));
    Ok(response)
}


fn upload_origin_secret_key(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Upload Origin Secret Key {:?}", req);
    let ctx = &depot.context;

    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => {
            return Ok(response);
        }
    };

    let params = req.extensions.get::<Router>().unwrap();

    let name = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let revision = match params.find("revision") {
        Some(revision) => revision,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(&depot, session.get_id(), &name) {
        return Ok(Response::with(status::Forbidden));
    }


    let o = match try!(get_origin(&depot, name)) {
        Some(o) => o,
        None => return Ok(Response::with(status::NotFound)),
    };

    let mut request = OriginSecretKeyCreate::new();
    request.set_owner_id(session.get_id());
    request.set_origin_id(o.get_id());
    request.set_name(name.to_string());
    request.set_revision(revision.to_string());

    let mut key_content = Vec::new();
    if let Err(e) = req.body.read_to_end(&mut key_content) {
        debug!("Can't read key content {}", e);
        return Ok(Response::with(status::BadRequest));
    }
    request.set_body(key_content);
    request.set_owner_id(0);

    let mut conn = Broker::connect(&ctx).unwrap();
    conn.route(&request).unwrap();
    Ok(Response::with(status::Ok))
}


fn upload_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };

    debug!("Upload {:?}", req);
    let checksum_from_param = match extract_query_value("checksum", req) {
        Some(checksum) => checksum,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let ident = ident_from_params(params);

    if !check_origin_access(&depot, session.get_id(), &ident.get_origin()) {
        return Ok(Response::with(status::Forbidden));
    }

    if !ident.fully_qualified() {
        return Ok(Response::with(status::BadRequest));
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
        let mut base_url = req.url.clone();
        base_url.path =
            vec![String::from("pkgs"), object.get_ident().to_string(), String::from("download")];
        response.headers.set(headers::Location(format!("{}", base_url)));
        Ok(response)
    } else {
        info!("Ident mismatch, expected={:?}, got={:?}",
              ident,
              object.get_ident());
        Ok(Response::with(status::UnprocessableEntity))
    }
}

fn download_origin_key(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Download origin key {:?}", req);
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
    // use set_raw because we're having problems with Iron's Hyper 0.8.x
    // and the newer Hyper 0.9.4. TODO: change back to set() once
    // Iron updates to Hyper 0.9.x.
    response.headers.set_raw("X-Filename", vec![xfilename.clone().into_bytes()]);
    response.headers.set_raw("content-disposition",
                             vec![format!("attachment; filename=\"{}\"", xfilename.clone())
                                      .into_bytes()]);
    Ok(response)
}


fn download_latest_origin_key(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Download latest origin key {:?}", req);
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
    // use set_raw because we're having problems with Iron's Hyper 0.8.x
    // and the newer Hyper 0.9.4. TODO: change back to set() once
    // Iron updates to Hyper 0.9.x.
    response.headers.set_raw("X-Filename", vec![xfilename.clone().into_bytes()]);
    response.headers.set_raw("content-disposition",
                             vec![format!("attachment; filename=\"{}\"", xfilename.clone())
                                      .into_bytes()]);
    Ok(response)
}

fn download_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    debug!("Download {:?}", req);
    let params = req.extensions.get::<Router>().unwrap();
    let ident = ident_from_params(params);

    match depot.datastore.packages.find(&ident) {
        Ok(ident) => {
            if let Some(archive) = depot.archive(&ident) {
                match fs::metadata(&archive.path) {
                    Ok(_) => {
                        let mut response = Response::with((status::Ok, archive.path.clone()));
                        // use set_raw because we're having problems with Iron's Hyper 0.8.x
                        // and the newer Hyper 0.9.4. TODO: change back to set() once
                        // Iron updates to Hyper 0.9.x.

                        response.headers
                            .set_raw("X-Filename", vec![archive.file_name().clone().into_bytes()]);
                        response.headers.set_raw("content-disposition",
                                                 vec![format!("attachment; filename=\"{}\"",
                                                              archive.file_name().clone())
                                                          .into_bytes()]);
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

fn list_origin_keys(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let origin = match params.find("origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    match depot.datastore.origin_keys.all(origin) {
        Ok(revisions) => {
            let body = json::encode(&revisions.to_json()).unwrap();
            Ok(Response::with((status::Ok, body)))
        }
        Err(e) => {
            error!("list_origin_keys:1, err={:?}", e);
            Ok(Response::with(status::InternalServerError))
        }
    }

}


fn list_packages(depot: &Depot, req: &mut Request) -> IronResult<Response> {
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

    if let Some(view) = params.find("view") {
        match depot.datastore.views.view_pkg_idx.all(view, &ident) {
            Ok(packages) => {
                let count = depot.datastore.packages.index.count(&ident).unwrap();
                let body = json::encode(&packages).unwrap();
                let next_range = vec![format!("{}", num + 1).into_bytes()];
                let mut response = if count as isize >= (num + 1) {
                    let mut response = Response::with((status::PartialContent, body));
                    response.headers.set_raw("Next-Range", next_range);
                    response
                } else {
                    Response::with((status::Ok, body))
                };
                let range = vec![format!("{}..{}; count={}", offset, num, count).into_bytes()];
                response.headers.set_raw("Content-Range", range);
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
                let next_range = vec![format!("{}", num + 1).into_bytes()];
                let mut response = if count as isize >= (num + 1) {
                    let mut response = Response::with((status::PartialContent, body));
                    response.headers.set_raw("Next-Range", next_range);
                    response
                } else {
                    Response::with((status::Ok, body))
                };
                let range = vec![format!("{}..{}; count={}", offset, num, count).into_bytes()];
                response.headers.set_raw("Content-Range", range);
                response.headers.set(ContentType(Mime(TopLevel::Application,
                                                      SubLevel::Json,
                                                      vec![(Attr::Charset, Value::Utf8)])));
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

fn list_views(depot: &Depot, _req: &mut Request) -> IronResult<Response> {
    let views = try!(depot.datastore.views.all());
    let body = json::encode(&views).unwrap();
    Ok(Response::with((status::Ok, body)))
}

fn show_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let mut ident = ident_from_params(params);

    if let Some(view) = params.find("view") {
        if !ident.fully_qualified() {
            match depot.datastore.views.view_pkg_idx.latest(view, &ident.to_string()) {
                Ok(ident) => {
                    match depot.datastore.packages.find(&ident) {
                        Ok(pkg) => render_package(&pkg),
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
            match depot.datastore.views.view_pkg_idx.is_member(view, &ident) {
                Ok(true) => {
                    match depot.datastore.packages.find(&ident) {
                        Ok(pkg) => render_package(&pkg),
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
            Ok(pkg) => render_package(&pkg),
            Err(dbcache::Error::EntityNotFound) => Ok(Response::with(status::NotFound)),
            Err(e) => {
                error!("show_package:6, err={:?}", e);
                Ok(Response::with(status::InternalServerError))
            }
        }
    }
}

fn search_packages(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let (offset, num) = match extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    let params = req.extensions.get::<Router>().unwrap();
    let partial = params.find("query").unwrap();
    let packages = depot.datastore.packages.index.search(partial, offset, num).unwrap();
    let body = json::encode(&packages).unwrap();
    let next_range = vec![format!("{}", num + 1).into_bytes()];
    let mut response = if packages.len() as isize >= (num - offset) {
        let mut response = Response::with((status::PartialContent, body));
        response.headers.set_raw("Next-Range", next_range);
        response
    } else {
        Response::with((status::Ok, body))
    };
    let range = vec![format!("{}..{}", offset, num).into_bytes()];
    response.headers.set_raw("Content-Range", range);
    Ok(response)
}

fn render_package(pkg: &depotsrv::Package) -> IronResult<Response> {
    let body = json::encode(&pkg.to_json()).unwrap();
    let mut response = Response::with((status::Ok, body));
    // use set_raw because we're having problems with Iron's Hyper 0.8.x
    // and the newer Hyper 0.9.4.
    // TODO: change back to set() once Iron updates to Hyper 0.9.x.
    response.headers.set_raw("ETag", vec![pkg.get_checksum().to_string().into_bytes()]);
    Ok(response)
}

fn promote_package(depot: &Depot, req: &mut Request) -> IronResult<Response> {
    let ctx = &depot.context;
    let session = match authenticate(req, ctx) {
        Ok(session) => session,
        Err(response) => return Ok(response),
    };

    let params = req.extensions.get::<Router>().unwrap();
    let view = params.find("view").unwrap();

    match depot.datastore.views.is_member(view) {
        Ok(true) => {
            let ident = ident_from_params(params);
            if !check_origin_access(&depot, session.get_id(), &ident.get_origin()) {
                return Ok(Response::with(status::Forbidden));
            }
            match depot.datastore.packages.find(&ident) {
                Ok(package) => {
                    depot.datastore.views.associate(view, &package).unwrap();
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

struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers
            .set(headers::AccessControlExposeHeaders(vec![UniCase("content-range".to_owned()),
                                                          UniCase("next-range".to_owned())]));
        res.headers
            .set(headers::AccessControlAllowHeaders(vec![UniCase("authorization".to_owned()),
                                                         UniCase("range".to_owned())]));
        Ok(res)
    }
}

pub fn router(depot: Arc<Depot>) -> Result<Chain> {
    // we can't call depot.clone() in fn argument,
    // so we bind them here instead.
    let depot1 = depot.clone();
    let depot2 = depot.clone();
    let depot3 = depot.clone();
    let depot4 = depot.clone();
    let depot5 = depot.clone();
    let depot6 = depot.clone();
    let depot7 = depot.clone();
    let depot8 = depot.clone();
    let depot9 = depot.clone();
    let depot10 = depot.clone();
    let depot11 = depot.clone();
    let depot12 = depot.clone();
    let depot13 = depot.clone();
    let depot14 = depot.clone();
    let depot15 = depot.clone();
    let depot16 = depot.clone();
    let depot17 = depot.clone();
    let depot18 = depot.clone();
    let depot19 = depot.clone();
    let depot20 = depot.clone();
    let depot21 = depot.clone();
    let depot22 = depot.clone();
    let depot23 = depot.clone();
    let depot24 = depot.clone();
    let depot25 = depot.clone();
    let depot26 = depot.clone();
    let depot27 = depot.clone();

    let router = router!(
        get "/views" => move |r: &mut Request| list_views(&depot1, r),
        get "/views/:view/pkgs/:origin" => move |r: &mut Request| list_packages(&depot2, r),
        get "/views/:view/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&depot3, r),
        get "/views/:view/pkgs/:origin/:pkg/latest" => {
            move |r: &mut Request| show_package(&depot4, r)
        },
        get "/views/:view/pkgs/:origin/:pkg/:version" => {
            move |r: &mut Request| list_packages(&depot5, r)
        },
        get "/views/:view/pkgs/:origin/:pkg/:version/latest" => {
            move |r: &mut Request| show_package(&depot6, r)
        },
        get "/views/:view/pkgs/:origin/:pkg/:version/:release" => {
            move |r: &mut Request| show_package(&depot7, r)
        },
        post "/views/:view/pkgs/:origin/:pkg/:version/:release/promote" => {
            move |r: &mut Request| promote_package(&depot8, r)
        },

        get "/pkgs/search/:query" => move |r: &mut Request| search_packages(&depot9, r),
        get "/pkgs/:origin" => move |r: &mut Request| list_packages(&depot10, r),
        get "/pkgs/:origin/:pkg" => move |r: &mut Request| list_packages(&depot11, r),
        get "/pkgs/:origin/:pkg/latest" => move |r: &mut Request| show_package(&depot12, r),
        get "/pkgs/:origin/:pkg/:version" => move |r: &mut Request| list_packages(&depot13, r),
        get "/pkgs/:origin/:pkg/:version/latest" => {
            move |r: &mut Request| show_package(&depot14, r)
        },
        get "/pkgs/:origin/:pkg/:version/:release" => {
            move |r: &mut Request| show_package(&depot15, r)
        },

        get "/pkgs/:origin/:pkg/:version/:release/download" => {
            move |r: &mut Request| download_package(&depot16, r)
        },
        post "/pkgs/:origin/:pkg/:version/:release" => {
            move |r: &mut Request| upload_package(&depot17, r)
        },


        post "/origins" => move |r: &mut Request| origin_create(&depot18, r),
        // TODO
        //delete "/origins/:origin" => move |r: &mut Request| origin_delete(&depot17, r),

        get "/origins/:origin" => move |r: &mut Request| origin_show(&depot19, r),

        get "/origins/:origin/keys" => move |r: &mut Request| list_origin_keys(&depot20, r),
        get "/origins/:origin/keys/latest" => {
            move |r: &mut Request| download_latest_origin_key(&depot21, r)
        },
        get "/origins/:origin/keys/:revision" => {
            move |r: &mut Request| download_origin_key(&depot22, r)
        },
        post "/origins/:origin/keys/:revision" => {
            move |r: &mut Request| upload_origin_key(&depot23, r)
        },
        post "/origins/:origin/secret_keys/:revision" => {
            move |r: &mut Request| upload_origin_secret_key(&depot24, r)
        },
        post "/origins/:origin/users/:username/invitations" => {
            move |r: &mut Request| invite_to_origin(&depot25, r)
        },
        get "/origins/:origin/invitations" => {
            move |r: &mut Request| list_origin_invitations(&depot26, r)
        },
        get "/origins/:origin/users" => {
            move |r: &mut Request| list_origin_members(&depot27, r)
        },
    );
    let mut chain = Chain::new(router);
    chain.link_after(Cors);
    Ok(chain)
}

use hab_net::config::RouteAddrs;
pub fn run(config: Config) -> Result<()> {
    let listen_addr = config.listen_addr.clone();

    let ctx = Arc::new(Mutex::new(zmq::Context::new()));
    let depot = try!(Depot::new(config.clone(), ctx));
    let v1 = try!(router(depot.clone()));

    let broker = Broker::run(Depot::net_ident(),
                             depot.context.clone(),
                             &config.route_addrs().clone());

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
