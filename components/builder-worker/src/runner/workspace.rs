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

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use hab_core::package::{PackageArchive, PackageIdent};

use super::Job;
use error::{Error, Result};

pub struct Workspace {
    pub job: Job,
    out: PathBuf,
    src: PathBuf,
    studio: PathBuf,
    root: PathBuf,
}

impl Workspace {
    pub fn new<T>(data_path: T, job: Job) -> Self
    where
        T: AsRef<Path>,
    {
        let root = data_path.as_ref().join(job.get_id().to_string());
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
        let last_build = self.last_build_env();
        match StudioBuild::from_file(&last_build) {
            Ok(build) => {
                Ok(PackageArchive::new(
                    self.out().join(build.pkg_artifact.unwrap()),
                ))
            }
            Err(err) => Err(Error::BuildEnvFile(last_build, err)),
        }
    }

    /// Returns a `PackageIdent` representing the artifact that the studio attempted to build
    pub fn attempted_build(&self) -> Result<PackageIdent> {
        let last_build = self.pre_build_env();
        match StudioBuild::from_file(&last_build) {
            Ok(build) => {
                Ok(PackageIdent::new(
                    build.pkg_origin,
                    build.pkg_name,
                    Some(build.pkg_version),
                    Some(build.pkg_release),
                ))
            }
            Err(err) => Err(Error::BuildEnvFile(last_build, err)),
        }
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

    fn last_build_env(&self) -> PathBuf {
        self.out().join("last_build.env")
    }

    fn pre_build_env(&self) -> PathBuf {
        self.out().join("pre_build.env")
    }
}

#[derive(Debug)]
pub struct StudioBuild {
    pub pkg_origin: String,
    pub pkg_name: String,
    pub pkg_version: String,
    pub pkg_release: String,
    pub pkg_ident: String,
    pub pkg_artifact: Option<String>,
    pub pkg_sha256sum: Option<String>,
    pub pkg_blake2bsum: Option<String>,
}

impl StudioBuild {
    pub fn from_file<S>(path: S) -> io::Result<Self>
    where
        S: AsRef<Path>,
    {
        let mut build = StudioBuild::default();
        let mut buf: Vec<u8> = vec![];
        let mut f = File::open(path)?;
        f.read_to_end(&mut buf)?;
        Self::parse_into(&mut build, &buf);
        Ok(build)
    }

    pub fn parse_into(env: &mut StudioBuild, buf: &[u8]) {
        let content = String::from_utf8_lossy(buf).into_owned();
        for line in content.lines() {
            let split: Vec<&str> = line.split("=").map(|e| e.trim()).collect();
            match split[0] {
                "pkg_origin" => env.pkg_origin = split[1].to_string(),
                "pkg_name" => env.pkg_name = split[1].to_string(),
                "pkg_version" => env.pkg_version = split[1].to_string(),
                "pkg_release" => env.pkg_release = split[1].to_string(),
                "pkg_ident" => env.pkg_ident = split[1].to_string(),
                "pkg_artifact" => env.pkg_artifact = Some(split[1].to_string()),
                "pkg_sha256sum" => env.pkg_sha256sum = Some(split[1].to_string()),
                "pkg_blake2bsum" => env.pkg_blake2bsum = Some(split[1].to_string()),
                field => warn!("unknown field={}", field),
            }
        }
    }
}

impl Default for StudioBuild {
    fn default() -> Self {
        StudioBuild {
            pkg_origin: "".to_string(),
            pkg_name: "".to_string(),
            pkg_version: "".to_string(),
            pkg_release: "".to_string(),
            pkg_ident: "".to_string(),
            pkg_artifact: None,
            pkg_sha256sum: None,
            pkg_blake2bsum: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAST_BUILD: &'static str = "
    pkg_origin=core
    pkg_name=valgrind
    pkg_version=3.12.0
    pkg_release=20161031181251
    pkg_ident=core/valgrind/3.12.0/20161031181251
    pkg_artifact=core-valgrind-3.12.0-20161031181251-x86_64-linux.hart
    pkg_sha256sum=3aeacaca8cf8274740863caae350f545cf97b15c79bdf6f873c0811b1a1ffbcf
    pkg_blake2bsum=3b38af666a8f307b89ae47ff098cb75503ee15892d1a8a98d0ae24da1cfd153b
    ";

    const PRE_BUILD: &'static str = "
    pkg_origin=core
    pkg_name=redis
    pkg_version=3.2.4
    pkg_release=20170817102134
    pkg_ident=core/redis/3.2.4/20170817102134
    ";

    #[test]
    fn parse_last_env_file() {
        let mut build = StudioBuild::default();
        StudioBuild::parse_into(&mut build, LAST_BUILD.as_bytes());
        assert_eq!(build.pkg_origin, "core");
        assert_eq!(build.pkg_name, "valgrind");
        assert_eq!(build.pkg_version, "3.12.0");
        assert_eq!(build.pkg_release, "20161031181251");
        assert_eq!(build.pkg_ident, "core/valgrind/3.12.0/20161031181251");
        assert_eq!(
            build.pkg_artifact,
            Some(
                "core-valgrind-3.12.0-20161031181251-x86_64-linux.hart".to_string(),
            )
        );
        assert_eq!(
            build.pkg_sha256sum,
            Some(
                "3aeacaca8cf8274740863caae350f545cf97b15c79bdf6f873c0811b1a1ffbcf".to_string(),
            )
        );
        assert_eq!(
            build.pkg_blake2bsum,
            Some(
                "3b38af666a8f307b89ae47ff098cb75503ee15892d1a8a98d0ae24da1cfd153b".to_string(),
            )
        );
    }

    #[test]
    fn parse_pre_build_env_file() {
        let mut build = StudioBuild::default();
        StudioBuild::parse_into(&mut build, PRE_BUILD.as_bytes());
        assert_eq!(build.pkg_origin, "core");
        assert_eq!(build.pkg_name, "redis");
        assert_eq!(build.pkg_version, "3.2.4");
        assert_eq!(build.pkg_release, "20170817102134");
        assert_eq!(build.pkg_ident, "core/redis/3.2.4/20170817102134");
        assert_eq!(build.pkg_artifact, None);
        assert_eq!(build.pkg_sha256sum, None);
        assert_eq!(build.pkg_blake2bsum, None);
    }
}
