use std::{ffi::CStr,
          io};

use libc;

pub fn hostname() -> io::Result<String> {
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_slice().as_mut_ptr();
    match unsafe { gethostname(ptr as *mut libc::c_char, len as libc::size_t) } {
        0 => {
            let c_str = unsafe { CStr::from_ptr(ptr as *const libc::c_char) };
            Ok(c_str.to_string_lossy().into_owned())
        }
        code => Err(io::Error::from_raw_os_error(code)),
    }
}

extern "C" {
    pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}
