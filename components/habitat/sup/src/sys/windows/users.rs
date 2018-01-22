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

use hcore::package::PackageInstall;

use error::Result;
use util::users::default_user_and_group;

/// Always return Ok (for now) on windows since we just run as the current user
pub fn assert_pkg_user_and_group(_user: &str, _group: &str) -> Result<()> {
    Ok(())
}

/// For now we are ignoring any configured user and group
/// because we do not start the Supervisor on windows under
/// alternate credentials
pub fn get_user_and_group(_pkg_install: &PackageInstall) -> Result<(String, String)> {
    default_user_and_group()
}
