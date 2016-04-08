// crypto_onetimeauth.h

pub const crypto_onetimeauth_BYTES: usize =
    crypto_onetimeauth_poly1305_BYTES;
pub const crypto_onetimeauth_KEYBYTES: usize =
    crypto_onetimeauth_poly1305_KEYBYTES;
pub const crypto_onetimeauth_PRIMITIVE: &'static str =  "poly1305";


extern {
    pub fn crypto_onetimeauth_bytes() -> size_t;
    pub fn crypto_onetimeauth_keybytes() -> size_t;
    pub fn crypto_onetimeauth_primitive() -> *const c_char;
}


#[test]
fn test_crypto_onetimeauth_bytes() {
    assert!(unsafe { crypto_onetimeauth_bytes() as usize } ==
            crypto_onetimeauth_BYTES)
}
#[test]
fn test_crypto_onetimeauth_keybytes() {
    assert!(unsafe { crypto_onetimeauth_keybytes() as usize } ==
            crypto_onetimeauth_KEYBYTES)
}
#[test]
fn test_crypto_onetimeauth_primitive() {
    unsafe {
        let s = crypto_onetimeauth_primitive();
        let s = std::ffi::CStr::from_ptr(s).to_bytes();
        assert!(s == crypto_onetimeauth_PRIMITIVE.as_bytes());
    }
}
