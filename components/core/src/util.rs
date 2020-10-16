pub mod docker;
#[cfg(not(windows))]
pub mod posix_perm;
pub mod serde;
pub mod sys;
pub mod text_render;
#[cfg(windows)]
pub mod win_perm;

use std::{io::{self,
               BufRead},
          mem};

/// Same as `Result::ok()`, but logs the error case. Useful for
/// ignoring error cases, while still leaving a paper trail.
#[doc(hidden)]
#[macro_export]
macro_rules! __ok_log {
    ($log_level:expr, $result:expr) => {
        match $result {
            Ok(val) => Some(val),
            Err(e) => {
                log!($log_level,
                     "Intentionally ignored error ({}:{}): {:?}",
                     file!(),
                     line!(),
                     e);
                None
            }
        }
    };
}

/// Same as `Result::ok()`, but logs the error case at the `error` level.
#[macro_export]
macro_rules! ok_error {
    ($result:expr) => {
        $crate::__ok_log!(log::Level::Error, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `warn` level.
#[macro_export]
macro_rules! ok_warn {
    ($result:expr) => {
        $crate::__ok_log!(log::Level::Warn, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `info` level.
#[macro_export]
macro_rules! ok_info {
    ($result:expr) => {
        $crate::__ok_log!(log::Level::Info, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `debug` level.
#[macro_export]
macro_rules! ok_debug {
    ($result:expr) => {
        $crate::__ok_log!(log::Level::Debug, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `trace` level.
#[macro_export]
macro_rules! ok_trace {
    ($result:expr) => {
        $crate::__ok_log!(log::Level::Trace, $result)
    };
}

/// returns the common arguments to pass to pwsh.exe when spawning a powershell instance.
/// These arguments are optimized for a background powershell process running hooks.
/// The NonInteractive flag specifies that the console is not intended to interact with
/// human input and allows ctrl+break signals to trigger a graceful termination similar to
/// a SIGTERM on linux rather than an interactive debugging prompt. The ExecutionPolicy
/// ensures that if a more strict policy exists in the Windows Registry (ex "AllSigned"),
/// hook execution will not fail because hook scripts are never signed. RemoteSigned is the
/// default policy and just requires remote scripts to be signeed. Supervisor hooks are
/// always local so "RemoteSigned" does not interfere with supervisor behavior.
#[cfg(windows)]
pub fn pwsh_args(command: &str) -> Vec<&str> {
    vec!["-NonInteractive",
         "-ExecutionPolicy",
         "RemoteSigned",
         "-Command",
         command]
}

// This is copied from [here](https://github.com/rust-lang/rust/blob/d3cba254e464303a6495942f3a831c2bbd7f1768/src/libstd/io/mod.rs#L2495),
// but converted into a "lossy" version
#[derive(Debug)]
pub struct LossyLines<B> {
    buf: B,
}

impl<B: BufReadLossy> Iterator for LossyLines<B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line_lossy(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

// A lossy way to read lines
pub trait BufReadLossy: BufRead {
    fn read_line_lossy(&mut self, buf: &mut String) -> io::Result<usize> {
        let mut buffer = Vec::new();
        let size = self.read_until(b'\n', &mut buffer)?;
        let s = String::from_utf8_lossy(&buffer);
        buf.push_str(&s);
        Ok(size)
    }

    fn lines_lossy(self) -> LossyLines<Self>
        where Self: Sized
    {
        LossyLines { buf: self }
    }
}

// Implement `BufReadLossy` for all types that implement `BufRead`
impl<T: BufRead> BufReadLossy for T {}

/// Provide a way to convert numeric types safely to i64
pub trait ToI64 {
    fn to_i64(self) -> i64;
}

impl ToI64 for usize {
    fn to_i64(self) -> i64 {
        if mem::size_of::<usize>() >= mem::size_of::<i64>() && self > i64::max_value() as usize {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range usize ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range usize ({}) to i64; using \
                        i64::max_value()",
                       self);
                i64::max_value()
            }
        } else {
            self as i64
        }
    }
}

impl ToI64 for u64 {
    fn to_i64(self) -> i64 {
        if self > i64::max_value() as u64 {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range u64 ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range u64 ({}) to i64; using i64::max_value()",
                       self);
                i64::max_value()
            }
        } else {
            self as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn conversion_of_usize_to_i64() {
        let just_right: usize = 42;
        let zero: usize = 0;

        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn conversion_of_too_big_usize_panics_in_debug_mode() {
        let too_big = usize::max_value();
        too_big.to_i64();
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn conversion_of_too_big_usize_caps_in_release_mode() {
        let too_big = usize::max_value();
        assert_eq!(too_big.to_i64(), i64::max_value());
    }

    #[test]
    fn conversion_of_u64_to_i64() {
        let just_right: u64 = 42;
        let zero: u64 = 0;

        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn conversion_of_too_big_u64_panics_in_debug_mode() {
        let too_big = u64::max_value();
        too_big.to_i64();
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn conversion_of_too_big_u64_caps_in_release_mode() {
        let too_big = u64::max_value();
        assert_eq!(too_big.to_i64(), i64::max_value());
    }

    #[test]
    fn read_lines_lossy() {
        let mut buf = "line 1\r\nline « 2\r\nline 3\r\n".as_bytes().to_vec();
        // the char « is [194, 171] in utf8
        // we will swap in 171 which is it's pure ASCII value
        buf.push(174);
        let idx1 = buf.iter().position(|&x| x == 194).expect("no 194");
        buf.remove(idx1);
        let idx2 = buf.iter().position(|&x| x == 171).expect("no 171");
        buf.swap_remove(idx2);

        let reader = BufReader::new(buf.as_slice());
        let mut str_vec = Vec::new();
        for line in reader.lines_lossy() {
            let line = line.unwrap();
            str_vec.push(line);
        }

        assert_eq!(str_vec.len(), 3);
        assert_eq!(str_vec[0], "line 1");
        assert_eq!(str_vec[1], "line � 2");
        assert_eq!(str_vec[2], "line 3");
    }
}
