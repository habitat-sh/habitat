// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::process::Command;
use std::path::Path;

use error::{Error, Result};

pub fn set_owner<T: AsRef<Path>, X: AsRef<str>>(path: T, owner: X) -> Result<()> {
    let output = try!(Command::new("chown")
        .arg(owner.as_ref())
        .arg(path.as_ref())
        .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(Error::PermissionFailed),
    }
}

// When Rust stabilizes this interface, we can move to the cross
// platform abstraction. Until then, if we move to Windows or some
// other platform, this code will need to become platform specific.
pub fn set_permissions<T: AsRef<Path>, X: AsRef<str>>(path: T, perm: X) -> Result<()> {
    let output = try!(Command::new("chmod")
        .arg(perm.as_ref())
        .arg(path.as_ref())
        .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(Error::PermissionFailed),
    }
}
