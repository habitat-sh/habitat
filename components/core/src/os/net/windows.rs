use std::io;

use winapi::um::{winbase,
                 winnt::CHAR};

const MAX_LEN: usize = 15;

pub fn hostname() -> io::Result<String> {
    let mut buf = [0 as CHAR; MAX_LEN + 1];
    let mut len = buf.len() as u32;
    unsafe {
        if winbase::GetComputerNameA(buf.as_mut_ptr(), &mut len) == 0 {
            return Err(io::Error::last_os_error());
        }
    }
    let bytes = buf[0..len as usize].iter()
                                    .map(|&byte| byte as u8)
                                    .collect::<Vec<u8>>();
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
