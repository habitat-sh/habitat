// The following definitions are correct for aarch64 and may be wrong for x86_64

pub type c_long = i64;
pub type c_ulong = u64;
pub type mode_t = u32;
pub type off64_t = i64;

s! {
    pub struct sigaction {
        pub sa_flags: ::c_uint,
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        _restorer: *mut ::c_void,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::c_uint,
        pub st_nlink: ::c_uint,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad1: ::c_ulong,
        pub st_size: ::off64_t,
        pub st_blksize: ::c_int,
        __pad2: ::c_int,
        pub st_blocks: ::c_long,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_ulong,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_ulong,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_ulong,
        __unused4: ::c_uint,
        __unused5: ::c_uint,
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::c_uint,
        pub st_nlink: ::c_uint,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad1: ::c_ulong,
        pub st_size: ::off64_t,
        pub st_blksize: ::c_int,
        __pad2: ::c_int,
        pub st_blocks: ::c_long,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_ulong,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_ulong,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_ulong,
        __unused4: ::c_uint,
        __unused5: ::c_uint,
    }

    pub struct pthread_attr_t {
        pub flags: ::uint32_t,
        pub stack_base: *mut ::c_void,
        pub stack_size: ::size_t,
        pub guard_size: ::size_t,
        pub sched_policy: ::int32_t,
        pub sched_priority: ::int32_t,
        __reserved: [::c_char; 16],
    }

    pub struct pthread_mutex_t {
        value: ::c_int,
        __reserved: [::c_char; 36],
    }

    pub struct pthread_cond_t {
        value: ::c_int,
        __reserved: [::c_char; 44],
    }

    pub struct pthread_rwlock_t {
        numLocks: ::c_int,
        writerThreadId: ::c_int,
        pendingReaders: ::c_int,
        pendingWriters: ::c_int,
        attr: i32,
        __reserved: [::c_char; 36],
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct statfs {
        pub f_type: ::uint64_t,
        pub f_bsize: ::uint64_t,
        pub f_blocks: ::uint64_t,
        pub f_bfree: ::uint64_t,
        pub f_bavail: ::uint64_t,
        pub f_files: ::uint64_t,
        pub f_ffree: ::uint64_t,
        pub f_fsid: ::__fsid_t,
        pub f_namelen: ::uint64_t,
        pub f_frsize: ::uint64_t,
        pub f_flags: ::uint64_t,
        pub f_spare: [::uint64_t; 4],
    }
}

pub const SYS_gettid: ::c_long = 178;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    value: 0,
    __reserved: [0; 36],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    value: 0,
    __reserved: [0; 44],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    numLocks: 0,
    writerThreadId: 0,
    pendingReaders: 0,
    pendingWriters: 0,
    attr: 0,
    __reserved: [0; 36],
};
pub const PTHREAD_STACK_MIN: ::size_t = 4096 * 4;

extern {
    pub fn timegm(tm: *const ::tm) -> ::time64_t;
}
