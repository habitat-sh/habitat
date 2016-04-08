// utils.h

extern {
    pub fn sodium_memzero(pnt: *mut u8, len: size_t);
    pub fn sodium_memcmp(b1_: *const u8, b2_: *const u8, len: size_t) -> c_int;
    pub fn sodium_increment(n: *mut u8, len: size_t);

    pub fn sodium_mlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn sodium_munlock(addr: *const c_void, len: size_t) -> c_int;

    pub fn sodium_malloc(len: size_t) -> *mut c_void;
    pub fn sodium_allocarray(count: size_t, size: size_t) -> *mut c_void;
    pub fn sodium_free(ptr: *mut c_void);

    pub fn sodium_mprotect_noaccess(ptr: *const c_void) -> c_int;
    pub fn sodium_mprotect_readonly(ptr: *const c_void) -> c_int;
    pub fn sodium_mprotect_readwrite(ptr: *const c_void) -> c_int;
}
