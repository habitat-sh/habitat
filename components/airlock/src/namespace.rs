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

use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;

use errno;
use libc;

use {Error, Result};

pub fn setns_network<P: AsRef<Path>>(path: P) -> Result<()> {
    setns(path, libc::CLONE_NEWNET)
}

pub fn setns_user<P: AsRef<Path>>(path: P) -> Result<()> {
    setns(path, libc::CLONE_NEWUSER)
}

fn setns<P: AsRef<Path>>(path: P, nstype: libc::c_int) -> Result<()> {
    let file = File::open(path.as_ref())?;
    let fd = file.as_raw_fd();
    debug!(
        "calling setns(), path={}, fd={}, nstype={}",
        path.as_ref().display(),
        fd,
        nstype,
    );
    match unsafe { libc::setns(fd, nstype) } {
        rc if rc < 0 => {
            Err(Error::Setns(format!(
                "setns({}, {}) returned: {} ({})",
                path.as_ref().display(),
                nstype,
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}
