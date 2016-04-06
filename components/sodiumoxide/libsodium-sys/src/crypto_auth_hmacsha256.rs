// crypto_auth_hmacsha256.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_auth_hmacsha256_state {
    ictx: crypto_hash_sha256_state,
    octx: crypto_hash_sha256_state,
}

pub const crypto_auth_hmacsha256_BYTES: usize = 32;
pub const crypto_auth_hmacsha256_KEYBYTES: usize = 32;


extern {
    pub fn crypto_auth_hmacsha256_bytes() -> size_t;
    pub fn crypto_auth_hmacsha256_keybytes() -> size_t;
    pub fn crypto_auth_hmacsha256(
        a: *mut [u8; crypto_auth_hmacsha256_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_auth_hmacsha256_KEYBYTES]) -> c_int;
    pub fn crypto_auth_hmacsha256_verify(
        a: *const [u8; crypto_auth_hmacsha256_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_auth_hmacsha256_KEYBYTES]) -> c_int;
    pub fn crypto_auth_hmacsha256_init(
        state: *mut crypto_auth_hmacsha256_state,
        key: *const u8,
        keylen: size_t) -> c_int;
    pub fn crypto_auth_hmacsha256_update(
        state: *mut crypto_auth_hmacsha256_state,
        m: *const u8,
        mlen: c_ulonglong) -> c_int;
    pub fn crypto_auth_hmacsha256_final(
        state: *mut crypto_auth_hmacsha256_state,
        a: *mut [u8; crypto_auth_hmacsha256_BYTES]) -> c_int;
}


#[test]
fn test_crypto_auth_hmacsha256_bytes() {
    assert!(unsafe { crypto_auth_hmacsha256_bytes() as usize } ==
            crypto_auth_hmacsha256_BYTES)
}
#[test]
fn test_crypto_auth_hmacsha256_keybytes() {
    assert!(unsafe { crypto_auth_hmacsha256_keybytes() as usize } ==
            crypto_auth_hmacsha256_KEYBYTES)
}
