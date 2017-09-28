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

use std::collections::HashMap;
use std::str::FromStr;

use core::channel::{STABLE_CHANNEL, UNSTABLE_CHANNEL};
use hab_net::{ErrCode, NetError, NetOk, NetResult};
use hab_net::privilege::{self, FeatureFlags};
use http::controller::*;

use iron::status::{self, Status};
use protocol::originsrv::{CheckOriginAccessRequest, CheckOriginAccessResponse, Origin,
                          OriginChannel, OriginChannelCreate, OriginChannelGet, OriginGet,
                          OriginPackage, OriginPackageChannelListRequest,
                          OriginPackageChannelListResponse, OriginPackageGet,
                          OriginPackageGroupPromote, OriginPackageIdent,
                          OriginPackagePlatformListRequest, OriginPackagePlatformListResponse,
                          OriginPackagePromote, OriginPackageGroupPromoteResponse};
use protocol::scheduler::{Group, GroupGet, Project, ProjectState};
use protocol::sessionsrv::Session;
use serde::Serialize;
use serde_json;
use urlencoded::UrlEncodedQuery;
use protobuf::RepeatedField;

use router::Router;
use super::controller::route_message;

// Builder services (eg, scheduler or build worker) can call APIs without a
// login session. We need a way to identify that there is no session.
// TODO (SA): Push this down the stack, origin calls should support no session
const NO_SESSION: u64 = 0;

pub fn builder_session_id() -> u64 {
    return NO_SESSION;
}

pub fn get_authenticated_session(req: &mut Request) -> Option<Session> {
    let session = req.extensions.get::<Authenticated>().unwrap();
    let flags = FeatureFlags::from_bits(session.get_flags()).unwrap();
    if flags.contains(privilege::BUILD_WORKER) {
        None
    } else {
        Some(session.to_owned())
    }
}

pub fn validate_params(
    req: &mut Request,
    expected_params: &[&str],
) -> Result<HashMap<String, String>, Status> {
    let mut res = HashMap::new();
    // Get the expected params
    {
        let params = req.extensions.get::<Router>().unwrap();

        if expected_params.iter().any(|p| params.find(p).is_none()) {
            return Err(status::BadRequest);
        }

        for p in expected_params {
            res.insert(p.to_string(), params.find(p).unwrap().to_string());
        }
    }
    // Check that we have origin access
    {
        let session_id = {
            req.extensions.get::<Authenticated>().unwrap().get_id()
        };
        if check_origin_access(req, session_id, &res["origin"]).is_err() {
            debug!(
                "Failed origin access check, session: {}, origin: {}",
                session_id,
                &res["origin"]
            );
            return Err(status::Forbidden);
        }
    }
    Ok(res)
}

const PAGINATION_RANGE_DEFAULT: isize = 0;
const PAGINATION_RANGE_MAX: isize = 50;

#[derive(Serialize)]
struct PaginatedResults<'a, T: 'a> {
    range_start: isize,
    range_end: isize,
    total_count: isize,
    data: &'a Vec<T>,
}

pub fn paginated_response<T>(
    body: &Vec<T>,
    count: isize,
    start: isize,
    end: isize,
) -> IronResult<Response>
where
    T: Serialize,
{
    let body = package_results_json(body, count, start, end);

    if count > end + 1 {
        Ok(Response::with((status::PartialContent, body)))
    } else {
        Ok(Response::with((status::Ok, body)))
    }
}

pub fn package_results_json<T: Serialize>(
    packages: &Vec<T>,
    count: isize,
    start: isize,
    end: isize,
) -> String {
    let results = PaginatedResults {
        range_start: start,
        range_end: end,
        total_count: count,
        data: packages,
    };

    serde_json::to_string(&results).unwrap()
}

