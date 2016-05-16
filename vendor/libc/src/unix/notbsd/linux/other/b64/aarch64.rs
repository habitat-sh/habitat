//! AArch64-specific definitions for 64-bit linux-like values

pub type c_char = u8;
pub type wchar_t = u32;
pub type nlink_t = u32;
pub type blksize_t = i32;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad1: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        __pad2: ::c_int,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_int; 2],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad1: ::dev_t,
        pub st_size: ::off64_t,
        pub st_blksize: ::blksize_t,
        __pad2: ::c_int,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_int; 2],
    }

    pub struct pthread_attr_t {
        __size: [u64; 8]
    }
}

pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 48;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 8;

pub const O_DIRECT: ::c_int = 0x10000;
pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_NOFOLLOW: ::c_int = 0x8000;

pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;

pub const EDEADLOCK: ::c_int = 35;

pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

pub const SYS_gettid: ::c_long = 178;
pub const SYS_perf_event_open: ::c_long = 241;
