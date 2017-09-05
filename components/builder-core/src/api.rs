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
use std::result;
use std::str::FromStr;

use hab_core::channel::{STABLE_CHANNEL, UNSTABLE_CHANNEL};
use hab_net::routing::Broker;
use protocol::originsrv::{CheckOriginAccessRequest, CheckOriginAccessResponse, Origin,
                          OriginChannel, OriginChannelCreate, OriginChannelGet, OriginGet,
                          OriginPackage, OriginPackageGet, OriginPackageGroupPromote,
                          OriginPackageIdent, OriginPackagePromote};
use protocol::net::{ErrCode, NetError, NetOk};
use protocol::scheduler::{Group, GroupGet, Project, ProjectState};
use protocol::sessionsrv::{Session, SessionCreate, SessionGet};

use data_structures::PartialJobGroupPromote;

use error::{Error, Result};

pub fn get_origin<T: ToString>(origin: T) -> result::Result<Option<Origin>, NetError> {
    let mut conn = Broker::connect().unwrap();
    let mut request = OriginGet::new();
    request.set_name(origin.to_string());

    match conn.route::<OriginGet, Origin>(&request) {
        Ok(origin) => Ok(Some(origin)),
        Err(err) => {
            if err.get_code() == ErrCode::ENTITY_NOT_FOUND {
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}

pub fn check_origin_access<T: ToString>(
    account_id: u64,
    origin: T,
) -> result::Result<bool, NetError> {
    let mut conn = Broker::connect().unwrap();
    let mut request = CheckOriginAccessRequest::new();
    request.set_account_id(account_id);
    request.set_origin_name(origin.to_string());
    match conn.route::<CheckOriginAccessRequest, CheckOriginAccessResponse>(&request) {
        Ok(response) => Ok(response.get_has_access()),
        Err(err) => Err(err),
    }
}

pub fn create_channel(origin: &str, channel: &str, session_id: u64) -> Result<OriginChannel> {
    let origin_id = match get_origin(origin).map_err(Error::NetError)? {
        Some(o) => o.get_id(),
        None => {
            debug!("Origin {} not found!", origin);
            return Err(Error::OriginNotFound(origin.to_string()));
        }
    };

    let mut conn = Broker::connect().unwrap();
    let mut request = OriginChannelCreate::new();

    request.set_owner_id(session_id);
    request.set_origin_name(origin.to_string());
    request.set_origin_id(origin_id);
    request.set_name(channel.to_string());

    match conn.route::<OriginChannelCreate, OriginChannel>(&request) {
        Ok(origin_channel) => Ok(origin_channel),
        Err(err) => Err(Error::NetError(err)),
    }
}

pub fn promote_package_to_channel(
    ident: &OriginPackageIdent,
    channel: &str,
    session_id: u64,
) -> Result<()> {
    if !check_origin_access(session_id, ident.get_origin())
        .map_err(Error::NetError)?
    {
        return Err(Error::OriginAccessDenied);
    }

    let mut conn = Broker::connect().unwrap();
    let mut channel_req = OriginChannelGet::new();
    channel_req.set_origin_name(ident.get_origin().to_string());
    channel_req.set_name(channel.to_string());

    match conn.route::<OriginChannelGet, OriginChannel>(&channel_req) {
        Ok(origin_channel) => {
            let mut request = OriginPackageGet::new();
            request.set_ident(ident.clone());
            match conn.route::<OriginPackageGet, OriginPackage>(&request) {
                Ok(package) => {
                    let mut promote = OriginPackagePromote::new();
                    promote.set_channel_id(origin_channel.get_id());
                    promote.set_package_id(package.get_id());
                    promote.set_ident(ident.clone());
                    match conn.route::<OriginPackagePromote, NetOk>(&promote) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(Error::NetError(err)),
                    }
                }
                Err(err) => Err(Error::NetError(err)),
            }
        }
        Err(err) => Err(Error::NetError(err)),
    }
}

pub fn promote_job_group_to_channel(group_id: u64, channel: &str, session_id: u64) -> Result<()> {
    let mut group_get = GroupGet::new();
    group_get.set_group_id(group_id);

    let mut conn = Broker::connect().unwrap();
    let group = match conn.route::<GroupGet, Group>(&group_get) {
        Ok(g) => g,
        Err(err) => return Err(Error::NetError(err)),
    };

    // This only makes sense if the group is complete. If the group isn't complete, return now and
    // let the user know. Check the completion state by checking the individual project states,
    // as if this is called by the scheduler it needs to promote the group before marking it
    // Complete.
    if group.get_projects().iter().any(|&ref p| {
        p.get_state() == ProjectState::NotStarted || p.get_state() == ProjectState::InProgress
    })
    {
        return Err(Error::GroupNotComplete);
    }

    let mut failed_projects = Vec::new();
    let mut origin_map = HashMap::new();

    let channel_fn = |org, chn, ses| {
        if chn == STABLE_CHANNEL || chn == UNSTABLE_CHANNEL {
            return Ok(());
        }

        match create_channel(org, chn, ses) {
            Ok(_) => Ok(()),
            Err(Error::NetError(err)) => {
                match err.get_code() {
                    ErrCode::ENTITY_CONFLICT => Ok(()),
                    _ => return Err(Error::NetError(err)),
                }
            }
            Err(e) => return Err(e),
        }
    };

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
            failed_projects.push(project.get_ident().to_string());
        }
    }

    // Now that we've sorted all the projects into a HashMap keyed on origin, we can process them,
    // one origin at a time. We do "core" first, since it's not possible to do the entire operation
    // atomically. Instead, we prioritize the core origin since it's the most important one.
    if let Some(core_projects) = origin_map.remove("core") {
        if let Err(e) = channel_fn("core", channel, session_id) {
            return Err(e);
        }

        let promote_result = do_group_promotion(channel, core_projects, "core", session_id);
        if promote_result.is_err() {
            return promote_result;
        }
    }

    for (origin, projects) in origin_map.iter() {
        if let Err(e) = channel_fn(&origin, channel, session_id) {
            return Err(e);
        }

        let promote_result = do_group_promotion(channel, projects.to_vec(), &origin, session_id);
        if promote_result.is_err() {
            return promote_result;
        }
    }

    if failed_projects.is_empty() {
        Ok(())
    } else {
        let pjgp = PartialJobGroupPromote {
            group_id: group.get_id(),
            failed_projects: failed_projects,
        };
        Err(Error::PartialJobGroupPromote(pjgp))
    }
}

