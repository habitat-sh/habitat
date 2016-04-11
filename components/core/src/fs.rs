// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::path::PathBuf;

pub const ROOT_PATH: &'static str = "/opt/bldr";
pub const PACKAGE_HOME: &'static str = "/opt/bldr/pkgs";
pub const SERVICE_HOME: &'static str = "/opt/bldr/svc";
pub const PACKAGE_CACHE: &'static str = "/opt/bldr/cache/pkgs";
/// The default path where source artifacts are downloaded, extracted, & compiled
pub const CACHE_SRC_PATH: &'static str = "/opt/bldr/cache/src";
pub const GPG_CACHE: &'static str = "/opt/bldr/cache/gpg";
pub const KEY_CACHE: &'static str = "/opt/bldr/cache/keys";

pub fn service_path(service_name: &str) -> PathBuf {
    PathBuf::from(SERVICE_HOME).join(service_name)
}
