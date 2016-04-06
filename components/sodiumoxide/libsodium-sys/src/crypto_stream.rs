// crypto_stream.h

pub const crypto_stream_KEYBYTES: usize = crypto_stream_xsalsa20_KEYBYTES;
pub const crypto_stream_NONCEBYTES: usize =
    crypto_stream_xsalsa20_NONCEBYTES;
pub const crypto_stream_PRIMITIVE: &'static str = "xsalsa20";


extern {
    pub fn crypto_stream_keybytes() -> size_t;
    pub fn crypto_stream_noncebytes() -> size_t;
    pub fn crypto_stream_primitive() -> *const c_char;
}


#[test]
fn test_crypto_stream_keybytes() {
    assert!(unsafe { crypto_stream_keybytes() as usize } ==
            crypto_stream_KEYBYTES)
}
#[test]
fn test_crypto_stream_noncebytes() {
    assert!(unsafe { crypto_stream_noncebytes() as usize } ==
            crypto_stream_NONCEBYTES)
}
#[test]
fn test_crypto_stream_primitive() {
    unsafe {
        let s = crypto_stream_primitive();
        let s = std::ffi::CStr::from_ptr(s).to_bytes();
        assert!(s == crypto_stream_PRIMITIVE.as_bytes());
    }
}
