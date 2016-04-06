// crypto_stream_salsa2012.h

pub const crypto_stream_salsa2012_KEYBYTES: usize = 32;
pub const crypto_stream_salsa2012_NONCEBYTES: usize = 8;


extern {
    pub fn crypto_stream_salsa2012(
        c: *mut u8,
        clen: c_ulonglong,
        n: *const [u8; crypto_stream_salsa2012_NONCEBYTES],
        k: *const [u8; crypto_stream_salsa2012_KEYBYTES]) -> c_int;
    pub fn crypto_stream_salsa2012_xor(
        c: *mut u8,
        m: *const u8,
        mlen: c_ulonglong,
        n: *const [u8; crypto_stream_salsa2012_NONCEBYTES],
        k: *const [u8; crypto_stream_salsa2012_KEYBYTES]) -> c_int;
    pub fn crypto_stream_salsa2012_keybytes() -> size_t;
    pub fn crypto_stream_salsa2012_noncebytes() -> size_t;
}


#[test]
fn test_crypto_stream_salsa2012_keybytes() {
    assert!(unsafe { crypto_stream_salsa2012_keybytes() as usize } ==
            crypto_stream_salsa2012_KEYBYTES)
}
#[test]
fn test_crypto_stream_salsa2012_noncebytes() {
    assert!(unsafe { crypto_stream_salsa2012_noncebytes() as usize } ==
            crypto_stream_salsa2012_NONCEBYTES)
}
