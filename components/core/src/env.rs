use std::{self,
          env::VarError,
          ffi::{OsStr,
                OsString},
          str::FromStr};

/// Fetches the environment variable `key` from the current process, but only it is not empty.
///
/// This function augments the `std::env::var` function from the standard library, only by
/// returning a `VarError::NotPresent` if the environment variable is set, but the value is empty.
///
/// # Examples
///
/// ```
/// use habitat_core;
/// use std;
///
/// let key = "_I_AM_A_TEAPOT_COMMA_RIGHT_PEOPLE_QUESTION_MARK_";
/// std::env::set_var(key, "");
/// match habitat_core::env::var(key) {
///     Ok(val) => panic!("The environment variable {} is set but empty!", key),
///     Err(e) => {
///         println!("The environment variable {} is set, but empty. Not useful!",
///                  key)
///     }
/// }
/// ```
pub fn var<K: AsRef<OsStr>>(key: K) -> std::result::Result<String, VarError> {
    match std::env::var(key) {
        Ok(val) => {
            if val.is_empty() {
                Err(VarError::NotPresent)
            } else {
                Ok(val)
            }
        }
        Err(e) => Err(e),
    }
}

/// Fetches the environment variable `key` from the current process, but only it is not empty.
///
/// This function augments the `std::env::var_os` function from the standard library, only by
/// returning a `VarError::NotPresent` if the environment variable is set, but the value is empty.
///
/// # Examples
///
/// ```
/// use habitat_core;
/// use std;
///
/// let key = "_I_AM_A_TEAPOT_COMMA_RIGHT_PEOPLE_QUESTION_MARK_";
/// std::env::set_var(key, "");
/// match habitat_core::env::var_os(key) {
///     Some(val) => panic!("The environment variable {} is set but empty!", key),
///     None => {
///         println!("The environment variable {} is set, but empty. Not useful!",
///                  key)
///     }
/// }
/// ```
pub fn var_os<K: AsRef<OsStr>>(key: K) -> std::option::Option<OsString> {
    match std::env::var_os(key) {
        Some(val) => {
            if val.to_string_lossy().as_ref().is_empty() {
                None
            } else {
                Some(val)
            }
        }
        None => None,
    }
}

/// Declare a struct that implements the `Config` trait with a minimum of boilerplate.
/// This declares a simple newtype struct whose name comes from `$wrapping_type`,
/// which wraps the provided `$wrapped_type` and can be overridden by setting the
/// environment variable specified in `$env_var`.
///
/// Additionally, this macro provides a `From` (and implicitly `Into`) implementation
/// so the `$wrapping_type` can be ergonomically converted to `$wrapped_type`.
///
/// For example, if `$wrapping_type` were `Foo`, to access the overridden value and
/// pass it to a function `bar` which accepts `$wrapped_type`:
///
/// ```ignore
/// bar(Foo::configured_value().into());
/// ```
///
/// See `Config::configured_value`'s documentation for more.
///
/// In general this exists to make the work of other macros which implement wrappers for
/// specific wrapped types simpler since the remaining arguments which specify the
/// `FromStr` implementation will generally be the same for a given wrapped type.
#[macro_export]
macro_rules! env_config {
    (
        #[$attr:meta],
        $vis:vis $wrapping_type:ident,
        $wrapped_type:ty,
        $env_var:ident,
        $default_value:expr,
        $from_str_err_type:ty,
        $from_str_input:ident,
        $from_str_return:expr
    ) => {
        #[allow(unused_imports)]
        use $crate::env::Config as _;

        #[$attr]
        $vis struct $wrapping_type($wrapped_type);
        // A little trickery to avoid env var name collisions:
        // This enum can't ever be instantiated, but the compiler will give
        // an error if two invocations in a namespace give the same env_var.
        // It'd be nice to make this work globally, but I don't see a good way
        #[allow(non_camel_case_types, dead_code)]
        enum $env_var {}

        impl $crate::env::Config for $wrapping_type {
            const ENVVAR: &'static str = stringify!($env_var);
        }

        impl Default for $wrapping_type {
            fn default() -> Self { Self($default_value) }
        }

        impl std::str::FromStr for $wrapping_type {
            type Err = $from_str_err_type;

            fn from_str($from_str_input: &str) -> std::result::Result<Self, Self::Err> {
                $from_str_return
            }
        }

        impl std::convert::From<$wrapping_type> for $wrapped_type {
            fn from(x: $wrapping_type) -> $wrapped_type { x.0 }
        }
    };
}