// Returns a tuple representing the from and to values representing a paginated set.
// The range (start, stop) values are zero-based.
pub fn extract_pagination(req: &mut Request) -> Result<(isize, isize), Response> {
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

pub fn extract_query_value(key: &str, req: &mut Request) -> Option<String> {
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

// Get channels for a package
pub fn channels_for_package_ident(
    req: &mut Request,
    package: &OriginPackageIdent,
    session_id: Option<u64>,
) -> Option<Vec<String>> {
    let mut opclr = OriginPackageChannelListRequest::new();
    opclr.set_ident(package.clone());
    if session_id.is_some() {
        opclr.set_account_id(session_id.unwrap());
    }

    match route_message::<OriginPackageChannelListRequest, OriginPackageChannelListResponse>(
        req,
        &opclr,
    ) {
        Ok(channels) => {
            let list: Vec<String> = channels
                .get_channels()
                .iter()
                .map(|channel| channel.get_name().to_string())
                .collect();

            Some(list)
        }
        Err(_) => None,
    }
}

// Get platforms for a package
pub fn platforms_for_package_ident(
    req: &mut Request,
    package: &OriginPackageIdent,
    session_id: Option<u64>,
) -> Option<Vec<String>> {
    let mut opplr = OriginPackagePlatformListRequest::new();
    opplr.set_ident(package.clone());
    if session_id.is_some() {
        opplr.set_account_id(session_id.unwrap());
    }

    match route_message::<OriginPackagePlatformListRequest, OriginPackagePlatformListResponse>(
        req,
        &opplr,
    ) {
        Ok(p) => Some(p.get_platforms().to_vec()),
        Err(_) => None,
    }
}

pub fn get_origin<T>(req: &mut Request, origin: T) -> NetResult<Origin>
where
    T: ToString,
{
    let mut request = OriginGet::new();
    request.set_name(origin.to_string());
    route_message::<OriginGet, Origin>(req, &request)
}

pub fn check_origin_access<T>(req: &mut Request, account_id: u64, origin: T) -> NetResult<bool>
where
    T: ToString,
{
    let mut request = CheckOriginAccessRequest::new();
    request.set_account_id(account_id);
    request.set_origin_name(origin.to_string());
    match route_message::<CheckOriginAccessRequest, CheckOriginAccessResponse>(req, &request) {
        Ok(response) => Ok(response.get_has_access()),
        Err(err) => Err(err),
    }
}

pub fn create_channel(
    req: &mut Request,
    origin: &str,
    channel: &str,
    session_id: Option<u64>,
) -> NetResult<OriginChannel> {
    let mut origin = get_origin(req, origin)?;
    let mut request = OriginChannelCreate::new();
    request.set_owner_id(session_id.unwrap_or(NO_SESSION));
    request.set_origin_name(origin.take_name());
    request.set_origin_id(origin.get_id());
    request.set_name(channel.to_string());
    route_message::<OriginChannelCreate, OriginChannel>(req, &request)
}

pub fn promote_package_to_channel(
    req: &mut Request,
    ident: &OriginPackageIdent,
    channel: &str,
    session_id_opt: Option<u64>,
) -> NetResult<NetOk> {
    if let Some(session_id) = session_id_opt {
        if !check_origin_access(req, session_id, ident.get_origin())? {
            return Err(NetError::new(
                ErrCode::ACCESS_DENIED,
                "core:promote-package-to-channel:0",
            ));
        }
    }
    let mut channel_req = OriginChannelGet::new();
    channel_req.set_origin_name(ident.get_origin().to_string());
    channel_req.set_name(channel.to_string());

    let origin_channel = route_message::<OriginChannelGet, OriginChannel>(req, &channel_req)?;
    let mut request = OriginPackageGet::new();
    request.set_ident(ident.clone());
    if let Some(session_id) = session_id_opt {
        request.set_account_id(session_id);
    }
    let package = route_message::<OriginPackageGet, OriginPackage>(req, &request)?;
    let mut promote = OriginPackagePromote::new();
    promote.set_channel_id(origin_channel.get_id());
    promote.set_package_id(package.get_id());
    promote.set_ident(ident.clone());
    route_message::<OriginPackagePromote, NetOk>(req, &promote)
}

pub fn promote_job_group_to_channel(
    req: &mut Request,
    group_id: u64,
    channel: &str,
    session_id: Option<u64>,
) -> NetResult<OriginPackageGroupPromoteResponse> {
    let mut group_get = GroupGet::new();
    group_get.set_group_id(group_id);
    let group = route_message::<GroupGet, Group>(req, &group_get)?;

    // This only makes sense if the group is complete. If the group isn't complete, return now and
    // let the user know. Check the completion state by checking the individual project states,
    // as if this is called by the scheduler it needs to promote the group before marking it
    // Complete.
    if group.get_projects().iter().any(|&ref p| {
        p.get_state() == ProjectState::NotStarted || p.get_state() == ProjectState::InProgress
    })
    {
        return Err(NetError::new(
            ErrCode::GROUP_NOT_COMPLETE,
            "hg:promote-job-group:0",
        ));
    }

    let mut opgpr = OriginPackageGroupPromoteResponse::new();
    let mut not_promoted = RepeatedField::new();
    let mut origin_map = HashMap::new();

    // We can't assume that every project in the group belongs to the same origin. It's entirely
    // possible that there are multiple origins present within the group. Because of this, there's
    // no way to atomically commit the entire promotion at once. It's possible origin shards can be
    // on different machines, so for now, the best we can do is partition the projects by origin,
    // and commit each origin at once. Ultimately, it'd be nice to have a way to atomically commit
    // the entire promotion at once, but that would require a cross-shard tool that we don't
    // currently have.
    for project in group.get_projects().into_iter() {
        if project.get_state() == ProjectState::Success {
            let ident = OriginPackageIdent::from_str(project.get_ident()).unwrap();

            let mut project_list = origin_map.entry(ident.get_origin().to_string()).or_insert(
                Vec::new(),
            );
            project_list.push(project);
        } else {
            not_promoted.push(OriginPackageIdent::from_str(project.get_name()).unwrap());
        }
    }

    for (origin, projects) in origin_map.iter() {
        match do_group_promotion(req, channel, projects.to_vec(), &origin, session_id) {
            Ok(_) => (),
            Err(e) => {
                if e.get_code() != ErrCode::ACCESS_DENIED {
                    info!("Failed to promote group, err: {:?}", e);
                    return Err(e);
                }
            }
        }
    }

    opgpr.set_group_id(group_id);
    opgpr.set_not_promoted(not_promoted);
    Ok(opgpr)
}

pub fn get_optional_session_id(req: &mut Request) -> Option<u64> {
    match req.extensions.get::<Authenticated>() {
        Some(session) => Some(session.get_id()),
        None => None,
    }
}

fn do_group_promotion(
    req: &mut Request,
    channel: &str,
    projects: Vec<&Project>,
    origin: &str,
    session_id_opt: Option<u64>,
) -> NetResult<NetOk> {
    if let Some(session_id) = session_id_opt {
        if !check_origin_access(req, session_id, origin)? {
            return Err(NetError::new(
                ErrCode::ACCESS_DENIED,
                "hg:promote-job-group:0",
            ));
        }
    }

    let mut ocg = OriginChannelGet::new();
    ocg.set_origin_name(origin.to_string());
    ocg.set_name(channel.to_string());

    let channel = match route_message::<OriginChannelGet, OriginChannel>(req, &ocg) {
        Ok(channel) => channel,
        Err(e) => {
            if e.get_code() == ErrCode::ENTITY_NOT_FOUND {
                if channel != STABLE_CHANNEL || channel != UNSTABLE_CHANNEL {
                    create_channel(req, &origin, channel, session_id_opt)?
                } else {
                    info!("Unable to retrieve default channel, err: {:?}", e);
                    return Err(e);
                }
            } else {
                info!("Unable to retrieve channel, err: {:?}", e);
                return Err(e);
            }
        }
    };

    let mut package_ids = Vec::new();

    for project in projects {
        let opi = OriginPackageIdent::from_str(project.get_ident()).unwrap();
        let mut opg = OriginPackageGet::new();
        opg.set_ident(opi);
        if let Some(session_id) = session_id_opt {
            opg.set_account_id(session_id);
        }

        let op = route_message::<OriginPackageGet, OriginPackage>(req, &opg)?;
        package_ids.push(op.get_id());
    }

    let mut opgp = OriginPackageGroupPromote::new();
    opgp.set_channel_id(channel.get_id());
    opgp.set_package_ids(package_ids);
    opgp.set_origin(origin.to_string());

    route_message::<OriginPackageGroupPromote, NetOk>(req, &opgp)
}
