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

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use errno;
use libc;

use {Error, Result};

#[derive(Debug)]
pub enum Mount {
    Nonrecursive,
    Recursive,
}

pub fn bind<S, T>(source: S, target: T, recurse: Mount, flags: Option<libc::c_ulong>) -> Result<()>
where
    S: AsRef<Path>,
    T: AsRef<Path>,
{
    let c_source = CString::new(source.as_ref().as_os_str().as_bytes())?;
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;
    let mut c_flags = libc::MS_BIND;
    if let Mount::Recursive = recurse {
        c_flags = c_flags | libc::MS_REC;
    }
    if let Some(flags) = flags {
        c_flags = c_flags | flags;
    };

    debug!(
        "bind mounting, src={} target={}, recurse={:?}, flags={:?}",
        source.as_ref().display(),
        target.as_ref().display(),
        recurse,
        flags
    );
    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            ptr::null(),
            c_flags,
            ptr::null(),
        )
    } {
        rc if rc < 0 => {
            Err(Error::Mount(format!(
                "mount_bind({}, {}, ...) returned: {} ({})",
                source.as_ref().display(),
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}

// mount("none", "/", NULL, MS_REC|MS_PRIVATE, NULL) = 0
pub fn private<T>(target: T, recurse: Mount) -> Result<()>
where
    T: AsRef<Path>,
{
    let c_source = CString::new("none".as_bytes())?;
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;
    let mut flags = libc::MS_PRIVATE;
    if let Mount::Recursive = recurse {
        flags = flags | libc::MS_REC;
    }

    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            ptr::null(),
            flags,
            ptr::null(),
        )
    } {
        rc if rc < 0 => {
            Err(Error::Mount(format!(
                "mount_private({}, ...) returned: {} ({})",
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}

// mount("proc", "/tmp/rootfs.fol06wNUnpLw/proc", "proc", MS_MGC_VAL|MS_NODEV, NULL) = 0
pub fn procfs<T>(target: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let c_source = CString::new("proc".as_bytes())?;
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;
    let c_type = CString::new("proc".as_bytes())?;
    let flags = libc::MS_MGC_VAL | libc::MS_NODEV;

    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            c_type.as_ptr(),
            flags,
            ptr::null(),
        )
    } {
        rc if rc < 0 => {
            Err(Error::Mount(format!(
                "mount_proc({}, ...) returned: {} ({})",
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}

// mount("tmpfs", "/tmp/rootfs.suP5gd8rt32R/dev", "tmpfs", MS_NOSUID|MS_NODEV|MS_STRICTATIME,
// "mode=755,size=65536k") = 0
pub fn tmpfs<T>(
    source: &str,
    target: T,
    flags: Option<libc::c_ulong>,
    mode: Option<libc::mode_t>,
    size_kb: Option<u32>,
) -> Result<()>
where
    T: AsRef<Path>,
{
    let c_source = CString::new(source.as_bytes())?;
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;
    let c_type = CString::new("tmpfs".as_bytes())?;
    let c_flags = match flags {
        Some(f) => f,
        None => 0,
    };
    let mut data = String::new();
    if let Some(mode) = mode {
        data.push_str(&format!("mode={:04o}", mode));
    };
    if let Some(size_kb) = size_kb {
        data.push_str(&format!(",size={}k", size_kb));
    };
    let c_data = CString::new(data.as_bytes())?;

    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            c_type.as_ptr(),
            c_flags,
            c_data.as_ptr() as *const libc::c_void,
        )
    } {
        rc if rc < 0 => {
            Err(Error::Mount(format!(
                "mount_tmpfs({}, ...) returned: {} ({})",
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}

pub fn devpts<T>(
    target: T,
    flags: libc::c_ulong,
    mode: Option<libc::mode_t>,
    ptmxmode: Option<libc::mode_t>,
) -> Result<()>
where
    T: AsRef<Path>,
{
    let c_source = CString::new("devpts".as_bytes())?;
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;
    let c_type = CString::new("devpts".as_bytes())?;
    let mut data = String::from("newinstance");
    if let Some(mode) = mode {
        data.push_str(&format!(",mode={:04o}", mode));
    };
    if let Some(ptmxmode) = ptmxmode {
        data.push_str(&format!(",ptmxmode={:04o}", ptmxmode));
    };
    let c_data = CString::new(data.as_bytes())?;

    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            c_type.as_ptr(),
            flags,
            c_data.as_ptr() as *const libc::c_void,
        )
    } {
        rc if rc < 0 => {
            Err(Error::Mount(format!(
                "mount_devpts({}, ...) returned: {} ({})",
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}

pub fn mqueue<T>(target: T, flags: libc::c_ulong) -> Result<()>
where
    T: AsRef<Path>,
{
    let c_source = CString::new("mqueue".as_bytes())?;
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;
    let c_type = CString::new("mqueue".as_bytes())?;

    match unsafe {
        libc::mount(
            c_source.as_ptr(),
            c_target.as_ptr(),
            c_type.as_ptr(),
            flags,
            ptr::null(),
        )
    } {
        rc if rc < 0 => {
            Err(Error::Mount(format!(
                "mount_mqueue({}, ...) returned: {} ({})",
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
    }
}

pub fn umount<T: AsRef<Path>>(target: T, flags: Option<libc::c_int>) -> Result<()> {
    let c_target = CString::new(target.as_ref().as_os_str().as_bytes())?;

    match flags {
        Some(c_flags) => {
            match unsafe { libc::umount2(c_target.as_ptr(), c_flags) } {
                rc if rc < 0 => {
                    Err(Error::Mount(format!(
                "umount2({},{}) returned: {} ({})",
                target.as_ref().display(),
                c_flags,
                rc,
                errno::errno(),
            )))
                }
                _ => Ok(()),
            }
        }
        None => {
            match unsafe { libc::umount(c_target.as_ptr()) } {
                rc if rc < 0 => {
                    Err(Error::Mount(format!(
                "umount({}) returned: {} ({})",
                target.as_ref().display(),
                rc,
                errno::errno(),
            )))
                }
                _ => Ok(()),
            }
        }
    }
}