/// Declare a struct `$wrapping_type` that stores a `std::time::Duration` and
/// implements the `Config` trait so that its value can be overridden by `$env_var`.
///
/// This is a thin wrapper around `env_config`. See its documentation for more details.
///
/// Example usage:
/// ```
/// use std::time::Duration;
/// habitat_core::env_config_duration!(PersistLoopPeriod,
///                                    HAB_PERSIST_LOOP_PERIOD_SECS => from_secs
///                                    Duration::from_secs(30));
/// ```
#[macro_export]
macro_rules! env_config_duration {
    ($wrapping_type:ident, $env_var:ident => $from_str_fn:ident, $default_value:expr) => {
        $crate::env_config!(#[derive(Debug)],
                            $wrapping_type,
                            std::time::Duration,
                            $env_var,
                            $default_value,
                            std::num::ParseIntError,
                            s,
                            Ok(Self(std::time::Duration::$from_str_fn(s.parse()?))));
    };
}

/// Declare a struct `$wrapping_type` that stores an integer of type `$type` and
/// implements the `Config` trait so that its value can be overridden by `$env_var`.
///
/// This is a thin wrapper around `env_config`. See its documentation for more details.
///
/// Example usage:
/// ```
/// habitat_core::env_config_int!(RecvTimeoutMillis, i32, HAB_PULL_RECV_TIMEOUT_MS, 5_000);
///
/// habitat_core::env_config_int!(#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)],
///                               TokioThreadCount,
///                               usize,
///                               HAB_TOKIO_THREAD_COUNT,
///                               num_cpus::get().max(1));
/// ```
#[macro_export]
macro_rules! env_config_int {
    (#[$attr:meta], $wrapping_type:ident, $type:ty, $env_var:ident, $default_value:expr) => {
        $crate::env_config!(#[$attr],
                            $wrapping_type,
                            $type,
                            $env_var,
                            $default_value,
                            std::num::ParseIntError,
                            s,
                            Ok(Self((s.parse()?))));
    };

    ($wrapping_type:ident, $type:ty, $env_var:ident, $default_value:expr) => {
        $crate::env_config!(#[derive(Debug)],
                            $wrapping_type,
                            $type,
                            $env_var,
                            $default_value,
                            std::num::ParseIntError,
                            s,
                            Ok(Self((s.parse()?))));
    };
}

/// Declare a struct `$wrapping_type` that stores a `String` and
/// implements the `Config` trait so that its value can be overridden by `$env_var`.
///
/// This is a thin wrapper around `env_config`. See its documentation for more details.
///
/// Example usage:
/// ```
/// habitat_core::env_config_string!(#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)],
///                                  pub ChannelIdent,
///                                  HAB_BLDR_CHANNEL,
///                                  STABLE_CHANNEL_IDENT.to_string());
/// ```
#[macro_export]
macro_rules! env_config_string {
    (#[$attr:meta], $vis:vis $wrapping_type:ident, $env_var:ident, $default_value:expr) => {
        $crate::env_config!(#[$attr],
                            $vis $wrapping_type,
                            String,
                            $env_var,
                            $default_value,
                            $crate::Impossible,
                            s,
                            Ok(Self(s.to_string())));

        impl std::convert::From<&str> for $wrapping_type {
            fn from(s: &str) -> Self { Self(s.to_string()) }
        }

        impl std::convert::From<String> for $wrapping_type {
            fn from(s: String) -> Self { Self(s) }
        }
    };
}

#[macro_export]
macro_rules! default_as_str {
    ($wrapping_type:ident) => {
        impl $wrapping_type {
            pub fn default_as_str() -> &'static str {
                lazy_static! {
                    pub static ref DEFAULT: String = { $wrapping_type::default().to_string() };
                }
                &DEFAULT
            }
        }
    };
}

