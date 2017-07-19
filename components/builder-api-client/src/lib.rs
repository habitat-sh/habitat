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
extern crate serde_json;
extern crate url;

pub mod error;
pub use error::{Error, Result};

use std::io::Read;
use std::path::Path;

use hab_core::package::PackageIdent;
use hab_http::ApiClient;
use hyper::client::{IntoUrl, Response, RequestBuilder};
use hyper::header::{Accept, Authorization, Bearer, ContentType};
use hyper::status::StatusCode;
use regex::Regex;

pub struct Client(ApiClient);

impl Client {
    pub fn new<U>(
        depot_url: U,
        product: &str,
        version: &str,
        fs_root_path: Option<&Path>,
    ) -> Result<Self>
    where
        U: IntoUrl,
    {
        let re = Regex::new(r"depot/?$").unwrap();
        let mut url = depot_url.into_url().map_err(Error::URL)?;

        if re.is_match(url.path()) {
            let mut psm = url.path_segments_mut().unwrap();
            psm.pop();
        }

        let api_url = url.as_str();

        Ok(Client(
            ApiClient::new(api_url, product, version, fs_root_path)
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
    pub fn create_job(&self, ident: &PackageIdent, token: &str) -> Result<()> {
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
            Ok(Response { status: StatusCode::Created, .. }) => Ok(()),
            Ok(mut response) => {
                if response.status == StatusCode::Unauthorized {
                    Err(Error::APIError(
                        response.status,
                        "Your GitHub token requires both user:email and read:org \
                                         permissions."
                            .to_string(),
                    ))
                } else {
                    let mut s = String::new();
                    response.read_to_string(&mut s).map_err(Error::IO)?;
                    Err(Error::APIError(response.status, s))
                }
            }
            Err(e) => Err(Error::HyperError(e)),
        }
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
    }
}
