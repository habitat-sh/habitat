// crypto_box_curve25519xsalsa20poly1305.h

pub const crypto_box_curve25519xsalsa20poly1305_SEEDBYTES: usize = 32;
pub const crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES: usize = 32;
pub const crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES: usize = 32;
pub const crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES: usize = 32;
pub const crypto_box_curve25519xsalsa20poly1305_NONCEBYTES: usize = 24;
pub const crypto_box_curve25519xsalsa20poly1305_ZEROBYTES: usize = 32;
pub const crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES: usize = 16;
pub const crypto_box_curve25519xsalsa20poly1305_MACBYTES: usize =
    crypto_box_curve25519xsalsa20poly1305_ZEROBYTES -
    crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES;


extern {
    pub fn crypto_box_curve25519xsalsa20poly1305_keypair(
        pk: *mut [u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES],
        sk: *mut [u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES])
        -> c_int;
    pub fn crypto_box_curve25519xsalsa20poly1305(
        c: *mut u8,
        m: *const u8,
        mlen: c_ulonglong,
        n: *const [u8; crypto_box_curve25519xsalsa20poly1305_NONCEBYTES],
        pk: *const [u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES],
        sk: *const [u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES])
        -> c_int;
    pub fn crypto_box_curve25519xsalsa20poly1305_open(
        m: *mut u8,
        c: *const u8,
        clen: c_ulonglong,
        n: *const [u8; crypto_box_curve25519xsalsa20poly1305_NONCEBYTES],
        pk: *const [u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES],
        sk: *const [u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES])
        -> c_int;
    pub fn crypto_box_curve25519xsalsa20poly1305_beforenm(
        k: *mut [u8; crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES],
        pk: *const [u8; crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES],
        sk: *const [u8; crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES])
        -> c_int;
    pub fn crypto_box_curve25519xsalsa20poly1305_afternm(
        c: *mut u8,
        m: *const u8,
        mlen: c_ulonglong,
        n: *const [u8; crypto_box_curve25519xsalsa20poly1305_NONCEBYTES],
        k: *const [u8; crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES])
        -> c_int;
    pub fn crypto_box_curve25519xsalsa20poly1305_open_afternm(
        m: *mut u8,
        c: *const u8,
        clen: c_ulonglong,
        n: *const [u8; crypto_box_curve25519xsalsa20poly1305_NONCEBYTES],
        k: *const [u8; crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES])
        -> c_int;
    pub fn crypto_box_curve25519xsalsa20poly1305_seedbytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_publickeybytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_secretkeybytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_beforenmbytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_noncebytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_zerobytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_boxzerobytes() -> size_t;
    pub fn crypto_box_curve25519xsalsa20poly1305_macbytes() -> size_t;
}


#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_seedbytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_seedbytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_SEEDBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_publickeybytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_publickeybytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_PUBLICKEYBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_secretkeybytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_secretkeybytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_SECRETKEYBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_beforenmbytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_beforenmbytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_BEFORENMBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_noncebytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_noncebytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_NONCEBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_zerobytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_zerobytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_ZEROBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_boxzerobytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_boxzerobytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_BOXZEROBYTES)
}
#[test]
fn test_crypto_box_curve25519xsalsa20poly1305_macbytes() {
    assert!(unsafe {
        crypto_box_curve25519xsalsa20poly1305_macbytes() as usize
    } == crypto_box_curve25519xsalsa20poly1305_MACBYTES)
}
