// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use env as henv;

/// Default Depot URL
pub const DEFAULT_DEPOT_URL: &'static str = "http://willem.habitat.sh:9636/v1/depot";

/// Default Depot URL environment variable
pub const DEPOT_URL_ENVVAR: &'static str = "HAB_DEPOT_URL";

pub fn default_depot_url() -> String {
    match henv::var(DEPOT_URL_ENVVAR) {
        Ok(val) => val,
        Err(_) => DEFAULT_DEPOT_URL.to_string(),
    }
}
