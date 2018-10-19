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

use error::Result;
use hcore::fs as hfs;
use hcore::package::list;
use hcore::package::PackageIdent;
use std::path::Path;

/// There are three options for what we can list:
///   - All packages (no prefix supplied)
///   - All packages in an origin (string with no '/')
///   - A set of packages with a common package ident (e.g. /ORIGIN/NAME) with optionally version/release
#[derive(Debug)]
pub enum ListingType {
    AllPackages,
    Origin(String),
    Ident(PackageIdent),
}

pub fn start(listing: &ListingType, fs_root_path: &Path) -> Result<()> {
    let package_path = hfs::pkg_root_path(Some(&fs_root_path));

    let mut packages = match listing {
        ListingType::AllPackages => list::all_packages(&package_path)?,
        ListingType::Origin(origin) => list::package_list_for_origin(&package_path, &origin)?,
        ListingType::Ident(ident) => list::package_list_for_ident(&package_path, &ident)?,
    };

    packages.sort_unstable_by(|a, b| a.by_parts_cmp(b));
    for p in &packages {
        println!("{}", &p);
    }

    Ok(())
}
