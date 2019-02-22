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

use base64::DecodeError;
use rusoto_ecr::GetAuthorizationTokenError;
use std::{process::ExitStatus,
          result,
          string::FromUtf8Error};

use failure;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Base64DecodeError(DecodeError),
    #[fail(display = "Docker build failed with exit code: {}", _0)]
    BuildFailed(ExitStatus),
    #[fail(display = "Could not determine Docker image ID for image: {}", _0)]
    DockerImageIdNotFound(String),
    #[fail(
        display = "Switch to Windows containers to export Docker images on Windows. Current \
                   Docker Server OS is set to: {}",
        _0
    )]
    DockerNotInWindowsMode(String),
    #[fail(display = "Invalid registry type: {}", _0)]
    InvalidRegistryType(String),
    #[fail(display = "{}", _0)]
    InvalidToken(FromUtf8Error),
    #[fail(display = "Docker login failed with exit code: {}", _0)]
    LoginFailed(ExitStatus),
    #[fail(display = "Docker logout failed with exit code: {}", _0)]
    LogoutFailed(ExitStatus),
    #[fail(display = "No ECR Tokens returned")]
    NoECRTokensReturned,
    #[fail(display = "{}", _0)]
    TokenFetchFailed(GetAuthorizationTokenError),
    #[fail(
        display = "A primary service package could not be determined from: {:?}. At least one \
                   package with a run hook must be provided.",
        _0
    )]
    PrimaryServicePackageNotFound(Vec<String>),
    #[fail(display = "Docker image push failed with exit code: {}", _0)]
    PushImageFailed(ExitStatus),
    #[fail(display = "Removing Docker local images failed with exit code: {}", _0)]
    RemoveImageFailed(ExitStatus),
}
