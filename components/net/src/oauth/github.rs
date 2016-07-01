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

use std::collections::HashMap;
use std::fmt;
use std::io::Read;

use hyper::{self, Url};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use protocol::sessionsrv;
use rustc_serialize::json;

use config;
use error::{Error, Result};

const USER_AGENT: &'static str = "Habitat-Builder";

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
        let url =
            Url::parse(&format!("https://github.com/login/oauth/access_token?\
                                client_id={}&client_secret={}&code={}",
                                self.client_id,
                                self.client_secret,
                                code))
                .unwrap();
        let mut rep = try!(http_post(url));
        if rep.status.is_success() {
            let mut encoded = String::new();
            try!(rep.read_to_string(&mut encoded));
            match json::decode(&encoded) {
                Ok(msg @ AuthOk { .. }) => {
                    let scope = "user:email".to_string();
                    if msg.has_scope(&scope) {
                        Ok(msg.access_token)
                    } else {
                        Err(Error::MissingScope(scope))
                    }
                }
                Err(_) => {
                    let err: AuthErr = try!(json::decode(&encoded));
                    Err(Error::from(err))
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
            let err: HashMap<String, String> = try!(json::decode(&body));
            return Err(Error::GitHubAPI(err));
        }
        let contents: Contents = json::decode(&body).unwrap();
        Ok(contents)
    }

    pub fn repo(&self, token: &str, owner: &str, repo: &str) -> Result<Repo> {
        let url = Url::parse(&format!("{}/repos/{}/{}", self.url, owner, repo)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(json::decode(&body));
            return Err(Error::GitHubAPI(err));
        }
        let repo: Repo = json::decode(&body).unwrap();
        Ok(repo)
    }

    pub fn user(&self, token: &str) -> Result<User> {
        let url = Url::parse(&format!("{}/user", self.url)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(json::decode(&body));
            return Err(Error::GitHubAPI(err));
        }
        let user: User = json::decode(&body).unwrap();
        Ok(user)
    }

    pub fn emails(&self, token: &str) -> Result<Vec<Email>> {
        let url = Url::parse(&format!("{}/user/emails", self.url)).unwrap();
        let mut rep = try!(http_get(url, token));
        let mut body = String::new();
        try!(rep.read_to_string(&mut body));
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = try!(json::decode(&body));
            return Err(Error::GitHubAPI(err));
        }
        let emails: Vec<Email> = try!(json::decode(&body));
        Ok(emails)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: String,
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Organization {
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
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Permissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Email {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct AuthOk {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

impl AuthOk {
    pub fn has_scope(&self, grant: &str) -> bool {
        self.scope.split(",").collect::<Vec<&str>>().iter().any(|&p| p == grant)
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
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

#[derive(RustcDecodable, RustcEncodable)]
pub enum AuthResp {
    AuthOk,
    AuthErr,
}

fn http_get(url: Url, token: &str) -> Result<hyper::client::response::Response> {
    hyper::Client::new()
        .get(url)
        .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]))
        .header(Authorization(Bearer { token: token.to_owned() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(|e| Error::from(e))
}

fn http_post(url: Url) -> Result<hyper::client::response::Response> {
    hyper::Client::new()
        .post(url)
        .header(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]))
        .send()
        .map_err(|e| Error::from(e))
}
