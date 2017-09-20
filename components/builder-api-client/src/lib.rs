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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate habitat_http_client as hab_http;
extern crate habitat_core as hab_core;
extern crate hyper;
extern crate hyper_openssl;
#[macro_use]
extern crate log;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;

pub mod error;
pub use error::{Error, Result};

use std::io::Read;
use std::path::Path;

use hab_core::package::PackageIdent;
use hab_http::ApiClient;
use hab_http::util::decoded_response;
use hyper::client::{IntoUrl, Response, RequestBuilder};
use hyper::header::{Accept, Authorization, Bearer, ContentType};
use hyper::status::StatusCode;

const DEFAULT_API_PATH: &'static str = "/v1";

pub struct Client(ApiClient);

#[derive(Deserialize)]
pub struct ReverseDependencies {
    pub origin: String,
    pub name: String,
    pub rdeps: Vec<String>,
}

#[derive(Default, Deserialize)]
pub struct JobGroupPromoteResponse {
    pub group_id: u64,
    pub failed_projects: Vec<String>,
}

impl Client {
    pub fn new<U>(
        endpoint: U,
        product: &str,
        version: &str,
        fs_root_path: Option<&Path>,
    ) -> Result<Self>
    where
        U: IntoUrl,
    {
        let mut endpoint = endpoint.into_url().map_err(Error::URL)?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }
        Ok(Client(
            ApiClient::new(endpoint, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        ))
    }

    /// Create a job.
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn create_job(&self, ident: &PackageIdent, token: &str) -> Result<(String)> {
        debug!("Creating a job for {}", ident);

        let body = json!({
            "project_id": format!("{}", ident)
        });

        let sbody = serde_json::to_string(&body).unwrap();

        let result = self.add_authz(self.0.post("jobs"), token)
            .body(&sbody)
            .header(Accept::json())
            .header(ContentType::json())
            .send();
        match result {
            Ok(mut response) => {
                match response.status {
                    StatusCode::Created => {
                        let mut encoded = String::new();
                        response.read_to_string(&mut encoded).map_err(Error::IO)?;
                        debug!("Body: {:?}", encoded);
                        let v: serde_json::Value =
                            serde_json::from_str(&encoded).map_err(Error::Json)?;
                        let id = v["id"].as_str().unwrap();
                        Ok(id.to_string())
                    }
                    StatusCode::Unauthorized => {
                        Err(Error::APIError(
                            response.status,
                            "Your GitHub token requires both user:email and read:org \
                                             permissions."
                                .to_string(),
                        ))
                    }
                    _ => Err(err_from_response(response)),
                }
            }
            Err(e) => Err(Error::HyperError(e)),
        }
    }

    /// Fetch the reverse dependencies for a package
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub fn fetch_rdeps(&self, ident: &PackageIdent) -> Result<Vec<String>> {
        debug!("Fetching the reverse dependencies for {}", ident);

        let url = format!("rdeps/{}", ident);
        let mut res = self.0.get(&url).send().map_err(Error::HyperError)?;
        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded).map_err(Error::IO)?;
        debug!("Body: {:?}", encoded);
        let rd: ReverseDependencies = serde_json::from_str(&encoded).map_err(Error::Json)?;
        Ok(rd.rdeps.to_vec())
    }

    /// Promote a job group to a channel
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub fn job_group_promote(
        &self,
        group_id: u64,
        channel: &str,
        token: &str,
    ) -> Result<Vec<String>> {
        debug!("Promoting job group {} to channel {}", group_id, channel);
        let url = format!("jobs/group/{}/promote/{}", group_id, channel);
        let res = self.add_authz(self.0.post(&url), token).send().map_err(
            Error::HyperError,
        )?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        // At first glance, this might look like a situation of an-error-thats-not-an-error. What's
        // actually happening is the "decoded_response" function returns a generic T that
        // implements the serde::de::Deserialized trait, meaning anything that serde can
        // deserialize. However, if serde attempts to deserialize an empty response body (as the
        // server will sometimes return in success cases) then serde will return an error and
        // is_eof() will return true. We handle that edge case here by returning Ok, because for
        // us, this is a success case, even though it's an error case for serde. In the future, we
        // might want to investigate a way to have decoded_response handle this oddity.
        match decoded_response::<JobGroupPromoteResponse>(res).map_err(Error::HabitatHttpClient) {
            Ok(value) => Ok(value.failed_projects),
            Err(Error::HabitatHttpClient(hab_http::Error::Json(e))) => {
                if e.is_eof() {
                    return Ok(Vec::new());
                } else {
                    return Err(Error::Json(e));
                }
            }
            Err(e) => return Err(e),
        }
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
    }
}

fn err_from_response(mut response: Response) -> Error {
    let mut s = String::new();
    response.read_to_string(&mut s).map_err(Error::IO).unwrap();
    Error::APIError(response.status, s)
}
