// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use hab_core::package::PackageArchive;

use super::Job;
use error::Result;

pub struct Workspace {
    pub job: Job,
    out: PathBuf,
    src: PathBuf,
    studio: PathBuf,
    root: PathBuf,
}

impl Workspace {
    pub fn new(data_path: String, job: Job) -> Self {
        let root = PathBuf::from(data_path).join(job.get_id().to_string());
        Workspace {
            job: job,
            out: root.join("out"),
            src: root.join("src"),
            studio: root.join("studio"),
            root: root,
        }
    }

    /// Returns a `PackageArchive` representing the last built artifact from studio build
    pub fn last_built(&self) -> Result<PackageArchive> {
        let build = try!(LastBuild::from_file(self.out().join("last_build.env")));
        Ok(PackageArchive::new(self.out().join(build.pkg_artifact)))
    }

    /// Directory to the output directory containing built artifacts from studio build
    pub fn out(&self) -> &Path {
        &self.out
    }

    /// Root directory of the workspace
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Directory containing cloned source for the build
    pub fn src(&self) -> &Path {
        &self.src
    }

    /// Directory containing the studio for the build
    pub fn studio(&self) -> &Path {
        &self.studio
    }
}

#[derive(Debug)]
pub struct LastBuild {
    pub pkg_origin: String,
    pub pkg_name: String,
    pub pkg_version: String,
    pub pkg_release: String,
    pub pkg_ident: String,
    pub pkg_artifact: String,
    pub pkg_sha256sum: String,
    pub pkg_blake2bsum: String,
}

impl LastBuild {
    pub fn from_file<S: AsRef<Path>>(path: S) -> Result<Self> {
        let mut build = LastBuild::default();
        let mut buf: Vec<u8> = vec![];
        let mut f = try!(File::open(path));
        try!(f.read_to_end(&mut buf));
        Self::parse_into(&mut build, &buf);
        Ok(build)
    }

    pub fn parse_into(env: &mut LastBuild, buf: &[u8]) {
        let content = String::from_utf8_lossy(buf).into_owned();
        for line in content.lines() {
            let split: Vec<&str> = line.split("=").map(|e| e.trim()).collect();
            match split[0] {
                "pkg_origin" => env.pkg_origin = split[1].to_string(),
                "pkg_name" => env.pkg_name = split[1].to_string(),
                "pkg_version" => env.pkg_version = split[1].to_string(),
                "pkg_release" => env.pkg_release = split[1].to_string(),
                "pkg_ident" => env.pkg_ident = split[1].to_string(),
                "pkg_artifact" => env.pkg_artifact = split[1].to_string(),
                "pkg_sha256sum" => env.pkg_sha256sum = split[1].to_string(),
                "pkg_blake2bsum" => env.pkg_blake2bsum = split[1].to_string(),
                field => warn!("unknown field={}", field),
            }
        }
    }
}

impl Default for LastBuild {
    fn default() -> Self {
        LastBuild {
            pkg_origin: "".to_string(),
            pkg_name: "".to_string(),
            pkg_version: "".to_string(),
            pkg_release: "".to_string(),
            pkg_ident: "".to_string(),
            pkg_artifact: "".to_string(),
            pkg_sha256sum: "".to_string(),
            pkg_blake2bsum: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENV: &'static str = "
    pkg_origin=core
    pkg_name=valgrind
    pkg_version=3.12.0
    pkg_release=20161031181251
    pkg_ident=core/valgrind/3.12.0/20161031181251
    pkg_artifact=core-valgrind-3.12.0-20161031181251-x86_64-linux.hart
    pkg_sha256sum=3aeacaca8cf8274740863caae350f545cf97b15c79bdf6f873c0811b1a1ffbcf
    pkg_blake2bsum=3b38af666a8f307b89ae47ff098cb75503ee15892d1a8a98d0ae24da1cfd153b
    ";

    #[test]
    fn parse_last_env_file() {
        let mut build = LastBuild::default();
        LastBuild::parse_into(&mut build, ENV.as_bytes());
        assert_eq!(build.pkg_origin, "core");
        assert_eq!(build.pkg_name, "valgrind");
        assert_eq!(build.pkg_version, "3.12.0");
        assert_eq!(build.pkg_ident, "core/valgrind/3.12.0/20161031181251");
        assert_eq!(build.pkg_artifact,
                   "core-valgrind-3.12.0-20161031181251-x86_64-linux.hart");
        assert_eq!(build.pkg_sha256sum,
                   "3aeacaca8cf8274740863caae350f545cf97b15c79bdf6f873c0811b1a1ffbcf");
        assert_eq!(build.pkg_blake2bsum,
                   "3b38af666a8f307b89ae47ff098cb75503ee15892d1a8a98d0ae24da1cfd153b");
    }
}
