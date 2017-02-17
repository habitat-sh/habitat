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
        let mut contents: Contents = serde_json::from_str(&body).unwrap();

        // We need to strip line feeds as the Github API has started to return
        // base64 content with line feeds.
        if contents.encoding == "base64" {
            contents.content = contents.content.replace("\n", "");
        }

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


#[derive(Debug, Deserialize, Serialize)]
pub struct Contents {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: usize,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: u32,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: String,
    pub has_issues: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u32,
    pub mirror_url: Option<String>,
    pub open_issues_count: u32,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
    pub permissions: Permissions,
    pub organization: Option<Organization>,
    pub network_count: u32,
    pub subscribers_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub url: String,
    pub company: Option<String>,
    pub description: Option<String>,
    pub gravatar_id: Option<String>,
    pub hooks_url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    pub members_url: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String,
    pub members_count: u64,
    pub repos_count: u64,
    pub organization: Organization,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub public_repos: Option<u32>,
    pub public_gists: Option<u32>,
    pub followers: Option<u32>,
    pub following: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthOk {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthErr {
    pub error: String,
    pub error_description: String,
    pub error_uri: String,
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

#[derive(Deserialize, Serialize)]
pub enum AuthResp {
    AuthOk,
    AuthErr,
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
