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

use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{BufWriter, Read, Write};
use std::result;
use std::str::FromStr;

use base64;
use bldr_core;
use bldr_core::helpers::transition_visibility;
use bodyparser;
use github_api_client::GitHubClient;
use hab_core::package::{ident, FromArchive, Identifiable, PackageArchive, PackageIdent,
                        PackageTarget};
use hab_core::crypto::keys::PairType;
use hab_core::crypto::{BoxKeyPair, SigKeyPair};
use hab_core::crypto::PUBLIC_BOX_KEY_VERSION;
use hab_core::event::*;
use http_gateway::http::controller::*;
use http_gateway::http::helpers::{self, all_visibilities, check_origin_access, check_origin_owner,
                                  dont_cache_response, get_param, visibility_for_optional_session};
use http_gateway::http::middleware::{SegmentCli, XRouteClient};
use hab_net::{privilege, ErrCode, NetOk, NetResult};
use hyper::header::{Charset, ContentDisposition, DispositionParam, DispositionType};
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use iron::headers::{ContentType, UserAgent};
use iron::middleware::BeforeMiddleware;
use iron::request::Body;
use persistent;
use protobuf;
use protocol::originsrv::*;
use protocol::jobsrv::{JobGroup, JobGroupOriginGet, JobGroupOriginResponse, JobGroupSpec,
                       JobGroupGet, JobGraphPackageStatsGet, JobGraphPackageStats,
                       JobGraphPackagePreCreate, JobGroupAbort};
use protocol::sessionsrv::{Account, AccountGet, AccountOriginRemove};
use regex::Regex;
use router::{Params, Router};
use segment_api_client::SegmentClient;
use serde_json;
use typemap;
use url;
use uuid::Uuid;

use super::DepotUtil;
use error::{Error, Result};
use handlers;

define_event_log!();

