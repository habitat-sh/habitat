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

// The purpose of this module is to have a place to put common data structures that are required by
// both the client and the server, e.g. the server generates a JSON response and the client needs
// to read and parse it.

// The purpose of this struct is to capture the relevant details of a scenario where there was an
// attempt to promote a job group to a channel and not all of the projects in that job group were
// able to be promoted (e.g. they might have failed or been skipped). "group_id" is the ID of the
// job group that was to be promoted and "projects" is a vector of strings representing the idents
// of all of the failed projects.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialJobGroupPromote {
    pub group_id: u64,
    pub failed_projects: Vec<String>,
}
