use crate::error::{Error,
                   Result};
use hex::FromHex;
use serde::Serialize;
use std::{convert::{TryFrom,
                    TryInto},
          fmt,
          fs::File,
          io::{BufReader,
               Read},
          path::Path,
          ptr,
          str::FromStr};

const BUF_SIZE: usize = 1024;

/// The size of a Blake2b hash digest (32 bytes)
const HASH_DIGEST_SIZE: usize = libsodium_sys::crypto_generichash_BYTES as usize;

/// Convenience wrapper type for a 32-byte Blake2b hash digest.
///
/// Implements secure equality comparison, as well as hex-encoding via
/// `std::fmt::Display`.
///
/// There are intentionally no explicit constructors; to get a
/// instance, you'll need to either convert it directly from a `&[u8]`
/// or parse it from a hex string.
#[derive(Clone, Debug)]
pub struct Blake2bHash {
    digest:     [u8; HASH_DIGEST_SIZE],
    /// Temporary field to support Deref<str> for backwards
    /// compatibility with Builder until it can use the new types.
    hex_string: String,
}

impl Blake2bHash {
    /// Temporary constructor while we store the hex encoding in the
    /// type directly.
    fn new(digest: [u8; HASH_DIGEST_SIZE]) -> Self {
        let hex_string = hex::encode(&digest).to_lowercase();
        Blake2bHash { digest, hex_string }
    }
}

impl TryFrom<Vec<u8>> for Blake2bHash {
    type Error = Error;

    // Create a Blake2bHash from a vector of bytes. Ensures the vector
    // is the correct length for the hash.
    fn try_from(value: Vec<u8>) -> Result<Self> {
        let boxed: Box<[u8]> = value.into_boxed_slice();
        let boxed: Box<[u8; HASH_DIGEST_SIZE]> =
            boxed.try_into().map_err(|_| {
                                 Error::CryptoError(format!("Did not get {} bytes for digest",
                                                            HASH_DIGEST_SIZE))
                             })?;
        Ok(Blake2bHash::new(*boxed))
    }
}

impl AsRef<[u8]> for Blake2bHash {
    fn as_ref(&self) -> &[u8] { &self.digest }
}

impl PartialEq for Blake2bHash {
    /// Implement secure equality comparison for our hashes here so we
    /// don't have to think about it elsewhere.
    // Strictly speaking, secure equality comparison shouldn't be
    // necessary; however, having a type encapsulate this gives us a
    // convenient way to modify this in the future.
    fn eq(&self, other: &Blake2bHash) -> bool { crate::crypto::secure_eq(self, other) }
}

impl Eq for Blake2bHash {}

impl fmt::Display for Blake2bHash {
    /// Displays a Blake2bHash as a lowercase hex-encoded string.
    ///
    /// Due to historical precedent, the lowercasing *is* significant,
    /// as we sign the lowercase hex-encoded version of a Blake2b
    /// hash, and not simply the Blake2b hash itself, when we sign a
    /// HART file.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // hex::encode currently outputs lowercase strings, but we
        // want to strictly enforce this and guard against any future
        // changes to that crate.
        hex::encode(self).to_lowercase().fmt(f)
    }
}

impl FromStr for Blake2bHash {
    type Err = Error;

    /// Convert a hex-encoded byte string into a Blake2bHash. Ensures
    /// the string represents a byte array of proper length.
    ///
    /// Case of the incoming string is not significant (e.g.,
    /// "DEADBEEF" and "deadbeef" are equivalent).
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // FromHex has an implementation for [u8; 32], so this ensures
        // the proper length of bytes... well, that and the compiler,
        // of course :)
        FromHex::from_hex(s).map(Self::new).map_err(|e| {
                                               Error::CryptoError(format!("Could not parse \
                                                                           Blake2bHash from \
                                                                           string: {}",
                                                                          e))
                                           })
    }
}

impl Serialize for Blake2bHash {
    /// Serializes a `Blake2bHash` according to its `Display`
    /// implementation (i.e., a lowercase hex-encoded string).
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Temporary implementation to ease adoption in Builder. Once that's
/// been updated, remove this (and the `hex_string` field).
impl std::ops::Deref for Blake2bHash {
    type Target = str;

