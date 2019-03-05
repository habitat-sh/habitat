// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to launch `helm` command. Please ensure Helm is installed.")]
    HelmLaunchFailed,
    #[fail(display = "{}. Please ensure Helm is initialized. For only setting up the Helm \
                      client, please run `helm init -c`.",
           _0)]
    HelmNotSetup(String),
    #[fail(display = "Invalid maintainer specification '{}', must be of the form \
                      NAME[,EMAIL[,URL]]",
           _0)]
    InvalidMaintainer(String),
    #[fail(display = "Invalid URL '{}': {}", _0, _1)]
    InvalidUrl(String, String),
}
