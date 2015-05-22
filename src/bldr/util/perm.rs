//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use error::{BldrResult, BldrError};
use std::process::Command;

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
        false => {
            Err(BldrError::PermissionFailed)
        },
    }
}
