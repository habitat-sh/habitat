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

// JW TODO: After updating to Rust 1.15, move the types contained in this module back into
// `server.rs`

#[derive(Clone, Serialize, Deserialize)]
pub struct OriginCreateReq {
    pub name: String,
}

#[derive(Serialize)]
pub struct PackageResults<'a, T: 'a> {
    pub range_start: isize,
    pub range_end: isize,
    pub total_count: isize,
    pub package_list: &'a Vec<T>,
}
