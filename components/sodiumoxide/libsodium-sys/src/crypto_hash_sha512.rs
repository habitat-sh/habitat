// crypto_hash_sha512.h

#[repr(C)]
#[derive(Copy)]
pub struct crypto_hash_sha512_state {
    state: [u64; 8],
    count: [u64; 2],
    buf: [u8; 128],
}
impl Clone for crypto_hash_sha512_state { fn clone(&self) -> crypto_hash_sha512_state { *self } }
pub const crypto_hash_sha512_BYTES: usize = 64;


extern {
    pub fn crypto_hash_sha512_bytes() -> size_t;    
    pub fn crypto_hash_sha512(h: *mut [u8; crypto_hash_sha512_BYTES],
                              m: *const u8,
                              mlen: c_ulonglong) -> c_int;
    pub fn crypto_hash_sha512_init(state: *mut crypto_hash_sha512_state) -> c_int;
    pub fn crypto_hash_sha512_update(state: *mut crypto_hash_sha512_state,
                                     m: *const u8,
                                     mlen: c_ulonglong) -> c_int;
    pub fn crypto_hash_sha512_final(state: *mut crypto_hash_sha512_state,
                                    h: *mut [u8; crypto_hash_sha512_BYTES]) -> c_int;
}


#[test]
fn test_crypto_hash_sha512_bytes() {
    assert!(unsafe { crypto_hash_sha512_bytes() as usize } ==
            crypto_hash_sha512_BYTES)
}
