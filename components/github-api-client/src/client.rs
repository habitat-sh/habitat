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
use std::io::Read;
use std::time::{UNIX_EPOCH, Duration, SystemTime};

use hyper::{self, Url};
use hyper::client::{IntoUrl, Response};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use hyper::mime::{Mime, TopLevel, SubLevel};
use jwt;
use regex::Regex;
use serde_json;

use config::GitHubCfg;
use error::{HubError, HubResult};
use types::*;

const USER_AGENT: &'static str = "Habitat-Builder";
const HTTP_TIMEOUT: u64 = 3_000;

lazy_static! {
    // Until we can migrate to Hyper 0.11.x, which has typed support
    // for interacting with Link headers, we need to roll our own
    // support using regular expressions.
    //
    // Given a header string, this regex extracts the 'rel="next"'
    // URL, if there is one. Used with Github's pagination scheme.
    static ref LINK_NEXT_RE: Regex = Regex::new("<(?P<url>[^>].*)>;.*rel=\"next\"").unwrap();
}

#[derive(Clone)]
pub struct GitHubClient {
    pub url: String,
    pub web_url: String,
    pub client_id: String,
    pub client_secret: String,
    app_id: u32,
    app_private_key: String,
    pub webhook_secret: String,
}

impl GitHubClient {
    pub fn new(config: GitHubCfg) -> Self {
        GitHubClient {
            url: config.url,
            web_url: config.web_url,
            client_id: config.client_id,
            client_secret: config.client_secret,
            app_id: config.app_id,
            app_private_key: config.app_private_key,
            webhook_secret: config.webhook_secret,
        }
    }

