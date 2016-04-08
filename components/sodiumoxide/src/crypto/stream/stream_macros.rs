macro_rules! stream_module (($stream_name:ident,
                             $xor_name:ident,
                             $keybytes:expr,
                             $noncebytes:expr) => (

use libc::c_ulonglong;
use std::iter::repeat;
use randombytes::randombytes_into;
#[cfg(feature = "default")]
use rustc_serialize;

/// Number of bytes in a `Key`.
pub const KEYBYTES: usize = $keybytes;

/// Number of bytes in a `Nonce`.
pub const NONCEBYTES: usize = $noncebytes;

new_type! {
    /// `Key` for symmetric encryption
    ///
    /// When a `Key` goes out of scope its contents
    /// will be zeroed out
    secret Key(KEYBYTES);
}

new_type! {
    /// `Nonce` for symmetric encryption
    nonce Nonce(NONCEBYTES);
}

/// `gen_key()` randomly generates a key for symmetric encryption
///
/// THREAD SAFETY: `gen_key()` is thread-safe provided that you have
/// called `sodiumoxide::init()` once before using any other function
/// from sodiumoxide.
pub fn gen_key() -> Key {
    let mut key = [0; KEYBYTES];
    randombytes_into(&mut key);
    Key(key)
}

/// `gen_nonce()` randomly generates a nonce for symmetric encryption
///
/// THREAD SAFETY: `gen_nonce()` is thread-safe provided that you have
/// called `sodiumoxide::init()` once before using any other function
/// from sodiumoxide.
///
/// NOTE: When using primitives with short nonces (e.g. salsa20, salsa208, salsa2012)
/// do not use random nonces since the probability of nonce-collision is not negligible
pub fn gen_nonce() -> Nonce {
    let mut nonce = [0; NONCEBYTES];
    randombytes_into(&mut nonce);
    Nonce(nonce)
}

/// `stream()` produces a `len`-byte stream `c` as a function of a
/// secret key `k` and a nonce `n`.
pub fn stream(len: usize,
              &Nonce(ref n): &Nonce,
              &Key(ref k): &Key) -> Vec<u8> {
    unsafe {
        let mut c: Vec<u8> = repeat(0u8).take(len).collect();
        $stream_name(c.as_mut_ptr(),
                     c.len() as c_ulonglong,
                     n,
                     k);
        c
    }
}

/// `stream_xor()` encrypts a message `m` using a secret key `k` and a nonce `n`.
/// The `stream_xor()` function returns the ciphertext `c`.
///
/// `stream_xor()` guarantees that the ciphertext has the same length as the plaintext,
/// and is the plaintext xor the output of `stream()`.
/// Consequently `stream_xor()` can also be used to decrypt.
pub fn stream_xor(m: &[u8],
                  &Nonce(ref n): &Nonce,
                  &Key(ref k): &Key) -> Vec<u8> {
    unsafe {
        let mut c: Vec<u8> = repeat(0u8).take(m.len()).collect();
        $xor_name(c.as_mut_ptr(),
                  m.as_ptr(),
                  m.len() as c_ulonglong,
                  n,
                  k);
        c
    }
}

/// `stream_xor_inplace` encrypts a message `m` using a secret key `k` and a nonce `n`.
/// The `stream_xor_inplace()` function encrypts the message in place.
///
/// `stream_xor_inplace()` guarantees that the ciphertext has the same length as
/// the plaintext, and is the plaintext xor the output of `stream_inplace()`.
/// Consequently `stream_xor_inplace()` can also be used to decrypt.
pub fn stream_xor_inplace(m: &mut [u8],
                          &Nonce(ref n): &Nonce,
                          &Key(ref k): &Key) {
    unsafe {
        $xor_name(m.as_mut_ptr(),
                  m.as_ptr(),
                  m.len() as c_ulonglong,
                  n,
                  k);
    }
}

#[cfg(test)]
mod test_m {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        use randombytes::randombytes;
        for i in 0..1024usize {
            let k = gen_key();
            let n = gen_nonce();
            let m = randombytes(i);
            let c = stream_xor(&m, &n, &k);
            let m2 = stream_xor(&c, &n, &k);
            assert!(m == m2);
        }
    }

    #[test]
    fn test_stream_xor() {
        use randombytes::randombytes;
        for i in 0..1024usize {
            let k = gen_key();
            let n = gen_nonce();
            let m = randombytes(i);
            let mut c = m.clone();
            let s = stream(c.len(), &n, &k);
            for (e, v) in c.iter_mut().zip(s.iter()) {
                *e ^= *v;
            }
            let c2 = stream_xor(&m, &n, &k);
            assert!(c == c2);
        }
    }

    #[test]
    fn test_stream_xor_inplace() {
        use randombytes::randombytes;
        for i in 0..1024usize {
            let k = gen_key();
            let n = gen_nonce();
            let mut m = randombytes(i);
            let mut c = m.clone();
            let s = stream(c.len(), &n, &k);
            for (e, v) in c.iter_mut().zip(s.iter()) {
                *e ^= *v;
            }
            stream_xor_inplace(&mut m, &n, &k);
            assert!(c == m);
        }
    }

    #[cfg(feature = "default")]
    #[test]
    fn test_serialisation() {
        use test_utils::round_trip;
        for _ in 0..1024usize {
            let k = gen_key();
            let n = gen_nonce();
            round_trip(k);
            round_trip(n);
        }
    }
}

#[cfg(feature = "benchmarks")]
#[cfg(test)]
mod bench_m {
    extern crate test;
    use super::*;

    const BENCH_SIZES: [usize; 14] = [0, 1, 2, 4, 8, 16, 32, 64,
                                      128, 256, 512, 1024, 2048, 4096];

    #[bench]
    fn bench_stream(b: &mut test::Bencher) {
        let k = gen_key();
        let n = gen_nonce();
        b.iter(|| {
            for size in BENCH_SIZES.iter() {
                stream(*size, &n, &k);
            }
        });
    }
}

));
