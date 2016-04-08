// crypto_core_hsalsa20.h

pub const crypto_core_hsalsa20_OUTPUTBYTES: usize = 32;
pub const crypto_core_hsalsa20_INPUTBYTES: usize = 16;
pub const crypto_core_hsalsa20_KEYBYTES: usize = 32;
pub const crypto_core_hsalsa20_CONSTBYTES: usize = 16;


extern {
    pub fn crypto_core_hsalsa20_outputbytes() -> size_t;
    pub fn crypto_core_hsalsa20_inputbytes() -> size_t;
    pub fn crypto_core_hsalsa20_keybytes() -> size_t;
    pub fn crypto_core_hsalsa20_constbytes() -> size_t;

    pub fn crypto_core_hsalsa20(
        out: *mut [u8; crypto_core_hsalsa20_OUTPUTBYTES],
        in_: *const [u8; crypto_core_hsalsa20_INPUTBYTES],
        k: *const [u8; crypto_core_hsalsa20_KEYBYTES],
        c: *const [u8; crypto_core_hsalsa20_CONSTBYTES]) -> c_int;
}


#[test]
fn test_crypto_core_hsalsa20_outputbytes() {
    assert!(unsafe {
        crypto_core_hsalsa20_outputbytes() as usize
    } == crypto_core_hsalsa20_OUTPUTBYTES)
}
#[test]
fn test_crypto_core_hsalsa20_inputbytes() {
    assert!(unsafe {
        crypto_core_hsalsa20_inputbytes() as usize
    } == crypto_core_hsalsa20_INPUTBYTES)
}
#[test]
fn test_crypto_core_hsalsa20_keybytes() {
    assert!(unsafe {
        crypto_core_hsalsa20_keybytes() as usize
    } == crypto_core_hsalsa20_KEYBYTES)
}
#[test]
fn test_crypto_core_hsalsa20_constbytes() {
    assert!(unsafe {
        crypto_core_hsalsa20_constbytes() as usize
    } == crypto_core_hsalsa20_CONSTBYTES)
}
