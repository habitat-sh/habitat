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
use std::io::{BufReader, Read};
use std::mem;
use std::path::Path;
use std::ptr;

use hex::ToHex;
use libsodium_sys;

use error::Result;

const BUF_SIZE: usize = 1024;

/// Calculate the BLAKE2b hash of a file, return as a hex string
/// digest size = 32 BYTES
/// NOTE: the hashing is keyless
pub fn hash_file<P>(filename: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename.as_ref())?;
    let mut reader = BufReader::new(file);
    hash_reader(&mut reader)
}

pub fn hash_string(data: &str) -> String {
    let mut out = [0u8; libsodium_sys::crypto_generichash_BYTES];
    let mut st = vec![0u8; (unsafe { libsodium_sys::crypto_generichash_statebytes() })];
    let pst = unsafe {
        mem::transmute::<*mut u8, *mut libsodium_sys::crypto_generichash_state>(st.as_mut_ptr())
    };
    unsafe {
        libsodium_sys::crypto_generichash_init(pst, ptr::null_mut(), 0, out.len());
        libsodium_sys::crypto_generichash_update(pst, data[..].as_ptr(), data.len() as u64);
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    out.to_hex()
}

pub fn hash_bytes(data: &[u8]) -> String {
    let mut out = [0u8; libsodium_sys::crypto_generichash_BYTES];
    let mut st = vec![0u8; (unsafe { libsodium_sys::crypto_generichash_statebytes() })];
    let pst = unsafe {
        mem::transmute::<*mut u8, *mut libsodium_sys::crypto_generichash_state>(st.as_mut_ptr())
    };
    unsafe {
        libsodium_sys::crypto_generichash_init(pst, ptr::null_mut(), 0, out.len());
        libsodium_sys::crypto_generichash_update(pst, data[..].as_ptr(), data.len() as u64);
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    out.to_hex()
}

pub fn hash_reader(reader: &mut BufReader<File>) -> Result<String> {
    let mut out = [0u8; libsodium_sys::crypto_generichash_BYTES];
    let mut st = vec![0u8; (unsafe { libsodium_sys::crypto_generichash_statebytes() })];
    let pst = unsafe {
        mem::transmute::<*mut u8, *mut libsodium_sys::crypto_generichash_state>(st.as_mut_ptr())
    };
    unsafe {
        libsodium_sys::crypto_generichash_init(pst, ptr::null_mut(), 0, out.len());
    }
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        let chunk = &buf[0..bytes_read];
        unsafe {
            libsodium_sys::crypto_generichash_update(pst, chunk.as_ptr(), chunk.len() as u64);
        }
    }
    unsafe {
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    Ok(out.to_hex())
}

#[cfg(test)]
mod test {
    use std::env;
    #[allow(unused_imports)]
    use std::fs::{self, File};
    #[allow(unused_imports)]
    use std::io;
    use std::path::PathBuf;

    #[cfg(feature = "functional")]
    use hyper::{header, Client, Url};
    use super::*;
    use super::super::test_support::*;

    #[allow(dead_code)]
    fn mk_local_tmpdir() -> PathBuf {
        let dir = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("tmp");
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn hash_file_working() {
        // The expected values were computed using the `b2sum` program from
        // https://github.com/dchest/b2sum using the the `-s=32` option. For example:
        //      b2sum -s=32 signme.dat

        let computed = hash_file(&fixture("signme.dat")).unwrap();
        let expected = "20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233";
        assert_eq!(computed, expected);

        let computed = hash_file(&fixture("happyhumans-20160424223347.sig.key")).unwrap();
        let expected = "e966844bbc50b7a3a6d81e94dd38e27b92814b944095a8e55f1780921bfcfbe1";
        assert_eq!(computed, expected);

        let computed = hash_file(&fixture("happyhumans-20160424223347.pub")).unwrap();
        let expected = "b80c4f412f9a0a7727b6e6f115e1b5fa3bae79ad2fcf47f769ed4e42cfb12265";
        assert_eq!(computed, expected);
    }

    #[test]
    #[cfg(feature = "functional")]
    fn hash_file_large_binary() {
        let url = "http://ftp.kernel.org/pub/linux/kernel/v4.x/linux-4.3.tar.gz";
        let dst: PathBuf = {
            let file = mk_local_tmpdir().join(url.rsplit('/').next().unwrap());
            if !file.is_file() {
                let client = match env::var("http_proxy") {
                    Ok(url) => {
                        let url = Url::parse(&url).unwrap();
                        Client::with_http_proxy(
                            url.host_str().unwrap().to_string(),
                            url.port_or_known_default().unwrap(),
                        )
                    }
                    _ => Client::new(),
                };
                let mut response = client
                    .get(url)
                    .header(header::Connection::close())
                    .send()
                    .unwrap();
                let mut f = File::create(&file).unwrap();
                io::copy(&mut response, &mut f).unwrap();
            }
            file
        };
        let computed = hash_file(&dst);
        let expected = "ba640dc063f0ed27e60b38dbb7cf19778cf7805d9fc91eb129fb68b409d46414";
        assert_eq!(computed, expected);
    }
}
