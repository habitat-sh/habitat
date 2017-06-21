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

use std::path::{Path, PathBuf};
use walkdir::{WalkDir, Iter};
use hab_core::package::{FromArchive, PackageArchive};
use protocol::originsrv::OriginPackage;
use protocol::scheduler;

pub struct FileWalker {
    walker: Iter,
}

impl FileWalker {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        FileWalker { walker: WalkDir::new(path.as_ref()).follow_links(false).into_iter() }
    }
}

pub fn extract_package<T: AsRef<Path>>(path: T) -> Option<OriginPackage> {
    let mut archive = PackageArchive::new(PathBuf::from(path.as_ref()));

    match archive.ident() {
        Ok(_) => {
            match OriginPackage::from_archive(&mut archive) {
                Ok(p) => {
                    return Some(p);
                }
                Err(e) => {
                    error!("Error parsing package from archive: {:?}", e);
                    return None;
                }
            }
        }
        Err(e) => {
            error!("Error reading, archive={:?} error={:?}", &archive, &e);
            return None;
        }
    }
}

impl Iterator for FileWalker {
    type Item = scheduler::Package;

    fn next(&mut self) -> Option<scheduler::Package> {
        loop {
            match self.walker.next() {
                Some(entry) => {
                    let entry = entry.unwrap();
                    if entry.metadata().unwrap().is_dir() {
                        continue;
                    } else {
                        match extract_package(entry.path()) {
                            Some(p) => return Some(scheduler::Package::from(p)),
                            None => continue,
                        }
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}
