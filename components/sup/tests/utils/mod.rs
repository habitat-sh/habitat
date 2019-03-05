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

use std::{fs::{self,
               File},
          io::Write,
          path::Path,
          string::ToString,
          thread,
          time::Duration};

use crate::hcore::{os::users,
                   package::PackageInstall};

pub mod fixture_root;
pub mod hab_root;
pub mod test_butterfly;
pub mod test_sup;

// Re-export the key structs of this package for ergonomics.
pub use self::{fixture_root::FixtureRoot,
               hab_root::HabRoot,
               test_sup::TestSup};

/// Sleep for the specified number of seconds!
pub fn sleep_seconds(seconds: u64) { thread::sleep(Duration::from_secs(seconds)) }

/// Copy fixture package files from `fixture_root` over to `hab_root`
/// in the appropriate places for the Supervisor to find them.
pub fn setup_package_files(origin_name: &str,
                           package_name: &str,
                           service_group: &str,
                           fixture_root: &FixtureRoot,
                           hab_root: &HabRoot) {
    let origin_name = origin_name.to_string();
    let package_name = package_name.to_string();
    let service_group = service_group.to_string();

    // Ensure the directory for the spec files exists
    let spec_dir = hab_root.spec_dir(&service_group);
    fs::create_dir_all(spec_dir).expect("could not create spec directory");

    // Copy the spec file over
    let spec_source = fixture_root.spec_path(&package_name);
    let spec_destination = hab_root.spec_path(&package_name, &service_group);
    assert!(spec_source.exists(),
            format!("Missing a spec file at {:?}", spec_source));
    fs::copy(&spec_source, &spec_destination).unwrap_or_else(|_| {
                                                 panic!("Could not copy {:?} to {:?}",
                                                        spec_source, spec_destination)
                                             });

    // Copy the expanded package directory over
    let expanded_fixture_dir = fixture_root.expanded_package_dir(&package_name);
    let hab_pkg_path = hab_root.pkg_path(&origin_name, &package_name);
    copy_dir(&expanded_fixture_dir, &hab_pkg_path);
    write_default_svc_user_and_group_metafiles(&hab_root, &origin_name, &package_name);

    let install =
        PackageInstall::load(&hab_root.pkg_ident(&origin_name, &package_name),
                             Some(hab_root.as_ref())).unwrap_or_else(|_| {
                                                         panic!("Could not load package {:?}/{:?}",
                                                                &origin_name, &package_name)
                                                     });
    if let Ok(tdeps) = install.tdeps() {
        for dependency in tdeps.iter() {
            let fixture_dir = fixture_root.expanded_package_dir(&dependency.name);
            let pkg_path = hab_root.pkg_path(&dependency.origin, &dependency.name);
            copy_dir(&fixture_dir, &pkg_path);
        }
    }
}

/// Recursively copy the contents of `source_dir` into `dest_dir`
pub fn copy_dir<S, D>(source_dir: S, dest_dir: D)
    where S: AsRef<Path>,
          D: AsRef<Path>
{
    let source_dir = source_dir.as_ref().to_path_buf();
    assert!(source_dir.exists(),
            format!("Source directory {:?} does not exist", source_dir));
    assert!(source_dir.is_dir(),
            format!("Source directory {:?} is not a directory", source_dir));
    let dest_dir = dest_dir.as_ref().to_path_buf();

    fs::create_dir_all(&dest_dir).unwrap_or_else(|_| {
                                     panic!("Could not create directory {:?}", dest_dir)
                                 });

    let source_dir_entries = fs::read_dir(&source_dir).unwrap_or_else(|_| {
                                                          panic!("Could not read entries in {:?}",
                                                                 source_dir)
                                                      });

    for entry in source_dir_entries {
        let source = entry.unwrap().path();
        let destination = &dest_dir.join(source.file_name().unwrap());

        if source.is_file() {
            fs::copy(&source, &destination).unwrap_or_else(|_| {
                                               panic!("could not copy {:?} to {:?}",
                                                      source, destination)
                                           });
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
    where S: AsRef<Path>,
          T: AsRef<Path>
{
    let svc_user_metafile = hab_root.svc_user_path(&pkg_origin, &pkg_name);
    let svc_group_metafile = hab_root.svc_group_path(&pkg_origin, &pkg_name);

    if !svc_user_metafile.is_file() {
        write_metafile(svc_user_metafile,
                       users::get_current_username().expect("Could not determine current \
                                                             username")
                                                    .as_str());
    }

    if !svc_group_metafile.is_file() {
        write_metafile(svc_group_metafile,
                       users::get_current_groupname().expect("Could not determine current \
                                                              groupname")
                                                     .as_str());
    }
}

/// Write package metafile with provided content.
fn write_metafile<P>(metafile: P, content: &str)
    where P: AsRef<Path>
{
    let mut f = File::create(&metafile).unwrap_or_else(|_| {
                                           panic!("Could not create metafile {}",
                                                  metafile.as_ref().display())
                                       });
    f.write_all(content.as_bytes()).unwrap_or_else(|_| {
                                       panic!("Could not write file contents to metafile {}",
                                              metafile.as_ref().display())
                                   });
}
