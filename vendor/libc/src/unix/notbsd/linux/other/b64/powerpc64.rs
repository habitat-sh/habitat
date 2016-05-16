//! PowerPC64-specific definitions for 64-bit linux-like values

pub type c_char = u8;
pub type wchar_t = i32;
pub type nlink_t = u64;
pub type blksize_t = i64;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        __pad0: ::c_int,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_long; 3],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino64_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        __pad0: ::c_int,
        pub st_rdev: ::dev_t,
        pub st_size: ::off64_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __reserved: [::c_long; 3],
    }

    pub struct pthread_attr_t {
        __size: [u64; 7]
    }
}

pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;

pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_NOFOLLOW: ::c_int = 0x8000;
pub const O_DIRECT: ::c_int = 0x20000;

pub const MAP_LOCKED: ::c_int = 0x00080;
pub const MAP_NORESERVE: ::c_int = 0x00040;

pub const EDEADLOCK: ::c_int = 58;

pub const SO_PEERCRED: ::c_int = 21;
pub const SO_RCVLOWAT: ::c_int = 16;
pub const SO_SNDLOWAT: ::c_int = 17;
pub const SO_RCVTIMEO: ::c_int = 18;
pub const SO_SNDTIMEO: ::c_int = 19;

pub const FIOCLEX: ::c_ulong = 0x20006601;
pub const FIONBIO: ::c_ulong = 0x8004667e;

pub const SYS_gettid: ::c_long = 207;
pub const SYS_perf_event_open: ::c_long = 319;
