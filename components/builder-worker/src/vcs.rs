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
use url::Url;

use error::{Error, Result};

pub struct VCS {
    pub vcs_type: String,
    pub data: String,
    pub auth_token: Option<String>,
    pub username: Option<String>,
}

impl VCS {
    pub fn new(
        vcs_type: String,
        data: String,
        auth_token: Option<String>,
        username: Option<String>,
    ) -> VCS {
        VCS {
            vcs_type: vcs_type,
            data: data,
            auth_token: auth_token,
            username: username,
        }
    }

    pub fn clone(&self, path: &Path) -> Result<()> {
        match self.vcs_type.as_ref() {
            "git" => {
                debug!(
                    "cloning git repository, url={}, path={:?}",
                    self.url()?,
                    path
                );
                git2::Repository::clone(&(self.url()?).as_str(), path)?;
                Ok(())
            }
            _ => panic!("Unknown vcs type"),
        }
    }

    pub fn url(&self) -> Result<Url> {
        let mut url = Url::parse(self.data.as_str()).map_err(
            |e| Error::UrlParseError(e),
        )?;
        if self.data.starts_with("https://") {
            let mut cred_parts = 0;
            if let Some(ref username) = self.username {
                url.set_username(username).map_err(
                    |_| Error::CannotAddCreds,
                )?;
                cred_parts += 1;
            }
            if let Some(ref token) = self.auth_token {
                url.set_password(Some(token.as_str())).map_err(|_| {
                    Error::CannotAddCreds
                })?;
                cred_parts += 1;
            }
            if cred_parts == 1 {
                return Err(Error::IncompleteCredentials);
            }
        } else {
            return Err(Error::NotHTTPSCloneUrl(url));
        }
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::Error;

    #[test]
    fn build_url_with_auth_token() {
        let vcs = VCS::new(
            String::from("git"),
            String::from("https://github.com/owner/repo"),
            Some(String::from("token")),
            Some(String::from("user")),
        );
        assert_eq!(
            vcs.url().unwrap().as_str(),
            "https://user:token@github.com/owner/repo"
        );
    }

    #[test]
    fn build_url_with_no_auth_token() {
        let vcs = VCS::new(
            String::from("git"),
            String::from("https://github.com/owner/repo"),
            None,
            None,
        );
        assert_eq!(vcs.url().unwrap().as_str(), "https://github.com/owner/repo");
    }

    #[test]
    fn err_when_url_is_not_https() {
        let vcs = VCS::new(
            String::from("git"),
            String::from("git://github.com/owner/repo"),
            None,
            None,
        );
        match vcs.url() {
            Err(Error::NotHTTPSCloneUrl(_)) => {}
            Ok(_) | Err(_) => panic!("did not return a NotHTTPSCloneUrl err"),
        }
    }

    #[test]
    fn err_when_url_is_invalid() {
        let vcs = VCS::new(
            String::from("git"),
            String::from("blergyblargy"),
            None,
            None,
        );
        match vcs.url() {
            Err(Error::UrlParseError(_)) => {}
            Ok(_) | Err(_) => panic!("did not return a UrlParseError err"),
        }
    }

    #[test]
    fn err_when_only_username_set() {
        let vcs = VCS::new(
            String::from("git"),
            String::from("https://github.com/owner/repo"),
            None,
            Some(String::from("user")),
        );
        match vcs.url() {
            Err(Error::IncompleteCredentials) => {}
            Ok(_) | Err(_) => panic!("did not return a IncompleteCredentials err"),
        }
    }

    #[test]
    fn err_when_only_token_set() {
        let vcs = VCS::new(
            String::from("git"),
            String::from("https://github.com/owner/repo"),
            Some(String::from("token")),
            None,
        );
        match vcs.url() {
            Err(Error::IncompleteCredentials) => {}
            Ok(_) | Err(_) => panic!("did not return a IncompleteCredentials err"),
        }
    }
}
