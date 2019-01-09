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

pub mod archive;
pub mod ident;
pub mod install;
pub mod list;
pub mod metadata;
pub mod plan;
pub mod target;

pub use self::archive::{FromArchive, PackageArchive};
pub use self::ident::{Identifiable, PackageIdent};
pub use self::install::PackageInstall;
pub use self::list::all_packages;
pub use self::plan::Plan;
pub use self::target::PackageTarget;

#[cfg(test)]
pub mod test_support {
    use super::metadata::MetaFile;
    use super::*;
    use crate::fs;
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use time;

    pub fn fixture_path(name: &str) -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name);
        path
    }

    /// Creates a minimal installed package under an fs_root and return a corresponding loaded
    /// `PackageInstall` suitable for testing against. The `IDENT` and `TARGET` metafiles are
    /// created and for the target system the tests are running on. Further subdirectories, files,
    /// and metafile can be created under this path.
    pub fn testing_package_install(ident: &str, fs_root: &Path) -> PackageInstall {
        fn write_file(path: &Path, content: &str) {
            let mut f = File::create(path).unwrap();
            f.write_all(content.as_bytes()).unwrap()
        }

        let mut pkg_ident = PackageIdent::from_str(ident).unwrap();
        if !pkg_ident.fully_qualified() {
            if let None = pkg_ident.version {
                pkg_ident.version = Some(String::from("1.0.0"));
            }
            if let None = pkg_ident.release {
                pkg_ident.release = Some(
                    time::now_utc()
                        .strftime("%Y%m%d%H%M%S")
                        .unwrap()
                        .to_string(),
                );
            }
        }
        let pkg_install_path = fs::pkg_install_path(&pkg_ident, Some(fs_root));

        create_dir_all(&pkg_install_path).unwrap();
        write_file(
            &pkg_install_path.join(MetaFile::Ident.to_string()),
            &pkg_ident.to_string(),
        );
        write_file(
            &pkg_install_path.join(MetaFile::Target.to_string()),
            PackageTarget::active_target(),
        );

        PackageInstall::load(&pkg_ident, Some(fs_root))
            .expect(&format!("PackageInstall should load for {}", &pkg_ident))
    }
}
