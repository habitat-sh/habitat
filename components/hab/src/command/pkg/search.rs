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

use api_client::Client;
use error::Result;
use {PRODUCT, VERSION};

pub fn start(st: &str, bldr_url: &str, token: Option<&str>) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    let (packages, more) = api_client.search_package(st, token)?;
    match packages.len() {
        0 => println!("No packages found that match '{}'", st),
        _ => {
            for p in &packages {
                if let (&Some(ref version), &Some(ref release)) = (&p.version, &p.release) {
                    println!("{}/{}/{}/{}", p.origin, p.name, version, release);
                } else {
                    println!("{}/{}", p.origin, p.name);
                }
            }
            if more {
                println!(
                    "Search returned too many items, only showing the first {}",
                    packages.len()
                );
            }
        }
    }
    Ok(())
}
