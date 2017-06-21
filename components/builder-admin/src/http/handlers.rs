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

//! A collection of handlers for the HTTP server's router

use bodyparser;
use hab_net::http::controller::*;
use hab_net::privilege;
use hab_net::routing::Broker;
use iron::prelude::*;
use iron::status;
use protocol::sessionsrv::*;
use router::Router;

#[derive(Clone, Serialize, Deserialize)]
struct FeatureGrant {
    team_id: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct FeatureFlagList(Vec<FeatureFlag>);

impl Default for FeatureFlagList {
    fn default() -> Self {
        let mut list = vec![];
        list.push(FeatureFlag::new("Admin", privilege::ADMIN.bits()));
        FeatureFlagList(list)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct FeatureFlag {
    name: String,
    id: u32,
}

impl FeatureFlag {
    pub fn new(name: &'static str, id: u32) -> Self {
        FeatureFlag {
            name: name.to_string(),
            id: id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SearchTerm {
    attr: String,
    entity: String,
    value: String,
}

pub fn account_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let stringy_id = params.find("id").unwrap();
    let id = match stringy_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    let mut account_get_id = AccountGetId::new();
    account_get_id.set_id(id);
    let mut conn = Broker::connect().unwrap();
    match conn.route::<AccountGetId, Account>(&account_get_id) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(&err)),
    }
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
                    Ok(Response::with((
                        status::UnprocessableEntity,
                        format!("Unknown search entity: {}", entity),
                    )))
                }
            }
        }
        _ => Ok(Response::with(status::UnprocessableEntity)),
    }
}

fn search_account(key: String, value: String) -> IronResult<Response> {
    match key.as_str() {
        "id" => {
            let mut account_get_id = AccountGetId::new();
            let id = match value.parse::<u64>() {
                Ok(id) => id,
                Err(_) => return Ok(Response::with(status::BadRequest)),
            };
            account_get_id.set_id(id);
            let mut conn = Broker::connect().unwrap();
            match conn.route::<AccountGetId, Account>(&account_get_id) {
                Ok(account) => Ok(render_json(status::Ok, &account)),
                Err(err) => Ok(render_net_error(&err)),
            }
        }
        "name" => {
            let mut account_get = AccountGet::new();
            account_get.set_name(value);
            let mut conn = Broker::connect().unwrap();
            match conn.route::<AccountGet, Account>(&account_get) {
                Ok(account) => Ok(render_json(status::Ok, &account)),
                Err(err) => Ok(render_net_error(&err)),
            }
        }
        _ => Ok(Response::with(status::UnprocessableEntity)),
    }
}