/// Declare a struct `$wrapping_type` that stores a `SocketAddr` and
/// implements the `Config` trait so that its value can be overridden by `$env_var`.
///
/// This is a thin wrapper around `env_config`. See its documentation for more details.
///
/// Example usage:
/// ```
/// habitat_core::env_config_socketaddr!(#[derive(Clone, Copy, PartialEq, Eq, Debug)],
///                                      pub ListenCtlAddr,
///                                      HAB_LISTEN_CTL,
///                                      SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, Self::DEFAULT_PORT)));
/// ```
#[macro_export]
macro_rules! env_config_socketaddr {
    (#[$attr:meta], $vis:vis $wrapping_type:ident, $env_var:ident, $default_ip:expr, $default_port:expr) => {
        $crate::env_config!(#[$attr],
                            $vis $wrapping_type,
                            SocketAddr,
                            $env_var,
                            SocketAddr::V4(SocketAddrV4::new($default_ip, $default_port)),
                            std::net::AddrParseError,
                            val,
                            Ok(val.parse::<SocketAddr>()?.into()));

        $crate::default_as_str!($wrapping_type);

        impl From<SocketAddr> for $wrapping_type {
            fn from(socket_addr: SocketAddr) -> Self { Self(socket_addr) }
        }

        impl std::fmt::Display for $wrapping_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }
    };

    (#[$attr:meta], $vis:vis $wrapping_type:ident, $env_var:ident, $default_ip_a:literal, $default_ip_b:literal, $default_ip_c:literal, $default_ip_d:literal, $default_port:expr) => {
        $crate::env_config!(#[$attr],
                            $vis $wrapping_type,
                            SocketAddr,
                            $env_var,
                            SocketAddr::V4(SocketAddrV4::new(std::net::Ipv4Addr::new($default_ip_a,
                                                                                     $default_ip_b,
                                                                                     $default_ip_c,
                                                                                     $default_ip_d),
                                                             $default_port)),
                            std::net::AddrParseError,
                            val,
                            Ok(val.parse::<SocketAddr>()?.into()));

        $crate::default_as_str!($wrapping_type);

        impl From<SocketAddr> for $wrapping_type {
            fn from(socket_addr: SocketAddr) -> Self { Self(socket_addr) }
        }

        impl std::fmt::Display for $wrapping_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }
    };

}

/// Enable the creation of a value based on an environment variable
/// that can be supplied at runtime by the user.
pub trait Config: Default + FromStr {
    /// The environment variable that will be parsed to create an
    /// instance of `Self`.
    const ENVVAR: &'static str;

    /// Generate an instance of `Self` from the value of the
    /// environment variable `Self::ENVVAR`.
    ///
    /// If the environment variable is present and not empty, its
    /// value will be parsed as `Self`. If it cannot be parsed, or the
    /// environment variable is not present, the default value of the
    /// type will be given instead.
    fn configured_value() -> Self {
        match var(Self::ENVVAR) {
            Err(VarError::NotPresent) => Self::default(),
            Ok(val) => {
                match val.parse() {
                    Ok(parsed) => {
                        Self::log_parsable(&val);
                        parsed
                    }
                    Err(_) => {
                        Self::log_unparsable(&val);
                        Self::default()
                    }
                }
            }
            Err(VarError::NotUnicode(nu)) => {
                Self::log_unparsable(nu.to_string_lossy());
                Self::default()
            }
        }
    }

    /// Overridable function for logging when an environment variable
    /// value was found and was successfully parsed as a `Self`.
    ///
    /// By default, we log a message at the `warn` level.
    fn log_parsable(env_value: &str) {
        warn!("Found '{}' in environment; using value '{}'",
              Self::ENVVAR,
              env_value);
    }

    /// Overridable function for logging when an environment variable
    /// value was found and was _not_ successfully parsed as a `Self`.
    ///
    /// By default, we log a message at the `warn` level.
    fn log_unparsable<S>(env_value: S)
        where S: AsRef<str>
    {
        warn!("Found '{}' in environment, but value '{}' was unparsable; using default instead",
              Self::ENVVAR,
              env_value.as_ref());
    }
}
