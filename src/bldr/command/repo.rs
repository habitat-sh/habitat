// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Runs a bldr package repository.
//!
//! The repository is an HTTP service that runs on port `9632`.
//!
//! Look in the [repo](../../repo) module for more information on how the service itself operates.
//!
//! # Examples
//!
//! ```bash
//! $ bldr repo
//! ```
//!
//! Starts a bldr repository, with the data stored in `/opt/bldr/srvc/bldr/data`.
//!
//! ```bash
//! $ bldr repo -p /tmp/whatever
//! ```
//!
//! Does the same, but the data is stored in `/tmp/whatever`.

use config::Config;
use error::{BldrError, BldrResult, ErrorKind};
use repo::{self, data_object};
use repo::data_store::{self, Cursor, Database, Transaction};

static LOGKEY: &'static str = "CR";

/// Create a repository with the given name in the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A write transaction cannot be acquired.
pub fn create_repository(name: &str, config: &Config) -> BldrResult<()> {
    let repo = try!(repo::Repo::new(String::from(config.path())));
    let txn = try!(repo.datastore.views.txn_rw());
    let object = data_object::View::new(name);
    try!(repo.datastore.views.write(&txn, &object));
    Ok(())
}

/// List all repositories in the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A read transaction cannot be acquired.
pub fn list_repositories(config: &Config) -> BldrResult<()> {
    let repo = try!(repo::Repo::new(String::from(config.path())));
    let mut views: Vec<data_object::View> = vec![];
    let txn = try!(repo.datastore.views.txn_ro());
    let mut cursor = try!(txn.cursor_ro());
    match cursor.first() {
        Err(BldrError {err: ErrorKind::MdbError(data_store::MdbError::NotFound), ..}) => {
            outputln!("No repositories. Create one with `bldr repo-create`.");
            return Ok(());
        }
        Err(e) => return Err(e),
        Ok(value) => views.push(value),
    }
    loop {
        match cursor.next() {
            Ok((_, value)) => views.push(value),
            Err(_) => break,
        }
    }
    outputln!("Listing {} repositories", views.len());
    for view in views.iter() {
        outputln!("     {}", view);
    }
    Ok(())
}

/// Starts the depot server.
///
/// # Failures
///
/// * Fails if the depot server fails to start - canot bind to the port, etc.
pub fn start(config: &Config) -> BldrResult<()> {
    outputln!("Repo listening on {:?}", config.repo_addr());
    repo::run(&config)
}

/// Analyzes the integrity of the depot's metadata by comparing the metadata with the packages
/// on disk. If a package is found on disk that is not present in the metadata it is added to the
/// metadata and if an entry in the metadata doesn't have a matching package archive on disk the
/// entry is dropped from the database.
///
/// # Failures
///
/// * The database cannot be read
/// * A write transaction cannot be acquired
pub fn repair(config: &Config) -> BldrResult<()> {
    outputln!("Repairing repo at {:?}", config.path());
    repo::repair(&config)
}
