//! `crypto_box_curve25519xsalsa20poly1305` , a particular
//! combination of Curve25519, Salsa20, and Poly1305 specified in
//! [Cryptography in NaCl](http://nacl.cr.yp.to/valid.html).
//!
//! This function is conjectured to meet the standard notions of privacy and
//! third-party unforgeability.
use ffi;
use marshal::marshal;
use randombytes::randombytes_into;
#[cfg(feature = "default")]
use rustc_serialize;

/// Number of bytes in a `PublicKey`.
pub const PUBLICKEYBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES;

/// Number of bytes in a `SecretKey`.
pub const SECRETKEYBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES;

/// Number of bytes in a `Nonce`.
pub const NONCEBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_NONCEBYTES;

/// Number of bytes in a `PrecomputedKey`.
pub const PRECOMPUTEDKEYBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES;

const ZEROBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_ZEROBYTES;
const BOXZEROBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES;

/// Number of bytes in the authenticator tag of an encrypted message
/// i.e. the number of bytes by which the ciphertext is larger than the
/// plaintext.
pub const MACBYTES: usize = ffi::crypto_box_curve25519xsalsa20poly1305_MACBYTES;

new_type! {
    /// `SecretKey` for asymmetric authenticated encryption
    ///
    /// When a `SecretKey` goes out of scope its contents
    /// will be zeroed out
    secret SecretKey(SECRETKEYBYTES);
}

new_type! {
    /// `PublicKey` for asymmetric authenticated encryption
    public PublicKey(PUBLICKEYBYTES);
}

new_type! {
    /// `Nonce` for asymmetric authenticated encryption
    nonce Nonce(NONCEBYTES);
}

/// `gen_keypair()` randomly generates a secret key and a corresponding public key.
///
/// THREAD SAFETY: `gen_keypair()` is thread-safe provided that you have
/// called `sodiumoxide::init()` once before using any other function
/// from sodiumoxide.
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    unsafe {
        let mut pk = [0u8; PUBLICKEYBYTES];
        let mut sk = [0u8; SECRETKEYBYTES];
        ffi::crypto_box_curve25519xsalsa20poly1305_keypair(
            &mut pk,
            &mut sk);
        (PublicKey(pk), SecretKey(sk))
    }
}

/// `gen_nonce()` randomly generates a nonce
///
/// THREAD SAFETY: `gen_nonce()` is thread-safe provided that you have
/// called `sodiumoxide::init()` once before using any other function
/// from sodiumoxide.
pub fn gen_nonce() -> Nonce {
    let mut n = [0; NONCEBYTES];
    randombytes_into(&mut n);
    Nonce(n)
}

/// `seal()` encrypts and authenticates a message `m` using the senders secret key `sk`,
/// the receivers public key `pk` and a nonce `n`. It returns a ciphertext `c`.
pub fn seal(m: &[u8],
            &Nonce(ref n): &Nonce,
            &PublicKey(ref pk): &PublicKey,
            &SecretKey(ref sk): &SecretKey) -> Vec<u8> {
    let (c, _) = marshal(m, ZEROBYTES, BOXZEROBYTES, |dst, src, len| {
        unsafe {
            ffi::crypto_box_curve25519xsalsa20poly1305(dst,
                                                       src,
                                                       len,
                                                       n,
                                                       pk,
                                                       sk);
        }
    });
    c
}

/// `open()` verifies and decrypts a ciphertext `c` using the receiver's secret key `sk`,
/// the senders public key `pk`, and a nonce `n`. It returns a plaintext `Ok(m)`.
/// If the ciphertext fails verification, `open()` returns `Err(())`.
pub fn open(c: &[u8],
            &Nonce(ref n): &Nonce,
            &PublicKey(ref pk): &PublicKey,
            &SecretKey(ref sk): &SecretKey) -> Result<Vec<u8>, ()> {
    if c.len() < BOXZEROBYTES {
        return Err(());
    }
    let (m, ret) = marshal(c, BOXZEROBYTES, ZEROBYTES, |dst, src, len| {
        unsafe {
            ffi::crypto_box_curve25519xsalsa20poly1305_open(dst,
                                                            src,
                                                            len,
                                                            n,
                                                            pk,
                                                            sk)
        }
    });
    if ret == 0 {
        Ok(m)
    } else {
        Err(())
    }
}

