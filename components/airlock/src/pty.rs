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

use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use std::path::{Path, PathBuf};

use libc;
use errno;

use {Error, Result};

const DEFAULT_PTMX: &'static str = "/dev/ptmx";

#[derive(Debug)]
pub struct Master(RawFd);

impl Master {
    // `Default` trait doesn't return a `Result` so that's why we're not implementing that trait.
    pub fn default() -> Result<Self> {
        Self::from(DEFAULT_PTMX)
    }

    pub fn from<P: AsRef<Path>>(ptmx_path: P) -> Result<Self> {
        let master = Self::new(ptmx_path)?;
        master.grantpt()?;
        master.unlockpt()?;
        Ok(master)
    }

    // Gets the name of the slave pseudoterminal
    pub fn ptsname(&self) -> Result<PathBuf> {
        let ptr = match unsafe { libc::ptsname(self.0) } {
            c if c.is_null() => {
                return Err(Error::Ptsname(
                    format!("ptsname({}) returned: NULL pointer", self.0,),
                ))
            }
            c => c,
        };
        let c_str = unsafe { CStr::from_ptr(ptr) };

        Ok(PathBuf::from(c_str.to_string_lossy().as_ref()))
    }

    fn new<P: AsRef<Path>>(ptmx_path: P) -> Result<Self> {
        let c_path = CString::new(ptmx_path.as_ref().to_string_lossy().as_ref())
            .map_err(|err| {
                Error::CreateMaster(format!(
                    "cannot create c string from path: {} ({})",
                    ptmx_path.as_ref().display(),
                    err
                ))
            })?;


        match unsafe { libc::open(c_path.as_ptr(), libc::O_RDWR) } {
            fd if fd < 0 => {
                Err(Error::CreateMaster(format!(
                "open({}) returned: {} ({})",
                ptmx_path.as_ref().display(),
                fd,
                errno::errno(),
            )))
            }
            fd => Ok(Master(fd)),
        }
    }

    // Grant access to the slave pseudoterminal
    fn grantpt(&self) -> Result<()> {
        match unsafe { libc::grantpt(self.0) } {
            rc if rc < 0 => {
                Err(Error::Grantpt(format!(
                "grantpt({}) returned: {} ({})",
                self.0,
                rc,
                errno::errno(),
            )))
            }
            _ => Ok(()),
        }
    }

    // Unlocks the pseudoterminal master/slave pair
    fn unlockpt(&self) -> Result<()> {
        match unsafe { libc::unlockpt(self.0) } {
            rc if rc < 0 => {
                Err(Error::Unlockpt(format!(
                "unlockpt({}) returned: {} ({})",
                self.0,
                rc,
                errno::errno(),
            )))
            }
            _ => Ok(()),
        }
    }
}
