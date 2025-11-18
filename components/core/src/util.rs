pub mod docker;
#[cfg(not(windows))]
pub mod posix_perm;
pub mod serde;
pub mod sys;
pub mod text_render;
#[cfg(windows)]
pub mod win_perm;

#[cfg(windows)]
use crate::{env as henv,
            error::Result,
            os::process::windows_child::Child};
use log::error;

#[cfg(windows)]
use std::{collections::HashMap,
          env,
          path::PathBuf};

use std::{io::{self,
               BufRead},
          mem};

/// Same as `Result::ok()`, but logs the error case. Useful for
/// ignoring error cases, while still leaving a paper trail.
#[doc(hidden)]
#[macro_export]
macro_rules! __ok_log {
    ($log_level:expr_2021, $result:expr_2021) => {
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
    ($result:expr_2021) => {
        $crate::__ok_log!(log::Level::Error, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `warn` level.
#[macro_export]
macro_rules! ok_warn {
    ($result:expr_2021) => {
        $crate::__ok_log!(log::Level::Warn, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `info` level.
#[macro_export]
macro_rules! ok_info {
    ($result:expr_2021) => {
        $crate::__ok_log!(log::Level::Info, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `debug` level.
#[macro_export]
macro_rules! ok_debug {
    ($result:expr_2021) => {
        $crate::__ok_log!(log::Level::Debug, $result)
    };
}

/// Same as `Result::ok()`, but logs the error case at the `trace` level.
#[macro_export]
macro_rules! ok_trace {
    ($result:expr_2021) => {
        $crate::__ok_log!(log::Level::Trace, $result)
    };
}

/// This macro implements `TryFrom<String>` and `Into<String>` for a list of types implementing
/// `FromStr` and `Display`. The traits `FromStr` and `Display` are preferred. However, occasionally
/// there are instances where type bounds require `TryFrom<String>` and `Into<String>`. For example,
/// using the serde tag `#[serde(try_from = "String", into = "String")]`
#[macro_export]
macro_rules! impl_try_from_string_and_into_string {
    ($($ty:ty),*) => {
        $(
            impl std::convert::TryFrom<String> for $ty {
                type Error = Error;

                fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
            }

            impl From<$ty> for String {
                fn from(t: $ty) -> Self { t.to_string() }
            }
        )*
    };
}

/// Spawns a background powershell process optimized for running hooks.
#[cfg(windows)]
pub fn spawn_pwsh<U, P>(command: &str,
                        env: &HashMap<String, String>,
                        svc_user: U,
                        svc_encrypted_password: Option<P>)
                        -> Result<Child>
    where U: ToString,
          P: ToString
{
    // The NonInteractive flag specifies that the console is not intended to interact with
    // human input and allows ctrl+break signals to trigger a graceful termination similar to
    // a SIGTERM on linux rather than an interactive debugging prompt. The ExecutionPolicy
    // ensures that if a more strict policy exists in the Windows Registry (ex "AllSigned"),
    // hook execution will not fail because hook scripts are never signed. RemoteSigned is the
    // default policy and just requires remote scripts to be signeed. Supervisor hooks are
    // always local so "RemoteSigned" does not interfere with supervisor behavior.
    let args = vec!["-NonInteractive",
                    "-ExecutionPolicy",
                    "RemoteSigned",
                    "-Command",
                    command];

    let mut new_env = HashMap::new();
    // Opts out of powershell application insights telemetry code on shell startup
    new_env.insert("POWERSHELL_TELEMETRY_OPTOUT".to_string(), "1".to_string());
    new_env.extend(env.iter().map(|(k, v)| (k.clone(), v.clone())));

    with_ps_module_path(&mut new_env);

    Child::spawn("pwsh.exe",
                 &args,
                 &new_env,
                 svc_user,
                 svc_encrypted_password)
}

/// Makes sure the modules path inside the same package as pwsh.exe
/// is at the head of PSModulePath to eliminate the possibility that
/// the windows powershell modues might appear first and be preferred
#[cfg(windows)]
fn with_ps_module_path(env: &mut HashMap<String, String>) {
    if let Some(path) = env.get("PATH") {
        let mut pwsh_path = None;
        for path in env::split_paths(&path) {
            let candidate = PathBuf::from(&path).join("pwsh.exe");
            if candidate.is_file() {
                pwsh_path = Some(candidate);
                break;
            }
        }

        if let Some(pwsh_path) = pwsh_path {
            if let Some(pwsh_parent) = pwsh_path.parent() {
                let psmodulepath = pwsh_parent.join("Modules");
                let mut new_psmodulepath = psmodulepath.clone().into_os_string();
                let path_to_inherit = if let Some(path) = env.remove("PSModulePath") {
                    Some(path)
                } else {
                    henv::var_os("PSModulePath").map(|path| path.to_string_lossy().to_string())
                };

                if let Some(path) = path_to_inherit {
                    let mut paths = vec![psmodulepath.clone()];
                    for entry in env::split_paths(&path) {
                        if entry != psmodulepath {
                            paths.push(entry);
                        }
                    }
                    new_psmodulepath = env::join_paths(paths).unwrap();
                }

                env.insert("PSModulePath".to_string(),
                           new_psmodulepath.to_string_lossy().to_string());
            }
        }
    }
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
    #[allow(clippy::wrong_self_convention)]
    fn to_i64(self) -> i64;
}

impl ToI64 for usize {
    fn to_i64(self) -> i64 {
        if mem::size_of::<usize>() >= mem::size_of::<i64>() && self > i64::MAX as usize {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range usize ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range usize ({}) to i64; using \
                        i64::max_value()",
                       self);
                i64::MAX
            }
        } else {
            self as i64
        }
    }
}

impl ToI64 for u64 {
    fn to_i64(self) -> i64 {
        if self > i64::MAX as u64 {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range u64 ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range u64 ({}) to i64; using i64::max_value()",
                       self);
                i64::MAX
            }
        } else {
            self as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(windows)]
    use std::fs::File;
    use std::io::BufReader;
    #[cfg(windows)]
    use tempfile::tempdir;

    #[cfg(windows)]
    crate::locked_env_var!(PSMODULEPATH, lock_psmodulepath);

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
        let too_big = usize::MAX;
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
        let too_big = u64::MAX;
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

    #[test]
    #[cfg(windows)]
    fn ps_module_path_will_use_first_ps_path_on_path() {
        let l = lock_psmodulepath();
        l.unset();
        let mut env = HashMap::new();
        let ps_temp = tempdir().expect("couldn't create tempdir");
        let ps_temp2 = tempdir().expect("couldn't create tempdir2");
        let ps_path = ps_temp.path();
        File::create(ps_path.join("pwsh.exe")).expect("couldn't create pwsh");
        File::create(ps_temp2.path().join("pwsh.exe")).expect("couldn't create pwsh");
        env.insert("PATH".to_string(),
                   format!("{};{}",
                           ps_path.to_string_lossy(),
                           ps_temp2.path().to_string_lossy()));

        with_ps_module_path(&mut env);

        assert_eq!(env["PSModulePath"],
                   ps_path.join("Modules").to_string_lossy())
    }

    #[test]
    #[cfg(windows)]
    fn ps_module_path_with_none_already_in_env() {
        let l = lock_psmodulepath();
        l.unset();
        let mut env = HashMap::new();
        let ps_temp = tempdir().expect("couldn't create tempdir");
        let ps_path = ps_temp.path();
        File::create(ps_path.join("pwsh.exe")).expect("couldn't create pwsh");
        env.insert("PATH".to_string(), ps_path.to_string_lossy().to_string());

        with_ps_module_path(&mut env);

        assert_eq!(env["PSModulePath"],
                   ps_path.join("Modules").to_string_lossy())
    }

    #[test]
    #[cfg(windows)]
    fn ps_module_path_with_none_in_env_will_append_and_dedupe_from_current_process() {
        let mut env = HashMap::new();
        let ps_temp = tempdir().expect("couldn't create tempdir");
        let ps_path = ps_temp.path();
        let ps_str = ps_path.to_string_lossy();
        File::create(ps_path.join("pwsh.exe")).expect("couldn't create pwsh");
        env.insert("PATH".to_string(), ps_str.to_string());
        let l = lock_psmodulepath();
        l.set(format!("path1;{}\\Modules;path2", ps_str));

        with_ps_module_path(&mut env);

        assert_eq!(env["PSModulePath"],
                   format!("{}\\Modules;path1;path2", ps_str))
    }

    #[test]
    #[cfg(windows)]
    fn ps_module_path_already_in_env_is_appended() {
        let mut env = HashMap::new();
        env.insert("PSModulePath".to_string(), "provided_path".to_string());
        let ps_temp = tempdir().expect("couldn't create tempdir");
        let ps_path = ps_temp.path();
        let ps_str = ps_path.to_string_lossy();
        File::create(ps_path.join("pwsh.exe")).expect("couldn't create pwsh");
        env.insert("PATH".to_string(), ps_str.to_string());

        with_ps_module_path(&mut env);

        assert_eq!(env["PSModulePath"],
                   format!("{}\\Modules;provided_path", ps_str))
    }

    #[test]
    #[cfg(windows)]
    fn ps_module_path_already_in_env_is_appended_and_deduped() {
        let mut env = HashMap::new();
        let ps_temp = tempdir().expect("couldn't create tempdir");
        let ps_path = ps_temp.path();
        let ps_str = ps_path.to_string_lossy();
        env.insert("PSModulePath".to_string(),
                   format!("provided_path1;{}\\Modules;provided_path2", ps_str));
        File::create(ps_path.join("pwsh.exe")).expect("couldn't create pwsh");
        env.insert("PATH".to_string(), ps_str.to_string());

        with_ps_module_path(&mut env);

        assert_eq!(env["PSModulePath"],
                   format!("{}\\Modules;provided_path1;provided_path2", ps_str))
    }
}
