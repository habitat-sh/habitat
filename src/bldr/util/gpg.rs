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

use std::process::Command;
use std::fs;

use fs::GPG_CACHE;
use error::{BldrResult, ErrorKind};
use util::perm;

static LOGKEY: &'static str = "GP";

pub fn import(status: &str, keyfile: &str) -> BldrResult<()> {
    try!(fs::create_dir_all(GPG_CACHE));
    try!(perm::set_permissions(GPG_CACHE, "0700"));
    let output = try!(gpg_cmd()
                          .arg("--import")
                          .arg(keyfile)
                          .output());
    match output.status.success() {
        true => {
            outputln!("{} GPG key imported", status);
            Ok(())
        }
        false => Err(bldr_error!(ErrorKind::GPGImportFailed(String::from_utf8_lossy(&output.stderr)
                                                                .into_owned()))),
    }
}

pub fn verify(file: &str) -> BldrResult<()> {
    let output = try!(gpg_cmd()
                          .arg("--verify")
                          .arg(file)
                          .output());
    match output.status.success() {
        true => Ok(()),
        false => Err(bldr_error!(ErrorKind::GPGVerifyFailed(String::from_utf8_lossy(&output.stderr)
                                                                .into_owned()))),
    }
}

fn gpg_cmd() -> Command {
    let mut command = Command::new("gpg");
    command.arg("--homedir");
    command.arg(GPG_CACHE);
    command
}
