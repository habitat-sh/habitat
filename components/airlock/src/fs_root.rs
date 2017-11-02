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

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use rand::{self, Rng};

use {Error, Result};

const NUM_RETRIES: u32 = 1 << 31;
const NUM_RAND_CHARS: usize = 12;

#[derive(Debug)]
pub enum FsRootPolicy {
    Cleanup,
    Persist,
}

#[derive(Debug)]
pub struct FsRoot(Option<PathBuf>, FsRootPolicy);

impl FsRoot {
    pub fn at(path: PathBuf, policy: FsRootPolicy) -> Result<Self> {
        debug!(
            "creating fs root, path={}, policy={:?}",
            path.display(),
            policy
        );
        fs::create_dir(&path).map_err(
            |e| Error::FsRoot(path.clone(), e),
        )?;
        Ok(FsRoot(Some(path), policy))
    }

    pub fn in_tmpdir(policy: FsRootPolicy) -> Result<Self> {
        Self::at(tmp_path()?, policy)
    }

    pub fn finish(mut self) -> Result<()> {
        match self.1 {
            FsRootPolicy::Cleanup => {
                debug!("removing fs root, path={}", self.as_ref().display());
                fs::remove_dir_all(self.as_ref()).map_err(|e| {
                    Error::FsRoot(self.as_ref().into(), e)
                })?;
                // Prevent `Drop` from removing the dir a second time
                self.0 = None;
                Ok(())
            }
            FsRootPolicy::Persist => {
                debug!("persisting fs root, path={}", self.as_ref().display());
                // Prevent `Drop` from checking the dir a second time
                self.0 = None;
                Ok(())
            }
        }
    }
}

impl AsRef<Path> for FsRoot {
    fn as_ref(&self) -> &Path {
        self.0.as_ref().unwrap()
    }
}

impl Drop for FsRoot {
    fn drop(&mut self) {
        if let Some(ref path) = self.0 {
            match self.1 {
                FsRootPolicy::Cleanup => {
                    debug!("removing fs root, path={}", path.display());
                    let _ = fs::remove_dir_all(path);
                }
                FsRootPolicy::Persist => {
                    debug!("persisting fs root, path={}", self.as_ref().display());
                }
            }
        }
    }
}

fn tmp_path() -> Result<PathBuf> {
    let mut rng = rand::thread_rng();
    // Find a nonexistent candidate directory path and return the first success
    for _ in 0..NUM_RETRIES {
        let suffix: String = rng.gen_ascii_chars().take(NUM_RAND_CHARS).collect();
        let path = env::temp_dir().join(format!("airlock-fsroot.{}", suffix));
        if !path.exists() {
            return Ok(path);
        }
    }
    // If not candidate directories are successful, then return err
    Err(Error::FsRoot(
        env::temp_dir(),
        io::Error::new(
            io::ErrorKind::AlreadyExists,
            "too many temporary directories already exist",
        ),
    ))
}
