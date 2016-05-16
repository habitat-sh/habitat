//! x86_64-specific definitions for 64-bit linux-like values

pub type c_char = i8;
pub type wchar_t = i32;
pub type nlink_t = u64;
pub type blksize_t = i64;
pub type greg_t = i64;

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
        pub st_size: ::off_t,
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

    pub struct _libc_fpxreg {
        pub significand: [u16; 4],
        pub exponent: u16,
        __private: [u16; 3],
    }

    pub struct _libc_xmmreg {
        pub element: [u32; 4],
    }

    pub struct _libc_fpstate {
        pub cwd: u16,
        pub swd: u16,
        pub ftw: u16,
        pub fop: u16,
        pub rip: u64,
        pub rdp: u64,
        pub mxcsr: u32,
        pub mxcr_mask: u32,
        pub _st: [_libc_fpxreg; 8],
        pub _xmm: [_libc_xmmreg; 16],
        __private: [u64; 12],
    }

    pub struct mcontext_t {
        pub gregs: [greg_t; 23],
        pub fpregs: *mut _libc_fpstate,
        __private: [u64; 8],
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
        __private: [u8; 512],
    }
}

pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;

pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;
pub const MAP_32BIT: ::c_int = 0x0040;

pub const EDEADLOCK: ::c_int = 35;

pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

pub const PTRACE_GETFPREGS: ::c_uint = 14;
pub const PTRACE_SETFPREGS: ::c_uint = 15;
pub const PTRACE_GETFPXREGS: ::c_uint = 18;
pub const PTRACE_SETFPXREGS: ::c_uint = 19;
pub const PTRACE_GETREGS: ::c_uint = 12;
pub const PTRACE_SETREGS: ::c_uint = 13;

pub const SYS_gettid: ::c_long = 186;
pub const SYS_perf_event_open: ::c_long = 298;

extern {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t,
                       func:  extern fn (),
                       argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t,
                       ucp: *const ucontext_t) -> ::c_int;
}
