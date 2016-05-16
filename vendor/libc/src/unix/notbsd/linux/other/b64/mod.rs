//! 64-bit specific definitions for linux-like values

pub type c_long = i64;
pub type c_ulong = u64;
pub type clock_t = i64;
pub type time_t = i64;
pub type suseconds_t = i64;
pub type ino_t = u64;
pub type off_t = i64;
pub type blkcnt_t = i64;
pub type __fsword_t = ::c_long;

s! {
    pub struct sigset_t {
        __val: [::c_ulong; 16],
    }
}

pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(any(target_arch = "powerpc64"))] {
        mod powerpc64;
        pub use self::powerpc64::*;
    } else if #[cfg(any(target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // Unknown target_arch
    }
}