#[derive(Clone, Serialize, Deserialize)]
struct OriginCreateReq {
    name: String,
    default_package_visibility: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct OriginUpdateReq {
    default_package_visibility: String,
}

const ONE_YEAR_IN_SECS: usize = 31536000;

pub fn origin_update(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginUpdate::new();
    match get_param(req, "name") {
        Some(origin) => request.set_name(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }

    if !check_origin_access(req, request.get_name()).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    match req.get::<bodyparser::Struct<OriginUpdateReq>>() {
        Ok(Some(body)) => {
            let dpv = match body.default_package_visibility
                .parse::<OriginPackageVisibility>() {
                Ok(x) => x,
                Err(_) => return Ok(Response::with(status::UnprocessableEntity)),
            };
            request.set_default_package_visibility(dpv);
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    }
    match helpers::get_origin(req, request.get_name()) {
        Ok(origin) => request.set_id(origin.get_id()),
        Err(err) => return Ok(render_net_error(&err)),
    }
    match route_message::<OriginUpdate, NetOk>(req, &request) {
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
            if let Some(vis) = body.default_package_visibility {
                match vis.parse::<OriginPackageVisibility>() {
                    Ok(vis) => request.set_default_package_visibility(vis),
                    Err(_) => return Ok(Response::with(status::UnprocessableEntity)),
                }
            }
            request.set_name(body.name);
        }
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    }
    if !ident::is_valid_origin_name(request.get_name()) {
        return Ok(Response::with(status::UnprocessableEntity));
    }

    match route_message::<OriginCreate, Origin>(req, &request) {
        Ok(origin) => Ok(render_json(status::Created, &origin)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn origin_show(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginGet::new();
    match get_param(req, "origin") {
        Some(origin) => request.set_name(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match route_message::<OriginGet, Origin>(req, &request) {
        Ok(origin) => {
            let mut response = render_json(status::Ok, &origin);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn rescind_invitation(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginInvitationRescindRequest::new();
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        request.set_owner_id(session.get_id());
    }
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let invitation = match get_param(req, "invitation_id") {
        Some(invitation) => invitation,
        None => return Ok(Response::with(status::BadRequest)),
    };
    match invitation.parse::<u64>() {
        Ok(invitation_id) => request.set_invitation_id(invitation_id),
        Err(_) => return Ok(Response::with(status::BadRequest)),
    }

    debug!(
        "Rescinding invitation id {} for user {} origin {}",
        request.get_invitation_id(),
        request.get_owner_id(),
        &origin
    );

    match route_message::<OriginInvitationRescindRequest, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn ignore_invitation(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginInvitationIgnoreRequest::new();
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        request.set_account_id(session.get_id());
    }
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let invitation = match get_param(req, "invitation_id") {
        Some(invitation) => invitation,
        None => return Ok(Response::with(status::BadRequest)),
    };
    match invitation.parse::<u64>() {
        Ok(invitation_id) => request.set_invitation_id(invitation_id),
        Err(_) => return Ok(Response::with(status::BadRequest)),
    }

    debug!(
        "Ignoring invitation id {} for user {} origin {}",
        request.get_invitation_id(),
        request.get_account_id(),
        &origin
    );

    match route_message::<OriginInvitationIgnoreRequest, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn accept_invitation(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginInvitationAcceptRequest::new();
    request.set_ignore(false);
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        request.set_account_id(session.get_id());
    }
    match get_param(req, "origin") {
        Some(origin) => request.set_origin_name(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }
    let invitation = match get_param(req, "invitation_id") {
        Some(invitation) => invitation,
        None => return Ok(Response::with(status::BadRequest)),
    };
    match invitation.parse::<u64>() {
        Ok(invitation_id) => request.set_invite_id(invitation_id),
        Err(_) => return Ok(Response::with(status::BadRequest)),
    }

    debug!(
        "Accepting invitation for user {} origin {}",
        &request.get_account_id(),
        request.get_origin_name()
    );

    match route_message::<OriginInvitationAcceptRequest, NetOk>(req, &request) {
        Ok(_) => {
            log_event!(
                req,
                Event::OriginInvitationAccept {
                    id: request.get_invite_id().to_string(),
                    account: request.get_account_id().to_string(),
                }
            );
            Ok(Response::with(status::NoContent))
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn invite_to_origin(req: &mut Request) -> IronResult<Response> {
    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let user_to_invite = match get_param(req, "username") {
        Some(username) => username,
        None => return Ok(Response::with(status::BadRequest)),
    };

    debug!(
        "Creating invitation for user {} origin {}",
        &user_to_invite,
        &origin
    );

    if !check_origin_access(req, &origin).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = AccountGet::new();
    let mut invite_request = OriginInvitationCreate::new();
    request.set_name(user_to_invite.to_string());

    match route_message::<AccountGet, Account>(req, &request) {
        Ok(mut account) => {
            invite_request.set_account_id(account.get_id());
            invite_request.set_account_name(account.take_name());
        }
        Err(err) => return Ok(render_net_error(&err)),
    };

    match helpers::get_origin(req, &origin) {
        Ok(mut origin) => {
            invite_request.set_origin_id(origin.get_id());
            invite_request.set_origin_name(origin.take_name());
        }
        Err(err) => return Ok(render_net_error(&err)),
    }

    invite_request.set_owner_id(session.get_id());

    // store invitations in the originsrv
    match route_message::<OriginInvitationCreate, OriginInvitation>(req, &invite_request) {
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
        Err(err) => {
            if err.get_code() == ErrCode::ENTITY_CONFLICT {
                Ok(Response::with(status::NoContent))
            } else {
                Ok(render_net_error(&err))
            }
        }
    }
}

pub fn list_origin_invitations(req: &mut Request) -> IronResult<Response> {
    let origin_name = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(req, &origin_name).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginInvitationListRequest::new();
    match helpers::get_origin(req, origin_name.as_str()) {
        Ok(origin) => request.set_origin_id(origin.get_id()),
        Err(err) => return Ok(render_net_error(&err)),
    }

    match route_message::<OriginInvitationListRequest, OriginInvitationListResponse>(
        req,
        &request,
    ) {
        Ok(list) => {
            let mut response = render_json(status::Ok, &list);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn list_origin_members(req: &mut Request) -> IronResult<Response> {
    let origin_name = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(req, &origin_name).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginMemberListRequest::new();
    match helpers::get_origin(req, origin_name.as_str()) {
        Ok(origin) => request.set_origin_id(origin.get_id()),
        Err(err) => return Ok(render_net_error(&err)),
    }
    match route_message::<OriginMemberListRequest, OriginMemberListResponse>(req, &request) {
        Ok(list) => {
            let mut response = render_json(status::Ok, &list);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn origin_member_delete(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_owner(req, session.get_id(), &origin).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let account_name = match get_param(req, "username") {
        Some(user) => user,
        None => return Ok(Response::with(status::BadRequest)),
    };

    // Do not allow the owner to be removed which would orphan the origin
    if account_name == session.get_name() {
        return Ok(Response::with(status::BadRequest));
    }

    debug!(
        "Deleting user name {} for user {} origin {}",
        &account_name,
        &session.get_id(),
        &origin
    );

    let mut session_request = AccountOriginRemove::new();
    let mut origin_request = OriginMemberRemove::new();

    match helpers::get_origin(req, origin) {
        Ok(origin) => {
            session_request.set_origin_id(origin.get_id());
            origin_request.set_origin_id(origin.get_id());
        }
        Err(err) => return Ok(render_net_error(&err)),
    }
    session_request.set_account_name(account_name.to_string());
    origin_request.set_account_name(account_name.to_string());

    if let Err(err) = route_message::<AccountOriginRemove, NetOk>(req, &session_request) {
        return Ok(render_net_error(&err));
    }

    match route_message::<OriginMemberRemove, NetOk>(req, &origin_request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
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

fn generate_origin_keys(req: &mut Request) -> IronResult<Response> {
    debug!("Generate Origin Keys {:?}", req);
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    match get_param(req, "origin") {
        Some(origin) => {
            if !check_origin_access(req, &origin).unwrap_or(false) {
                return Ok(Response::with(status::Forbidden));
            }

            match helpers::get_origin(req, origin) {
                Ok(origin) => {
                    match helpers::generate_origin_keys(req, session, origin) {
                        Ok(_) => Ok(Response::with(status::Created)),
                        Err(err) => Ok(render_net_error(&err)),
                    }
                }
                Err(err) => Ok(render_net_error(&err)),
            }
        }
        None => Ok(Response::with(status::BadRequest)),
    }
}

fn upload_origin_key(req: &mut Request) -> IronResult<Response> {
    debug!("Upload Origin Public Key {:?}", req);
    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let mut request = OriginPublicKeyCreate::new();
    request.set_owner_id(session.get_id());

    let origin = match get_param(req, "origin") {
        Some(origin) => {
            if !check_origin_access(req, &origin).unwrap_or(false) {
                return Ok(Response::with(status::Forbidden));
            }

            match helpers::get_origin(req, &origin) {
                Ok(mut origin) => {
                    request.set_name(origin.take_name());
                    request.set_origin_id(origin.get_id());
                }
                Err(err) => return Ok(render_net_error(&err)),
            }
            origin
        }
        None => return Ok(Response::with(status::BadRequest)),
    };

    match get_param(req, "revision") {
        Some(revision) => request.set_revision(revision),
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
    match route_message::<OriginPublicKeyCreate, OriginPublicKey>(req, &request) {
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
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(req, &origin).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut request = OriginSecretKeyGet::new();
    match helpers::get_origin(req, origin) {
        Ok(mut origin) => {
            request.set_owner_id(origin.get_owner_id());
            request.set_origin(origin.take_name());
        }
        Err(err) => return Ok(render_net_error(&err)),
    }
    let key = match route_message::<OriginSecretKeyGet, OriginSecretKey>(req, &request) {
        Ok(key) => key,
        Err(err) => return Ok(render_net_error(&err)),
    };

    let xfilename = format!("{}-{}.sig.key", key.get_name(), key.get_revision());
    download_content_as_file(key.get_body(), xfilename)
}

fn upload_origin_secret_key(req: &mut Request) -> IronResult<Response> {
    debug!("Upload Origin Secret Key {:?}", req);
    // TODO: SA - Eliminate need to clone the session
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let mut request = OriginSecretKeyCreate::new();
    request.set_owner_id(session.get_id());

    let origin = match get_param(req, "origin") {
        Some(origin) => {
            if !check_origin_access(req, &origin).unwrap_or(false) {
                return Ok(Response::with(status::Forbidden));
            }

            match helpers::get_origin(req, &origin) {
                Ok(mut origin) => {
                    request.set_name(origin.take_name());
                    request.set_origin_id(origin.get_id());
                }
                Err(err) => return Ok(render_net_error(&err)),
            }
            origin
        }
        None => return Ok(Response::with(status::BadRequest)),
    };

    match get_param(req, "revision") {
        Some(revision) => request.set_revision(revision),
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
    match route_message::<OriginSecretKeyCreate, OriginSecretKey>(req, &request) {
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
    let ident = ident_from_req(req);
    let session_id = helpers::get_optional_session_id(req);

    if !ident.valid() || !ident.fully_qualified() {
        info!(
            "Invalid or not fully qualified package identifier: {}",
            ident
        );
        return Ok(Response::with(status::BadRequest));
    }

    if !check_origin_access(req, &ident.get_origin()).unwrap_or(false) {
        debug!("Failed origin access check, ident: {}", ident);

        return Ok(Response::with(status::Forbidden));
    }

    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );

    let depot = lock.read().expect("depot read lock is poisoned");
    let checksum_from_param = match helpers::extract_query_value("checksum", req) {
        Some(checksum) => checksum,
        None => return Ok(Response::with(status::BadRequest)),
    };

    debug!(
        "UPLOADING checksum={}, ident={}",
        checksum_from_param,
        ident
    );

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
    ident_req.set_visibilities(visibility_for_optional_session(
        req,
        session_id,
        &ident.get_origin(),
    ));

    // Return conflict only if we have BOTH package metadata and a valid
    // archive on disk.
    let origin_package_found =
        match route_message::<OriginPackageGet, OriginPackage>(req, &ident_req) {
            Ok(_) => true,
            Err(err) => {
                if err.get_code() == ErrCode::ENTITY_NOT_FOUND {
                    false
                } else {
                    return Ok(render_net_error(&err));
                }
            }
        };

    if origin_package_found && depot.archive(&ident, &target_from_artifact).is_some() {
        return Ok(Response::with((status::Conflict)));
    };

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
    let mut pcr_req = JobGraphPackagePreCreate::new();
    pcr_req.set_ident(format!("{}", ident));
    pcr_req.set_target(target_from_artifact.to_string());

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

    match route_message::<JobGraphPackagePreCreate, NetOk>(req, &pcr_req) {
        Ok(_) => (),
        Err(err) => {
            if err.get_code() == ErrCode::ENTITY_CONFLICT {
                warn!(
                    "Failed package circular dependency check: {:?}, err: {:?}",
                    ident,
                    err
                );
                return Ok(Response::with(status::FailedDependency));
            }
            return Ok(render_net_error(&err));
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
        {
            let session = req.extensions.get::<Authenticated>().unwrap();
            package.set_owner_id(session.get_id());
        }

        // let's make sure this origin actually exists
        match helpers::get_origin(req, &ident.get_origin()) {
            Ok(origin) => package.set_origin_id(origin.get_id()),
            Err(err) => return Ok(render_net_error(&err)),
        }

        // Zero this out initially
        package.clear_visibility();

        // First, try to fetch visibility settings from a project, if one exists
        let mut project_get = OriginProjectGet::new();
        let project_name = format!("{}/{}", ident.get_origin(), ident.get_name());
        project_get.set_name(project_name);

        match route_message::<OriginProjectGet, OriginProject>(req, &project_get) {
            Ok(proj) => {
                if proj.has_visibility() {
                    package.set_visibility(proj.get_visibility());
                }
            }
            Err(_) => {
                // There's no project for this package. No worries - we'll check the origin
                let mut origin_get = OriginGet::new();
                origin_get.set_name(ident.get_origin().to_string());

                match route_message::<OriginGet, Origin>(req, &origin_get) {
                    Ok(o) => {
                        if o.has_default_package_visibility() {
                            package.set_visibility(o.get_default_package_visibility());
                        }
                    }
                    Err(err) => return Ok(render_net_error(&err)),
                }
            }
        }

        // If, after checking both the project and the origin, there's still no visibility set
        // (this is highly unlikely), then just make it public.
        if !package.has_visibility() {
            package.set_visibility(OriginPackageVisibility::Public);
        }

        // Don't re-create the origin package if it already exists
        if !origin_package_found {
            if let Err(err) = route_message::<OriginPackageCreate, OriginPackage>(req, &package) {
                return Ok(render_net_error(&err));
            }

            // Schedule re-build of dependent packages (if requested)
            // Don't schedule builds if the upload is being done by the builder
            if depot.config.builds_enabled &&
                (ident.get_origin() == "core" || depot.config.non_core_builds_enabled) &&
                !match helpers::extract_query_value("builder", req) {
                    Some(_) => true,
                    None => false,
                }
            {
                let mut request = JobGroupSpec::new();
                request.set_origin(ident.get_origin().to_string());
                request.set_package(ident.get_name().to_string());
                request.set_target(target_from_artifact.to_string());
                request.set_deps_only(true);
                request.set_origin_only(!depot.config.non_core_builds_enabled);
                request.set_package_only(false);

                match route_message::<JobGroupSpec, JobGroup>(req, &request) {
                    Ok(group) => {
                        debug!(
                            "Scheduled reverse dependecy build for {}, group id: {}, origin_only: {}",
                            ident,
                            group.get_id(),
                            !depot.config.non_core_builds_enabled
                        )
                    }
                    Err(err) => warn!("Unable to schedule build, err: {:?}", err),
                }
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
    let mut request = JobGraphPackageStatsGet::new();
    match get_param(req, "origin") {
        Some(origin) => request.set_origin(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }

    match route_message::<JobGraphPackageStatsGet, JobGraphPackageStats>(req, &request) {
        Ok(stats) => {
            let mut response = render_json(status::Ok, &stats);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn schedule(req: &mut Request) -> IronResult<Response> {
    let session = req.extensions.get::<Authenticated>().unwrap().clone();
    let segment = req.get::<persistent::Read<SegmentCli>>().unwrap();
    let origin_name = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    {
        let lock = req.get::<persistent::State<DepotUtil>>().unwrap();
        let depot = lock.read().unwrap();
        if !depot.config.builds_enabled ||
            (origin_name != "core" && !depot.config.non_core_builds_enabled)
        {
            return Ok(Response::with(status::Forbidden));
        }
    }
    let package = match get_param(req, "pkg") {
        Some(pkg) => pkg,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let target = match helpers::extract_query_value("target", req) {
        Some(target) => target,
        None => String::from("x86_64-linux"),
    };
    let deps_only = helpers::extract_query_value("deps_only", req).is_some();
    let origin_only = helpers::extract_query_value("origin_only", req).is_some();
    let package_only = helpers::extract_query_value("package_only", req).is_some();

    // We only support building for Linux x64 only currently
    if target != "x86_64-linux" {
        info!("Rejecting build with target: {}", target);
        return Ok(Response::with(status::BadRequest));
    }

    let mut secret_key_request = OriginSecretKeyGet::new();
    let origin = match helpers::get_origin(req, &origin_name) {
        Ok(origin) => {
            secret_key_request.set_owner_id(origin.get_owner_id());
            secret_key_request.set_origin(origin_name.clone());
            origin
        }
        Err(err) => return Ok(render_net_error(&err)),
    };
    let account_name = session.get_name().to_string();
    let need_keys =
        match route_message::<OriginSecretKeyGet, OriginSecretKey>(req, &secret_key_request) {
            Ok(key) => {
                let mut pub_key_request = OriginPublicKeyGet::new();
                pub_key_request.set_origin(origin_name.clone());
                pub_key_request.set_revision(key.get_revision().to_string());
                route_message::<OriginPublicKeyGet, OriginPublicKey>(req, &pub_key_request).is_err()
            }
            Err(_) => true,
        };

    if need_keys {
        if let Err(err) = helpers::generate_origin_keys(req, session, origin) {
            return Ok(render_net_error(&err));
        }
    }

    let mut request = JobGroupSpec::new();
    request.set_origin(origin_name);
    request.set_package(package);
    request.set_target(target);
    request.set_deps_only(deps_only);
    request.set_origin_only(origin_only);
    request.set_package_only(package_only);

    match route_message::<JobGroupSpec, JobGroup>(req, &request) {
        Ok(group) => {
            let msg = format!("Scheduled job group for {}", group.get_project_name());

            // We don't really want to abort anything just because a call to segment failed. Let's
            // just log it and move on.
            if let Err(e) = segment.track(&account_name, &msg) {
                warn!("Error tracking scheduling of job group in segment, {}", e);
            }

            let mut response = render_json(status::Ok, &group);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn get_origin_schedule_status(req: &mut Request) -> IronResult<Response> {
    let mut request = JobGroupOriginGet::new();

    match get_param(req, "origin") {
        Some(origin) => request.set_origin(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }

    match route_message::<JobGroupOriginGet, JobGroupOriginResponse>(req, &request) {
        Ok(jgor) => Ok(render_json(status::Ok, &jgor.get_job_groups())),
        Err(e) => Ok(render_net_error(&e)),
    }
}

fn get_schedule(req: &mut Request) -> IronResult<Response> {
    let group_id = {
        let group_id_str = match get_param(req, "groupid") {
            Some(s) => s,
            None => return Ok(Response::with(status::BadRequest)),
        };

        match group_id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let mut request = JobGroupGet::new();
    request.set_group_id(group_id);

    match route_message::<JobGroupGet, JobGroup>(req, &request) {
        Ok(group) => {
            let mut response = render_json(status::Ok, &group);
            dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

// TODO (SA) This is an experiemental dev-only function for now
fn abort_schedule(req: &mut Request) -> IronResult<Response> {
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

    let mut request = JobGroupAbort::new();
    request.set_group_id(group_id);

    match route_message::<JobGroupAbort, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::Ok)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

// This function should not require authentication (session/auth token)
fn download_origin_key(req: &mut Request) -> IronResult<Response> {
    let mut request = OriginPublicKeyGet::new();
    match get_param(req, "origin") {
        Some(origin) => request.set_origin(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "revision") {
        Some(revision) => request.set_revision(revision),
        None => return Ok(Response::with(status::BadRequest)),
    }
    let key = match route_message::<OriginPublicKeyGet, OriginPublicKey>(req, &request) {
        Ok(key) => key,
        Err(err) => return Ok(render_net_error(&err)),
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
    let mut request = OriginPublicKeyLatestGet::new();
    match get_param(req, "origin") {
        Some(origin) => request.set_origin(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }
    let key = match route_message::<OriginPublicKeyLatestGet, OriginPublicKey>(req, &request) {
        Ok(key) => key,
        Err(err) => return Ok(render_net_error(&err)),
    };

    let xfilename = format!("{}-{}.pub", key.get_name(), key.get_revision());
    download_content_as_file(key.get_body(), xfilename)
}

fn package_channels(req: &mut Request) -> IronResult<Response> {
    let session_id = helpers::get_optional_session_id(req);
    let mut request = OriginPackageChannelListRequest::new();
    let ident = ident_from_req(req);

    if !ident.fully_qualified() {
        return Ok(Response::with(status::BadRequest));
    }

    request.set_visibilities(visibility_for_optional_session(
        req,
        session_id,
        &ident.get_origin(),
    ));
    request.set_ident(ident);

    match route_message::<OriginPackageChannelListRequest, OriginPackageChannelListResponse>(
        req,
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
    let session_id = helpers::get_optional_session_id(req);
    let mut ident_req = OriginPackageGet::new();
    let ident = ident_from_req(req);
    let mut vis = visibility_for_optional_session(req, session_id, &ident.get_origin());
    vis.push(OriginPackageVisibility::Hidden);
    ident_req.set_visibilities(vis);
    ident_req.set_ident(ident);

    let agent_target = target_from_headers(&req.headers.get::<UserAgent>().unwrap()).unwrap();
    if !depot.config.targets.contains(&agent_target) {
        return Ok(Response::with((
            status::NotImplemented,
            "Unsupported client platform ({}).",
        )));
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
                error!("Inconsistentcy between metadata and filesystem!");
                Ok(Response::with(status::InternalServerError))
            }
        }
        Err(err) => return Ok(render_net_error(&err)),
    }
}

fn list_origin_keys(req: &mut Request) -> IronResult<Response> {
    let origin_name = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut request = OriginPublicKeyListRequest::new();
    match helpers::get_origin(req, &origin_name) {
        Ok(origin) => request.set_origin_id(origin.get_id()),
        Err(err) => return Ok(render_net_error(&err)),
    }
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
    let session_id = helpers::get_optional_session_id(req);
    let mut request = OriginPackageUniqueListRequest::new();
    let (start, stop) = match helpers::extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };

    request.set_start(start as u64);
    request.set_stop(stop as u64);

    match get_param(req, "origin") {
        Some(origin) => {
            request.set_visibilities(visibility_for_optional_session(req, session_id, &origin));
            request.set_origin(origin);
        }
        None => return Ok(Response::with(status::BadRequest)),
    }

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
            let body = helpers::package_results_json(
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
        Err(err) => return Ok(render_net_error(&err)),
    }
}

fn list_package_versions(req: &mut Request) -> IronResult<Response> {
    let session_id = helpers::get_optional_session_id(req);
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let name = match get_param(req, "pkg") {
        Some(pkg) => pkg,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut request = OriginPackageVersionListRequest::new();
    request.set_visibilities(visibility_for_optional_session(req, session_id, &origin));
    request.set_origin(origin);
    request.set_name(name);

    match route_message::<OriginPackageVersionListRequest, OriginPackageVersionListResponse>(
        req,
        &request,
    ) {
        Ok(packages) => {
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
        Err(err) => return Ok(render_net_error(&err)),
    }
}

fn package_privacy_toggle(req: &mut Request) -> IronResult<Response> {
    let origin = match get_param(req, "origin") {
        Some(o) => o,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let visibility = match get_param(req, "visibility") {
        Some(v) => v,
        None => return Ok(Response::with(status::BadRequest)),
    };

    // users aren't allowed to set packages to hidden manually
    if visibility.to_lowercase() == "hidden" {
        return Ok(Response::with(status::BadRequest));
    }

    let ident = ident_from_req(req);

    if !ident.valid() || !ident.fully_qualified() {
        info!(
            "Invalid or not fully qualified package identifier: {}",
            ident
        );
        return Ok(Response::with(status::BadRequest));
    }

    let opv: OriginPackageVisibility = match visibility.parse() {
        Ok(o) => o,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };

    if !check_origin_access(req, &origin).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut opg = OriginPackageGet::new();
    opg.set_ident(ident);
    opg.set_visibilities(all_visibilities());

    match route_message::<OriginPackageGet, OriginPackage>(req, &opg) {
        Ok(mut package) => {
            let real_visibility = transition_visibility(opv, package.get_visibility());
            let mut opu = OriginPackageUpdate::new();
            package.set_visibility(real_visibility);
            opu.set_pkg(package);

            match route_message::<OriginPackageUpdate, NetOk>(req, &opu) {
                Ok(_) => Ok(Response::with(status::Ok)),
                Err(e) => Ok(render_net_error(&e)),
            }
        }
        Err(e) => Ok(render_net_error(&e)),
    }
}

fn list_packages(req: &mut Request) -> IronResult<Response> {
    let session_id = helpers::get_optional_session_id(req);
    let mut distinct = false;
    let (start, stop) = match helpers::extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };

    let (origin, ident, channel) = {
        let params = req.extensions.get::<Router>().unwrap();

        let origin = match params.find("origin") {
            Some(origin) => origin.to_string(),
            None => return Ok(Response::with(status::BadRequest)),
        };

        let ident: String = if params.find("pkg").is_none() {
            origin.clone()
        } else {
            ident_from_params(&params).to_string()
        };

        let channel = match params.find("channel") {
            Some(ch) => Some(ch.to_string()),
            None => None,
        };

        (origin, ident, channel)
    };

    let packages: NetResult<OriginPackageListResponse>;
    match channel {
        Some(channel) => {
            let mut request = OriginChannelPackageListRequest::new();
            request.set_name(channel);
            request.set_start(start as u64);
            request.set_stop(stop as u64);
            request.set_visibilities(visibility_for_optional_session(req, session_id, &origin));

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
            request.set_visibilities(visibility_for_optional_session(req, session_id, &origin));

            // only set this if "distinct" is present as a URL parameter, e.g. ?distinct=true
            if helpers::extract_query_value("distinct", req).is_some() {
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
                    channels = helpers::channels_for_package_ident(req, &package);
                    platforms = helpers::platforms_for_package_ident(req, &package);
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

            let body = helpers::package_results_json(
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
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn list_channels(req: &mut Request) -> IronResult<Response> {
    let origin_name = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut request = OriginChannelListRequest::new();
    request.set_include_sandbox_channels(false);

    match helpers::get_origin(req, &origin_name) {
        Ok(origin) => request.set_origin_id(origin.get_id()),
        Err(err) => return Ok(render_net_error(&err)),
    }

    // Pass ?sandbox=true to this endpoint to include sanbox channels in the list. They are not
    // there by default.
    if let Some(sandbox) = helpers::extract_query_value("sandbox", req) {
        if sandbox == "true" {
            request.set_include_sandbox_channels(true);
        }
    }

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
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let channel = match get_param(req, "channel") {
        Some(channel) => channel,
        None => return Ok(Response::with(status::BadRequest)),
    };

    match helpers::create_channel(req, &origin, &channel) {
        Ok(origin_channel) => Ok(render_json(status::Created, &origin_channel)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn delete_channel(req: &mut Request) -> IronResult<Response> {
    let origin = match get_param(req, "origin") {
        Some(origin) => origin,
        None => return Ok(Response::with(status::BadRequest)),
    };
    let channel = match get_param(req, "channel") {
        Some(channel) => channel,
        None => return Ok(Response::with(status::BadRequest)),
    };

    let mut channel_req = OriginChannelGet::new();
    channel_req.set_origin_name(origin.clone());
    channel_req.set_name(channel.clone());
    match route_message::<OriginChannelGet, OriginChannel>(req, &channel_req) {
        Ok(origin_channel) => {
            // make sure the person trying to create the channel has access to do so
            if !check_origin_access(req, &origin).unwrap_or(false) {
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
                Err(err) => return Ok(render_net_error(&err)),
            }
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn show_package(req: &mut Request) -> IronResult<Response> {
    let session_id = helpers::get_optional_session_id(req);
    let channel = get_param(req, "channel");

    let mut ident = ident_from_req(req);
    let qualified = ident.fully_qualified();

    if let Some(channel) = channel {
        if !qualified {
            let target = target_from_headers(&req.headers.get::<UserAgent>().unwrap())
                .unwrap()
                .to_string();
            let mut request = OriginChannelPackageLatestGet::new();
            request.set_name(channel.clone());
            request.set_target(target);
            request.set_visibilities(visibility_for_optional_session(
                req,
                session_id,
                &ident.get_origin(),
            ));
            request.set_ident(ident);

            match route_message::<OriginChannelPackageLatestGet, OriginPackageIdent>(
                req,
                &request,
            ) {
                Ok(id) => ident = id.into(),
                Err(err) => return Ok(render_net_error(&err)),
            }
        }

        let mut request = OriginChannelPackageGet::new();
        request.set_name(channel);
        request.set_visibilities(visibility_for_optional_session(
            req,
            session_id,
            &ident.get_origin(),
        ));
        request.set_ident(ident);

        match route_message::<OriginChannelPackageGet, OriginPackage>(req, &request) {
            Ok(pkg) => render_package(req, &pkg, false),
            Err(err) => Ok(render_net_error(&err)),
        }
    } else {
        if !qualified {
            let target = target_from_headers(&req.headers.get::<UserAgent>().unwrap())
                .unwrap()
                .to_string();
            let mut request = OriginPackageLatestGet::new();
            request.set_target(target);
            request.set_visibilities(visibility_for_optional_session(
                req,
                session_id,
                &ident.get_origin(),
            ));
            request.set_ident(ident);

            match route_message::<OriginPackageLatestGet, OriginPackageIdent>(req, &request) {
                Ok(id) => ident = id.into(),
                Err(err) => return Ok(render_net_error(&err)),
            }
        }

        let mut request = OriginPackageGet::new();
        request.set_visibilities(visibility_for_optional_session(
            req,
            session_id,
            &ident.get_origin(),
        ));
        request.set_ident(ident.clone());

        match route_message::<OriginPackageGet, OriginPackage>(req, &request) {
            Ok(pkg) => {
                let lock = req.get::<persistent::State<DepotUtil>>().expect(
                    "depot not found",
                );

                let depot = lock.read().expect("depot read lock is poisoned");

                // If we don't have a valid archive on disk, return NotFound
                let target = target_from_headers(&req.headers.get::<UserAgent>().unwrap()).unwrap();

                if !depot.archive(&ident, &target).is_some() {
                    return Ok(Response::with((status::NotFound)));
                };

                // If the request was for a fully qualified ident, cache the response, otherwise do
                // not cache
                if qualified {
                    render_package(req, &pkg, true)
                } else {
                    render_package(req, &pkg, false)
                }
            }
            Err(err) => Ok(render_net_error(&err)),
        }
    }
}

fn search_packages(req: &mut Request) -> IronResult<Response> {
    let session_id = helpers::get_optional_session_id(req);
    let mut request = OriginPackageSearchRequest::new();
    let (start, stop) = match helpers::extract_pagination(req) {
        Ok(range) => range,
        Err(response) => return Ok(response),
    };
    request.set_start(start as u64);
    request.set_stop(stop as u64);

    if session_id.is_some() {
        let mut my_origins = MyOriginsRequest::new();
        my_origins.set_account_id(session_id.unwrap());

        match route_message::<MyOriginsRequest, MyOriginsResponse>(req, &my_origins) {
            Ok(response) => {
                request.set_my_origins(protobuf::RepeatedField::from_vec(
                    response.get_origins().to_vec(),
                ))
            }
            Err(e) => {
                debug!(
                    "Error fetching origins for account id {}, {}",
                    session_id.unwrap(),
                    e
                );
                return Ok(Response::with(status::BadRequest));
            }
        }
    }

    // First, try to parse the query like it's a PackageIdent, since it seems reasonable to expect
    // that many people will try searching using that kind of string, e.g. core/redis.  If that
    // works, set the origin appropriately and do a regular search.  If that doesn't work, do a
    // search across all origins, similar to how the "distinct" search works now, but returning all
    // the details instead of just names.
    let query = match get_param(req, "query") {
        Some(q) => q,
        None => return Ok(Response::with(status::BadRequest)),
    };

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

    debug!("search_packages called with: {}", request.get_query());

    // Setting distinct to true makes this query ignore any origin set, because it's going to
    // search both the origin name and the package name for the query string provided. This is
    // likely sub-optimal for performance but it makes things work right now and we should probably
    // switch to some kind of full-text search engine in the future anyway.
    // Also, to get this behavior, you need to ensure that "distinct" is a URL parameter in your
    // request, e.g. blah?distinct=true
    if helpers::extract_query_value("distinct", req).is_some() {
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
            let body = helpers::package_results_json(
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
        Err(err) => return Ok(render_net_error(&err)),
    }
}

fn render_package(
    req: &mut Request,
    pkg: &OriginPackage,
    should_cache: bool,
) -> IronResult<Response> {
    let mut pkg_json = serde_json::to_value(pkg.clone()).unwrap();
    let channels = helpers::channels_for_package_ident(req, pkg.get_ident());
    pkg_json["channels"] = json!(channels);
    pkg_json["is_a_service"] = json!(is_a_service(req, pkg.get_ident()));

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
    let mut ident = OriginPackageIdent::new();
    match get_param(req, "origin") {
        Some(origin) => ident.set_origin(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "pkg") {
        Some(pkg) => ident.set_name(pkg),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "version") {
        Some(version) => ident.set_version(version),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "release") {
        Some(release) => ident.set_release(release),
        None => return Ok(Response::with(status::BadRequest)),
    }
    let channel = match get_param(req, "channel") {
        Some(channel) => channel,
        None => return Ok(Response::with(status::BadRequest)),
    };

    match helpers::promote_package_to_channel(req, &ident, &channel) {
        Ok(_) => Ok(Response::with(status::Ok)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

fn demote_package(req: &mut Request) -> IronResult<Response> {
    let mut ident = OriginPackageIdent::new();
    match get_param(req, "origin") {
        Some(origin) => ident.set_origin(origin),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "pkg") {
        Some(pkg) => ident.set_name(pkg),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "version") {
        Some(version) => ident.set_version(version),
        None => return Ok(Response::with(status::BadRequest)),
    }
    match get_param(req, "release") {
        Some(release) => ident.set_release(release),
        None => return Ok(Response::with(status::BadRequest)),
    }
    let channel = match get_param(req, "channel") {
        Some(channel) => channel,
        None => return Ok(Response::with(status::BadRequest)),
    };

    // you can't demote from "unstable"
    if channel == "unstable" {
        return Ok(Response::with(status::Forbidden));
    }

    if !check_origin_access(req, &ident.get_origin()).unwrap_or(false) {
        return Ok(Response::with(status::Forbidden));
    }

    let mut channel_req = OriginChannelGet::new();
    channel_req.set_origin_name(ident.get_origin().to_string());
    channel_req.set_name(channel);
    match route_message::<OriginChannelGet, OriginChannel>(req, &channel_req) {
        Ok(origin_channel) => {
            let mut request = OriginPackageGet::new();
            request.set_ident(ident.clone());
            request.set_visibilities(all_visibilities());
            match route_message::<OriginPackageGet, OriginPackage>(req, &request) {
                Ok(package) => {
                    let mut demote = OriginPackageDemote::new();
                    demote.set_channel_id(origin_channel.get_id());
                    demote.set_package_id(package.get_id());
                    demote.set_ident(ident);
                    match route_message::<OriginPackageDemote, NetOk>(req, &demote) {
                        Ok(_) => Ok(Response::with(status::Ok)),
                        Err(err) => return Ok(render_net_error(&err)),
                    }
                }
                Err(err) => Ok(render_net_error(&err)),
            }
        }
        Err(err) => Ok(render_net_error(&err)),
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
        bldr_core::keys::BUILDER_KEY_NAME,
        &depot.config.key_dir,
    ) {
        Ok(p) => p,
        Err(_) => return Ok(Response::with((status::NotFound, "key-pair"))),
    };

    let key = match kp.public() {
        Ok(k) => k,
        Err(_) => return Ok(Response::with((status::NotFound, "public-key"))),
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

fn ident_from_req(req: &mut Request) -> OriginPackageIdent {
    let params = req.extensions.get::<Router>().unwrap();
    ident_from_params(&params)
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

fn download_content_as_file(content: &[u8], filename: String) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, content));
    response.headers.set(ContentDisposition(
        format!("attachment; filename=\"{}\"", filename),
    ));
    response.headers.set(XFileName(filename));
    dont_cache_response(&mut response);
    Ok(response)
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

fn is_a_service<T>(req: &mut Request, ident: &T) -> bool
where
    T: Identifiable,
{
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");
    let agent_target = target_from_headers(&req.headers.get::<UserAgent>().unwrap()).unwrap();

    match depot.archive(ident, &agent_target) {
        Some(mut archive) => archive.is_a_service(),
        None => false,
    }
}

fn do_cache_response(response: &mut Response) {
    response.headers.set(CacheControl(
        format!("public, max-age={}", ONE_YEAR_IN_SECS),
    ));
}

pub fn routes<M>(basic: Authenticated, worker: M) -> Router
where
    M: BeforeMiddleware + Clone,
{
    let opt = basic.clone().optional();

    router!(
        channels: get "/channels/:origin" => list_channels,
        channel_packages: get "/channels/:origin/:channel/pkgs" => {
            XHandler::new(list_packages).before(opt.clone())
        },
        channel_packages_pkg: get "/channels/:origin/:channel/pkgs/:pkg" => {
            XHandler::new(list_packages).before(opt.clone())
        },
        channel_package_latest: get "/channels/:origin/:channel/pkgs/:pkg/latest" => {
            XHandler::new(show_package).before(opt.clone())
        },
        channel_packages_version: get
        "/channels/:origin/:channel/pkgs/:pkg/:version" => {
            XHandler::new(list_packages).before(opt.clone())
        },
        channel_packages_version_latest: get
        "/channels/:origin/:channel/pkgs/:pkg/:version/latest" => {
            XHandler::new(show_package).before(opt.clone())
        },
        channel_package_release: get
        "/channels/:origin/:channel/pkgs/:pkg/:version/:release" => {
            XHandler::new(show_package).before(opt.clone())
        },
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
        package_search: get "/pkgs/search/:query" => {
            XHandler::new(search_packages).before(opt.clone())
        },
        packages: get "/pkgs/:origin" => {
            XHandler::new(list_packages).before(opt.clone())
        },
        packages_unique: get "/:origin/pkgs" => {
            XHandler::new(list_unique_packages).before(opt.clone())
        },
        packages_pkg: get "/pkgs/:origin/:pkg" => {
            XHandler::new(list_packages).before(opt.clone())
        },
        package_pkg_versions: get "/pkgs/:origin/:pkg/versions" => {
            XHandler::new(list_package_versions).before(opt.clone())
        },
        package_pkg_latest: get "/pkgs/:origin/:pkg/latest" => {
            XHandler::new(show_package).before(opt.clone())
        },
        packages_version: get "/pkgs/:origin/:pkg/:version" => {
            XHandler::new(list_packages).before(opt.clone())
        },
        package_version_latest: get "/pkgs/:origin/:pkg/:version/latest" => {
            XHandler::new(show_package).before(opt.clone())
        },
        package: get "/pkgs/:origin/:pkg/:version/:release" => {
            XHandler::new(show_package).before(opt.clone())
        },
        package_channels: get "/pkgs/:origin/:pkg/:version/:release/channels" => {
            XHandler::new(package_channels).before(opt.clone())
        },
        package_download: get "/pkgs/:origin/:pkg/:version/:release/download" => {
            XHandler::new(download_package).before(opt.clone())
        },
        package_upload: post "/pkgs/:origin/:pkg/:version/:release" => {
            XHandler::new(upload_package).before(basic.clone())
        },
        package_privacy_toggle: patch "/pkgs/:origin/:pkg/:version/:release/:visibility" => {
            XHandler::new(package_privacy_toggle).before(basic.clone())
        },
        packages_stats: get "/pkgs/origins/:origin/stats" => package_stats,
        schedule: post "/pkgs/schedule/:origin/:pkg" => {
            XHandler::new(schedule).before(basic.clone())
        },
        schedule_get: get "/pkgs/schedule/:groupid" => get_schedule,
        schedule_get_global: get "/pkgs/schedule/:origin/status" => get_origin_schedule_status,
        schedule_abort: delete "/pkgs/schedule/:groupid" => {
            XHandler::new(abort_schedule).before(worker.clone())
        },
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
        origin_key_generate: post "/origins/:origin/keys" => {
            XHandler::new(generate_origin_keys).before(basic.clone())
        },
        origin_key_create: post "/origins/:origin/keys/:revision" => {
            XHandler::new(upload_origin_key).before(basic.clone())
        },
        origin_secret_key_create: post "/origins/:origin/secret_keys/:revision" => {
            XHandler::new(upload_origin_secret_key).before(basic.clone())
        },
        origin_secret_key_latest: get "/origins/:origin/secret_keys/latest" => {
            XHandler::new(download_latest_origin_secret_key).before(basic.clone())
        },

        builder_key_latest: get "/builder/keys/latest" => download_latest_builder_key,

        origin_integration_get_names: get "/origins/:origin/integrations/:integration/names" => {
            XHandler::new(
                handlers::integrations::fetch_origin_integration_names).before(basic.clone()
            )
        },
        origin_integration_put: put "/origins/:origin/integrations/:integration/:name" => {
            XHandler::new(handlers::integrations::create_origin_integration).before(basic.clone())
        },
        origin_integration_delete: delete "/origins/:origin/integrations/:integration/:name" => {
            XHandler::new(handlers::integrations::delete_origin_integration).before(basic.clone())
        },
        origin_integrations: get "/origins/:origin/integrations" => {
            XHandler::new(
                handlers::integrations::fetch_origin_integrations).before(basic.clone()
            )
        },
        origin_invitation_create: post "/origins/:origin/users/:username/invitations" => {
            XHandler::new(invite_to_origin).before(basic.clone())
        },
        origin_invitation_accept: put "/origins/:origin/invitations/:invitation_id" => {
            XHandler::new(accept_invitation).before(basic.clone())
        },
        origin_invitation_ignore: put "/origins/:origin/invitations/:invitation_id/ignore" => {
            XHandler::new(ignore_invitation).before(basic.clone())
        },
        origin_invitation_rescind: delete "/origins/:origin/invitations/:invitation_id" => {
            XHandler::new(rescind_invitation).before(basic.clone())
        },
        origin_invitations: get "/origins/:origin/invitations" => {
            XHandler::new(list_origin_invitations).before(basic.clone())
        },
        origin_users: get "/origins/:origin/users" => {
            XHandler::new(list_origin_members).before(basic.clone())
        },
        origin_member_delete: delete "/origins/:origin/users/:username" => {
            XHandler::new(origin_member_delete).before(basic.clone())
        },
    )
}

pub fn router(depot: DepotUtil) -> Result<Chain> {
    let basic = Authenticated::new(depot.config.github.clone());
    let worker = Authenticated::new(depot.config.github.clone()).require(privilege::BUILD_WORKER);
    let router = routes(basic, worker);
    let mut chain = Chain::new(router);
    chain.link(persistent::Read::<EventLog>::both(EventLogger::new(
        &depot.config.log_dir,
        depot.config.events_enabled,
    )));
    chain.link(persistent::Read::<GitHubCli>::both(
        GitHubClient::new(depot.config.github.clone()),
    ));
    chain.link(persistent::Read::<SegmentCli>::both(
        SegmentClient::new(depot.config.segment.clone()),
    ));
    chain.link(persistent::State::<DepotUtil>::both(depot));
    chain.link_before(XRouteClient);
    chain.link_after(Cors);
    Ok(chain)
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        IronError {
            error: Box::new(err),
            response: Response::with((status::InternalServerError, "Internal Habitat error")),
        }
    }
}
