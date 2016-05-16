pub type c_char = i8;
pub type wchar_t = i32;
pub type greg_t = i32;

s! {
    pub struct _libc_fpreg {
        pub significand: [u16; 4],
        pub exponent: u16,
    }

    pub struct _libc_fpstate {
        pub cw: ::c_ulong,
        pub sw: ::c_ulong,
        pub tag: ::c_ulong,
        pub ipoff: ::c_ulong,
        pub cssel: ::c_ulong,
        pub dataoff: ::c_ulong,
        pub datasel: ::c_ulong,
        pub _st: [_libc_fpreg; 8],
        pub status: ::c_ulong,
    }

    pub struct mcontext_t {
        pub gregs: [greg_t; 19],
        pub fpregs: *mut _libc_fpstate,
        pub oldmask: ::c_ulong,
        pub cr2: ::c_ulong,
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
        __private: [u8; 112],
    }
}

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

pub const SYS_gettid: ::c_long = 224;
pub const SYS_perf_event_open: ::c_long = 336;

extern {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t,
                       func:  extern fn (),
                       argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t,
                       ucp: *const ucontext_t) -> ::c_int;
}
