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

use std::ptr::null_mut;
use std::io::Error;

use kernel32::LocalFree;
use widestring::WideCString;
use winapi::{HLOCAL, LPCWSTR, BOOL, PSID};

extern "system" {
    fn ConvertSidToStringSidW(Sid: PSID, StringSid: LPCWSTR) -> BOOL;
}

pub struct Sid {
    pub raw: Vec<u8>,
}

impl Sid {
    pub fn to_string(&self) -> String {
        let mut buffer: LPCWSTR = null_mut();
        let ret = unsafe {
            ConvertSidToStringSidW(self.raw.as_ptr() as PSID,
                                   (&mut buffer as *mut LPCWSTR) as LPCWSTR)
        };
        if ret == 0 {
            panic!("Failed to convert sid to string: {}",
                   Error::last_os_error());
        } else {
            let widestr = unsafe { WideCString::from_ptr_str(buffer) };
            unsafe { LocalFree(buffer as HLOCAL) };
            widestr.to_string_lossy()
        }
    }
}
