// crypto_onetimeauth_poly1305.h

pub const crypto_onetimeauth_poly1305_BYTES: usize = 16;
pub const crypto_onetimeauth_poly1305_KEYBYTES: usize = 32;


extern {
    pub fn crypto_onetimeauth_poly1305(
        a: *mut [u8; crypto_onetimeauth_poly1305_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_onetimeauth_poly1305_KEYBYTES]) -> c_int;
    pub fn crypto_onetimeauth_poly1305_verify(
        a: *const [u8; crypto_onetimeauth_poly1305_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_onetimeauth_poly1305_KEYBYTES]) -> c_int;
    pub fn crypto_onetimeauth_poly1305_bytes() -> size_t;
    pub fn crypto_onetimeauth_poly1305_keybytes() -> size_t;
}


#[test]
fn test_crypto_onetimeauth_poly1305_bytes() {
    assert!(unsafe { crypto_onetimeauth_poly1305_bytes() as usize } ==
            crypto_onetimeauth_poly1305_BYTES)
}
#[test]
fn test_crypto_onetimeauth_poly1305_keybytes() {
    assert!(unsafe { crypto_onetimeauth_poly1305_keybytes() as usize } ==
            crypto_onetimeauth_poly1305_KEYBYTES)
}
