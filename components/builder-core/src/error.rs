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

use std::error;
use std::fmt;
use std::result;
use std::string;

use base64;
use hab_core;
use github_api_client;
use git2;
use url;

#[derive(Debug)]
pub enum Error {
    Base64Error(base64::DecodeError),
    CannotAddCreds,
    DecryptError(String),
    EncryptError(String),
    FromUtf8Error(string::FromUtf8Error),
    HabitatCore(hab_core::Error),
    Git(git2::Error),
    GithubAppAuthErr(github_api_client::HubError),
    NotHTTPSCloneUrl(url::Url),
    UrlParseError(url::ParseError),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Base64Error(ref e) => format!("{}", e),
            Error::CannotAddCreds => format!("Cannot add credentials to url"),
            Error::DecryptError(ref e) => format!("{}", e),
            Error::EncryptError(ref e) => format!("{}", e),
            Error::FromUtf8Error(ref e) => format!("{}", e),
            Error::Git(ref e) => format!("{}", e),
            Error::GithubAppAuthErr(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::NotHTTPSCloneUrl(ref e) => {
                format!(
                    "Attempted to clone {}. Only HTTPS clone urls are supported",
                    e
                )
            }
            Error::UrlParseError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Base64Error(ref e) => e.description(),
            Error::CannotAddCreds => "Cannot add credentials to url",
            Error::DecryptError(_) => "Error decrypting integration",
            Error::EncryptError(_) => "Error encrypting integration",
            Error::FromUtf8Error(ref e) => e.description(),
            Error::Git(ref err) => err.description(),
            Error::GithubAppAuthErr(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::NotHTTPSCloneUrl(_) => "Only HTTPS clone urls are supported",
            Error::UrlParseError(ref err) => err.description(),
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}
