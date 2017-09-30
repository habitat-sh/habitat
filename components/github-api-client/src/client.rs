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
use std::fmt;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

use base64;
use hyper::{self, Url};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use serde_json;

use config;
use error::{HubError, HubResult};

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
    pub web_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl GitHubClient {
    pub fn new<T>(config: &T) -> Self
    where
        T: config::GitHubOAuth,
    {
        GitHubClient {
            url: config.github_url().to_string(),
            web_url: config.github_web_url().to_string(),
            client_id: config.github_client_id().to_string(),
            client_secret: config.github_client_secret().to_string(),
        }
    }

    pub fn authenticate(&self, code: &str) -> HubResult<String> {
        let url = Url::parse(&format!(
            "{}/login/oauth/access_token?\
                                client_id={}&client_secret={}&code={}",
            self.web_url,
            self.client_id,
            self.client_secret,
            code
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_post(url)?;
        if rep.status.is_success() {
            let mut encoded = String::new();
            rep.read_to_string(&mut encoded)?;
            match serde_json::from_str::<AuthOk>(&encoded) {
                Ok(msg) => {
                    let missing = msg.missing_auth_scopes();
                    if missing.is_empty() {
                        Ok(msg.access_token)
                    } else {
                        Err(HubError::AuthScope(missing))
                    }
                }
                Err(_) => {
                    let err = serde_json::from_str::<AuthErr>(&encoded)?;
                    Err(HubError::Auth(err))
                }
            }
        } else {
            Err(HubError::HttpResponse(rep.status))
        }
    }

    /// Returns the contents of a file or directory in a repository.
    pub fn contents(
        &self,
        token: &str,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> HubResult<Contents> {
        let url = Url::parse(&format!(
            "{}/repos/{}/{}/contents/{}",
            self.url,
            owner,
            repo,
            path
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let mut contents: Contents = serde_json::from_str(&body)?;

        // We need to strip line feeds as the Github API has started to return
        // base64 content with line feeds.
        if contents.encoding == "base64" {
            contents.content = contents.content.replace("\n", "");
        }

        Ok(contents)
    }

    pub fn repo(&self, token: &str, owner: &str, repo: &str) -> HubResult<Repository> {
        let url = Url::parse(&format!("{}/repos/{}/{}", self.url, owner, repo)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }

        let repo = match serde_json::from_str::<Repository>(&body) {
            Ok(r) => r,
            Err(e) => {
                debug!("github repo decode failed: {}. response body: {}", e, body);
                return Err(HubError::from(e));
            }
        };

        Ok(repo)
    }

    pub fn user(&self, token: &str) -> HubResult<User> {
        let url = Url::parse(&format!("{}/user", self.url)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let user: User = serde_json::from_str(&body)?;
        Ok(user)
    }

    pub fn other_user(&self, token: &str, username: &str) -> HubResult<User> {
        let url = Url::parse(&format!("{}/users/{}", self.url, username)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let user: User = serde_json::from_str(&body)?;
        Ok(user)
    }

    pub fn emails(&self, token: &str) -> HubResult<Vec<Email>> {
        let url = Url::parse(&format!("{}/user/emails", self.url)).unwrap();
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let emails: Vec<Email> = serde_json::from_str(&body)?;
        Ok(emails)
    }

    pub fn orgs(&self, token: &str) -> HubResult<Vec<Organization>> {
        let url = Url::parse(&format!("{}/user/orgs", self.url)).map_err(
            HubError::HttpClientParse,
        )?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let orgs: Vec<Organization> = serde_json::from_str(&body)?;
        Ok(orgs)
    }

    pub fn search_file(&self, token: &str, repo: &str, file: &str) -> HubResult<Search> {
        let url = Url::parse(&format!(
            "{}/search/code?q={}+in:path+repo:{}",
            self.url,
            file,
            repo
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let search = serde_json::from_str::<Search>(&body)?;
        Ok(search)
    }

    pub fn teams(&self, token: &str) -> HubResult<Vec<Team>> {
        let url = Url::parse(&format!("{}/user/teams", self.url)).map_err(
            HubError::HttpClientParse,
        )?;
        let mut rep = http_get(url, token)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let teams: Vec<Team> = serde_json::from_str(&body)?;
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

impl Contents {
    pub fn decode(&self) -> HubResult<Vec<u8>> {
        base64::decode(&self.content).map_err(HubError::ContentDecode)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitHubOwner {
    /// Public name of commit author
    pub name: String,
    /// Public email of commit author
    pub email: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub mirror_url: Option<String>,
    default_branch: String,
    master_branch: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
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
            if !self.scope.split(",").collect::<Vec<&str>>().iter().any(
                |p| {
                    p == scope
                },
            )
            {
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
        write!(
            f,
            "err={}, desc={}, uri={}",
            self.error,
            self.error_description,
            self.error_uri
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Search {
    total_count: u64,
    incomplete_results: bool,
    items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchItem {
    name: String,
    path: PathBuf,
    sha: String,
    url: String,
    git_url: String,
    html_url: String,
    repository: Repository,
    score: f64,
}

fn http_get(url: Url, token: &str) -> HubResult<hyper::client::response::Response> {
    hyper_client()
        .get(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
        ]))
        .header(Authorization(Bearer { token: token.to_owned() }))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(HubError::HttpClient)
}

fn http_post(url: Url) -> HubResult<hyper::client::response::Response> {
    hyper_client()
        .post(url)
        .header(Accept(vec![
            qitem(
                Mime(TopLevel::Application, SubLevel::Json, vec![])
            ),
        ]))
        .header(UserAgent(USER_AGENT.to_string()))
        .send()
        .map_err(HubError::HttpClient)
}

fn hyper_client() -> hyper::Client {
    let ssl = OpensslClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = hyper::Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client.set_write_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client
}
