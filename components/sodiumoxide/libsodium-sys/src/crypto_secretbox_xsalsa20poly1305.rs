// crypto_secretbox_xsalsa20poly1305.h

pub const crypto_secretbox_xsalsa20poly1305_KEYBYTES: usize = 32;
pub const crypto_secretbox_xsalsa20poly1305_NONCEBYTES: usize = 24;
pub const crypto_secretbox_xsalsa20poly1305_ZEROBYTES: usize = 32;
pub const crypto_secretbox_xsalsa20poly1305_BOXZEROBYTES: usize = 16;
pub const crypto_secretbox_xsalsa20poly1305_MACBYTES: usize =
    crypto_secretbox_xsalsa20poly1305_ZEROBYTES -
    crypto_secretbox_xsalsa20poly1305_BOXZEROBYTES;


extern {
    pub fn crypto_secretbox_xsalsa20poly1305(
        c: *mut u8,
        m: *const u8,
        mlen: c_ulonglong,
        n: *const [u8; crypto_secretbox_xsalsa20poly1305_NONCEBYTES],
        k: *const [u8; crypto_secretbox_xsalsa20poly1305_KEYBYTES]) -> c_int;
    pub fn crypto_secretbox_xsalsa20poly1305_open(
        m: *mut u8,
        c: *const u8,
        clen: c_ulonglong,
        n: *const [u8; crypto_secretbox_xsalsa20poly1305_NONCEBYTES],
        k: *const [u8; crypto_secretbox_xsalsa20poly1305_KEYBYTES]) -> c_int;
    pub fn crypto_secretbox_xsalsa20poly1305_keybytes() -> size_t;
    pub fn crypto_secretbox_xsalsa20poly1305_noncebytes() -> size_t;
    pub fn crypto_secretbox_xsalsa20poly1305_zerobytes() -> size_t;
    pub fn crypto_secretbox_xsalsa20poly1305_boxzerobytes() -> size_t;
    pub fn crypto_secretbox_xsalsa20poly1305_macbytes() -> size_t;
}


#[test]
fn test_crypto_secretbox_xsalsa20poly1305_keybytes() {
    assert!(unsafe {
        crypto_secretbox_xsalsa20poly1305_keybytes() as usize
    } == crypto_secretbox_xsalsa20poly1305_KEYBYTES)
}
#[test]
fn test_crypto_secretbox_xsalsa20poly1305_noncebytes() {
    assert!(unsafe {
        crypto_secretbox_xsalsa20poly1305_noncebytes() as usize
    } == crypto_secretbox_xsalsa20poly1305_NONCEBYTES)
}
#[test]
fn test_crypto_secretbox_xsalsa20poly1305_zerobytes() {
    assert!(unsafe {
        crypto_secretbox_xsalsa20poly1305_zerobytes() as usize
    } == crypto_secretbox_xsalsa20poly1305_ZEROBYTES)
}
#[test]
fn test_crypto_secretbox_xsalsa20poly1305_boxzerobytes() {
    assert!(unsafe {
        crypto_secretbox_xsalsa20poly1305_boxzerobytes() as usize
    } == crypto_secretbox_xsalsa20poly1305_BOXZEROBYTES)
}
#[test]
fn test_crypto_secretbox_xsalsa20poly1305_macbytes() {
    assert!(unsafe {
        crypto_secretbox_xsalsa20poly1305_macbytes() as usize
    } == crypto_secretbox_xsalsa20poly1305_MACBYTES)
}
