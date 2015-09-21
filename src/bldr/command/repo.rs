//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

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


/// Starts the repository.
///
/// # Failures
///
/// * Fails if the repository fails to start - canot bind to the port, etc.
pub fn start(config: &Config) -> BldrResult<()> {
    repo::run(&config)
}
