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

use clap::ArgMatches;
use serde_json;
use std::{path::Path,
          result::Result as StdResult,
          str::FromStr};

use crate::export_docker::Result;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct PersistentStorage {
    pub size: String,
    pub path: String,
    pub class: String,
}

impl PersistentStorage {
    pub fn from_args(matches: &ArgMatches<'_>) -> Result<Option<Self>> {
        if let Some(arg) = matches.value_of("PERSISTENT_STORAGE") {
            Ok(Some(arg.parse::<Self>()?))
        } else {
            Ok(None)
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "size": self.size,
            "path": self.path,
            "class": self.class,
        })
    }

    fn valid_size(size: &str) -> bool {
        #[derive(Clone, Copy, PartialEq)]
        enum State {
            Nothing,
            Digits,
            E,
            FixedPoint,
            PowerOfTen,
            PowerOfTwo,
        };
        let mut state = State::Nothing;
        let end_states = vec![
            State::Digits,
            State::FixedPoint,
            State::PowerOfTen,
            State::PowerOfTwo,
        ];
        for c in size.chars() {
            match state {
                State::Nothing => match c {
                    '0'..='9' => state = State::Digits,
                    _ => return false,
                },
                State::Digits => match c {
                    '0'..='9' => (),
                    'e' => state = State::E,
                    'E' | 'P' | 'T' | 'G' | 'M' | 'K' => state = State::PowerOfTen,
                    _ => return false,
                },
                State::E => match c {
                    '0'..='9' => state = State::FixedPoint,
                    _ => return false,
                },
                State::FixedPoint => match c {
                    '0'..='9' => (),
                    _ => return false,
                },
                State::PowerOfTen => match c {
                    'i' => state = State::PowerOfTwo,
                    _ => return false,
                },
                State::PowerOfTwo => match c {
                    _ => return false,
                },
            }
        }
        end_states.contains(&state)
    }

    fn valid_path(path_str: &str) -> bool {
        let path = Path::new(path_str);

        path.is_absolute()
    }
}

impl FromStr for PersistentStorage {
    type Err = Error;

    fn from_str(persistent_storage_str: &str) -> StdResult<Self, Self::Err> {
        let values: Vec<&str> = persistent_storage_str.splitn(3, ':').collect();
        if values.len() != 3
            || !PersistentStorage::valid_size(values[0])
            || !PersistentStorage::valid_path(values[1])
            || values[2].is_empty()
        {
            return Err(Error::InvalidPersistentStorageSpec(
                persistent_storage_str.to_string(),
            ));
        }

        Ok(PersistentStorage {
            size: values[0].to_string(),
            path: values[1].to_string(),
            class: values[2].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PersistentStorage;

    #[test]
    fn test_k8s_size() {
        let valid_sizes = vec![
            "9", "99", "99e9", "9E", "9P", "9T", "9G", "9M", "9K", "9Ei", "9Pi", "9Ti", "9Gi",
            "9Mi", "9Ki",
        ];
        let invalid_sizes = vec!["", "foo", "99e", "99ex", "99X", "9Kx", "9Kix", "99e9K"];

        for size in valid_sizes {
            assert!(
                PersistentStorage::valid_size(size),
                "size '{}' ought to be valid, but is not",
                size
            );
        }
        for size in invalid_sizes {
            assert!(
                !PersistentStorage::valid_size(size),
                "size '{}' ought to be invalid, but is not",
                size
            );
        }
    }

    #[test]
    fn test_persistent_storage_from_str() {
        let valid = vec![
            ("10G:/foo:storage_class", "10G", "/foo", "storage_class"),
            (
                "10G:/foo:storage_class:with_colons",
                "10G",
                "/foo",
                "storage_class:with_colons",
            ),
        ];
        let invalid = vec![
            "10G:/not/enough/parts",
            "10G:/foo/empty/storage/class:",
            "10G::empty_path",
            ":/empty/size:sc",
            "invalid size:/foo:sc",
            "10G:relative/path:sc",
        ];

        for (raw, size, path, storage_class) in valid {
            let result = raw.parse::<PersistentStorage>();
            assert!(result.is_ok(), "failed to parse valid raw string '{}'", raw);
            let storage = result.unwrap();
            assert_eq!(storage.size, size, "tested valid raw string '{}'", raw);
            assert_eq!(storage.path, path, "tested valid raw string '{}'", raw);
            assert_eq!(
                storage.class, storage_class,
                "tested valid raw string '{}'",
                raw
            );
        }

        for raw in invalid {
            assert!(
                raw.parse::<PersistentStorage>().is_err(),
                "invalid raw string '{}' parsed successfully to PersistentStorage",
                raw
            );
        }
    }
}
