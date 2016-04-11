// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::path::PathBuf;

pub const ROOT_PATH: &'static str = "/opt/bldr";
/// The default download root path for package artifacts, used on package installation
pub const CACHE_ARTIFACT_PATH: &'static str = "/opt/bldr/cache/artifacts";
/// The default path where gpg keys are stored
pub const CACHE_GPG_PATH: &'static str = "/opt/bldr/cache/gpg";
/// The default path where cryptographic keys are stored
pub const CACHE_KEY_PATH: &'static str = "/opt/bldr/cache/keys";
/// The default path where source artifacts are downloaded, extracted, & compiled
pub const CACHE_SRC_PATH: &'static str = "/opt/bldr/cache/src";
/// The root path containing all locally installed packages
pub const PKG_PATH: &'static str = "/opt/bldr/pkgs";
/// The root path containing all runtime service directories and files
pub const SVC_PATH: &'static str = "/opt/bldr/svc";

pub fn service_path(service_name: &str) -> PathBuf {
    PathBuf::from(SVC_PATH).join(service_name)
}
