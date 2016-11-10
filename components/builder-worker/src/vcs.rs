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

use std::path::Path;

use git2;
use protocol::vault;

use error::Result;

pub trait RemoteSource {
    fn clone(&self, path: &Path) -> Result<()>;
}

impl RemoteSource for vault::VCSGit {
    fn clone(&self, path: &Path) -> Result<()> {
        debug!("cloning git repository, url={}, path={:?}",
               self.get_url(),
               path);
        try!(git2::Repository::clone(self.get_url(), path));
        Ok(())
    }
}
