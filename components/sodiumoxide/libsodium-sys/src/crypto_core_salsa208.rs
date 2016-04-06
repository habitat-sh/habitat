// crypto_core_salsa208.h

pub const crypto_core_salsa208_OUTPUTBYTES: usize = 64;
pub const crypto_core_salsa208_INPUTBYTES: usize = 16;
pub const crypto_core_salsa208_KEYBYTES: usize = 32;
pub const crypto_core_salsa208_CONSTBYTES: usize = 16;

extern {
    pub fn crypto_core_salsa208_outputbytes() -> size_t;
    pub fn crypto_core_salsa208_inputbytes() -> size_t;
    pub fn crypto_core_salsa208_keybytes() -> size_t;
    pub fn crypto_core_salsa208_constbytes() -> size_t;

    pub fn crypto_core_salsa208(
        out: *mut [u8; crypto_core_salsa208_OUTPUTBYTES],
        in_: *const [u8; crypto_core_salsa208_INPUTBYTES],
        k: *const [u8; crypto_core_salsa208_KEYBYTES],
        c: *const [u8; crypto_core_salsa208_CONSTBYTES]) -> c_int;
}

#[test]
fn test_crypto_core_salsa208_outputbytes() {
    assert!(unsafe {
        crypto_core_salsa208_outputbytes() as usize
    } == crypto_core_salsa208_OUTPUTBYTES)
}

#[test]
fn test_crypto_core_salsa208_inputbytes() {
    assert!(unsafe {
        crypto_core_salsa208_inputbytes() as usize
    } == crypto_core_salsa208_INPUTBYTES)
}

#[test]
fn test_crypto_core_salsa208_keybytes() {
    assert!(unsafe {
        crypto_core_salsa208_keybytes() as usize
    } == crypto_core_salsa208_KEYBYTES)
}

#[test]
fn test_crypto_core_salsa208_constbytes() {
    assert!(unsafe {
        crypto_core_salsa208_constbytes() as usize
    } == crypto_core_salsa208_CONSTBYTES)
}
