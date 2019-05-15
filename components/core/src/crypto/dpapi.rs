use crate::error::{Error,
                   Result};
use base64;
use std::{io,
          mem,
          ptr};
use winapi::{shared::minwindef::DWORD,
             um::{dpapi,
                  wincrypt::CRYPTOAPI_BLOB}};

const COMPLEXITY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/hab-crypt"));

pub fn decrypt(secret: &str) -> Result<String> {
    unsafe {
        let mut bytes = base64::decode(secret).unwrap();
        let mut in_blob = CRYPTOAPI_BLOB { cbData: bytes.len() as DWORD,
                                           pbData: bytes.as_mut_ptr(), };
        let mut out_blob = CRYPTOAPI_BLOB { cbData: 0,
                                            pbData: ptr::null_mut(), };
        let mut entropy = ptr::null_mut();
        let mut blob: CRYPTOAPI_BLOB = mem::zeroed::<CRYPTOAPI_BLOB>();
        let mut complexity_vec = COMPLEXITY.to_vec();
        if !complexity_vec.is_empty() {
            blob.cbData = complexity_vec.len() as DWORD;
            blob.pbData = complexity_vec.as_mut_ptr();
            entropy = &mut blob;
        }
        let ret = dpapi::CryptUnprotectData(&mut in_blob,
                                            ptr::null_mut(),
                                            entropy,
                                            ptr::null_mut(),
                                            ptr::null_mut(),
                                            0,
                                            &mut out_blob);
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
        let mut in_blob = CRYPTOAPI_BLOB { cbData: secret_bytes.len() as DWORD,
                                           pbData: secret_bytes.as_mut_ptr(), };
        let mut out_blob = CRYPTOAPI_BLOB { cbData: 0,
                                            pbData: ptr::null_mut(), };
        let mut entropy = ptr::null_mut();
        let mut blob: CRYPTOAPI_BLOB = mem::zeroed::<CRYPTOAPI_BLOB>();
        let mut complexity_vec = COMPLEXITY.to_vec();
        if !complexity_vec.is_empty() {
            blob.cbData = complexity_vec.len() as DWORD;
            blob.pbData = complexity_vec.as_mut_ptr();
            entropy = &mut blob;
        }
        let ret = dpapi::CryptProtectData(&mut in_blob,
                                          ptr::null(),
                                          entropy,
                                          ptr::null_mut(),
                                          ptr::null_mut(),
                                          4, // CRYPTPROTECT_LOCAL_MACHINE
                                          &mut out_blob);
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
