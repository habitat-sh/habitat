//! `ed25519`, a signature scheme specified in
//! [Ed25519](http://ed25519.cr.yp.to/). This function is conjectured to meet the
//! standard notion of unforgeability for a public-key signature scheme under
//! chosen-message attacks.
use ffi;
use libc::c_ulonglong;
use std::iter::repeat;
#[cfg(feature = "default")]
use rustc_serialize;

/// Number of bytes in a `Seed`.
pub const SEEDBYTES: usize = ffi::crypto_sign_ed25519_SEEDBYTES;

/// Number of bytes in a `SecretKey`.
pub const SECRETKEYBYTES: usize = ffi::crypto_sign_ed25519_SECRETKEYBYTES;

/// Number of bytes in a `PublicKey`.
pub const PUBLICKEYBYTES: usize = ffi::crypto_sign_ed25519_PUBLICKEYBYTES;

/// Number of bytes in a `Signature`.
pub const SIGNATUREBYTES: usize = ffi::crypto_sign_ed25519_BYTES;

new_type! {
    /// `Seed` that can be used for keypair generation
    ///
    /// The `Seed` is used by `keypair_from_seed()` to generate
    /// a secret and public signature key.
    ///
    /// When a `Seed` goes out of scope its contents
    /// will be zeroed out
    secret Seed(SEEDBYTES);
}

new_type! {
    /// `SecretKey` for signatures
    ///
    /// When a `SecretKey` goes out of scope its contents
    /// will be zeroed out
    secret SecretKey(SECRETKEYBYTES);
}

new_type! {
    /// `PublicKey` for signatures
    public PublicKey(PUBLICKEYBYTES);
}

new_type! {
    /// Detached signature
    public Signature(SIGNATUREBYTES);
}

/// `gen_keypair()` randomly generates a secret key and a corresponding public
/// key.
///
/// THREAD SAFETY: `gen_keypair()` is thread-safe provided that you have
/// called `sodiumoxide::init()` once before using any other function
/// from sodiumoxide.
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    unsafe {
        let mut pk = [0u8; PUBLICKEYBYTES];
        let mut sk = [0u8; SECRETKEYBYTES];
        ffi::crypto_sign_ed25519_keypair(&mut pk, &mut sk);
        (PublicKey(pk), SecretKey(sk))
    }
}

/// `keypair_from_seed()` computes a secret key and a corresponding public key
/// from a `Seed`.
pub fn keypair_from_seed(&Seed(ref seed): &Seed) -> (PublicKey, SecretKey) {
    unsafe {
        let mut pk = [0u8; PUBLICKEYBYTES];
        let mut sk = [0u8; SECRETKEYBYTES];
        ffi::crypto_sign_ed25519_seed_keypair(&mut pk,
                                              &mut sk,
                                              seed);
        (PublicKey(pk), SecretKey(sk))
    }
}

/// `sign()` signs a message `m` using the signer's secret key `sk`.
/// `sign()` returns the resulting signed message `sm`.
pub fn sign(m: &[u8],
            &SecretKey(ref sk): &SecretKey) -> Vec<u8> {
    unsafe {
        let mut sm: Vec<u8> = repeat(0u8).take(m.len() + SIGNATUREBYTES).collect();
        let mut smlen = 0;
        ffi::crypto_sign_ed25519(sm.as_mut_ptr(),
                                 &mut smlen,
                                 m.as_ptr(),
                                 m.len() as c_ulonglong,
                                 sk);
        sm.truncate(smlen as usize);
        sm
    }
}

/// `verify()` verifies the signature in `sm` using the signer's public key `pk`.
/// `verify()` returns the message `Ok(m)`.
/// If the signature fails verification, `verify()` returns `Err(())`.
pub fn verify(sm: &[u8],
              &PublicKey(ref pk): &PublicKey) -> Result<Vec<u8>, ()> {
    unsafe {
        let mut m: Vec<u8> = repeat(0u8).take(sm.len()).collect();
        let mut mlen = 0;
        if ffi::crypto_sign_ed25519_open(m.as_mut_ptr(),
                                         &mut mlen,
                                         sm.as_ptr(),
                                         sm.len() as c_ulonglong,
                                         pk) == 0 {
            m.truncate(mlen as usize);
            Ok(m)
        } else {
            Err(())
        }
    }
}

