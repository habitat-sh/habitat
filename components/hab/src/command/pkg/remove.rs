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

use std::collections::HashSet;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::cmp::Ordering;

use walkdir::WalkDir;

use error::Result;
use hcore::fs::PKG_PATH;
use hcore::package::{PackageIdent, PackageInstall};
use common::ui::{Status, UI};

pub fn start(ui: &mut UI, pkg_name: &PackageIdent, fs_root_path: &Path) -> Result<()> {
    let package_install = PackageInstall::load(pkg_name, Some(fs_root_path))?;
    let package_ident = package_install.ident();
    let pkg_root = fs_root_path.join(PKG_PATH);

    let mut dependent_packages = HashSet::new();

    ui.begin(format!(
        "Checking for installed packages that depend on {}",
        package_install
    ))?;

    for entry in WalkDir::new(pkg_root).into_iter().filter_map(|e| e.ok()) {
        if let Some(e) = entry.path().file_name().and_then(|e| e.to_str()) {
            if e == "IDENT" {
                let mut f = File::open(entry.path().to_str().unwrap())?;
                let mut ident_str = String::new();

                f.read_to_string(&mut ident_str)?;

                let pkg_ident = PackageIdent::from_str(&ident_str.trim())?;
                let pkg_install = PackageInstall::load(&pkg_ident, Some(fs_root_path))?;

                for tdep in pkg_install.tdeps().unwrap() {
                    if package_ident.cmp(&tdep) == Ordering::Equal {
                        dependent_packages.insert(pkg_install.ident().clone());
                    }
                }
            }
        }
    }

    if dependent_packages.is_empty() {
        ui.status(
            Status::Deleting,
            format!("{:?}", package_install.installed_path()),
        )?;
        fs::remove_dir_all(package_install.installed_path()).expect("Unable to remove package");
    } else {
        println!(
            "Unable to delete {}.  The following packages depend on it:",
            package_ident
        );
        for pkg in dependent_packages {
            println!("  {}", pkg);
        }
    }

    Ok(())
}
