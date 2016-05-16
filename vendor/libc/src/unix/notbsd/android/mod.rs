//! Android-specific definitions for linux-like values

pub type c_char = u8;
pub type clock_t = ::c_long;
pub type time_t = ::c_long;
pub type suseconds_t = ::c_long;
pub type wchar_t = u32;
pub type off_t = ::c_long;
pub type blkcnt_t = ::c_ulong;
pub type blksize_t = ::c_ulong;
pub type nlink_t = u32;
pub type useconds_t = u32;
pub type socklen_t = i32;
pub type pthread_t = ::c_long;
pub type pthread_mutexattr_t = ::c_long;
pub type sigset_t = ::c_ulong;
pub type time64_t = i64; // N/A on android
pub type fsfilcnt_t = ::c_ulong;
pub type fsblkcnt_t = ::c_ulong;
pub type nfds_t = ::c_uint;
pub type rlim_t = ::c_ulong;
pub type dev_t = ::c_ulong;
pub type ino_t = ::c_ulong;

s! {
    pub struct dirent {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct dirent64 {
        pub d_ino: u64,
        pub d_off: i64,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct rlimit64 {
        pub rlim_cur: u64,
        pub rlim_max: u64,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
    }

    pub struct __fsid_t {
        __val: [::c_int; 2],
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::c_int,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::size_t,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t,
        pub msg_flags: ::c_int,
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }
}

pub const BUFSIZ: ::c_uint = 1024;
pub const FILENAME_MAX: ::c_uint = 1024;
pub const FOPEN_MAX: ::c_uint = 20;
pub const L_tmpnam: ::c_uint = 1024;
pub const TMP_MAX: ::c_uint = 308915776;
pub const _PC_NAME_MAX: ::c_int = 4;

pub const FIONBIO: ::c_int = 0x5421;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_BC_BASE_MAX: ::c_int = 1;
pub const _SC_BC_DIM_MAX: ::c_int = 2;
pub const _SC_BC_SCALE_MAX: ::c_int = 3;
pub const _SC_BC_STRING_MAX: ::c_int = 4;
pub const _SC_CHILD_MAX: ::c_int = 5;
pub const _SC_CLK_TCK: ::c_int = 6;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 7;
pub const _SC_EXPR_NEST_MAX: ::c_int = 8;
pub const _SC_LINE_MAX: ::c_int = 9;
pub const _SC_NGROUPS_MAX: ::c_int = 10;
pub const _SC_OPEN_MAX: ::c_int = 11;
pub const _SC_2_C_BIND: ::c_int = 13;
pub const _SC_2_C_DEV: ::c_int = 14;
pub const _SC_2_C_VERSION: ::c_int = 15;
pub const _SC_2_CHAR_TERM: ::c_int = 16;
pub const _SC_2_FORT_DEV: ::c_int = 17;
pub const _SC_2_FORT_RUN: ::c_int = 18;
pub const _SC_2_LOCALEDEF: ::c_int = 19;
pub const _SC_2_SW_DEV: ::c_int = 20;
pub const _SC_2_UPE: ::c_int = 21;
pub const _SC_2_VERSION: ::c_int = 22;
pub const _SC_JOB_CONTROL: ::c_int = 23;
pub const _SC_SAVED_IDS: ::c_int = 24;
pub const _SC_VERSION: ::c_int = 25;
pub const _SC_RE_DUP_MAX: ::c_int = 26;
pub const _SC_STREAM_MAX: ::c_int = 27;
pub const _SC_TZNAME_MAX: ::c_int = 28;
pub const _SC_XOPEN_CRYPT: ::c_int = 29;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 30;
pub const _SC_XOPEN_SHM: ::c_int = 31;
pub const _SC_XOPEN_VERSION: ::c_int = 32;
pub const _SC_XOPEN_XCU_VERSION: ::c_int = 33;
pub const _SC_XOPEN_REALTIME: ::c_int = 34;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 35;
pub const _SC_XOPEN_LEGACY: ::c_int = 36;
pub const _SC_ATEXIT_MAX: ::c_int = 37;
pub const _SC_IOV_MAX: ::c_int = 38;
pub const _SC_PAGESIZE: ::c_int = 39;
pub const _SC_PAGE_SIZE: ::c_int = 40;
pub const _SC_XOPEN_UNIX: ::c_int = 41;
pub const _SC_MQ_PRIO_MAX: ::c_int = 51;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 71;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 72;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 73;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 74;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 75;
pub const _SC_THREAD_STACK_MIN: ::c_int = 76;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 77;
pub const _SC_TTY_NAME_MAX: ::c_int = 78;
pub const _SC_THREADS: ::c_int = 79;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 80;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 81;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 82;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 83;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 84;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 85;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 97;

pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 1;

pub const FIOCLEX: ::c_int = 0x5451;

pub const SA_ONSTACK: ::c_ulong = 0x08000000;
pub const SA_SIGINFO: ::c_ulong = 0x00000004;
pub const SA_NOCLDWAIT: ::c_ulong = 0x00000002;

pub const SIGCHLD: ::c_int = 17;
pub const SIGBUS: ::c_int = 7;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGURG: ::c_int = 23;
pub const SIGIO: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGUNUSED: ::c_int = 31;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIG_SETMASK: ::c_int = 2;
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;

pub const RUSAGE_CHILDREN: ::c_int = -1;

pub const LC_PAPER: ::c_int = 7;
pub const LC_NAME: ::c_int = 8;
pub const LC_ADDRESS: ::c_int = 9;
pub const LC_TELEPHONE: ::c_int = 10;
pub const LC_MEASUREMENT: ::c_int = 11;
pub const LC_IDENTIFICATION: ::c_int = 12;
pub const LC_PAPER_MASK: ::c_int = (1 << LC_PAPER);
pub const LC_NAME_MASK: ::c_int = (1 << LC_NAME);
pub const LC_ADDRESS_MASK: ::c_int = (1 << LC_ADDRESS);
pub const LC_TELEPHONE_MASK: ::c_int = (1 << LC_TELEPHONE);
pub const LC_MEASUREMENT_MASK: ::c_int = (1 << LC_MEASUREMENT);
pub const LC_IDENTIFICATION_MASK: ::c_int = (1 << LC_IDENTIFICATION);
pub const LC_ALL_MASK: ::c_int = ::LC_CTYPE_MASK
                               | ::LC_NUMERIC_MASK
                               | ::LC_TIME_MASK
                               | ::LC_COLLATE_MASK
                               | ::LC_MONETARY_MASK
                               | ::LC_MESSAGES_MASK
                               | LC_PAPER_MASK
                               | LC_NAME_MASK
                               | LC_ADDRESS_MASK
                               | LC_TELEPHONE_MASK
                               | LC_MEASUREMENT_MASK
                               | LC_IDENTIFICATION_MASK;

pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_ANONYMOUS: ::c_int = 0x0020;
pub const MAP_GROWSDOWN: ::c_int = 0x0100;
pub const MAP_DENYWRITE: ::c_int = 0x0800;
pub const MAP_EXECUTABLE: ::c_int = 0x01000;
pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;
pub const MAP_POPULATE: ::c_int = 0x08000;
pub const MAP_NONBLOCK: ::c_int = 0x010000;
pub const MAP_STACK: ::c_int = 0x020000;

pub const EDEADLK: ::c_int = 35;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENOLCK: ::c_int = 37;
pub const ENOSYS: ::c_int = 38;
pub const ENOTEMPTY: ::c_int = 39;
pub const ELOOP: ::c_int = 40;
pub const ENOMSG: ::c_int = 42;
pub const EIDRM: ::c_int = 43;
pub const ECHRNG: ::c_int = 44;
pub const EL2NSYNC: ::c_int = 45;
pub const EL3HLT: ::c_int = 46;
pub const EL3RST: ::c_int = 47;
pub const ELNRNG: ::c_int = 48;
pub const EUNATCH: ::c_int = 49;
pub const ENOCSI: ::c_int = 50;
pub const EL2HLT: ::c_int = 51;
pub const EBADE: ::c_int = 52;
pub const EBADR: ::c_int = 53;
pub const EXFULL: ::c_int = 54;
pub const ENOANO: ::c_int = 55;
pub const EBADRQC: ::c_int = 56;
pub const EBADSLT: ::c_int = 57;

pub const EMULTIHOP: ::c_int = 72;
pub const EBADMSG: ::c_int = 74;
pub const EOVERFLOW: ::c_int = 75;
pub const ENOTUNIQ: ::c_int = 76;
pub const EBADFD: ::c_int = 77;
pub const EREMCHG: ::c_int = 78;
pub const ELIBACC: ::c_int = 79;
pub const ELIBBAD: ::c_int = 80;
pub const ELIBSCN: ::c_int = 81;
pub const ELIBMAX: ::c_int = 82;
pub const ELIBEXEC: ::c_int = 83;
pub const EILSEQ: ::c_int = 84;
pub const ERESTART: ::c_int = 85;
pub const ESTRPIPE: ::c_int = 86;
pub const EUSERS: ::c_int = 87;
pub const ENOTSOCK: ::c_int = 88;
pub const EDESTADDRREQ: ::c_int = 89;
pub const EMSGSIZE: ::c_int = 90;
pub const EPROTOTYPE: ::c_int = 91;
pub const ENOPROTOOPT: ::c_int = 92;
pub const EPROTONOSUPPORT: ::c_int = 93;
pub const ESOCKTNOSUPPORT: ::c_int = 94;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const EAFNOSUPPORT: ::c_int = 97;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const ENETDOWN: ::c_int = 100;
pub const ENETUNREACH: ::c_int = 101;
pub const ENETRESET: ::c_int = 102;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EISCONN: ::c_int = 106;
pub const ENOTCONN: ::c_int = 107;
pub const ESHUTDOWN: ::c_int = 108;
pub const ETOOMANYREFS: ::c_int = 109;
pub const ETIMEDOUT: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EHOSTDOWN: ::c_int = 112;
pub const EHOSTUNREACH: ::c_int = 113;
pub const EALREADY: ::c_int = 114;
pub const EINPROGRESS: ::c_int = 115;
pub const ESTALE: ::c_int = 116;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;
pub const EDQUOT: ::c_int = 122;
pub const ENOMEDIUM: ::c_int = 123;
pub const EMEDIUMTYPE: ::c_int = 124;
pub const ECANCELED: ::c_int = 125;
pub const ENOKEY: ::c_int = 126;
pub const EKEYEXPIRED: ::c_int = 127;
pub const EKEYREVOKED: ::c_int = 128;
pub const EKEYREJECTED: ::c_int = 129;
pub const EOWNERDEAD: ::c_int = 130;
pub const ENOTRECOVERABLE: ::c_int = 131;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;

pub const SOL_SOCKET: ::c_int = 1;

pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_LINGER: ::c_int = 13;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;
pub const SO_ACCEPTCONN: ::c_int = 30;

pub const O_ACCMODE: ::c_int = 3;
pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NOCTTY: ::c_int = 256;
pub const O_NONBLOCK: ::c_int = 2048;
pub const O_SYNC: ::c_int = 0x101000;
pub const O_DIRECT: ::c_int = 0x10000;
pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_NOFOLLOW: ::c_int = 0x8000;
pub const O_ASYNC: ::c_int = 0x2000;
pub const O_NDELAY: ::c_int = 0x800;

pub const NI_MAXHOST: ::size_t = 1025;

pub const NCCS: usize = 19;
pub const TCSBRKP: ::c_int = 0x5425;
pub const TCSANOW: ::c_int = 0;
pub const TCSADRAIN: ::c_int = 0x1;
pub const TCSAFLUSH: ::c_int = 0x2;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const VEOF: usize = 4;
pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: ::tcflag_t = 0x00008000;
pub const TOSTOP: ::tcflag_t = 0x00000100;
pub const FLUSHO: ::tcflag_t = 0x00001000;

pub const ADFS_SUPER_MAGIC: ::c_long = 0x0000adf5;
pub const AFFS_SUPER_MAGIC: ::c_long = 0x0000adff;
pub const CODA_SUPER_MAGIC: ::c_long = 0x73757245;
pub const CRAMFS_MAGIC: ::c_long = 0x28cd3d45;
pub const EFS_SUPER_MAGIC: ::c_long = 0x00414a53;
pub const EXT2_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT3_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT4_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const HPFS_SUPER_MAGIC: ::c_long = 0xf995e849;
pub const HUGETLBFS_MAGIC: ::c_long = 0x958458f6;
pub const ISOFS_SUPER_MAGIC: ::c_long = 0x00009660;
pub const JFFS2_SUPER_MAGIC: ::c_long = 0x000072b6;
pub const MINIX_SUPER_MAGIC: ::c_long = 0x0000137f;
pub const MINIX_SUPER_MAGIC2: ::c_long = 0x0000138f;
pub const MINIX2_SUPER_MAGIC: ::c_long = 0x00002468;
pub const MINIX2_SUPER_MAGIC2: ::c_long = 0x00002478;
pub const MSDOS_SUPER_MAGIC: ::c_long = 0x00004d44;
pub const NCP_SUPER_MAGIC: ::c_long = 0x0000564c;
pub const NFS_SUPER_MAGIC: ::c_long = 0x00006969;
pub const OPENPROM_SUPER_MAGIC: ::c_long = 0x00009fa1;
pub const PROC_SUPER_MAGIC: ::c_long = 0x00009fa0;
pub const QNX4_SUPER_MAGIC: ::c_long = 0x0000002f;
pub const REISERFS_SUPER_MAGIC: ::c_long = 0x52654973;
pub const SMB_SUPER_MAGIC: ::c_long = 0x0000517b;
pub const TMPFS_MAGIC: ::c_long = 0x01021994;
pub const USBDEVICE_SUPER_MAGIC: ::c_long = 0x00009fa2;

pub const MADV_HUGEPAGE: ::c_int = 14;
pub const MADV_NOHUGEPAGE: ::c_int = 15;
pub const MAP_HUGETLB: ::c_int = 0x040000;

pub const PTRACE_TRACEME: ::c_int = 0;
pub const PTRACE_PEEKTEXT: ::c_int = 1;
pub const PTRACE_PEEKDATA: ::c_int = 2;
pub const PTRACE_PEEKUSER: ::c_int = 3;
pub const PTRACE_POKETEXT: ::c_int = 4;
pub const PTRACE_POKEDATA: ::c_int = 5;
pub const PTRACE_POKEUSER: ::c_int = 6;
pub const PTRACE_CONT: ::c_int = 7;
pub const PTRACE_KILL: ::c_int = 8;
pub const PTRACE_SINGLESTEP: ::c_int = 9;
pub const PTRACE_ATTACH: ::c_int = 16;
pub const PTRACE_DETACH: ::c_int = 17;
pub const PTRACE_SYSCALL: ::c_int = 24;
pub const PTRACE_SETOPTIONS: ::c_int = 0x4200;
pub const PTRACE_GETEVENTMSG: ::c_int = 0x4201;
pub const PTRACE_GETSIGINFO: ::c_int = 0x4202;
pub const PTRACE_SETSIGINFO: ::c_int = 0x4203;
pub const PTRACE_GETFPREGS: ::c_int = 14;
pub const PTRACE_SETFPREGS: ::c_int = 15;
pub const PTRACE_GETREGS: ::c_int = 12;
pub const PTRACE_SETREGS: ::c_int = 13;

pub const EFD_NONBLOCK: ::c_int = 0x800;

pub const F_GETLK: ::c_int = 5;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETOWN: ::c_int = 8;
pub const F_SETLK: ::c_int = 6;
pub const F_SETLKW: ::c_int = 7;

pub const TCGETS: ::c_int = 0x5401;
pub const TCSETS: ::c_int = 0x5402;
pub const TCSETSW: ::c_int = 0x5403;
pub const TCSETSF: ::c_int = 0x5404;
pub const TCGETA: ::c_int = 0x5405;
pub const TCSETA: ::c_int = 0x5406;
pub const TCSETAW: ::c_int = 0x5407;
pub const TCSETAF: ::c_int = 0x5408;
pub const TCSBRK: ::c_int = 0x5409;
pub const TCXONC: ::c_int = 0x540A;
pub const TCFLSH: ::c_int = 0x540B;
pub const TIOCGSOFTCAR: ::c_int = 0x5419;
pub const TIOCSSOFTCAR: ::c_int = 0x541A;
pub const TIOCINQ: ::c_int = 0x541B;
pub const TIOCLINUX: ::c_int = 0x541C;
pub const TIOCGSERIAL: ::c_int = 0x541E;
pub const TIOCEXCL: ::c_int = 0x540C;
pub const TIOCNXCL: ::c_int = 0x540D;
pub const TIOCSCTTY: ::c_int = 0x540E;
pub const TIOCGPGRP: ::c_int = 0x540F;
pub const TIOCSPGRP: ::c_int = 0x5410;
pub const TIOCOUTQ: ::c_int = 0x5411;
pub const TIOCSTI: ::c_int = 0x5412;
pub const TIOCGWINSZ: ::c_int = 0x5413;
pub const TIOCSWINSZ: ::c_int = 0x5414;
pub const TIOCMGET: ::c_int = 0x5415;
pub const TIOCMBIS: ::c_int = 0x5416;
pub const TIOCMBIC: ::c_int = 0x5417;
pub const TIOCMSET: ::c_int = 0x5418;
pub const FIONREAD: ::c_int = 0x541B;
pub const TIOCCONS: ::c_int = 0x541D;

pub const RTLD_GLOBAL: ::c_int = 0x2;
pub const RTLD_NOLOAD: ::c_int = 0x4;
pub const RTLD_NOW: ::c_int = 0;
pub const RTLD_DEFAULT: *mut ::c_void = -1isize as *mut ::c_void;

f! {
    pub fn sigemptyset(set: *mut sigset_t) -> ::c_int {
        *set = 0;
        return 0
    }
    pub fn sigaddset(set: *mut sigset_t, signum: ::c_int) -> ::c_int {
        *set |= signum as sigset_t;
        return 0
    }
    pub fn sigfillset(set: *mut sigset_t) -> ::c_int {
        *set = !0;
        return 0
    }
    pub fn sigdelset(set: *mut sigset_t, signum: ::c_int) -> ::c_int {
        *set &= !(signum as sigset_t);
        return 0
    }
    pub fn sigismember(set: *const sigset_t, signum: ::c_int) -> ::c_int {
        (*set & (signum as sigset_t)) as ::c_int
    }
    pub fn cfgetispeed(termios: *const ::termios) -> ::speed_t {
        (*termios).c_cflag & ::CBAUD
    }
    pub fn cfgetospeed(termios: *const ::termios) -> ::speed_t {
        (*termios).c_cflag & ::CBAUD
    }
    pub fn cfsetispeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int {
        let cbaud = ::CBAUD;
        (*termios).c_cflag = ((*termios).c_cflag & !cbaud) | (speed & cbaud);
        return 0
    }
    pub fn cfsetospeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int {
        let cbaud = ::CBAUD;
        (*termios).c_cflag = ((*termios).c_cflag & !cbaud) | (speed & cbaud);
        return 0
    }
    pub fn tcgetattr(fd: ::c_int, termios: *mut ::termios) -> ::c_int {
        ioctl(fd, ::TCGETS, termios)
    }
    pub fn tcsetattr(fd: ::c_int,
                     optional_actions: ::c_int,
                     termios: *const ::termios) -> ::c_int {
        ioctl(fd, optional_actions, termios)
    }
    pub fn tcflow(fd: ::c_int, action: ::c_int) -> ::c_int {
        ioctl(fd, ::TCXONC, action as *mut ::c_void)
    }
    pub fn tcflush(fd: ::c_int, action: ::c_int) -> ::c_int {
        ioctl(fd, ::TCFLSH, action as *mut ::c_void)
    }
    pub fn tcsendbreak(fd: ::c_int, duration: ::c_int) -> ::c_int {
        ioctl(fd, TCSBRKP, duration as *mut ::c_void)
    }
}

extern {
    static mut __progname: *mut ::c_char;
}

extern {
    pub fn madvise(addr: *const ::c_void, len: ::size_t, advice: ::c_int)
                   -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;
    pub fn readlink(path: *const ::c_char,
                    buf: *mut ::c_char,
                    bufsz: ::size_t)
                    -> ::c_int;
    pub fn msync(addr: *const ::c_void, len: ::size_t,
                 flags: ::c_int) -> ::c_int;
    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn sysconf(name: ::c_int) -> ::c_long;
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_int, addr: *const ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::size_t,
                       serv: *mut ::c_char,
                       sevlen: ::size_t,
                       flags: ::c_int) -> ::c_int;
    pub fn ptrace(request: ::c_int, ...) -> ::c_long;
}

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        mod b32;
        pub use self::b32::*;
    } else if #[cfg(target_pointer_width = "64")] {
        mod b64;
        pub use self::b64::*;
    } else {
        // Unknown target_pointer_width
    }
}
