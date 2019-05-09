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

pub fn ai_canonname() -> i32 { winapi::shared::ws2def::AI_CANONNAME }
