// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{env::VarError,
          str::FromStr};

use crate::hcore::env as henv;

/// Enable the creation of a value based on an environment variable
/// that can be supplied at runtime by the user.
pub trait EnvConfig: Default + FromStr {
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
        match henv::var(Self::ENVVAR) {
            Err(VarError::NotPresent) => Self::default(),
            Ok(val) => match val.parse() {
                Ok(parsed) => {
                    Self::log_parsable(&val);
                    parsed
                }
                Err(_) => {
                    Self::log_unparsable(&val);
                    Self::default()
                }
            },
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
        warn!(
            "Found '{}' in environment; using value '{}'",
            Self::ENVVAR,
            env_value
        );
    }

    /// Overridable function for logging when an environment variable
    /// value was found and was _not_ successfully parsed as a `Self`.
    ///
    /// By default, we log a message at the `warn` level.
    fn log_unparsable<S>(env_value: S)
    where
        S: AsRef<str>,
    {
        warn!(
            "Found '{}' in environment, but value '{}' was unparsable; using default instead",
            Self::ENVVAR,
            env_value.as_ref()
        );
    }
}
