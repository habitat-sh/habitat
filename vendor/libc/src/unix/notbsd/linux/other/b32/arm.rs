pub type c_char = u8;
pub type wchar_t = u32;

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

pub const SYS_gettid: ::c_long = 224;
pub const SYS_perf_event_open: ::c_long = 364;
