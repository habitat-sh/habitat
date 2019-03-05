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

use std::path::Path;

use crate::error::Result;

/// Creates a root file system under the given path.
///
/// # Errors
///
/// * If files and/or directories cannot be created
/// * If permissions for files and/or directories cannot be set
#[cfg(unix)]
pub fn create<T>(root: T) -> Result<()>
    where T: AsRef<Path>
{
    use std::fs;

    use crate::hcore::util;

    let root = root.as_ref();
    fs::create_dir_all(root)?;
    util::posix_perm::set_permissions(root.to_str().unwrap(), 0o0750)?;
    Ok(())
}

#[cfg(windows)]
pub fn create<T>(_root: T) -> Result<()>
    where T: AsRef<Path>
{
    unimplemented!()
}
