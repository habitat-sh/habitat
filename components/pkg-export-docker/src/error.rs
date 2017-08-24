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

use std::fmt;
use std::io;
use std::result;

use common;
use hab;
use handlebars;
use hcore;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DockerImageIdNotFound(String),
    Hab(hab::error::Error),
    HabitatCommon(common::Error),
    HabitatCore(hcore::Error),
    PrimaryServicePackageNotFound(Vec<String>),
    TemplateRenderError(handlebars::TemplateRenderError),
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::DockerImageIdNotFound(ref image_tag) => {
                format!(
                    "Could not determine Docker image ID for image: {}",
                    image_tag
                )
            }
            Error::Hab(ref err) => format!("{}", err),
            Error::HabitatCommon(ref err) => format!("{}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::PrimaryServicePackageNotFound(ref idents) => {
                format!(
                    "A primary service package could not be determined from: {}. \
                    At least one package with a run hook must be provided.",
                    idents.join(", ")
                )
            }
            Error::TemplateRenderError(ref err) => format!("{}", err),
            Error::IO(ref err) => format!("{}", err),
        };
        write!(f, "{}", msg)
    }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Self {
        Error::HabitatCommon(err)
    }
}

impl From<hab::error::Error> for Error {
    fn from(err: hab::error::Error) -> Self {
        Error::Hab(err)
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Self {
        Error::TemplateRenderError(err)
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Self {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}
