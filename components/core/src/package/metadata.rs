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

use std::fmt;
use std::str::FromStr;

use error::{Error, Result};

#[derive(Debug)]
pub struct Bind {
    pub service: String,
    pub exports: Vec<String>,
}

impl FromStr for Bind {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split('=');
        let service = match parts.next() {
            None => return Err(Error::MetaFileBadBind),
            Some(service) => service.to_string(),
        };
        let exports = match parts.next() {
            None => return Err(Error::MetaFileBadBind),
            Some(exports) => exports.split(' ').map(|t| t.to_string()).collect(),
        };
        Ok(Bind {
               service: service,
               exports: exports,
           })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum MetaFile {
    Binds,
    BindsOptional,
    CFlags,
    Config,
    Deps,
    TDeps,
    Environment,
    EnvironmentSep,
    Exports,
    Exposes,
    Ident,
    LdRunPath,
    LdFlags,
    Manifest,
    Path,
    SvcUser,
    SvcGroup,
    Target,
}

impl fmt::Display for MetaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match *self {
            MetaFile::Binds => "BINDS",
            MetaFile::BindsOptional => "BINDS_OPTIONAL",
            MetaFile::CFlags => "CFLAGS",
            MetaFile::Config => "default.toml",
            MetaFile::Deps => "DEPS",
            MetaFile::TDeps => "TDEPS",
            MetaFile::Environment => "ENVIRONMENT",
            MetaFile::EnvironmentSep => "ENVIRONMENT_SEP",
            MetaFile::Exports => "EXPORTS",
            MetaFile::Exposes => "EXPOSES",
            MetaFile::Ident => "IDENT",
            MetaFile::LdRunPath => "LD_RUN_PATH",
            MetaFile::LdFlags => "LDFLAGS",
            MetaFile::Manifest => "MANIFEST",
            MetaFile::Path => "PATH",
            MetaFile::SvcUser => "SVC_USER",
            MetaFile::SvcGroup => "SVC_GROUP",
            MetaFile::Target => "TARGET",
        };
        write!(f, "{}", id)
    }
}
