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

pub mod archive;
pub mod ident;
pub mod install;
pub mod plan;

pub use self::archive::{FromArchive, PackageArchive};
pub use self::ident::{Identifiable, PackageIdent};
pub use self::install::PackageInstall;
pub use self::plan::Plan;

use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum MetaFile {
    CFlags,
    Config,
    Deps,
    TDeps,
    Exposes,
    Ident,
    LdRunPath,
    LdFlags,
    Manifest,
    Path,
    SvcUser,
    SvcGroup,
}

impl fmt::Display for MetaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match *self {
            MetaFile::CFlags => "CFLAGS",
            MetaFile::Config => "default.toml",
            MetaFile::Deps => "DEPS",
            MetaFile::TDeps => "TDEPS",
            MetaFile::Exposes => "EXPOSES",
            MetaFile::Ident => "IDENT",
            MetaFile::LdRunPath => "LD_RUN_PATH",
            MetaFile::LdFlags => "LDFLAGS",
            MetaFile::Manifest => "MANIFEST",
            MetaFile::Path => "PATH",
            MetaFile::SvcUser => "SVC_USER",
            MetaFile::SvcGroup => "SVC_GROUP",
        };
        write!(f, "{}", id)
    }
}
