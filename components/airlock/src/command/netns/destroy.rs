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

use std::fs;
use std::path::Path;

use libc;

use Result;
use mount;
use namespace;
use user;

pub fn run<P: AsRef<Path>>(ns_dir: P) -> Result<()> {
    user::check_running_user_is_root()?;

    mount::umount(namespace::netns_file(&ns_dir), Some(libc::MNT_DETACH))?;
    mount::umount(namespace::userns_file(&ns_dir), Some(libc::MNT_DETACH))?;
    fs::remove_dir_all(&ns_dir)?;

    info!(
        "Network namespace directory {} destroyed.",
        ns_dir.as_ref().display()
    );

    Ok(())
}
