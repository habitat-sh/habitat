// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std;
use std::env::VarError;
use std::ffi::{OsStr, OsString};

/// Fetches the environment variable `key` from the current process, but only it is not empty.
///
/// This function augments the `std::env::var` function from the standard library, only by
/// returning a `VarError::NotPresent` if the environment variable is set, but the value is empty.
///
/// # Examples
///
/// ```
/// use std;
/// use habitat_core;
///
/// let key = "_I_AM_A_TEAPOT_COMMA_RIGHT_PEOPLE_QUESTION_MARK_";
/// std::env::set_var(key, "");
/// match habitat_core::env::var(key) {
///     Ok(val) => panic!("The environment variable {} is set but empty!", key),
///     Err(e) => println!("The environment variable {} is set, but empty. Not useful!", key),
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
/// use std;
/// use habitat_core;
///
/// let key = "_I_AM_A_TEAPOT_COMMA_RIGHT_PEOPLE_QUESTION_MARK_";
/// std::env::set_var(key, "");
/// match habitat_core::env::var_os(key) {
///     Some(val) => panic!("The environment variable {} is set but empty!", key),
///     None => println!("The environment variable {} is set, but empty. Not useful!", key),
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

/// Fetches the environment variable `SUDO_USER` from the current process, but only if the value is
/// not `"root"`.
///
/// This function is special-purpose for a Habitat-centric interpretation of this value. If the
/// root user is running a command with `sudo`, then the environment will contain a
/// `SUDO_USER=root` value. However, Habitat considers root's home for caches, etc. to be under
/// the `/hab` directory (as opposed to root's `$HOME`).
///
/// # Examples
///
/// With no environment variable present:
///
/// ```
/// use std;
/// use habitat_core;
///
/// std::env::remove_var("SUDO_USER");
/// match habitat_core::env::sudo_user() {
///     Some(val) => panic!("The environment variable is set but should be unset!"),
///     None => println!("No SUDO_USER set in the environment"),
/// }
/// ```
///
/// With a non-root user set:
///
/// ```
/// use std;
/// use habitat_core;
///
/// std::env::set_var("SUDO_USER", "bob");
/// match habitat_core::env::sudo_user() {
///     Some(val) => assert_eq!(val, "bob"),
///     None => panic!("The environment variable is set and should be bob"),
/// }
/// ```
///
/// With the root user set:
///
/// ```
/// use std;
/// use habitat_core;
///
/// std::env::set_var("SUDO_USER", "root");
/// match habitat_core::env::sudo_user() {
///     Some(val) => panic!("The environment variable is set to root and should return with None!"),
///     None => println!("No non-root SUDO_USER set in the environment"),
/// }
/// ```
///
pub fn sudo_user() -> std::option::Option<String> {
    match self::var("SUDO_USER") {
        Ok(val) => if val != "root" { Some(val) } else { None },
        Err(_) => None,
    }
}
