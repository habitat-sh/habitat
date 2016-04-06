// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::process::Command;

use error::{Error, Result};

pub fn set_owner(path: &str, owner: &str) -> Result<()> {
    let output = try!(Command::new("chown")
                          .arg(owner)
                          .arg(path)
                          .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(Error::PermissionFailed),
    }
}

// When Rust stabilizes this interface, we can move to the cross
// platform abstraction. Until then, if we move to Windows or some
// other platform, this code will need to become platform specific.
pub fn set_permissions(path: &str, perm: &str) -> Result<()> {
    let output = try!(Command::new("chmod")
                          .arg(perm)
                          .arg(path)
                          .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(Error::PermissionFailed),
    }
}
