// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_builder_dbcache as dbcache;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
#[macro_use]
extern crate bitflags;
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
extern crate protobuf;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
#[macro_use]
extern crate router;
extern crate rustc_serialize;
extern crate time;
extern crate toml;
extern crate unicase;
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

use std::sync::{Arc, Mutex};
use std::fs;
use std::path::{Path, PathBuf};

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use hab_core::package::{Identifiable, PackageArchive};
use data_store::DataStore;

use hab_net::oauth::github::GitHubClient;
use hab_net::server::NetIdent;

pub struct Depot {
    pub config: Config,
    pub datastore: DataStore,
    context: Arc<Mutex<zmq::Context>>,
    github: GitHubClient,
}

impl Depot {
    pub fn new(config: Config, ctx: Arc<Mutex<zmq::Context>>) -> Result<Arc<Depot>> {
        let datastore = try!(DataStore::open(&config));
        let github = GitHubClient::new(&config);
        Ok(Arc::new(Depot {
            config: config,
            datastore: datastore,
            context: ctx,
            github: github,
        }))
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

impl NetIdent for Depot {}