    pub fn app(&self) -> HubResult<App> {
        let app_token = generate_app_token(&self.app_private_key, &self.app_id);
        let url = Url::parse(&format!("{}/app", self.url)).map_err(
            HubError::HttpClientParse,
        )?;
        let mut rep = http_get(url, Some(app_token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let contents = serde_json::from_str::<App>(&body)?;
        Ok(contents)
    }

    pub fn app_installation_token(&self, install_id: u32) -> HubResult<String> {
        let app_token = generate_app_token(&self.app_private_key, &self.app_id);
        let url = Url::parse(&format!(
            "{}/installations/{}/access_tokens",
            self.url,
            install_id
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_post(url, Some(app_token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
        match serde_json::from_str::<AppInstallationToken>(&body) {
            Ok(msg) => Ok(msg.token),
            Err(_) => {
                let err = serde_json::from_str::<AppAuthErr>(&body)?;
                Err(HubError::AppAuth(err))
            }
        }
    }

    pub fn authenticate(&self, code: &str) -> HubResult<String> {
        let url = Url::parse(&format!(
            "{}/login/oauth/access_token?client_id={}&client_secret={}&code={}",
            self.web_url,
            self.client_id,
            self.client_secret,
            code
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_post(url, None::<String>)?;
        if rep.status.is_success() {
            let mut body = String::new();
            rep.read_to_string(&mut body)?;
            debug!("GitHub response body, {}", body);
            match serde_json::from_str::<AuthOk>(&body) {
                Ok(msg) => Ok(msg.access_token),
                Err(_) => {
                    let err = serde_json::from_str::<AuthErr>(&body)?;
                    Err(HubError::Auth(err))
                }
            }
        } else {
            Err(HubError::HttpResponse(rep.status))
        }
    }

    pub fn check_team_membership(
        &self,
        token: &str,
        team: u32,
        user: &str,
    ) -> HubResult<TeamMembership> {
        let url = Url::parse(&format!("{}/teams/{}/memberships/{}", self.url, team, user))
            .map_err(HubError::HttpClientParse)?;
        let mut rep = http_get(url, Some(token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let contents = serde_json::from_str(&body)?;
        Ok(contents)
    }

    /// Returns the contents of a file or directory in a repository.
    pub fn contents(&self, token: &str, repo: u32, path: &str) -> HubResult<Contents> {
        let url = Url::parse(&format!(
            "{}/repositories/{}/contents/{}",
            self.url,
            repo,
            path
        )).map_err(HubError::HttpClientParse)?;
        let mut rep = http_get(url, Some(token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
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

    pub fn repo(&self, token: &str, repo: u32) -> HubResult<Repository> {
        let url = Url::parse(&format!("{}/repositories/{}", self.url, repo)).unwrap();
        let mut rep = http_get(url, Some(token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let value = serde_json::from_str(&body)?;
        Ok(value)
    }

    pub fn repositories(&self, token: &str, install_id: u32) -> HubResult<Vec<Repository>> {

        let initial_url = Url::parse(&format!(
            "{}/user/installations/{}/repositories",
            self.url,
            install_id
        )).unwrap();

        let mut repositories = Vec::new();
        let mut current_url = Some(initial_url);

        while let Some(url) = current_url {
            let mut rep = http_get(url, Some(token))?;
            let mut body = String::new();
            rep.read_to_string(&mut body)?;
            debug!("GitHub response body, {}", body);
            if rep.status != StatusCode::Ok {
                let err: HashMap<String, String> = serde_json::from_str(&body)?;
                return Err(HubError::ApiError(rep.status, err));
            }
            let mut list = serde_json::from_str::<RepositoryList>(&body)?;
            repositories.append(&mut list.repositories);

            // Determine the next page to grab and do it again.
            current_url = Self::next_page_url(&rep);
        }

        Ok(repositories)
    }

    /// Given an HTTP response from Github, extract the URL for the
    /// "next" page of results, if it exists.
    ///
    /// See
    /// https://developer.github.com/v3/guides/traversing-with-pagination/
    /// for further details.
    ///
    /// NOTE: Once we upgrade to Hyper 0.11.x, this implementation can
    /// get a whole lot less ugly because then we'll actually have
    /// TYPED ACCESS to the Link header.
    fn next_page_url(response: &Response) -> Option<Url> {
        let headers = &response.headers;
        if let Some(link) = headers.get_raw("link") {
            for value in link.iter() {
                let value = String::from_utf8_lossy(value);
                if let Some(captures) = LINK_NEXT_RE.captures(&value) {
                    return Some(Url::parse(&captures["url"]).unwrap());
                }
            }
        }
        return None;
    }

    pub fn user(&self, token: &str) -> HubResult<User> {
        let url = Url::parse(&format!("{}/user", self.url)).unwrap();
        let mut rep = http_get(url, Some(token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let user = serde_json::from_str(&body)?;
        Ok(user)
    }

    pub fn search_code(&self, token: &str, query: &str) -> HubResult<Search> {
        let url = Url::parse(&format!("{}/search/code?{}", self.url, query))
            .map_err(HubError::HttpClientParse)?;
        let mut rep = http_get(url, Some(token))?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;
        debug!("GitHub response body, {}", body);
        if rep.status != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(HubError::ApiError(rep.status, err));
        }
        let search = serde_json::from_str::<Search>(&body)?;
        Ok(search)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct RepositoryList {
    pub total_count: u32,
    pub repositories: Vec<Repository>,
}

fn generate_app_token<T, U>(key_path: T, app_id: U) -> String
where
    T: ToString,
    U: ToString,
{
    let mut payload = jwt::Payload::new();
    let header = jwt::Header::new(jwt::Algorithm::RS256);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let expiration = now + Duration::from_secs(10 * 60);
    payload.insert("iat".to_string(), now.as_secs().to_string());
    payload.insert("exp".to_string(), expiration.as_secs().to_string());
    payload.insert("iss".to_string(), app_id.to_string());
    jwt::encode(header, key_path.to_string(), payload)
}

fn http_get<T, U>(url: T, token: Option<U>) -> HubResult<hyper::client::response::Response>
where
    T: IntoUrl,
    U: ToString,
{
    let client = hyper_client();
    let req = client.get(url);
    let req = req.header(Accept(vec![
        qitem(
            Mime(TopLevel::Application, SubLevel::Json, vec![])
        ),
        qitem("application/vnd.github.v3+json".parse().unwrap()),
        qitem(
            "application/vnd.github.machine-man-preview+json"
                .parse()
                .unwrap()
        ),
    ])).header(UserAgent(USER_AGENT.to_string()));
    let req = match token {
        Some(token) => req.header(Authorization(Bearer { token: token.to_string() })),
        None => req,
    };
    req.send().map_err(HubError::HttpClient)
}

fn http_post<T, U>(url: T, token: Option<U>) -> HubResult<hyper::client::response::Response>
where
    T: IntoUrl,
    U: ToString,
{
    let client = hyper_client();
    let req = client.post(url);
    let req = req.header(Accept(vec![
        qitem(
            Mime(TopLevel::Application, SubLevel::Json, vec![])
        ),
        qitem("application/vnd.github.v3+json".parse().unwrap()),
        qitem(
            "application/vnd.github.machine-man-preview+json"
                .parse()
                .unwrap()
        ),
    ])).header(UserAgent(USER_AGENT.to_string()));
    let req = match token {
        Some(token) => req.header(Authorization(Bearer { token: token.to_string() })),
        None => req,
    };
    req.send().map_err(HubError::HttpClient)
}

fn hyper_client() -> hyper::Client {
    let ssl = OpensslClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = hyper::Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client.set_write_timeout(Some(Duration::from_millis(HTTP_TIMEOUT)));
    client
}
