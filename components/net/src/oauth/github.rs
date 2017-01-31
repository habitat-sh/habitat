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

use std::error::Error as StdError;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::result::Result as StdResult;
use std::time::Duration;

use hyper::{self, Url};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use protocol::{net, sessionsrv};
use serde_json;

pub use types::github::*;
use config;
use error::{Error, Result};

const USER_AGENT: &'static str = "Habitat-Builder";
const HTTP_TIMEOUT: u64 = 3_000;
// These OAuth scopes are required for a user to be authenticated. If this list is updated, then
// the front-end also needs to be updated in `components/builder-web/app/util.ts`. Both the
// front-end app and back-end app should have identical requirements to make things easier for
// our users and less cumbersome for us to message out.
// https://developer.github.com/v3/oauth/#scopes
const AUTH_SCOPES: &'static [&'static str] = &["user:email", "read:org"];

#[derive(Clone)]
pub struct GitHubClient {
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl GitHubClient {
    pub fn new<T: config::GitHubOAuth>(config: &T) -> Self {
        GitHubClient {
            url: config.github_url().to_string(),
            client_id: config.github_client_id().to_string(),
            client_secret: config.github_client_secret().to_string(),
        }
    }

    pub fn authenticate(&self, code: &str) -> Result<String> {
        let url = Url::parse(&format!("https://github.com/login/oauth/access_token?\
                                client_id={}&client_secret={}&code={}",
                                      self.client_id,
                                      self.client_secret,
                                      code))
            .unwrap();
        let mut rep = try!(http_post(url));
        if rep.status.is_success() {
            let mut encoded = String::new();
            try!(rep.read_to_string(&mut encoded));
            match serde_json::from_str::<AuthOk>(&encoded) {
                Ok(msg) => {
                    let missing = msg.missing_auth_scopes();
                    if missing.is_empty() {
                        Ok(msg.access_token)
                    } else {
                        let msg = format!("Missing OAuth scope(s), '{}'", missing.join(", "));
                        let err = net::err(net::ErrCode::AUTH_SCOPE, msg);
                        Err(Error::from(err))
                    }
                }
                Err(_) => {
                    match serde_json::from_str::<AuthErr>(&encoded) {
                        Ok(gh_err) => {
                            let err = net::err(net::ErrCode::ACCESS_DENIED, gh_err.error);
                            Err(Error::from(err))
                        }
                        Err(_) => {
                            let err = net::err(net::ErrCode::BAD_REMOTE_REPLY, "net:github:0");
                            Err(Error::from(err))
                        }
                    }
                }
            }
        } else {
            Err(Error::HTTP(rep.status))
        }
    }

    /// Returns the contents of a file or directory in a repository.
    pub fn contents(&self, token: &str, owner: &str, repo: &str, path: &str) -> Result<Contents> {
        let url = Url::parse(&format!("{}/repos/{}/{}/contents/{}", self.url, owner, repo, path))
            .unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }
        let contents: Contents = serde_json::from_str(&body).unwrap();
        Ok(contents)
    }

    pub fn repo(&self, token: &str, owner: &str, repo: &str) -> Result<Repo> {
        let url = Url::parse(&format!("{}/repos/{}/{}", self.url, owner, repo)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }

        let repo: Repo = match serde_json::from_str(&body) {
            Ok(r) => r,
            Err(e) => {
                debug!("github repo decode failed: {}. response body: {}", e, body);
                return Err(Error::from(e));
            }
        };

        Ok(repo)
    }

    pub fn user(&self, token: &str) -> Result<User> {
        let url = Url::parse(&format!("{}/user", self.url)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }
        let user: User = serde_json::from_str(&body).unwrap();
        Ok(user)
    }

    pub fn other_user(&self, token: &str, username: &str) -> Result<User> {
        let url = Url::parse(&format!("{}/users/{}", self.url, username)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }
        let user: User = serde_json::from_str(&body).unwrap();
        Ok(user)
    }

    pub fn emails(&self, token: &str) -> Result<Vec<Email>> {
        let url = Url::parse(&format!("{}/user/emails", self.url)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }
        let emails: Vec<Email> = try!(serde_json::from_str(&body));
        Ok(emails)
    }

    pub fn orgs(&self, token: &str) -> Result<Vec<Organization>> {
        let url = Url::parse(&format!("{}/user/orgs", self.url)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }
        let orgs: Vec<Organization> = try!(serde_json::from_str(&body));
        Ok(orgs)
    }

    pub fn teams(&self, token: &str) -> Result<Vec<Team>> {
        let url = Url::parse(&format!("{}/user/teams", self.url)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(serde_json::from_str(&body));
            return Err(Error::GitHubAPI(rep.status, err));
        }
        let teams: Vec<Team> = try!(serde_json::from_str(&body));
        Ok(teams)
    }
}

impl From<User> for sessionsrv::Account {
    fn from(user: User) -> sessionsrv::Account {
        let mut account = sessionsrv::Account::new();
        account.set_name(user.login);
        if let Some(email) = user.email {
            account.set_email(email);
        }
        account
    }
}

impl AuthOk {
    pub fn missing_auth_scopes(&self) -> Vec<&'static str> {
        let mut scopes = vec![];
        for scope in AUTH_SCOPES.iter() {
            if !self.scope.split(",").collect::<Vec<&str>>().iter().any(|p| p == scope) {
                scopes.push(*scope);
            }
        }
        scopes
    }
}

impl fmt::Display for AuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "err={}, desc={}, uri={}",
               self.error,
               self.error_description,
               self.error_uri)
    }
}

fn http_get(url: Url, token: &str) -> StdResult<hyper::client::response::Response, net::NetError> {
    hyper_client()
        .get(url)
        .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]))
        .header(Authorization(Bearer { token: token.to_owned() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(hyper_to_net_err)
}

fn http_post(url: Url) -> StdResult<hyper::client::response::Response, net::NetError> {
    hyper_client()
        .post(url)
        .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(hyper_to_net_err)
}

fn hyper_client() -> hyper::Client {
    let ssl = OpensslClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = hyper::Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client.set_write_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client
}

fn hyper_to_net_err(err: hyper::error::Error) -> net::NetError {
    net::err(net::ErrCode::BAD_REMOTE_REPLY, err.description())
}
