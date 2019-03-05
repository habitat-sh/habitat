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

use std::{collections::HashSet,
          path::Path};

use walkdir::WalkDir;

use crate::{error::{Error,
                    Result},
            hcore::fs::PKG_PATH};

pub fn start(filename: &str,
             fs_root_path: &Path,
             full_releases: bool,
             full_path: bool)
             -> Result<()> {
    let mut found = HashSet::new();
    // count the # of directories in the path to the package dir
    // ex: /hab/pkg == 2
    let prefix_count = Path::new(PKG_PATH).components().count();
    // the location of installed packages
    let pkg_root = fs_root_path.join(PKG_PATH);

    let mut found_any = false;

    // recursively walk the directories in pkg_root looking for matches
    for entry in WalkDir::new(pkg_root).into_iter().filter_map(|e| e.ok()) {
        if let Some(f) = entry.path().file_name().and_then(|f| f.to_str()) {
            if filename == f {
                found_any = true;
                let mut comps = entry.path().components();

                // skip prefix_count segments of the path
                let _ = comps.nth(prefix_count)
                             .ok_or_else(|| Error::FileNotFound(f.to_string()))?;

                let segments = if full_releases {
                    // take all 4 segments of the path
                    // ex: core/busybox-static/1.24.2/20160708162350
                    comps.take(4)
                } else {
                    // only take 2 segments of the path
                    // ex: core/busybox-static
                    comps.take(2)
                };

                let mapped_segs: Vec<String> =
                    segments.map(|c| c.as_os_str().to_string_lossy().into_owned())
                            .collect();
                let pkg_name = mapped_segs.join("/");

                // if we show the full path, then don't bother stuffing
                // the result into the found HashSet, as we want to
                // print out each path we find.
                if full_path {
                    println!("{}: {}", &pkg_name, &entry.path().to_string_lossy());
                } else {
                    found.insert(pkg_name);
                }
            }
        }
    }
    // if we're not using full_path, then using a set will filter out
    // duplicates. This shows the filtered set of matches
    for entry in &found {
        println!("{}", entry);
    }
    if found_any {
        Ok(())
    } else {
        Err(Error::ProvidesError(filename.to_string()))
    }
}
