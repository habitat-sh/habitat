// crypto_verify_64.h

pub const crypto_verify_64_BYTES: usize = 64;

extern {
    pub fn crypto_verify_64_bytes() -> size_t;
    pub fn crypto_verify_64(
        x: *const [u8; crypto_verify_64_BYTES],
        y: *const [u8; crypto_verify_64_BYTES]) -> c_int;
}


#[test]
fn test_crypto_verify_64_bytes() {
   assert_eq!(unsafe { crypto_verify_64_bytes() as usize },
                       crypto_verify_64_BYTES);
}
