// crypto_scalarmult_curve25519.h

pub const crypto_scalarmult_curve25519_BYTES: usize = 32;
pub const crypto_scalarmult_curve25519_SCALARBYTES: usize = 32;

extern {
    pub fn crypto_scalarmult_curve25519_bytes() -> size_t;
    pub fn crypto_scalarmult_curve25519_scalarbytes() -> size_t;
    pub fn crypto_scalarmult_curve25519_base(
        q: *mut [u8; crypto_scalarmult_curve25519_BYTES],
        n: *const [u8; crypto_scalarmult_curve25519_SCALARBYTES]) -> c_int;
    pub fn crypto_scalarmult_curve25519(
        q: *mut [u8; crypto_scalarmult_curve25519_BYTES],
        n: *const [u8; crypto_scalarmult_curve25519_SCALARBYTES],
        p: *const [u8; crypto_scalarmult_curve25519_BYTES]) -> c_int;
}

#[test]
fn test_crypto_scalarmult_curve25519_bytes() {
    assert_eq!(unsafe { crypto_scalarmult_curve25519_bytes() as usize },
               crypto_scalarmult_curve25519_BYTES);
}

#[test]
fn test_crypto_scalarmult_curve25519_scalarbytes() {
    assert_eq!(unsafe { crypto_scalarmult_curve25519_scalarbytes() as usize },
               crypto_scalarmult_curve25519_SCALARBYTES);
}
