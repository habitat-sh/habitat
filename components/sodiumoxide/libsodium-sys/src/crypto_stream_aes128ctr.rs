// crypto_stream_aes128ctr.h
pub const crypto_stream_aes128ctr_KEYBYTES: usize = 16;
pub const crypto_stream_aes128ctr_NONCEBYTES: usize = 16;
pub const crypto_stream_aes128ctr_BEFORENMBYTES: usize = 1408;


extern {
    pub fn crypto_stream_aes128ctr(
        c: *mut u8,
        clen: c_ulonglong,
        n: *const [u8; crypto_stream_aes128ctr_NONCEBYTES],
        k: *const [u8; crypto_stream_aes128ctr_KEYBYTES]) -> c_int;
    pub fn crypto_stream_aes128ctr_xor(
        c: *mut u8,
        m: *const u8,
        mlen: c_ulonglong,
        n: *const [u8; crypto_stream_aes128ctr_NONCEBYTES],
        k: *const [u8; crypto_stream_aes128ctr_KEYBYTES]) -> c_int;
    pub fn crypto_stream_aes128ctr_keybytes() -> size_t;
    pub fn crypto_stream_aes128ctr_noncebytes() -> size_t;
    pub fn crypto_stream_aes128ctr_beforenmbytes() -> size_t;
}


#[test]
fn test_crypto_stream_aes128ctr_keybytes() {
    assert!(unsafe { crypto_stream_aes128ctr_keybytes() as usize } ==
            crypto_stream_aes128ctr_KEYBYTES)
}
#[test]
fn test_crypto_stream_aes128ctr_noncebytes() {
    assert!(unsafe { crypto_stream_aes128ctr_noncebytes() as usize } ==
            crypto_stream_aes128ctr_NONCEBYTES)
}
#[test]
fn test_crypto_stream_aes128ctr_beforenmbytes() {
    assert!(unsafe { crypto_stream_aes128ctr_beforenmbytes() as usize } ==
            crypto_stream_aes128ctr_BEFORENMBYTES)
}
