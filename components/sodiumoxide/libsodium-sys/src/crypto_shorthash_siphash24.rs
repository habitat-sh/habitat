// crypto_shorthash_siphash24.h

pub const crypto_shorthash_siphash24_BYTES: usize = 8;
pub const crypto_shorthash_siphash24_KEYBYTES: usize = 16;


extern {
    pub fn crypto_shorthash_siphash24(
        h: *mut [u8; crypto_shorthash_siphash24_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        k: *const [u8; crypto_shorthash_siphash24_KEYBYTES]) -> c_int;
    pub fn crypto_shorthash_siphash24_bytes() -> size_t;
    pub fn crypto_shorthash_siphash24_keybytes() -> size_t;
}


#[test]
fn test_crypto_shorthash_siphash24_bytes() {
    assert!(unsafe {
        crypto_shorthash_siphash24_bytes() as usize
    } == crypto_shorthash_siphash24_BYTES)
}
#[test]
fn test_crypto_shorthash_siphash24_keybytes() {
    assert!(unsafe {
        crypto_shorthash_siphash24_keybytes() as usize
    } == crypto_shorthash_siphash24_KEYBYTES)
}
