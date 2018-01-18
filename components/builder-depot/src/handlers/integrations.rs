// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use bldr_core;
use bodyparser;
use http_gateway::http::controller::*;
use http_gateway::http::helpers::{self, check_origin_access};
use iron::status::{self, Status};

use protocol::originsrv::*;
use protocol::net::NetOk;
use persistent;
use router::Router;
use serde_json;

use DepotUtil;

pub fn encrypt(req: &mut Request, content: &str) -> Result<String, Status> {
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");

    bldr_core::integrations::encrypt(&depot.config.key_dir, content)
        .map_err(|_| status::InternalServerError)
}

pub fn decrypt(req: &mut Request, content: &str) -> Result<String, Status> {
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");

    bldr_core::integrations::decrypt(&depot.config.key_dir, content)
        .map_err(|_| status::InternalServerError)
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

    if !check_origin_access(req, &res["origin"]).unwrap_or(false) {
        debug!("Failed origin access check, origin: {}", &res["origin"]);
        return Err(status::Forbidden);
    }

    Ok(res)
}

pub fn fetch_origin_integrations(req: &mut Request) -> IronResult<Response> {
    let params = match validate_params(req, &["origin"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };
    let mut request = OriginIntegrationRequest::new();
    request.set_origin(params["origin"].clone());
    match route_message::<OriginIntegrationRequest, OriginIntegrationResponse>(req, &request) {
        Ok(oir) => {
            let integrations_response: HashMap<String, Vec<String>> = oir.get_integrations()
                .iter()
                .fold(HashMap::new(), |mut acc, ref i| {
                    acc.entry(i.get_integration().to_owned())
                        .or_insert(Vec::new())
                        .push(i.get_name().to_owned());
                    acc
                });
            let mut response = render_json(status::Ok, &integrations_response);
            helpers::dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn fetch_origin_integration_names(req: &mut Request) -> IronResult<Response> {
    let params = match validate_params(req, &["origin", "integration"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    let mut request = OriginIntegrationGetNames::new();
    request.set_origin(params["origin"].clone());
    request.set_integration(params["integration"].clone());
    match route_message::<OriginIntegrationGetNames, OriginIntegrationNames>(req, &request) {
        Ok(integration) => {
            let mut response = render_json(status::Ok, &integration);
            helpers::dont_cache_response(&mut response);
            Ok(response)
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn create_origin_integration(req: &mut Request) -> IronResult<Response> {
    let params = match validate_params(req, &["origin", "integration", "name"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    let body = req.get::<bodyparser::Json>();
    match body {
        Ok(Some(_)) => (),
        Ok(None) => {
            debug!("create_origin_integration: Empty body in request");
            return Ok(Response::with(status::BadRequest));
        }
        Err(e) => {
            debug!("create_origin_integration, Error parsing body: {:?}", e);
            return Ok(Response::with(status::BadRequest));
        }
    };

    let mut oi = OriginIntegration::new();
    oi.set_origin(params["origin"].clone());
    oi.set_integration(params["integration"].clone());
    oi.set_name(params["name"].clone());

    // We know body exists and is valid, non-empty JSON, so we can unwrap safely
    let json_body = req.get::<bodyparser::Raw>().unwrap().unwrap();

    match encrypt(req, &json_body) {
        Ok(encrypted) => oi.set_body(encrypted),
        Err(st) => return Ok(Response::with(st)),
    }

    let mut request = OriginIntegrationCreate::new();
    request.set_integration(oi);

    match route_message::<OriginIntegrationCreate, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn delete_origin_integration(req: &mut Request) -> IronResult<Response> {
    let params = match validate_params(req, &["origin", "integration", "name"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    let mut oi = OriginIntegration::new();
    oi.set_origin(params["origin"].clone());
    oi.set_integration(params["integration"].clone());
    oi.set_name(params["name"].clone());

    let mut request = OriginIntegrationDelete::new();
    request.set_integration(oi);

    match route_message::<OriginIntegrationDelete, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn get_origin_integration(req: &mut Request) -> IronResult<Response> {
    let params = match validate_params(req, &["origin", "integration", "name"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    let mut oi = OriginIntegration::new();
    oi.set_origin(params["origin"].clone());
    oi.set_integration(params["integration"].clone());
    oi.set_name(params["name"].clone());

    let mut request = OriginIntegrationGet::new();
    request.set_integration(oi);

    match route_message::<OriginIntegrationGet, OriginIntegration>(req, &request) {
        Ok(integration) => {
            match decrypt(req, &integration.get_body()) {
                Ok(decrypted) => {
                    let val = serde_json::from_str(&decrypted).unwrap();
                    let mut map: serde_json::Map<String, serde_json::Value> =
                        serde_json::from_value(val).unwrap();

                    map.remove("password");

                    let sanitized = json!({
                        "origin": integration.get_origin().to_string(),
                        "integration": integration.get_integration().to_string(),
                        "name": integration.get_name().to_string(),
                        "body": serde_json::to_value(map).unwrap()
                    });

                    let mut response = render_json(status::Ok, &sanitized);
                    helpers::dont_cache_response(&mut response);
                    Ok(response)
                }
                Err(st) => return Ok(Response::with(st)),
            }
        }
        Err(err) => Ok(render_net_error(&err)),
    }
}
