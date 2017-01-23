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

//! A collection of handlers for the HTTP server's router

use bodyparser;
use hab_net::http::controller::*;
use hab_net::routing::Broker;
use iron::prelude::*;
use iron::status;
use protocol::net::NetOk;
use protocol::search::FromSearchPair;
use protocol::sessionsrv::*;
use router::Router;
use serde_json;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

impl Default for FeatureFlagList {
    fn default() -> Self {
        let mut list = vec![];
        list.push(FeatureFlag::new("Admin", privilege::ADMIN.bits()));
        list.push(FeatureFlag::new("Builder", privilege::BUILDER.bits()));
        FeatureFlagList(list)
    }
}

impl FeatureFlag {
    pub fn new(name: &'static str, id: u32) -> Self {
        FeatureFlag {
            name: name.to_string(),
            id: id,
        }
    }
}

pub fn account_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id = params.find("id").unwrap();
    if id.parse::<u64>().is_err() {
        return Ok(Response::with(status::BadRequest));
    }
    let search = AccountSearch::from_search_pair("id", id).unwrap();
    let mut conn = Broker::connect().unwrap();
    match conn.route::<AccountSearch, Account>(&search) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn feature_grant(req: &mut Request) -> IronResult<Response> {
    let mut msg = GrantFlagToTeam::new();
    {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u32>() {
            Ok(id) => msg.set_flag(id),
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    }
    match req.get::<bodyparser::Struct<FeatureGrant>>() {
        Ok(Some(body)) => msg.set_team_id(body.team_id),
        _ => return Ok(Response::with(status::UnprocessableEntity)),
    }
    let mut conn = Broker::connect().unwrap();
    match conn.route::<GrantFlagToTeam, NetOk>(&msg) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn feature_grants_list(req: &mut Request) -> IronResult<Response> {
    let mut msg = ListFlagGrants::new();
    {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u32>() {
            Ok(id) => msg.set_flag(id),
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    }
    let mut conn = Broker::connect().unwrap();
    match conn.route::<ListFlagGrants, FlagGrants>(&msg) {
        Ok(grants) => Ok(render_json(status::Ok, &grants)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn feature_revoke(req: &mut Request) -> IronResult<Response> {
    let mut msg = RevokeFlagFromTeam::new();
    {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("feature").unwrap().parse::<u32>() {
            Ok(feature) => msg.set_flag(feature),
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => msg.set_team_id(id),
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    }
    let mut conn = Broker::connect().unwrap();
    match conn.route::<RevokeFlagFromTeam, NetOk>(&msg) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn features_list(_req: &mut Request) -> IronResult<Response> {
    let flags = FeatureFlagList::default();
    let encoded = serde_json::to_string(&flags).unwrap();
    Ok(render_json(status::Ok, &encoded))
}

/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn search(req: &mut Request) -> IronResult<Response> {
    match req.get::<bodyparser::Struct<SearchTerm>>() {
        Ok(Some(body)) => {
            match &*body.entity.to_lowercase() {
                "account" => search_account(body.attr, body.value),
                entity => {
                    Ok(Response::with((status::UnprocessableEntity,
                                       format!("Unknown search entity: {}", entity))))
                }
            }
        }
        _ => Ok(Response::with(status::UnprocessableEntity)),
    }
}

fn search_account(key: String, value: String) -> IronResult<Response> {
    match AccountSearch::from_search_pair(key, value) {
        Ok(search) => {
            let mut conn = Broker::connect().unwrap();
            match conn.route::<AccountSearch, Account>(&search) {
                Ok(account) => Ok(render_json(status::Ok, &account)),
                Err(err) => Ok(render_net_error(&err)),
            }
        }
        Err(err) => Ok(Response::with((status::UnprocessableEntity, err.to_string()))),
    }
}
