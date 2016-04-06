// crypto_core_salsa2012.h

pub const crypto_core_salsa2012_OUTPUTBYTES: usize = 64;
pub const crypto_core_salsa2012_INPUTBYTES: usize = 16;
pub const crypto_core_salsa2012_KEYBYTES: usize = 32;
pub const crypto_core_salsa2012_CONSTBYTES: usize = 16;

extern {
    pub fn crypto_core_salsa2012_outputbytes() -> size_t;
    pub fn crypto_core_salsa2012_inputbytes() -> size_t;
    pub fn crypto_core_salsa2012_keybytes() -> size_t;
    pub fn crypto_core_salsa2012_constbytes() -> size_t;

    pub fn crypto_core_salsa2012(
        out: *mut [u8; crypto_core_salsa2012_OUTPUTBYTES],
        in_: *const [u8; crypto_core_salsa2012_INPUTBYTES],
        k: *const [u8; crypto_core_salsa2012_KEYBYTES],
        c: *const [u8; crypto_core_salsa2012_CONSTBYTES]) -> c_int;
}

#[test]
fn test_crypto_core_salsa2012_outputbytes() {
    assert!(unsafe {
        crypto_core_salsa2012_outputbytes() as usize
    } == crypto_core_salsa2012_OUTPUTBYTES)
}

#[test]
fn test_crypto_core_salsa2012_inputbytes() {
    assert!(unsafe {
        crypto_core_salsa2012_inputbytes() as usize
    } == crypto_core_salsa2012_INPUTBYTES)
}

#[test]
fn test_crypto_core_salsa2012_keybytes() {
    assert!(unsafe {
        crypto_core_salsa2012_keybytes() as usize
    } == crypto_core_salsa2012_KEYBYTES)
}

#[test]
fn test_crypto_core_salsa2012_constbytes() {
    assert!(unsafe {
        crypto_core_salsa2012_constbytes() as usize
    } == crypto_core_salsa2012_CONSTBYTES)
}