new_type! {
    /// Applications that send several messages to the same receiver can gain speed by
    /// splitting `seal()` into two steps, `precompute()` and `seal_precomputed()`.
    /// Similarly, applications that receive several messages from the same sender can gain
    /// speed by splitting `open()` into two steps, `precompute()` and `open_precomputed()`.
    ///
    /// When a `PrecomputedKey` goes out of scope its contents will be zeroed out
    secret PrecomputedKey(PRECOMPUTEDKEYBYTES);
}

/// `precompute()` computes an intermediate key that can be used by `seal_precomputed()`
/// and `open_precomputed()`
pub fn precompute(&PublicKey(ref pk): &PublicKey,
                  &SecretKey(ref sk): &SecretKey) -> PrecomputedKey {
    let mut k = [0u8; PRECOMPUTEDKEYBYTES];
    unsafe {
        ffi::crypto_box_curve25519xsalsa20poly1305_beforenm(&mut k,
                                                            pk,
                                                            sk);
    }
    PrecomputedKey(k)
}

/// `seal_precomputed()` encrypts and authenticates a message `m` using a precomputed key `k`,
/// and a nonce `n`. It returns a ciphertext `c`.
pub fn seal_precomputed(m: &[u8],
                        &Nonce(ref n): &Nonce,
                        &PrecomputedKey(ref k): &PrecomputedKey) -> Vec<u8> {
    let (c, _) = marshal(m, ZEROBYTES, BOXZEROBYTES, |dst, src, len| {
        unsafe {
            ffi::crypto_box_curve25519xsalsa20poly1305_afternm(dst,
                                                               src,
                                                               len,
                                                               n,
                                                               k);
        }
    });
    c
}

