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

pub mod perm;
pub mod sys;

use std::error;
use std::fmt;
use std::marker::PhantomData;
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
                Err(err) => {
                    Err(R::custom(format!(
                        "string cannot be parsed: \"{}\" ({})",
                        value,
                        err.description()
                    )))
                }
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
