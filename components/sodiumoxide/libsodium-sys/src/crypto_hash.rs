// crypto_hash.h

pub const crypto_hash_BYTES: usize = crypto_hash_sha512_BYTES;
pub const crypto_hash_PRIMITIVE: &'static str = "sha512";


extern {
    pub fn crypto_hash_bytes() -> size_t;
    pub fn crypto_hash(h: *mut [u8; crypto_hash_BYTES],
                       m: *const u8,
                       mlen: c_ulonglong) -> c_int;
    pub fn crypto_hash_primitive() -> *const c_char;
}


#[test]
fn test_crypto_hash_bytes() {
    assert!(unsafe { crypto_hash_bytes() as usize } == crypto_hash_BYTES)
}
#[test]
fn test_crypto_hash_primitive() {
    unsafe {
        let s = crypto_hash_primitive();
        let s = std::ffi::CStr::from_ptr(s).to_bytes();
        assert!(s == crypto_hash_PRIMITIVE.as_bytes());
    }
}
