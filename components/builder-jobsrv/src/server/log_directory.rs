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

use error::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Encapsulates the local filesystem directory in which in-process
/// build job logs will be collected prior to being sent to long-term
/// storage.
#[derive(Clone, Debug)]
pub struct LogDirectory(PathBuf);

impl LogDirectory {
    /// Create a new `LogDirectory` wrapping `path`.
    pub fn new<T>(path: T) -> Self
    where
        T: AsRef<Path>,
    {
        LogDirectory(path.as_ref().to_path_buf())
    }

    /// Ensures that the specified log directory can be used.
    ///
    /// Returns an `Err` if:
    /// * the path does not exist
    /// * the path is not a directory
    /// * the path is not writable
    pub fn validate(&self) -> Result<()> {
        let meta = fs::metadata(&self.0).map_err(|e| {
            Error::LogDirDoesNotExist(self.0.clone(), e)
        })?;
        if !meta.is_dir() {
            return Err(Error::LogDirIsNotDir(self.0.clone()));
        }
        if meta.permissions().readonly() {
            return Err(Error::LogDirNotWritable(self.0.clone()));
        }
        Ok(())
    }

    /// Returns the path to a particular job's log file within the
    /// `LogDirectory`. The file may not exist yet.
    pub fn log_file_path(&self, job_id: u64) -> PathBuf {
        self.0.join(format!("{}.log", job_id))
    }
}
