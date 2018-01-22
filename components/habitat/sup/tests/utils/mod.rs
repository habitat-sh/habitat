// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Utility functions for testing a Supervisor

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::string::ToString;
use std::thread;
use std::time::Duration;

use hcore::os::users;

pub mod fixture_root;
pub mod hab_root;
pub mod test_butterfly;
pub mod test_sup;

// Re-export the key structs of this package for ergonomics.
pub use self::fixture_root::FixtureRoot;
pub use self::hab_root::HabRoot;
pub use self::test_sup::TestSup;

/// Sleep for the specified number of seconds!
pub fn sleep_seconds(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds))
}

/// Copy fixture package files from `fixture_root` over to `hab_root`
/// in the appropriate places for the Supervisor to find them.
pub fn setup_package_files<O, P, S>(
    origin_name: O,
    package_name: P,
    service_group: S,
    fixture_root: &FixtureRoot,
    hab_root: &HabRoot,
) where
    O: ToString,
    P: ToString,
    S: ToString,
{
    let origin_name = origin_name.to_string();
    let package_name = package_name.to_string();
    let service_group = service_group.to_string();

    // Ensure the directory for the spec files exists
    let spec_dir = hab_root.spec_dir(&service_group);
    fs::create_dir_all(spec_dir).expect("could not create spec directory");

    // Copy the spec file over
    let spec_source = fixture_root.spec_path(&package_name);
    let spec_destination = hab_root.spec_path(&package_name, &service_group);
    assert!(
        spec_source.exists(),
        format!("Missing a spec file at {:?}", spec_source)
    );
    fs::copy(&spec_source, &spec_destination).expect(
        format!(
            "Could not copy {:?} to {:?}",
            spec_source,
            spec_destination
        ).as_str(),
    );

    // Copy the expanded package directory over
    let expanded_fixture_dir = fixture_root.expanded_package_dir(&package_name);
    let hab_pkg_path = hab_root.pkg_path(&origin_name, &package_name);
    copy_dir(&expanded_fixture_dir, &hab_pkg_path);
    write_default_svc_user_and_group_metafiles(&hab_root, &origin_name, &package_name);
}

/// Recursively copy the contents of `source_dir` into `dest_dir`
pub fn copy_dir<S, D>(source_dir: S, dest_dir: D)
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    let source_dir = source_dir.as_ref().to_path_buf();
    assert!(
        source_dir.exists(),
        format!("Source directory {:?} does not exist", source_dir)
    );
    assert!(
        source_dir.is_dir(),
        format!("Source directory {:?} is not a directory", source_dir)
    );
    let dest_dir = dest_dir.as_ref().to_path_buf();

    fs::create_dir_all(&dest_dir).expect(
        format!("Could not create directory {:?}", dest_dir).as_str(),
    );

    let source_dir_entries =
        fs::read_dir(&source_dir).expect(
            format!("Could not read entries in {:?}", source_dir).as_str(),
        );

    for entry in source_dir_entries {
        let source = entry.unwrap().path();
        let destination = &dest_dir.join(source.file_name().unwrap());

        if source.is_file() {
            fs::copy(&source, &destination).expect(
                format!(
                    "could not copy {:?} to {:?}",
                    source,
                    destination
                ).as_str(),
            );
        } else if source.is_dir() {
            copy_dir(&source, &destination);
        }
    }
}

/// Write default `SVC_USER` and `SVC_GROUP` package metafiles unless one is already present in
/// the target directory.
///
/// In an effort to execute a package when running test suites as a non-root user, the current
/// username and the user's primary groupname will be used. If a fixture contains one or both of
/// these metafiles, default values will *not* be used.
fn write_default_svc_user_and_group_metafiles<S, T>(hab_root: &HabRoot, pkg_origin: S, pkg_name: T)
where
    S: AsRef<Path>,
    T: AsRef<Path>,
{
    let svc_user_metafile = hab_root.svc_user_path(&pkg_origin, &pkg_name);
    let svc_group_metafile = hab_root.svc_group_path(&pkg_origin, &pkg_name);

    if !svc_user_metafile.is_file() {
        write_metafile(
            svc_user_metafile,
            users::get_current_username()
                .expect("Could not determine current username")
                .as_str(),
        );
    }

    if !svc_group_metafile.is_file() {
        write_metafile(
            svc_group_metafile,
            users::get_current_groupname()
                .expect("Could not determine current groupname")
                .as_str(),
        );
    }
}

/// Write package metafile with provided content.
fn write_metafile<P>(metafile: P, content: &str)
where
    P: AsRef<Path>,
{
    let mut f =
        File::create(&metafile).expect(
            format!("Could not create metafile {}", metafile.as_ref().display())
                .as_str(),
        );
    f.write_all(content.as_bytes()).expect(
        format!(
            "Could not write file contents to metafile {}",
            metafile.as_ref().display()
        ).as_str(),
    );
}
