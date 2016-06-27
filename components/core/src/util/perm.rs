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

use std::process::Command;
use std::path::Path;

use error::{Error, Result};

pub fn set_owner<T: AsRef<Path>, X: AsRef<str>>(path: T, owner: X) -> Result<()> {
    debug!("Attempting to set owner of {:?} to {:?}",
           &path.as_ref(),
           &owner.as_ref());
    let output = try!(Command::new("chown")
        .arg(owner.as_ref())
        .arg(path.as_ref())
        .output());
    match output.status.success() {
        true => Ok(()),
        false => {
            Err(Error::PermissionFailed(format!("Can't change owner of {:?} to {:?}",
                                                &path.as_ref(),
                                                &owner.as_ref())))
        }
    }
}

// When Rust stabilizes this interface, we can move to the cross
// platform abstraction. Until then, if we move to Windows or some
// other platform, this code will need to become platform specific.
pub fn set_permissions<T: AsRef<Path>, X: AsRef<str>>(path: T, perm: X) -> Result<()> {
    debug!("Attempting to set permissions on {:?} to {:?}",
           &path.as_ref(),
           &perm.as_ref());
    let output = try!(Command::new("chmod")
        .arg(perm.as_ref())
        .arg(path.as_ref())
        .output());
    match output.status.success() {
        true => Ok(()),
        false => {
            Err(Error::PermissionFailed(format!("Can't set permissions on {:?} to {:?}",
                                                &path.as_ref(),
                                                &perm.as_ref())))
        }
    }
}

