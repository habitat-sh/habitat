// crypto_sign_ed25519.h

pub const crypto_sign_ed25519_BYTES: usize = 64;
pub const crypto_sign_ed25519_SEEDBYTES: usize = 32;
pub const crypto_sign_ed25519_PUBLICKEYBYTES: usize = 32;
pub const crypto_sign_ed25519_SECRETKEYBYTES: usize = 64;


extern {
    pub fn crypto_sign_ed25519_keypair(
        pk: *mut [u8; crypto_sign_ed25519_PUBLICKEYBYTES],
        sk: *mut [u8; crypto_sign_ed25519_SECRETKEYBYTES]) -> c_int;
    pub fn crypto_sign_ed25519_seed_keypair(
        pk: *mut [u8; crypto_sign_ed25519_PUBLICKEYBYTES],
        sk: *mut [u8; crypto_sign_ed25519_SECRETKEYBYTES],
        seed: *const [u8; crypto_sign_ed25519_SEEDBYTES]) -> c_int;
    pub fn crypto_sign_ed25519(
        sm: *mut u8,
        smlen: *mut c_ulonglong,
        m: *const u8,
        mlen: c_ulonglong,
        sk: *const [u8; crypto_sign_ed25519_SECRETKEYBYTES]) -> c_int;
    pub fn crypto_sign_ed25519_open(
        m: *mut u8,
        mlen: *mut c_ulonglong,
        sm: *const u8,
        smlen: c_ulonglong,
        pk: *const [u8; crypto_sign_ed25519_PUBLICKEYBYTES]) -> c_int;
    pub fn crypto_sign_ed25519_detached(
        sig: *mut [u8; crypto_sign_ed25519_BYTES],
        siglen: *mut c_ulonglong,
        m: *const u8,
        mlen: c_ulonglong,
        sk: *const [u8; crypto_sign_ed25519_SECRETKEYBYTES]) -> c_int;
    pub fn crypto_sign_ed25519_verify_detached(
        sig: *const [u8; crypto_sign_ed25519_BYTES],
        m: *const u8,
        mlen: c_ulonglong,
        pk: *const [u8; crypto_sign_ed25519_PUBLICKEYBYTES]) -> c_int;
    pub fn crypto_sign_ed25519_bytes() -> size_t;
    pub fn crypto_sign_ed25519_seedbytes() -> size_t;
    pub fn crypto_sign_ed25519_publickeybytes() -> size_t;
    pub fn crypto_sign_ed25519_secretkeybytes() -> size_t;
}


#[test]
fn test_crypto_sign_ed25519_bytes() {
    assert!(unsafe {
        crypto_sign_ed25519_bytes() as usize
    } == crypto_sign_ed25519_BYTES)
}
#[test]
fn test_crypto_sign_ed25519_seedbytes() {
    assert!(unsafe {
        crypto_sign_ed25519_seedbytes() as usize
    } == crypto_sign_ed25519_SEEDBYTES)
}
#[test]
fn test_crypto_sign_ed25519_publickeybytes() {
    assert!(unsafe {
        crypto_sign_ed25519_publickeybytes() as usize
    } == crypto_sign_ed25519_PUBLICKEYBYTES)
}
#[test]
fn test_crypto_sign_ed25519_secretkeybytes() {
    assert!(unsafe {
        crypto_sign_ed25519_secretkeybytes() as usize
    } == crypto_sign_ed25519_SECRETKEYBYTES)
}
