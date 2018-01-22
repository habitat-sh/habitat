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

use std::fs::{self, File};
use std::os::unix::fs::{symlink as fs_symlink, PermissionsExt};
use std::path::Path;

use libc;

use Result;

pub fn chmod<P: AsRef<Path>>(path: P, mode: u32) -> Result<()> {
    let md = path.as_ref().metadata()?;
    let mut perms = md.permissions();
    perms.set_mode(mode);
    Ok(())
}

pub fn mkdir_p<P: AsRef<Path>>(path: P) -> Result<()> {
    debug!("creating directory, path={}", path.as_ref().display());
    Ok(fs::create_dir_all(path)?)
}

pub fn rmdir<P: AsRef<Path>>(path: P) -> Result<()> {
    debug!("removing directory, path={}", path.as_ref().display());
    Ok(fs::remove_dir(path)?)
}

pub fn symlink<S, T>(source: S, target: T) -> Result<()>
where
    S: AsRef<Path>,
    T: AsRef<Path>,
{
    debug!(
        "symlinking, src={}, target={}",
        source.as_ref().display(),
        target.as_ref().display()
    );
    Ok(fs_symlink(source, target)?)
}

pub fn touch<P: AsRef<Path>>(path: P) -> Result<()> {
    debug!("creating file, path={}", path.as_ref().display());
    let _ = File::create(path)?;
    Ok(())
}

pub fn umask(mode: libc::mode_t) -> libc::mode_t {
    unsafe { libc::umask(mode) }
}
