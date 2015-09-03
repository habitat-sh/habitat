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
use config::Config;
use std::fs::File;

use pkg;
use util::http;

pub fn key(config: &Config) -> BldrResult<()> {
    println!("   {}: uploading {}.asc", config.key(), config.key());
    let mut file = try!(File::open(&format!("/opt/bldr/cache/keys/{}.asc", config.key())));
    try!(http::upload(&format!("{}/keys/{}", config.url(), config.key()), &mut file));

    println!("   {}: complete", config.key());
    Ok(())
}

