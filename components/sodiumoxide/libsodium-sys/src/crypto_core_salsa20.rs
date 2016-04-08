// crypto_core_salsa20.h

pub const crypto_core_salsa20_OUTPUTBYTES: usize = 64;
pub const crypto_core_salsa20_INPUTBYTES: usize = 16;
pub const crypto_core_salsa20_KEYBYTES: usize = 32;
pub const crypto_core_salsa20_CONSTBYTES: usize = 16;

extern {
    pub fn crypto_core_salsa20_outputbytes() -> size_t;
    pub fn crypto_core_salsa20_inputbytes() -> size_t;
    pub fn crypto_core_salsa20_keybytes() -> size_t;
    pub fn crypto_core_salsa20_constbytes() -> size_t;

    pub fn crypto_core_salsa20(
        out: *mut [u8; crypto_core_salsa20_OUTPUTBYTES],
        in_: *const [u8; crypto_core_salsa20_INPUTBYTES],
        k: *const [u8; crypto_core_salsa20_KEYBYTES],
        c: *const [u8; crypto_core_salsa20_CONSTBYTES]) -> c_int;
}

#[test]
fn test_crypto_core_salsa20_outputbytes() {
    assert!(unsafe {
        crypto_core_salsa20_outputbytes() as usize
    } == crypto_core_salsa20_OUTPUTBYTES)
}

#[test]
fn test_crypto_core_salsa20_inputbytes() {
    assert!(unsafe {
        crypto_core_salsa20_inputbytes() as usize
    } == crypto_core_salsa20_INPUTBYTES)
}

#[test]
fn test_crypto_core_salsa20_keybytes() {
    assert!(unsafe {
        crypto_core_salsa20_keybytes() as usize
    } == crypto_core_salsa20_KEYBYTES)
}

#[test]
fn test_crypto_core_salsa20_constbytes() {
    assert!(unsafe {
        crypto_core_salsa20_constbytes() as usize
    } == crypto_core_salsa20_CONSTBYTES)
}
