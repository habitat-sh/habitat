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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate habitat_builder_protocol as protocol;
#[macro_use]
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
extern crate builder_core as bld_core;
extern crate bodyparser;
extern crate crypto;
extern crate hyper;
extern crate iron;
extern crate iron_test;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mount;
extern crate persistent;
extern crate protobuf;
extern crate regex;
extern crate r2d2;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tempfile;
extern crate time;
extern crate toml;
extern crate unicase;
extern crate url;
extern crate walkdir;
extern crate zmq;
extern crate uuid;
extern crate base64;

pub mod config;
pub mod error;
pub mod doctor;
pub mod server;
pub mod handlers;

pub use self::config::Config;
pub use self::error::{Error, Result};

use std::fs;
use std::path::{Path, PathBuf};

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use hab_core::package::{Identifiable, PackageArchive, PackageTarget};
use hab_net::server::NetIdent;
use iron::typemap;

pub struct DepotUtil {
    pub config: Config,
}

impl DepotUtil {
    pub fn new(config: Config) -> DepotUtil {
        DepotUtil { config: config }
    }

    // Return a PackageArchive representing the given package. None is returned if the Depot
    // doesn't have an archive for the given package.
    fn archive<T: Identifiable>(
        &self,
        ident: &T,
        target: &PackageTarget,
    ) -> Option<PackageArchive> {
        let file = self.archive_path(ident, target);
        match fs::metadata(&file) {
            Ok(_) => Some(PackageArchive::new(file)),
            Err(_) => {
                warn!("Package not found at {:?}", file);
                None
            }
        }
    }

    // Return a formatted string representing the filename of an archive for the given package
    // identifier pieces.
    fn archive_path<T: Identifiable>(&self, ident: &T, target: &PackageTarget) -> PathBuf {
        let mut digest = Sha256::new();
        let mut output = [0; 64];
        digest.input_str(&ident.to_string());
        digest.result(&mut output);
        self.packages_path()
            .join(format!("{:x}", output[0]))
            .join(format!("{:x}", output[1]))
            .join(format!(
                "{}-{}-{}-{}-{}-{}.hart",
                ident.origin(),
                ident.name(),
                ident.version().unwrap(),
                ident.release().unwrap(),
                target.architecture,
                target.platform
            ))
    }

    // Return a formatted string representing the folder location for an archive.
    fn archive_parent<T: Identifiable>(&self, ident: &T) -> PathBuf {
        let mut digest = Sha256::new();
        let mut output = [0; 64];
        digest.input_str(&ident.to_string());
        digest.result(&mut output);
        self.packages_path().join(format!("{:x}", output[0])).join(
            format!("{:x}", output[1]),
        )
    }

    fn packages_path(&self) -> PathBuf {
        Path::new(&self.config.path).join("pkgs")
    }
}

impl typemap::Key for DepotUtil {
    type Value = Self;
}

impl NetIdent for DepotUtil {}
