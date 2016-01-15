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
use error::BldrResult;
use repo;

static LOGKEY: &'static str = "CR";

/// Starts the repository.
///
/// # Failures
///
/// * Fails if the repository fails to start - canot bind to the port, etc.
pub fn start(config: &Config) -> BldrResult<()> {
    outputln!("Repo listening on {:?}", config.repo_addr());
    repo::run(&config)
}
