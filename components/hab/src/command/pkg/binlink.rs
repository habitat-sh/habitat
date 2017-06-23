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

use std::fs;
use std::path::Path;

use common::ui::{Status, UI};
use hcore::package::{PackageIdent, PackageInstall};
use hcore::os::filesystem;
use hcore::fs as hfs;

use error::{Error, Result};

pub fn start(
    ui: &mut UI,
    ident: &PackageIdent,
    binary: &str,
    dest_path: &Path,
    fs_root_path: &Path,
) -> Result<()> {
    let dst_path = fs_root_path.join(try!(dest_path.strip_prefix("/")));
    let dst = dst_path.join(&binary);
    try!(ui.begin(format!(
        "Symlinking {} from {} into {}",
        &binary,
        &ident,
        dst_path.display()
    )));
    let pkg_install = try!(PackageInstall::load(ident, Some(fs_root_path)));
    let src = match try!(hfs::find_command_in_pkg(binary, &pkg_install, fs_root_path)) {
        Some(c) => c,
        None => {
            return Err(Error::CommandNotFoundInPkg(
                (pkg_install.ident().to_string(), binary.to_string()),
            ))
        }
    };
    if !dst_path.is_dir() {
        try!(ui.status(
            Status::Creating,
            format!("parent directory {}", dst_path.display()),
        ));
        try!(fs::create_dir_all(&dst_path))
    }
    match fs::read_link(&dst) {
        Ok(path) => {
            if path != src {
                try!(fs::remove_file(&dst));
                try!(filesystem::symlink(&src, &dst));
            }
        }
        Err(_) => try!(filesystem::symlink(&src, &dst)),
    }
    try!(ui.end(format!(
        "Binary {} from {} symlinked to {}",
        &binary,
        &pkg_install.ident(),
        &dst.display()
    )));
    Ok(())
}

pub fn binlink_all_in_pkg(
    ui: &mut UI,
    pkg_ident: &PackageIdent,
    dest_path: &Path,
    fs_root_path: &Path,
) -> Result<()> {
    let pkg_path = PackageInstall::load(pkg_ident, Some(fs_root_path))?;
    for bin_path in pkg_path.paths()? {
        for bin in fs::read_dir(&bin_path)? {
            let bin_file = bin?;
            let bin_name = match bin_file.file_name().to_str() {
                Some(bn) => bn.to_owned(),
                None => {
                    try!(ui.warn(
                        "Found a binary with an invalid name.  Skipping binlink.",
                    ));
                    continue;
                }
            };
            self::start(ui, pkg_ident, &bin_name, dest_path, fs_root_path)?;
        }
    }
    Ok(())
}