pub fn authenticate_with_auth_token(auth_token: &str) -> Result<u64> {
    let mut conn = Broker::connect().unwrap();
    let mut request = SessionGet::new();
    request.set_token(auth_token.to_string());

    match conn.route::<SessionGet, Session>(&request) {
        Ok(session) => Ok(session.get_id()),
        Err(err) => {
            if err.get_code() == ErrCode::SESSION_EXPIRED {
                let mut create = SessionCreate::new();
                create.set_token(auth_token.to_string());

                match conn.route::<SessionCreate, Session>(&create) {
                    Ok(session) => Ok(session.get_id()),
                    Err(err) => Err(Error::NetError(err)),
                }
            } else {
                Err(Error::NetError(err))
            }
        }
    }
}

fn do_group_promotion(
    channel: &str,
    projects: Vec<&Project>,
    origin: &str,
    session_id: u64,
) -> Result<()> {
    let mut conn = Broker::connect().unwrap();
    let mut ocg = OriginChannelGet::new();
    ocg.set_origin_name(origin.to_string());
    ocg.set_name(channel.to_string());

    let mut package_ids = Vec::new();

    let channel = match conn.route::<OriginChannelGet, OriginChannel>(&ocg) {
        Ok(c) => {
            if !check_origin_access(session_id, origin).map_err(
                Error::NetError,
            )?
            {
                return Err(Error::OriginAccessDenied);
            }

            c
        }
        Err(err) => return Err(Error::NetError(err)),
    };

    for project in projects {
        let opi = OriginPackageIdent::from_str(project.get_ident()).unwrap();
        let mut opg = OriginPackageGet::new();
        opg.set_ident(opi);

        let op = match conn.route::<OriginPackageGet, OriginPackage>(&opg) {
            Ok(o) => o,
            Err(err) => return Err(Error::NetError(err)),
        };

        package_ids.push(op.get_id());
    }

    let mut opgp = OriginPackageGroupPromote::new();
    opgp.set_channel_id(channel.get_id());
    opgp.set_package_ids(package_ids);
    opgp.set_origin(origin.to_string());

    match conn.route::<OriginPackageGroupPromote, NetOk>(&opgp) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::NetError(err)),
    }
}
