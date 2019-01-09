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

use std::io;
use std::mem;
use std::ptr;

use base64;
use winapi::shared::minwindef::DWORD;
use winapi::um::dpapi;
use winapi::um::wincrypt::CRYPTOAPI_BLOB;

use crate::error::{Error, Result};

const COMPLEXITY: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/hab-crypt"));

pub fn decrypt(secret: String) -> Result<String> {
    unsafe {
        let mut bytes = base64::decode(secret.as_str()).unwrap();
        let mut in_blob = CRYPTOAPI_BLOB {
            cbData: bytes.len() as DWORD,
            pbData: bytes.as_mut_ptr(),
        };
        let mut out_blob = CRYPTOAPI_BLOB {
            cbData: 0,
            pbData: ptr::null_mut(),
        };
        let mut entropy = ptr::null_mut();
        let mut blob: CRYPTOAPI_BLOB = mem::zeroed::<CRYPTOAPI_BLOB>();
        let mut complexity_vec = COMPLEXITY.to_vec();
        if complexity_vec.len() > 0 {
            blob.cbData = complexity_vec.len() as DWORD;
            blob.pbData = complexity_vec.as_mut_ptr();
            entropy = &mut blob;
        }
        let ret = dpapi::CryptUnprotectData(
            &mut in_blob,
            ptr::null_mut(),
            entropy,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            &mut out_blob,
        );
        if ret == 0 {
            return Err(Error::CryptUnprotectDataFailed(format!(
                "Failed to decrypt secret: {}",
                io::Error::last_os_error()
            )));
        }
        let sz = out_blob.cbData as usize;
        let mut dst: Vec<u8> = Vec::with_capacity(sz);
        dst.set_len(sz);
        ptr::copy(out_blob.pbData, dst.as_mut_ptr(), sz);
        Ok(String::from_utf8(dst)?)
    }
}

pub fn encrypt(secret: String) -> Result<String> {
    unsafe {
        let mut secret_bytes = secret.into_bytes();
        let mut in_blob = CRYPTOAPI_BLOB {
            cbData: secret_bytes.len() as DWORD,
            pbData: secret_bytes.as_mut_ptr(),
        };
        let mut out_blob = CRYPTOAPI_BLOB {
            cbData: 0,
            pbData: ptr::null_mut(),
        };
        let mut entropy = ptr::null_mut();
        let mut blob: CRYPTOAPI_BLOB = mem::zeroed::<CRYPTOAPI_BLOB>();
        let mut complexity_vec = COMPLEXITY.to_vec();
        if complexity_vec.len() > 0 {
            blob.cbData = complexity_vec.len() as DWORD;
            blob.pbData = complexity_vec.as_mut_ptr();
            entropy = &mut blob;
        }
        let ret = dpapi::CryptProtectData(
            &mut in_blob,
            ptr::null(),
            entropy,
            ptr::null_mut(),
            ptr::null_mut(),
            4, // CRYPTPROTECT_LOCAL_MACHINE
            &mut out_blob,
        );
        if ret == 0 {
            return Err(Error::CryptProtectDataFailed(format!(
                "Failed to encrypt secret: {}",
                io::Error::last_os_error()
            )));
        }
        let sz = out_blob.cbData as usize;
        let mut dst: Vec<u8> = Vec::with_capacity(sz);
        dst.set_len(sz);
        ptr::copy(out_blob.pbData, dst.as_mut_ptr(), sz);
        Ok(base64::encode(&dst))
    }
}