    fn deref(&self) -> &Self::Target { self.hex_string.as_str() }
}

////////////////////////////////////////////////////////////////////////

/// Calculate the BLAKE2b hash of a file.
/// NOTE: the hashing is keyless
pub fn hash_file<P>(filename: P) -> Result<Blake2bHash>
    where P: AsRef<Path>
{
    let file = File::open(filename.as_ref())?;
    let mut reader = BufReader::new(file);
    hash_reader(&mut reader)
}

pub fn hash_string(data: &str) -> Blake2bHash {
    let mut out = [0u8; HASH_DIGEST_SIZE];
    let mut st = vec![0u8; unsafe { libsodium_sys::crypto_generichash_statebytes() }];
    #[allow(clippy::cast_ptr_alignment)]
    let pst = st.as_mut_ptr() as *mut libsodium_sys::crypto_generichash_state;
    unsafe {
        libsodium_sys::crypto_generichash_init(pst, ptr::null_mut(), 0, out.len());
        libsodium_sys::crypto_generichash_update(pst, data[..].as_ptr(), data.len() as u64);
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    Blake2bHash::new(out)
}

pub fn hash_bytes(data: &[u8]) -> Blake2bHash {
    let mut out = [0u8; HASH_DIGEST_SIZE];
    let mut st = vec![0u8; unsafe { libsodium_sys::crypto_generichash_statebytes() }];
    #[allow(clippy::cast_ptr_alignment)]
    let pst = st.as_mut_ptr() as *mut libsodium_sys::crypto_generichash_state;
    unsafe {
        libsodium_sys::crypto_generichash_init(pst, ptr::null_mut(), 0, out.len());
        libsodium_sys::crypto_generichash_update(pst, data[..].as_ptr(), data.len() as u64);
        libsodium_sys::crypto_generichash_final(pst, out.as_mut_ptr(), out.len());
    }
    Blake2bHash::new(out)
}

pub fn hash_reader(reader: &mut dyn Read) -> Result<Blake2bHash> {
    let mut out = [0u8; HASH_DIGEST_SIZE];
    let mut st = vec![0u8; unsafe { libsodium_sys::crypto_generichash_statebytes() }];
    #[allow(clippy::cast_ptr_alignment)]
    let pst = st.as_mut_ptr() as *mut libsodium_sys::crypto_generichash_state;
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
    Ok(Blake2bHash::new(out))
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use std::fs::{self,
                  File};
    #[allow(unused_imports)]
    use std::io;
    use std::{env,
              path::PathBuf};

    use super::{super::test_support::*,
                *};
    #[cfg(feature = "functional")]
    use hyper::{header,
                Client,
                Url};

    #[allow(dead_code)]
    fn mk_local_tmpdir() -> PathBuf {
        let dir = env::current_exe().unwrap()
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
        let expected = "20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233".parse()
                                                                                         .unwrap();
        assert_eq!(computed, expected);

        let computed = hash_file(&fixture("happyhumans-20160424223347.sig.key")).unwrap();
        let expected = "e966844bbc50b7a3a6d81e94dd38e27b92814b944095a8e55f1780921bfcfbe1".parse()
                                                                                         .unwrap();
        assert_eq!(computed, expected);

        let computed = hash_file(&fixture("happyhumans-20160424223347.pub")).unwrap();
        let expected = "b80c4f412f9a0a7727b6e6f115e1b5fa3bae79ad2fcf47f769ed4e42cfb12265".parse()
                                                                                         .unwrap();
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
                        Client::with_http_proxy(url.host_str().unwrap().to_string(),
                                                url.port_or_known_default().unwrap())
                    }
                    _ => Client::new(),
                };
                let mut response = client.get(url)
                                         .header(header::Connection::close())
                                         .send()
                                         .unwrap();
                let mut f = File::create(&file).unwrap();
                io::copy(&mut response, &mut f).unwrap();
            }
            file
        };
        let computed = hash_file(&dst);
        let expected = "ba640dc063f0ed27e60b38dbb7cf19778cf7805d9fc91eb129fb68b409d46414".parse()
                                                                                         .unwrap();
        assert_eq!(computed, expected);
    }

    mod blake2bhash {
        use super::*;

        #[test]
        fn eq() {
            let zeroes = Blake2bHash::new([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                           0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                           0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);

            let ones = Blake2bHash::new([1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                         1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                         1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8]);

            assert_ne!(zeroes, ones);
            assert_eq!(zeroes, zeroes);
            assert_eq!(ones, ones);
        }

        #[test]
        fn display() {
            let ones = Blake2bHash::new([1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                         1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                         1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8]);
            assert_eq!(ones.to_string(),
                       "0101010101010101010101010101010101010101010101010101010101010101");
        }

        #[test]
        fn from_str_good() {
            // Exactly 32 bytes long
            let h = "0101010101010101010101010101010101010101010101010101010101010101".parse::<Blake2bHash>();
            assert!(h.is_ok());
        }

        #[test]
        fn from_str_short() {
            // This is one byte too short
            let h = "01010101010101010101010101010101010101010101010101010101010101".parse::<Blake2bHash>();
            assert!(h.is_err());
        }
        #[test]
        fn from_str_long() {
            // This is one byte too long
            let h = "010101010101010101010101010101010101010101010101010101010101010101".parse::<Blake2bHash>();
            assert!(h.is_err());
        }

        #[test]
        fn from_str_case_is_insignificant() {
            let lower_case = "20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233";
            let upper_case = "20590A52C4F00588C500328B16D466C982A26FABAA5FA4DCC83052DD0A84F233";

            let l = lower_case.parse::<Blake2bHash>().unwrap();
            let u = upper_case.parse::<Blake2bHash>().unwrap();

            assert_eq!(l, u);
        }

        #[test]
        fn from_str_display_round_trip() {
            let input = "20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233";
            let output = input.parse::<Blake2bHash>().unwrap().to_string();
            assert_eq!(output, input);
        }

        #[test]
        fn serialize_as_hex_encoding() {
            let input = "20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233";
            let hash: Blake2bHash = input.parse().unwrap();
            serde_test::assert_ser_tokens(&hash, &[serde_test::Token::Str(input)]);
        }
    }
}
