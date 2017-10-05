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

use std::path::Path;

use git2;
use github_api_client::{GitHubClient, GitHubCfg};
use url::Url;

use error::{Error, Result};

pub struct VCS {
    pub vcs_type: String,
    pub data: String,
    pub github_client: GitHubClient,
    pub installation_id: Option<u32>,
}

impl VCS {
    pub fn new(
        vcs_type: String,
        data: String,
        config: GitHubCfg,
        installation_id: Option<u32>,
    ) -> Self {
        VCS {
            vcs_type: vcs_type,
            data: data,
            github_client: GitHubClient::new(config),
            installation_id: installation_id,
        }
    }

    pub fn clone(&self, path: &Path) -> Result<()> {
        match self.vcs_type.as_ref() {
            "git" => {
                let token = match self.installation_id {
                    None => None,
                    Some(id) => {
                        Some(self.github_client.app_installation_token(id).map_err(|e| {
                            Error::GithubAppAuthErr(e)
                        })?)
                    }
                };
                debug!(
                    "cloning git repository, url={}, path={:?}",
                    self.url(token.clone())?,
                    path
                );
                git2::Repository::clone(&(self.url(token)?).as_str(), path)
                    .map_err(|e| Error::Git(e))?;
                Ok(())
            }
            _ => panic!("Unknown vcs type"),
        }
    }

    pub fn url(&self, token: Option<String>) -> Result<Url> {
        let mut url = Url::parse(self.data.as_str()).map_err(
            |e| Error::UrlParseError(e),
        )?;
        if self.data.starts_with("https://") {
            if let Some(ref tok) = token {
                url.set_username("x-access-token").map_err(
                    |_| Error::CannotAddCreds,
                )?;
                url.set_password(Some(tok.as_str())).map_err(|_| {
                    Error::CannotAddCreds
                })?;
            }
        } else {
            return Err(Error::NotHTTPSCloneUrl(url));
        }
        Ok(url)
    }
}
