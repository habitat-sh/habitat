// crypto_auth.h

pub const crypto_auth_BYTES: usize = crypto_auth_hmacsha512256_BYTES;
pub const crypto_auth_KEYBYTES: usize = crypto_auth_hmacsha512256_KEYBYTES;
pub const crypto_auth_PRIMITIVE: &'static str = "hmacsha512256";


extern {
    pub fn crypto_auth_bytes() -> size_t;
    pub fn crypto_auth_keybytes() -> size_t;
    pub fn crypto_auth_primitive() -> *const c_char;
    pub fn crypto_auth(a: *mut [u8; crypto_auth_BYTES],
                       m: *const u8,
                       mlen: c_ulonglong,
                       k: *const [u8; crypto_auth_KEYBYTES]) -> c_int;
    pub fn crypto_auth_verify(a: *const [u8; crypto_auth_BYTES],
                              m: *const u8,
                              mlen: c_ulonglong,
                              k: *const [u8; crypto_auth_KEYBYTES]) -> c_int;
}


#[test]
fn test_crypto_auth_bytes() {
    assert!(unsafe { crypto_auth_bytes() as usize } == crypto_auth_BYTES)
}
#[test]
fn test_crypto_auth_keybytes() {
    assert!(unsafe { crypto_auth_keybytes() as usize } ==
            crypto_auth_KEYBYTES)
}
#[test]
fn test_crypto_auth_primitive() {
    unsafe {
        let s = crypto_auth_primitive();
        let s = std::ffi::CStr::from_ptr(s).to_bytes();
        assert!(s == crypto_auth_PRIMITIVE.as_bytes());
    }
}
