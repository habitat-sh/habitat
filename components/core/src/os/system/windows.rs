// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use os::system::Uname;
use error::Result;

pub fn uname() -> Result<Uname> {
    Ok(Uname {
        sys_name: String::from("Windows"),
        node_name: String::from("CHEF-WIN10"),
        release: String::from("10.0.14915"),
        version: String::from("Microsoft Windows 10 Enterprise Insider Preview"),
        machine: String::from("x86_64"),
    })
}
