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
use pkg;
use topology;

pub fn package(config: &Config) -> BldrResult<()> {
    let package = try!(pkg::latest(config.package()));
    match config.topology() {
        "standalone" => try!(topology::standalone::run(package)),
        "leader" => try!(topology::leader::run(package)),
        t => {
            return Err(BldrError::UnknownTopology(String::from(t)))
        }
    }
    Ok(())
}

