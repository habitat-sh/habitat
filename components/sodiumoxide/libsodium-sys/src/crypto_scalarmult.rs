// crypto_scalarmult.h

pub const crypto_scalarmult_BYTES: usize = crypto_scalarmult_curve25519_BYTES;
pub const crypto_scalarmult_SCALARBYTES: usize = crypto_scalarmult_curve25519_SCALARBYTES;
pub const crypto_scalarmult_PRIMITIVE: &'static str = "curve25519";

extern {
    pub fn crypto_scalarmult_bytes() -> size_t;
    pub fn crypto_scalarmult_scalarbytes() -> size_t;
    pub fn crypto_scalarmult_primitive() -> *const c_char;
    pub fn crypto_scalarmult_base(
        q: *mut [u8; crypto_scalarmult_BYTES],
        n: *const [u8; crypto_scalarmult_SCALARBYTES]) -> c_int;
    pub fn crypto_scalarmult(
        q: *mut [u8; crypto_scalarmult_BYTES],
        n: *const [u8; crypto_scalarmult_SCALARBYTES],
        p: *const [u8; crypto_scalarmult_BYTES]) -> c_int;
}

#[test]
fn test_crypto_scalarmult_bytes() {
    assert_eq!(unsafe { crypto_scalarmult_bytes() as usize },
               crypto_scalarmult_BYTES);
}

#[test]
fn test_crypto_scalarmult_scalarbytes() {
    assert_eq!(unsafe { crypto_scalarmult_scalarbytes() as usize },
               crypto_scalarmult_SCALARBYTES);
}

#[test]
fn test_crypto_scalarmult_primitive() {
    unsafe {
        let s = crypto_scalarmult_primitive();
        let s = std::ffi::CStr::from_ptr(s).to_bytes();
        assert_eq!(s, crypto_scalarmult_PRIMITIVE.as_bytes());
    }
}