/// `open_precomputed()` verifies and decrypts a ciphertext `c` using a precomputed
/// key `k` and a nonce `n`. It returns a plaintext `Ok(m)`.
/// If the ciphertext fails verification, `open_precomputed()` returns `Err(())`.
pub fn open_precomputed(c: &[u8],
                        &Nonce(ref n): &Nonce,
                        &PrecomputedKey(ref k): &PrecomputedKey) -> Result<Vec<u8>, ()> {
    if c.len() < BOXZEROBYTES {
        return Err(());
    }
    let (m, ret) = marshal(c, BOXZEROBYTES, ZEROBYTES, |dst, src, len| {
        unsafe {
            ffi::crypto_box_curve25519xsalsa20poly1305_open_afternm(dst,
                                                                    src,
                                                                    len,
                                                                    n,
                                                                    k)
        }
    });
    if ret == 0 {
        Ok(m)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seal_open() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let (pk1, sk1) = gen_keypair();
            let (pk2, sk2) = gen_keypair();
            let m = randombytes(i);
            let n = gen_nonce();
            let c = seal(&m, &n, &pk1, &sk2);
            let opened = open(&c, &n, &pk2, &sk1);
            assert!(Ok(m) == opened);
        }
    }

    #[test]
    fn test_seal_open_precomputed() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let (pk1, sk1) = gen_keypair();
            let (pk2, sk2) = gen_keypair();
            let k1 = precompute(&pk1, &sk2);
            let PrecomputedKey(k1buf) = k1;
            let k2 = precompute(&pk2, &sk1);
            let PrecomputedKey(k2buf) = k2;
            assert!(k1buf == k2buf);
            let m = randombytes(i);
            let n = gen_nonce();
            let c = seal_precomputed(&m, &n, &k1);
            let opened = open_precomputed(&c, &n, &k2);
            assert!(Ok(m) == opened);
        }
    }

    #[test]
    fn test_seal_open_tamper() {
        use randombytes::randombytes;
        for i in 0..32usize {
            let (pk1, sk1) = gen_keypair();
            let (pk2, sk2) = gen_keypair();
            let m = randombytes(i);
            let n = gen_nonce();
            let mut c = seal(&m, &n, &pk1, &sk2);
            for j in 0..c.len() {
                c[j] ^= 0x20;
                assert!(Err(()) == open(&mut c, &n, &pk2, &sk1));
                c[j] ^= 0x20;
            }
        }
    }

    #[test]
    fn test_seal_open_precomputed_tamper() {
        use randombytes::randombytes;
        for i in 0..32usize {
            let (pk1, sk1) = gen_keypair();
            let (pk2, sk2) = gen_keypair();
            let k1 = precompute(&pk1, &sk2);
            let k2 = precompute(&pk2, &sk1);
            let m = randombytes(i);
            let n = gen_nonce();
            let mut c = seal_precomputed(&m, &n, &k1);
            for j in 0..c.len() {
                c[j] ^= 0x20;
                assert!(Err(()) == open_precomputed(&mut c, &n, &k2));
                c[j] ^= 0x20;
            }
        }
    }

    #[test]
    fn test_vector_1() {
        // corresponding to tests/box.c and tests/box3.cpp from NaCl
        let alicesk = SecretKey([0x77,0x07,0x6d,0x0a,0x73,0x18,0xa5,0x7d,
                                 0x3c,0x16,0xc1,0x72,0x51,0xb2,0x66,0x45,
                                 0xdf,0x4c,0x2f,0x87,0xeb,0xc0,0x99,0x2a,
                                 0xb1,0x77,0xfb,0xa5,0x1d,0xb9,0x2c,0x2a]);
        let bobpk   = PublicKey([0xde,0x9e,0xdb,0x7d,0x7b,0x7d,0xc1,0xb4,
                                 0xd3,0x5b,0x61,0xc2,0xec,0xe4,0x35,0x37,
                                 0x3f,0x83,0x43,0xc8,0x5b,0x78,0x67,0x4d,
                                 0xad,0xfc,0x7e,0x14,0x6f,0x88,0x2b,0x4f]);
        let nonce   = Nonce([0x69,0x69,0x6e,0xe9,0x55,0xb6,0x2b,0x73,
                             0xcd,0x62,0xbd,0xa8,0x75,0xfc,0x73,0xd6,
                             0x82,0x19,0xe0,0x03,0x6b,0x7a,0x0b,0x37]);
        let m = [0xbe,0x07,0x5f,0xc5,0x3c,0x81,0xf2,0xd5,
                 0xcf,0x14,0x13,0x16,0xeb,0xeb,0x0c,0x7b,
                 0x52,0x28,0xc5,0x2a,0x4c,0x62,0xcb,0xd4,
                 0x4b,0x66,0x84,0x9b,0x64,0x24,0x4f,0xfc,
                 0xe5,0xec,0xba,0xaf,0x33,0xbd,0x75,0x1a,
                 0x1a,0xc7,0x28,0xd4,0x5e,0x6c,0x61,0x29,
                 0x6c,0xdc,0x3c,0x01,0x23,0x35,0x61,0xf4,
                 0x1d,0xb6,0x6c,0xce,0x31,0x4a,0xdb,0x31,
                 0x0e,0x3b,0xe8,0x25,0x0c,0x46,0xf0,0x6d,
                 0xce,0xea,0x3a,0x7f,0xa1,0x34,0x80,0x57,
                 0xe2,0xf6,0x55,0x6a,0xd6,0xb1,0x31,0x8a,
                 0x02,0x4a,0x83,0x8f,0x21,0xaf,0x1f,0xde,
                 0x04,0x89,0x77,0xeb,0x48,0xf5,0x9f,0xfd,
                 0x49,0x24,0xca,0x1c,0x60,0x90,0x2e,0x52,
                 0xf0,0xa0,0x89,0xbc,0x76,0x89,0x70,0x40,
                 0xe0,0x82,0xf9,0x37,0x76,0x38,0x48,0x64,
                 0x5e,0x07,0x05];
        let c = seal(&m, &nonce, &bobpk, &alicesk);
        let pk = precompute(&bobpk, &alicesk);
        let cpre = seal_precomputed(&m, &nonce, &pk);
        let cexp = vec![0xf3,0xff,0xc7,0x70,0x3f,0x94,0x00,0xe5,
                        0x2a,0x7d,0xfb,0x4b,0x3d,0x33,0x05,0xd9,
                        0x8e,0x99,0x3b,0x9f,0x48,0x68,0x12,0x73,
                        0xc2,0x96,0x50,0xba,0x32,0xfc,0x76,0xce,
                        0x48,0x33,0x2e,0xa7,0x16,0x4d,0x96,0xa4,
                        0x47,0x6f,0xb8,0xc5,0x31,0xa1,0x18,0x6a,
                        0xc0,0xdf,0xc1,0x7c,0x98,0xdc,0xe8,0x7b,
                        0x4d,0xa7,0xf0,0x11,0xec,0x48,0xc9,0x72,
                        0x71,0xd2,0xc2,0x0f,0x9b,0x92,0x8f,0xe2,
                        0x27,0x0d,0x6f,0xb8,0x63,0xd5,0x17,0x38,
                        0xb4,0x8e,0xee,0xe3,0x14,0xa7,0xcc,0x8a,
                        0xb9,0x32,0x16,0x45,0x48,0xe5,0x26,0xae,
                        0x90,0x22,0x43,0x68,0x51,0x7a,0xcf,0xea,
                        0xbd,0x6b,0xb3,0x73,0x2b,0xc0,0xe9,0xda,
                        0x99,0x83,0x2b,0x61,0xca,0x01,0xb6,0xde,
                        0x56,0x24,0x4a,0x9e,0x88,0xd5,0xf9,0xb3,
                        0x79,0x73,0xf6,0x22,0xa4,0x3d,0x14,0xa6,
                        0x59,0x9b,0x1f,0x65,0x4c,0xb4,0x5a,0x74,
                        0xe3,0x55,0xa5];
        assert!(c == cexp);
        assert!(cpre == cexp);
    }

    #[test]
    fn test_vector_2() {
        // corresponding to tests/box2.c and tests/box4.cpp from NaCl
        let bobsk = SecretKey([0x5d,0xab,0x08,0x7e,0x62,0x4a,0x8a,0x4b,
                               0x79,0xe1,0x7f,0x8b,0x83,0x80,0x0e,0xe6,
                               0x6f,0x3b,0xb1,0x29,0x26,0x18,0xb6,0xfd,
                               0x1c,0x2f,0x8b,0x27,0xff,0x88,0xe0,0xeb]);
        let alicepk = PublicKey([0x85,0x20,0xf0,0x09,0x89,0x30,0xa7,0x54,
                                 0x74,0x8b,0x7d,0xdc,0xb4,0x3e,0xf7,0x5a,
                                 0x0d,0xbf,0x3a,0x0d,0x26,0x38,0x1a,0xf4,
                                 0xeb,0xa4,0xa9,0x8e,0xaa,0x9b,0x4e,0x6a]);
        let nonce = Nonce([0x69,0x69,0x6e,0xe9,0x55,0xb6,0x2b,0x73,
                           0xcd,0x62,0xbd,0xa8,0x75,0xfc,0x73,0xd6,
                           0x82,0x19,0xe0,0x03,0x6b,0x7a,0x0b,0x37]);
        let c = [0xf3,0xff,0xc7,0x70,0x3f,0x94,0x00,0xe5,
                 0x2a,0x7d,0xfb,0x4b,0x3d,0x33,0x05,0xd9,
                 0x8e,0x99,0x3b,0x9f,0x48,0x68,0x12,0x73,
                 0xc2,0x96,0x50,0xba,0x32,0xfc,0x76,0xce,
                 0x48,0x33,0x2e,0xa7,0x16,0x4d,0x96,0xa4,
                 0x47,0x6f,0xb8,0xc5,0x31,0xa1,0x18,0x6a,
                 0xc0,0xdf,0xc1,0x7c,0x98,0xdc,0xe8,0x7b,
                 0x4d,0xa7,0xf0,0x11,0xec,0x48,0xc9,0x72,
                 0x71,0xd2,0xc2,0x0f,0x9b,0x92,0x8f,0xe2,
                 0x27,0x0d,0x6f,0xb8,0x63,0xd5,0x17,0x38,
                 0xb4,0x8e,0xee,0xe3,0x14,0xa7,0xcc,0x8a,
                 0xb9,0x32,0x16,0x45,0x48,0xe5,0x26,0xae,
                 0x90,0x22,0x43,0x68,0x51,0x7a,0xcf,0xea,
                 0xbd,0x6b,0xb3,0x73,0x2b,0xc0,0xe9,0xda,
                 0x99,0x83,0x2b,0x61,0xca,0x01,0xb6,0xde,
                 0x56,0x24,0x4a,0x9e,0x88,0xd5,0xf9,0xb3,
                 0x79,0x73,0xf6,0x22,0xa4,0x3d,0x14,0xa6,
                 0x59,0x9b,0x1f,0x65,0x4c,0xb4,0x5a,0x74,
                 0xe3,0x55,0xa5];
        let mexp = Ok(vec![0xbe,0x07,0x5f,0xc5,0x3c,0x81,0xf2,0xd5,
                             0xcf,0x14,0x13,0x16,0xeb,0xeb,0x0c,0x7b,
                             0x52,0x28,0xc5,0x2a,0x4c,0x62,0xcb,0xd4,
                             0x4b,0x66,0x84,0x9b,0x64,0x24,0x4f,0xfc,
                             0xe5,0xec,0xba,0xaf,0x33,0xbd,0x75,0x1a,
                             0x1a,0xc7,0x28,0xd4,0x5e,0x6c,0x61,0x29,
                             0x6c,0xdc,0x3c,0x01,0x23,0x35,0x61,0xf4,
                             0x1d,0xb6,0x6c,0xce,0x31,0x4a,0xdb,0x31,
                             0x0e,0x3b,0xe8,0x25,0x0c,0x46,0xf0,0x6d,
                             0xce,0xea,0x3a,0x7f,0xa1,0x34,0x80,0x57,
                             0xe2,0xf6,0x55,0x6a,0xd6,0xb1,0x31,0x8a,
                             0x02,0x4a,0x83,0x8f,0x21,0xaf,0x1f,0xde,
                             0x04,0x89,0x77,0xeb,0x48,0xf5,0x9f,0xfd,
                             0x49,0x24,0xca,0x1c,0x60,0x90,0x2e,0x52,
                             0xf0,0xa0,0x89,0xbc,0x76,0x89,0x70,0x40,
                             0xe0,0x82,0xf9,0x37,0x76,0x38,0x48,0x64,
                             0x5e,0x07,0x05]);
        let m = open(&c, &nonce, &alicepk, &bobsk);
        let pk = precompute(&alicepk, &bobsk);
        let m_pre = open_precomputed(&c, &nonce, &pk);
        assert!(m == mexp);
        assert!(m_pre == mexp);
    }

    #[cfg(feature = "default")]
    #[test]
    fn test_serialisation() {
        use test_utils::round_trip;
        for _ in 0..256usize {
            let (pk, sk) = gen_keypair();
            let n = gen_nonce();
            round_trip(pk);
            round_trip(sk);
            round_trip(n);
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
    fn bench_seal_open(b: &mut test::Bencher) {
        let (pk, sk) = gen_keypair();
        let n = gen_nonce();
        let ms: Vec<Vec<u8>> = BENCH_SIZES.iter().map(|s| {
            randombytes(*s)
        }).collect();
        b.iter(|| {
            for m in ms.iter() {
                open(&seal(m, &n, &pk, &sk), &n, &pk, &sk).unwrap();
            }
        });
    }

    #[bench]
    fn bench_precompute(b: &mut test::Bencher) {
        let (pk, sk) = gen_keypair();
        b.iter(|| {
            // we do this benchmark as many times as the other benchmarks so
            // that we can compare the times
            for _ in BENCH_SIZES.iter() {
                precompute(&pk, &sk);
                precompute(&pk, &sk);
            }
        });
    }
}
