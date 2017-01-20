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

extern crate habitat_builder_dbcache as dbcache;
extern crate habitat_builder_protocol as protocol;
#[macro_use]
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
#[macro_use]
extern crate builder_core as bld_core;
extern crate bodyparser;
extern crate crypto;
#[macro_use]
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mount;
extern crate persistent;
extern crate protobuf;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
#[macro_use]
extern crate router;
extern crate rustc_serialize;
extern crate serde_json;
extern crate time;
extern crate toml;
extern crate unicase;
extern crate url;
extern crate urlencoded;
extern crate walkdir;
extern crate zmq;

pub mod config;
pub mod error;
pub mod data_store;
pub mod doctor;
pub mod server;

pub use self::config::Config;
pub use self::error::{Error, Result};

use std::fs;
use std::path::{Path, PathBuf};

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use hab_core::package::{Identifiable, PackageArchive};
use hab_net::server::NetIdent;
use iron::typemap;

use data_store::DataStore;

pub struct Depot {
    pub config: Config,
    pub datastore: DataStore,
}

impl Depot {
    pub fn new(config: Config) -> Result<Depot> {
        let datastore = try!(DataStore::open(&config));
        Ok(Depot {
            config: config,
            datastore: datastore,
        })
    }

    // Return a PackageArchive representing the given package. None is returned if the Depot
    // doesn't have an archive for the given package.
    fn archive<T: Identifiable>(&self, ident: &T) -> Option<PackageArchive> {
        let file = self.archive_path(ident);
        match fs::metadata(&file) {
            Ok(_) => Some(PackageArchive::new(file)),
            Err(_) => None,
        }
    }

    // Return a formatted string representing the filename of an archive for the given package
    // identifier pieces.
    fn archive_path<T: Identifiable>(&self, ident: &T) -> PathBuf {
        let mut digest = Sha256::new();
        let mut output = [0; 64];
        digest.input_str(&ident.to_string());
        digest.result(&mut output);
        self.packages_path()
            .join(format!("{:x}", output[0]))
            .join(format!("{:x}", output[1]))
            .join(format!("{}-{}-{}-{}-x86_64-linux.hart",
                          ident.origin(),
                          ident.name(),
                          ident.version().unwrap(),
                          ident.release().unwrap()))
    }

    fn key_path(&self, key: &str, rev: &str) -> PathBuf {
        let mut digest = Sha256::new();
        let mut output = [0; 64];
        let key_with_rev = format!("{}-{}.pub", key, rev);
        digest.input_str(&key_with_rev.to_string());
        digest.result(&mut output);
        self.keys_path()
            .join(format!("{:x}", output[0]))
            .join(format!("{:x}", output[1]))
            .join(format!("{}-{}.pub", key, rev))
    }

    fn keys_path(&self) -> PathBuf {
        Path::new(&self.config.path).join("keys")
    }

    fn packages_path(&self) -> PathBuf {
        Path::new(&self.config.path).join("pkgs")
    }
}

impl typemap::Key for Depot {
    type Value = Self;
}

impl NetIdent for Depot {}
