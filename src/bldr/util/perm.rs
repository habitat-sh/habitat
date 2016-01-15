// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use error::{BldrResult, ErrorKind};
use std::process::Command;

static LOGKEY: &'static str = "UP";

pub fn set_owner(path: &str, owner: &str) -> BldrResult<()> {
    let output = try!(Command::new("chown")
                          .arg(owner)
                          .arg(path)
                          .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(bldr_error!(ErrorKind::PermissionFailed)),
    }
}

// When Rust stabilizes this interface, we can move to the cross
// platform abstraction. Until then, if we move to Windows or some
// other platform, this code will need to become platform specific.
pub fn set_permissions(path: &str, perm: &str) -> BldrResult<()> {
    let output = try!(Command::new("chmod")
                          .arg(perm)
                          .arg(path)
                          .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(bldr_error!(ErrorKind::PermissionFailed)),
    }
}
