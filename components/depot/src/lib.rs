// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_builder_dbcache as dbcache;
extern crate habitat_core as hcore;
extern crate habitat_depot_core as depot_core;
#[macro_use]
extern crate bitflags;
extern crate crypto;
#[macro_use]
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
#[macro_use]
extern crate router;
extern crate rustc_serialize;
extern crate time;
extern crate urlencoded;
extern crate walkdir;

pub mod config;
pub mod error;
pub mod data_store;
pub mod doctor;
pub mod server;

pub use self::config::Config;
pub use self::error::{Error, Result};

use std::net;
use std::sync::Arc;
use std::fs;
use std::path::{Path, PathBuf};

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use hcore::package::{self, PackageArchive};

use data_store::DataStore;

pub struct Depot {
    pub config: Config,
    pub datastore: DataStore,
}

impl Depot {
    pub fn new(config: Config) -> Result<Arc<Depot>> {
        let datastore = try!(DataStore::open(&config));
        Ok(Arc::new(Depot {
            config: config,
            datastore: datastore,
        }))
    }

    // Return a PackageArchive representing the given package. None is returned if the Depot
    // doesn't have an archive for the given package.
    fn archive<P: AsRef<package::PackageIdent>>(&self, ident: P) -> Option<PackageArchive> {
        let file = self.archive_path(ident);
        match fs::metadata(&file) {
            Ok(_) => Some(PackageArchive::new(file)),
            Err(_) => None,
        }
    }

    // Return a formatted string representing the filename of an archive for the given package
    // identifier pieces.
    fn archive_path<T: AsRef<package::PackageIdent>>(&self, ident: T) -> PathBuf {
        let ident = ident.as_ref();
        let mut digest = Sha256::new();
        let mut output = [0; 64];
        digest.input_str(&ident.to_string());
        digest.result(&mut output);
        self.packages_path()
            .join(format!("{:x}", output[0]))
            .join(format!("{:x}", output[1]))
            .join(format!("{}-{}-{}-{}.hart",
                          &ident.origin,
                          &ident.name,
                          ident.version.as_ref().unwrap(),
                          ident.release.as_ref().unwrap()))
    }

    fn key_path(&self, name: &str) -> PathBuf {
        self.keys_path().join(format!("{}.asc", name))
    }

    fn keys_path(&self) -> PathBuf {
        Path::new(&self.config.path).join("keys")
    }

    fn packages_path(&self) -> PathBuf {
        Path::new(&self.config.path).join("pkgs")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ListenAddr(pub net::Ipv4Addr);
#[derive(Debug, PartialEq, Eq)]
pub struct ListenPort(pub u16);

impl Default for ListenAddr {
    fn default() -> Self {
        ListenAddr(net::Ipv4Addr::new(0, 0, 0, 0))
    }
}

impl Default for ListenPort {
    fn default() -> Self {
        ListenPort(9632)
    }
}
