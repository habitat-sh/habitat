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
use iron::status::{self, Status};
use iron::prelude::*;
use router::Router;
use hab_net::http::controller::*;
use bodyparser;
use protocol::originsrv::*;
use protocol::net::{NetOk, ErrCode};
use persistent;
use DepotUtil;
use bld_core;

use super::super::server::{route_message, check_origin_access};

pub fn encrypt(req: &mut Request, content: &str) -> Result<String, Status> {
    let lock = req.get::<persistent::State<DepotUtil>>().expect(
        "depot not found",
    );
    let depot = lock.read().expect("depot read lock is poisoned");

    match bld_core::integrations::encrypt(&depot.config.key_dir, content) {
        Ok(c) => Ok(c),
        Err(_) => Err(status::InternalServerError),
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
    };

    // Check that we have origin access
    {
        let session = req.extensions.get::<Authenticated>().unwrap();
        if !check_origin_access(session.get_id(), &res["origin"])
            .map_err(|_| status::InternalServerError)?
        {
            debug!(
                "Failed origin access check, session: {}, origin: {}",
                session.get_id(),
                &res["origin"]
            );
            return Err(status::Forbidden);
        }
    }

    Ok(res)
}

// Handle GET /origins/:origin/integrations/:integration/names
pub fn fetch_origin_integration_names(req: &mut Request) -> IronResult<Response> {
    // Validate params
    let params = match validate_params(req, &["origin", "integration"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    // Issue the get command
    let mut request = OriginIntegrationGetNames::new();
    request.set_origin(params["origin"].clone());
    request.set_integration(params["integration"].clone());
    match route_message::<OriginIntegrationGetNames, OriginIntegrationNames>(req, &request) {
        Ok(integration) => Ok(render_json(status::Ok, &integration)),
        Err(err) => {
            match err.get_code() {
                ErrCode::ENTITY_NOT_FOUND => Ok(Response::with((status::NotFound))),
                _ => {
                    error!("create_integration:1, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

// Handle PUT /origins/:origin/integrations/:integration
pub fn create_origin_integration(req: &mut Request) -> IronResult<Response> {
    // Validate params
    let params = match validate_params(req, &["origin", "integration", "name"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    // Check that we got valid JSON in the body
    let body = req.get::<bodyparser::Json>();
    match body {
        Ok(Some(_)) => (),
        Ok(None) => {
            warn!("create_origin_integration: Empty body in request");
            return Ok(Response::with(status::BadRequest));
        }
        Err(e) => {
            warn!("create_origin_integration, Error parsing body: {:?}", e);
            return Ok(Response::with(status::BadRequest));
        }
    };

    // We know body exists and is valid, non-empty JSON, so we can unwrap safely
    let json_body = req.get::<bodyparser::Raw>().unwrap().unwrap();

    // Encrypt the body
    let encrypted = match encrypt(req, &json_body) {
        Ok(s) => s,
        Err(st) => return Ok(Response::with(st)),
    };

    // Issue the create command
    let mut oi = OriginIntegration::new();
    oi.set_origin(params["origin"].clone());
    oi.set_integration(params["integration"].clone());
    oi.set_name(params["name"].clone());
    oi.set_body(encrypted);

    let mut request = OriginIntegrationCreate::new();
    request.set_integration(oi);

    match route_message::<OriginIntegrationCreate, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => {
            if err.get_code() == ErrCode::ENTITY_CONFLICT {
                warn!("Failed to create integration as it already exists");
                Ok(Response::with(status::Conflict))
            } else {
                error!("create_integration:1, err={:?}", err);
                Ok(Response::with(status::InternalServerError))
            }
        }
    }
}

// Handle DELETE /origins/:origin/integrations/:integration
pub fn delete_origin_integration(req: &mut Request) -> IronResult<Response> {
    // Validate params
    let params = match validate_params(req, &["origin", "integration", "name"]) {
        Ok(p) => p,
        Err(st) => return Ok(Response::with(st)),
    };

    // Issue the delete command
    let mut oi = OriginIntegration::new();
    oi.set_origin(params["origin"].clone());
    oi.set_integration(params["integration"].clone());
    oi.set_name(params["name"].clone());

    let mut request = OriginIntegrationDelete::new();
    request.set_integration(oi);

    match route_message::<OriginIntegrationDelete, NetOk>(req, &request) {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => {
            error!("delete_integration:1, err={:?}", err);
            Ok(Response::with(status::InternalServerError))
        }
    }
}
