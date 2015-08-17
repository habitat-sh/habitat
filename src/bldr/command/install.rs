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
use util::{http, gpg};
use std::fs;

pub fn from_url(package: &str, url: &str) -> BldrResult<String> {
    try!(fs::create_dir_all("/opt/bldr/cache/pkgs"));
    let filename = try!(http::download_package(package, url, "/opt/bldr/cache/pkgs"));
    Ok(filename)
}

pub fn verify(package: &str, file: &str) -> BldrResult<()> {
    try!(gpg::verify(package, file));
    Ok(())
}

pub fn unpack(package: &str, file: &str) -> BldrResult<()> {
    let output = try!(Command::new("sh")
        .arg("-c")
        .arg(format!("gpg --homedir /opt/bldr/cache/gpg --decrypt {} | tar x", file))
        .output());
    match output.status.success() {
        true => println!("   {}: Installed", package),
        false => {
            println!("   {}: Failed to install", package);
            return Err(BldrError::UnpackFailed);
        },
    }
    Ok(())
}
