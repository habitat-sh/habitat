use crate::error::{Error,
                   Result};
use blake2b_simd::{Params,
                   State};
use hex::FromHex;
use serde::Serialize;
use std::{convert::TryInto,
          fmt,
          fs::File,
          io::{BufReader,
               Read},
          path::Path,
          str::FromStr};

/// When hashing byte streams, we'll read 1KB at a time, adding this to the
/// internal hashing state as we compute the final digest.
const BUF_SIZE: usize = 1024;

/// The size of our Blake2b hash digests (32 bytes)
const HASH_DIGEST_SIZE: usize = 32;

/// Convenience wrapper type for a 32-byte Blake2b hash digest.
///
/// Implements secure equality comparison, as well as hex-encoding via
/// `std::fmt::Display`.
///
/// There are intentionally no explicit constructors; to get a
/// instance, you'll need to either convert it directly from a `&[u8]`
/// or parse it from a hex string.
#[derive(Clone, Debug, Eq)]
pub struct Blake2bHash {
    digest: [u8; HASH_DIGEST_SIZE],
}

impl Blake2bHash {
    /// Calculate the BLAKE2b hash of the contents of a file.
    pub fn from_file<P>(filename: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let file = File::open(filename.as_ref())?;
        let mut reader = BufReader::new(file);
        Self::from_reader(&mut reader)
    }

    /// Calculate the BLAKE2b hash of a sequence of bytes.
    pub fn from_bytes<B>(data: B) -> Self
        where B: AsRef<[u8]>
    {
        let mut state = hash_state();
        state.update(data.as_ref());
        state.finalize().into()
    }

    /// Calculate the BLAKE2b hash of a Read implentation.
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self> {
        let mut state = hash_state();

        let mut buf = [0u8; BUF_SIZE];
        loop {
            let bytes_read = reader.read(&mut buf)?;
            if bytes_read == 0 {
                break;
            }
            let chunk = &buf[0..bytes_read];
            state.update(chunk);
        }

        Ok(state.finalize().into())
    }
}

// We *could* just wrap the `blake2b_simd::Hash` directly in our
// `Blake2bHash` type, but then we wouldn't be able to parse a
// `Blake2bHash` from a string, because `blake2b_simd::Hash does not
// have that ability. So, we'll just grab the bytes and call it a
// day.
impl From<blake2b_simd::Hash> for Blake2bHash {
    fn from(src: blake2b_simd::Hash) -> Self {
        let digest = src.as_bytes()
                        .try_into()
                        .expect("We know we can safely convert to a byte array");
        Blake2bHash { digest }
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
        let bytes = FromHex::from_hex(s).map_err(|e| {
                                            Error::CryptoError(format!("Could not parse \
                                                                        Blake2bHash from string: \
                                                                        {}",
                                                                       e))
                                        })?;
        Ok(Self { digest: bytes })
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

////////////////////////////////////////////////////////////////////////

/// Initialize the hasher state. In particular, set the digest length
/// to 32 bytes. All hashing functions must use this to ensure
/// consistency!
fn hash_state() -> State {
    let mut params = Params::new();
    params.hash_length(HASH_DIGEST_SIZE);
    params.to_state()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::test_support::fixture;

    /// Helper function to create a Blake2bHash directly from a
    /// hex-encoded string.
    fn hash_from_hex(hex: &str) -> Blake2bHash { hex.parse().unwrap() }

    #[test]
    fn strings_can_be_hashed() {
        let message = "supercalifragilisticexpialadocious";
        let expected =
            hash_from_hex("2ca8ebafca7e189de2a36125b92a1db20f393d1e2708f5daa55e51cf05114437");
        let actual = Blake2bHash::from_bytes(message);

        assert_eq!(expected, actual);
    }

    #[test]
    fn bytes_can_be_hashed() {
        let message = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
        let expected =
            hash_from_hex("8b57a796a5d07cb04cc1614dfc2acb3f73edc712d7f433619ca3bbe66bb15f49");
        let actual = Blake2bHash::from_bytes(&message);

        assert_eq!(expected, actual);
    }

    #[test]
    fn hash_file_working() {
        // The expected values were computed using the `b2sum` program from
        // https://github.com/dchest/b2sum using the the `-s=32` option. For example:
        //      b2sum -s=32 signme.dat

        let computed = Blake2bHash::from_file(&fixture("signme.dat")).unwrap();
        let expected =
            hash_from_hex("20590a52c4f00588c500328b16d466c982a26fabaa5fa4dcc83052dd0a84f233");
        assert_eq!(computed, expected);

        let computed =
            Blake2bHash::from_file(&fixture("happyhumans-20160424223347.sig.key")).unwrap();
        let expected =
            hash_from_hex("e966844bbc50b7a3a6d81e94dd38e27b92814b944095a8e55f1780921bfcfbe1");
        assert_eq!(computed, expected);

        let computed = Blake2bHash::from_file(&fixture("happyhumans-20160424223347.pub")).unwrap();
        let expected =
            hash_from_hex("b80c4f412f9a0a7727b6e6f115e1b5fa3bae79ad2fcf47f769ed4e42cfb12265");
        assert_eq!(computed, expected);
    }

    #[test]
    #[cfg(feature = "functional")]
    fn hash_file_large_binary() {
        let dir = tempfile::Builder::new().prefix("large_file")
                                          .tempdir()
                                          .unwrap();

        let url = "http://www.kernel.org/pub/linux/kernel/v4.x/linux-4.3.tar.gz";
        let file = dir.path().join(url.rsplit('/').next().unwrap());

        // Download the doc to the temp directory
        let mut f = File::create(&file).unwrap();
        let mut res = reqwest::blocking::get(url).unwrap();
        res.copy_to(&mut f).unwrap();

        let computed = Blake2bHash::from_file(&file).unwrap();
        let expected =
            hash_from_hex("ba640dc063f0ed27e60b38dbb7cf19778cf7805d9fc91eb129fb68b409d46414");
        assert_eq!(computed, expected);
    }

    #[test]
    fn eq() {
        let zeroes = Blake2bHash { digest: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], };

        let ones = Blake2bHash { digest: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                          1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                          1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8], };

        assert_ne!(zeroes, ones);
        assert_eq!(zeroes, zeroes);
        assert_eq!(ones, ones);
    }

    #[test]
    fn display() {
        let ones = Blake2bHash { digest: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                          1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
                                          1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8], };
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
        let h =
            "01010101010101010101010101010101010101010101010101010101010101".parse::<Blake2bHash>();
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
