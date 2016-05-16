#![deny(warnings)]

extern crate ctest;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let x86_64 = target.contains("x86_64");
    let windows = target.contains("windows");
    let mingw = target.contains("windows-gnu");
    let linux = target.contains("unknown-linux");
    let android = target.contains("android");
    let apple = target.contains("apple");
    let musl = target.contains("musl");
    let freebsd = target.contains("freebsd");
    let dragonfly = target.contains("dragonfly");
    let mips = target.contains("mips");
    let netbsd = target.contains("netbsd");
    let openbsd = target.contains("openbsd");
    let rumprun = target.contains("rumprun");
    let bsdlike = freebsd || apple || netbsd || openbsd || dragonfly;
    let mut cfg = ctest::TestGenerator::new();

    // Pull in extra goodies on linux/mingw
    if linux || android {
        cfg.define("_GNU_SOURCE", None);
    } else if windows {
        cfg.define("_WIN32_WINNT", Some("0x8000"));
    }

    // Android doesn't actually have in_port_t but it's much easier if we
    // provide one for us to test against
    if android {
        cfg.define("in_port_t", Some("uint16_t"));
    }

    cfg.header("errno.h")
       .header("fcntl.h")
       .header("limits.h")
       .header("locale.h")
       .header("stddef.h")
       .header("stdint.h")
       .header("stdio.h")
       .header("stdlib.h")
       .header("sys/stat.h")
       .header("sys/types.h")
       .header("time.h")
       .header("wchar.h");

    if windows {
        cfg.header("winsock2.h"); // must be before windows.h

        cfg.header("direct.h");
        cfg.header("io.h");
        cfg.header("sys/utime.h");
        cfg.header("windows.h");
        cfg.header("process.h");
        cfg.header("ws2ipdef.h");

        if target.contains("gnu") {
            cfg.header("ws2tcpip.h");
        }
    } else {
        cfg.header("ctype.h");
        cfg.header("dirent.h");
        if openbsd {
            cfg.header("sys/socket.h");
        }
        cfg.header("net/if.h");
        cfg.header("netdb.h");
        cfg.header("netinet/in.h");
        cfg.header("netinet/ip.h");
        cfg.header("netinet/tcp.h");
        cfg.header("pthread.h");
        cfg.header("dlfcn.h");
        cfg.header("signal.h");
        cfg.header("string.h");
        cfg.header("sys/file.h");
        cfg.header("sys/ioctl.h");
        cfg.header("sys/mman.h");
        cfg.header("sys/resource.h");
        cfg.header("sys/socket.h");
        cfg.header("sys/time.h");
        cfg.header("sys/un.h");
        cfg.header("sys/wait.h");
        cfg.header("unistd.h");
        cfg.header("utime.h");
        cfg.header("pwd.h");
        cfg.header("grp.h");
        cfg.header("sys/utsname.h");
        cfg.header("sys/ptrace.h");
        cfg.header("sys/mount.h");
        cfg.header("sys/uio.h");
        cfg.header("sched.h");
        cfg.header("termios.h");
        cfg.header("poll.h");
        cfg.header("syslog.h");
    }

    if android {
        cfg.header("arpa/inet.h");
        cfg.header("time64.h");
        cfg.header("xlocale.h");
    } else if !windows {
        cfg.header("glob.h");
        cfg.header("ifaddrs.h");
        cfg.header("sys/statvfs.h");
        cfg.header("langinfo.h");

        if !openbsd && !freebsd && !dragonfly {
            cfg.header("sys/quota.h");
        }

        if !musl {
            cfg.header("sys/sysctl.h");

            if !netbsd && !openbsd {
                cfg.header("execinfo.h");
                cfg.header("xlocale.h");
            }
        }
    }

    if apple {
        cfg.header("mach-o/dyld.h");
        cfg.header("mach/mach_time.h");
        cfg.header("malloc/malloc.h");
        cfg.header("util.h");
        if target.starts_with("x86") {
            cfg.header("crt_externs.h");
        }
    }

    if bsdlike {
        cfg.header("sys/event.h");

        if freebsd {
            cfg.header("libutil.h");
        } else {
            cfg.header("util.h");
        }
    }

    if linux {
        cfg.header("mqueue.h");
        cfg.header("ucontext.h");
        cfg.header("sys/signalfd.h");
        cfg.header("sys/xattr.h");
        cfg.header("sys/ipc.h");
        cfg.header("sys/shm.h");
        cfg.header("pty.h");
    }

    if linux || android {
        cfg.header("malloc.h");
        cfg.header("net/ethernet.h");
        cfg.header("netpacket/packet.h");
        cfg.header("sched.h");
        cfg.header("sys/epoll.h");
        cfg.header("sys/eventfd.h");
        cfg.header("sys/prctl.h");
        cfg.header("sys/sendfile.h");
        cfg.header("sys/vfs.h");
        cfg.header("sys/syscall.h");
        if !musl {
            cfg.header("linux/netlink.h");
            cfg.header("linux/magic.h");

            if !mips {
                cfg.header("linux/quota.h");
            }
        }
    }

    if freebsd {
        cfg.header("pthread_np.h");
        cfg.header("sched.h");
        cfg.header("ufs/ufs/quota.h");
    }

    if netbsd {
        cfg.header("ufs/ufs/quota.h");
        cfg.header("ufs/ufs/quota1.h");
        cfg.header("sys/ioctl_compat.h");
    }

    if openbsd {
        cfg.header("ufs/ufs/quota.h");
        cfg.header("rpcsvc/rex.h");
        cfg.header("pthread_np.h");
        cfg.header("sys/syscall.h");
    }

    if dragonfly {
        cfg.header("ufs/ufs/quota.h");
        cfg.header("pthread_np.h");
        cfg.header("sys/ioctl_compat.h");
    }

    cfg.type_name(move |ty, is_struct| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" |
            "fd_set" |
            "Dl_info" |
            "DIR" => ty.to_string(),

            // Fixup a few types on windows that don't actually exist.
            "time64_t" if windows => "__time64_t".to_string(),
            "ssize_t" if windows => "SSIZE_T".to_string(),

            // OSX calls this something else
            "sighandler_t" if bsdlike => "sig_t".to_string(),

            t if t.ends_with("_t") => t.to_string(),

            // Windows uppercase structs don't have `struct` in front, there's a
            // few special cases for windows, and then otherwise put `struct` in
            // front of everything.
            t if is_struct => {
                if windows && ty.chars().next().unwrap().is_uppercase() {
                    t.to_string()
                } else if windows && t == "stat" {
                    "struct __stat64".to_string()
                } else if windows && t == "utimbuf" {
                    "struct __utimbuf64".to_string()
                } else {
                    format!("struct {}", t)
                }
            }

            t => t.to_string(),
        }
    });

    let target2 = target.clone();
    cfg.field_name(move |struct_, field| {
        match field {
            "st_birthtime"      if openbsd && struct_ == "stat" => "__st_birthtime".to_string(),
            "st_birthtime_nsec" if openbsd && struct_ == "stat" => "__st_birthtimensec".to_string(),
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                if target2.contains("apple") {
                    s.replace("_nsec", "spec.tv_nsec")
                } else if target2.contains("android") {
                    s.to_string()
                } else {
                    s.replace("e_nsec", ".tv_nsec")
                }
            }
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            "sighandler_t" => true,

            _ => false
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            "sockaddr_nl" => musl,

            // The alignment of this is 4 on 64-bit OSX...
            "kevent" if apple && x86_64 => true,

            _ => false
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            "LARGE_INTEGER" |
            "mach_timebase_info_data_t" |
            "float" |
            "double" => true,
            // uuid_t is a struct, not an integer.
            "uuid_t" if dragonfly => true,
            n if n.starts_with("pthread") => true,

            // windows-isms
            n if n.starts_with("P") => true,
            n if n.starts_with("H") => true,
            n if n.starts_with("LP") => true,
            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // Apparently these don't exist in mingw headers?
            "MEM_RESET_UNDO" |
            "FILE_ATTRIBUTE_NO_SCRUB_DATA" |
            "FILE_ATTRIBUTE_INTEGRITY_STREAM" |
            "ERROR_NOTHING_TO_TERMINATE" if mingw => true,

            "SIG_IGN" => true, // sighandler_t weirdness

            // types on musl are defined a little differently
            n if musl && n.contains("__SIZEOF_PTHREAD") => true,

            // Skip constants not defined in MUSL but just passed down to the
            // kernel regardless
            "RLIMIT_NLIMITS" |
            "TCP_COOKIE_TRANSACTIONS" |
            "RLIMIT_RTTIME" if musl => true,
            // work around super old mips toolchain
            "SCHED_IDLE" | "SHM_NORESERVE" => mips,

            // weird signed extension or something like that?
            "MS_NOUSER" => true,
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // These OSX constants are flagged as deprecated
            "NOTE_EXIT_REPARENTED" |
            "NOTE_REAP" if apple => true,

            // The linux/quota.h header file which defines these can't be
            // included with sys/quota.h currently on MIPS, so we don't include
            // it and just ignore these constants
            "QFMT_VFS_OLD" |
            "QFMT_VFS_V0" if mips && linux => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64" |        // non-int in 2nd arg
            "strerror_r" if linux => true,   // actually xpg-something-or-other

            // typed 2nd arg on linux and android
            "gettimeofday" if linux || android || freebsd || openbsd || dragonfly => true,

            // not declared in newer android toolchains
            "getdtablesize" if android => true,

            "dlerror" if android => true, // const-ness is added
            "dladdr" if musl => true, // const-ness only added recently

            // OSX has 'struct tm *const' which we can't actually represent in
            // Rust, but is close enough to *mut
            "timegm" if apple => true,

            // OSX's daemon is deprecated in 10.5 so we'll get a warning (which
            // we turn into an error) so just ignore it.
            "daemon" if apple => true,

            // These functions presumably exist on netbsd but don't look like
            // they're implemented on rumprun yet, just let them slide for now.
            // Some of them look like they have headers but then don't have
            // corresponding actual definitions either...
            "shm_open" |
            "shm_unlink" |
            "syscall" |
            "ptrace" |
            "sigaltstack" if rumprun => true,

            // There seems to be a small error in EGLIBC's eventfd.h header. The
            // [underlying system call][1] always takes its first `count`
            // argument as an `unsigned int`, but [EGLIBC's <sys/eventfd.h>
            // header][2] declares it to take an `int`. [GLIBC's header][3]
            // matches the kernel.
            //
            // EGLIBC is no longer actively developed, and Debian, the largest
            // distribution that had been using it, switched back to GLIBC in
            // April 2015. So effectively all Linux <sys/eventfd.h> headers will
            // be using `unsigned int` soon.
            //
            // [1]: https://git.kernel.org/cgit/linux/kernel/git/stable/linux-stable.git/tree/fs/eventfd.c?id=refs/tags/v3.12.51#n397
            // [2]: http://bazaar.launchpad.net/~ubuntu-branches/ubuntu/trusty/eglibc/trusty/view/head:/sysdeps/unix/sysv/linux/sys/eventfd.h
            // [3]: https://sourceware.org/git/?p=glibc.git;a=blob;f=sysdeps/unix/sysv/linux/sys/eventfd.h;h=6295f32e937e779e74318eb9d3bdbe76aef8a8f3;hb=4e42b5b8f89f0e288e68be7ad70f9525aebc2cff#l34
            "eventfd" if linux => true,

            // The `uname` funcion in freebsd is now an inline wrapper that
            // delegates to another, but the symbol still exists, so don't check
            // the symbol.
            "uname" if freebsd => true,

            _ => false,
        }
    });

    cfg.skip_fn_ptrcheck(move |name| {
        match name {
            // dllimport weirdness?
            _ if windows => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        (struct_ == "siginfo_t" && field == "_pad") ||
        // musl names this __dummy1 but it's still there
        (musl && struct_ == "glob_t" && field == "gl_flags") ||
        // musl seems to define this as an *anonymous* bitfield
        (musl && struct_ == "statvfs" && field == "__f_unused")
    });

    cfg.fn_cname(move |name, cname| {
        if windows {
            cname.unwrap_or(name).to_string()
        } else {
            name.to_string()
        }
    });

    if env::var("SKIP_COMPILE").is_ok() {
        cfg.generate_files("../src/lib.rs", "all.rs");
    } else {
        cfg.generate("../src/lib.rs", "all.rs");
    }
}
