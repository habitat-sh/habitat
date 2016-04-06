// crypto_verify_16.h

pub const crypto_verify_16_BYTES: usize = 16;

extern {
    pub fn crypto_verify_16_bytes() -> size_t;
    pub fn crypto_verify_16(
        x: *const [u8; crypto_verify_16_BYTES],
        y: *const [u8; crypto_verify_16_BYTES])
        -> c_int;
}

#[test]
fn test_crypto_verify_16_bytes() {
   assert_eq!(unsafe { crypto_verify_16_bytes() as usize },
                       crypto_verify_16_BYTES);
}
