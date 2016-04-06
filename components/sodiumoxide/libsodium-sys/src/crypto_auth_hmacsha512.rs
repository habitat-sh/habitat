// crypto_auth_hmacsha512.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_auth_hmacsha512_state {
    ictx: crypto_hash_sha512_state,
    octx: crypto_hash_sha512_state,
}

pub const crypto_auth_hmacsha512_BYTES: usize = 64;
pub const crypto_auth_hmacsha512_KEYBYTES: usize = 32;


extern {
    pub fn crypto_auth_hmacsha512_bytes() -> size_t;
    pub fn crypto_auth_hmacsha512_keybytes() -> size_t;
    pub fn crypto_auth_hmacsha512(
        a: *mut [u8; crypto_auth_hmacsha512_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_auth_hmacsha512_KEYBYTES]) -> c_int;
    pub fn crypto_auth_hmacsha512_verify(
        a: *const [u8; crypto_auth_hmacsha512_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_auth_hmacsha512_KEYBYTES]) -> c_int;
    pub fn crypto_auth_hmacsha512_init(
        state: *mut crypto_auth_hmacsha512_state,
        key: *const u8,
        keylen: size_t) -> c_int;
    pub fn crypto_auth_hmacsha512_update(
        state: *mut crypto_auth_hmacsha512_state,
        m: *const u8,
        mlen: c_ulonglong) -> c_int;
    pub fn crypto_auth_hmacsha512_final(
        state: *mut crypto_auth_hmacsha512_state,
        a: *mut [u8; crypto_auth_hmacsha512_BYTES]) -> c_int;
}


#[test]
fn test_crypto_auth_hmacsha512_bytes() {
    assert!(unsafe { crypto_auth_hmacsha512_bytes() as usize } ==
            crypto_auth_hmacsha512_BYTES)
}
#[test]
fn test_crypto_auth_hmacsha512_keybytes() {
    assert!(unsafe { crypto_auth_hmacsha512_keybytes() as usize } ==
            crypto_auth_hmacsha512_KEYBYTES)
}
