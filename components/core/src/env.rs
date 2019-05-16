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

#[macro_export]
macro_rules! env_config {
    (
        $wrapping_type:ident,
        $wrapped_type:ty,
        $env_var:ident,
        $default_value:expr,
        $from_str_err_type:ty,
        $from_str_input:ident,
        $from_str_return:expr
    ) => {
        struct $wrapping_type($wrapped_type);
        // A little trickery to avoid env var name collisions:
        // This enum can't ever be instantiated, but the compiler will give
        // an error if two invocations in a namespace give the same env_var
        // It'd be nice to make this work globally, but I don't see a good way
        #[allow(non_camel_case_types, dead_code)]
        enum $env_var {}

        impl $crate::env::Config for $wrapping_type {
            const ENVVAR: &'static str = "$env_var";
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
    };
}

#[macro_export]
macro_rules! env_config_duration {
    ($wrapping_type:ident, $env_var:ident, $default_value:expr) => {
        $crate::env_config!($wrapping_type,
                            std::time::Duration,
                            $env_var,
                            $default_value,
                            std::num::ParseIntError,
                            s,
                            Ok(Self(std::time::Duration::from_secs(s.parse()?))));
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
