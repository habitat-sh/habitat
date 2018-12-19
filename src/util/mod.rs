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

#[cfg(not(windows))]
pub mod posix_perm;
pub mod sys;
#[cfg(windows)]
pub mod win_perm;

use std::error;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::result;
use std::str::FromStr;

use serde;

pub fn deserialize_using_from_str<'de, T, E, D>(d: D) -> result::Result<T, D::Error>
where
    T: FromStr<Err = E>,
    E: error::Error,
    D: serde::Deserializer<'de>,
{
    struct FromStringable<T, E>(PhantomData<T>, PhantomData<E>);

    impl<'de, T, E> serde::de::Visitor<'de> for FromStringable<T, E>
    where
        T: FromStr<Err = E>,
        E: error::Error,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<R>(self, value: &str) -> result::Result<T, R>
        where
            R: serde::de::Error,
        {
            match FromStr::from_str(value) {
                Ok(t) => Ok(t),
                Err(err) => Err(R::custom(format!(
                    "string cannot be parsed: \"{}\" ({})",
                    value,
                    err.description()
                ))),
            }
        }
    }

    d.deserialize_any(FromStringable(PhantomData, PhantomData))
}

pub fn serialize_using_to_string<T, S>(t: &T, s: S) -> result::Result<S::Ok, S::Error>
where
    T: ToString,
    S: serde::Serializer,
{
    s.serialize_str(&t.to_string())
}

/// Provide a way to convert numeric types safely to i64
trait ToI64 {
    fn to_i64(self) -> i64;
}

impl ToI64 for usize {
    fn to_i64(self) -> i64 {
        if mem::size_of::<usize>() >= mem::size_of::<i64>() && self > i64::max_value() as usize {
            error!(
                "Tried to convert an out-of-range usize ({}) to i64; using i64::max_value()",
                self
            );
            i64::max_value()
        } else {
            self as i64
        }
    }
}

impl ToI64 for u64 {
    fn to_i64(self) -> i64 {
        if mem::size_of::<u64>() >= mem::size_of::<i64>() && self > i64::max_value() as u64 {
            error!(
                "Tried to convert an out-of-range u64 ({}) to i64; using i64::max_value()",
                self
            );
            i64::max_value()
        } else {
            self as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_of_usize_to_i64() {
        let too_big = usize::max_value();
        let just_right: usize = 42;
        let zero: usize = 0;

        assert_eq!(too_big.to_i64(), i64::max_value());
        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    fn conversion_of_u64_to_i64() {
        let too_big = u64::max_value();
        let just_right: u64 = 42;
        let zero: u64 = 0;

        assert_eq!(too_big.to_i64(), i64::max_value());
        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }
}