/// `sign_detached()` signs a message `m` using the signer's secret key `sk`.
/// `sign_detached()` returns the resulting signature `sig`.
pub fn sign_detached(m: &[u8],
                     &SecretKey(ref sk): &SecretKey) -> Signature {
    unsafe {
        let mut sig = [0u8; SIGNATUREBYTES];
        let mut siglen: c_ulonglong = 0;
        ffi::crypto_sign_ed25519_detached(&mut sig,
                                          &mut siglen,
                                          m.as_ptr(),
                                          m.len() as c_ulonglong,
                                          sk);
        assert_eq!(siglen, SIGNATUREBYTES as c_ulonglong);
        Signature(sig)
    }
}

/// `verify_detached()` verifies the signature in `sig` against the message `m`
/// and the signer's public key `pk`.
/// `verify_detached()` returns true if the signature is valid, false otherwise.
pub fn verify_detached(&Signature(ref sig): &Signature,
                       m: &[u8],
                       &PublicKey(ref pk): &PublicKey) -> bool {
    unsafe {
        0 == ffi::crypto_sign_ed25519_verify_detached(sig,
                                                      m.as_ptr(),
                                                      m.len() as c_ulonglong,
                                                      pk)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign_verify() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let (pk, sk) = gen_keypair();
            let m = randombytes(i);
            let sm = sign(&m, &sk);
            let m2 = verify(&sm, &pk);
            assert!(Ok(m) == m2);
        }
    }

    #[test]
    fn test_sign_verify_tamper() {
        use randombytes::randombytes;
        for i in 0..32usize {
            let (pk, sk) = gen_keypair();
            let m = randombytes(i);
            let mut sm = sign(&m, &sk);
            for j in 0..sm.len() {
                sm[j] ^= 0x20;
                assert!(Err(()) == verify(&mut sm, &pk));
                sm[j] ^= 0x20;
            }
        }
    }

    #[test]
    fn test_sign_verify_detached() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let (pk, sk) = gen_keypair();
            let m = randombytes(i);
            let sig = sign_detached(&m, &sk);
            assert!(verify_detached(&sig, &m, &pk));
        }
    }

    #[test]
    fn test_sign_verify_detached_tamper() {
        use randombytes::randombytes;
        for i in 0..32usize {
            let (pk, sk) = gen_keypair();
            let m = randombytes(i);
            let Signature(mut sig) = sign_detached(&m, &sk);
            for j in 0..SIGNATUREBYTES {
                sig[j] ^= 0x20;
                assert!(!verify_detached(&Signature(sig), &m, &pk));
                sig[j] ^= 0x20;
            }
        }
    }

    #[test]
    fn test_sign_verify_seed() {
        use randombytes::{randombytes, randombytes_into};
        for i in 0..256usize {
            let mut seedbuf = [0; 32];
            randombytes_into(&mut seedbuf);
            let seed = Seed(seedbuf);
            let (pk, sk) = keypair_from_seed(&seed);
            let m = randombytes(i);
            let sm = sign(&m, &sk);
            let m2 = verify(&sm, &pk);
            assert!(Ok(m) == m2);
        }
    }

    #[test]
    fn test_sign_verify_tamper_seed() {
        use randombytes::{randombytes, randombytes_into};
        for i in 0..32usize {
            let mut seedbuf = [0; 32];
            randombytes_into(&mut seedbuf);
            let seed = Seed(seedbuf);
            let (pk, sk) = keypair_from_seed(&seed);
            let m = randombytes(i);
            let mut sm = sign(&m, &sk);
            for j in 0..sm.len() {
                sm[j] ^= 0x20;
                assert!(Err(()) == verify(&mut sm, &pk));
                sm[j] ^= 0x20;
            }
        }
    }

    #[test]
    fn test_vectors() {
        // test vectors from the Python implementation
        // from the [Ed25519 Homepage](http://ed25519.cr.yp.to/software.html)
        use rustc_serialize::hex::{FromHex, ToHex};
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let r = BufReader::new(File::open("testvectors/ed25519.input").unwrap());
        for mline in r.lines() {
            let line = mline.unwrap();
            let mut x = line.split(':');
            let x0 = x.next().unwrap();
            let x1 = x.next().unwrap();
            let x2 = x.next().unwrap();
            let x3 = x.next().unwrap();
            let seed_bytes = x0[..64].from_hex().unwrap();
            assert!(seed_bytes.len() == SEEDBYTES);
            let mut seedbuf = [0u8; SEEDBYTES];
            for (s, b) in seedbuf.iter_mut().zip(seed_bytes.iter()) {
                *s = *b
            }
            let seed = Seed(seedbuf);
            let (pk, sk) = keypair_from_seed(&seed);
            let m = x2.from_hex().unwrap();
            let sm = sign(&m, &sk);
            verify(&sm, &pk).unwrap();
            assert!(x1 == pk[..].to_hex());
            assert!(x3 == sm.to_hex());
        }
    }

    #[test]
    fn test_vectors_detached() {
        // test vectors from the Python implementation
        // from the [Ed25519 Homepage](http://ed25519.cr.yp.to/software.html)
        use rustc_serialize::hex::{FromHex, ToHex};
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let r = BufReader::new(File::open("testvectors/ed25519.input").unwrap());
        for mline in r.lines() {
            let line = mline.unwrap();
            let mut x = line.split(':');
            let x0 = x.next().unwrap();
            let x1 = x.next().unwrap();
            let x2 = x.next().unwrap();
            let x3 = x.next().unwrap();
            let seed_bytes = x0[..64].from_hex().unwrap();
            assert!(seed_bytes.len() == SEEDBYTES);
            let mut seedbuf = [0u8; SEEDBYTES];
            for (s, b) in seedbuf.iter_mut().zip(seed_bytes.iter()) {
                *s = *b
            }
            let seed = Seed(seedbuf);
            let (pk, sk) = keypair_from_seed(&seed);
            let m = x2.from_hex().unwrap();
            let sig = sign_detached(&m, &sk);
            assert!(verify_detached(&sig, &m, &pk));
            assert!(x1 == pk[..].to_hex());
            let sm = sig[..].to_hex() + x2; // x2 is m hex encoded
            assert!(x3 == sm);
        }
    }

    #[cfg(feature = "default")]
    #[test]
    fn test_serialisation() {
        use randombytes::randombytes;
        use test_utils::round_trip;
        for i in 0..256usize {
            let (pk, sk) = gen_keypair();
            let m = randombytes(i);
            let sig = sign_detached(&m, &sk);
            round_trip(pk);
            round_trip(sk);
            round_trip(sig);
        }
    }
}

#[cfg(feature = "benchmarks")]
#[cfg(test)]
mod bench {
    extern crate test;
    use randombytes::randombytes;
    use super::*;

    const BENCH_SIZES: [usize; 14] = [0, 1, 2, 4, 8, 16, 32, 64,
                                      128, 256, 512, 1024, 2048, 4096];

    #[bench]
    fn bench_sign(b: &mut test::Bencher) {
        let (_, sk) = gen_keypair();
        let ms: Vec<Vec<u8>> = BENCH_SIZES.iter().map(|s| {
            randombytes(*s)
        }).collect();
        b.iter(|| {
            for m in ms.iter() {
                sign(m, &sk);
            }
        });
    }

    #[bench]
    fn bench_verify(b: &mut test::Bencher) {
        let (pk, sk) = gen_keypair();
        let sms: Vec<Vec<u8>> = BENCH_SIZES.iter().map(|s| {
            let m = randombytes(*s);
            sign(&m, &sk)
        }).collect();
        b.iter(|| {
            for sm in sms.iter() {
                verify(sm, &pk);
            }
        });
    }
}
