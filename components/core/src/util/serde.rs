use serde::{de,
            Deserialize,
            Deserializer,
            Serialize,
            Serializer};
use std::{error::Error,
          str::FromStr};

/// `Serialize` and `Deserialize` a type using the `ToString` and `FromStr` traits.
pub mod string {
    use super::*;

    pub fn serialize<T, S>(t: &T, s: S) -> Result<S::Ok, S::Error>
        where T: ToString,
              S: Serializer
    {
        s.serialize_str(&t.to_string())
    }

    pub fn deserialize<'de, T, D>(d: D) -> Result<T, D::Error>
        where T: FromStr,
              T::Err: Error,
              D: Deserializer<'de>
    {
        String::deserialize(d)?.parse().map_err(de::Error::custom)
    }
}

/// `Serialize` and `Deserialize` a type using a proxy type that implements `Serialize` and
/// `Deserialize`.
pub mod proxy {
    use super::*;

    pub fn serialize<T, U, S>(t: T, s: S) -> Result<S::Ok, S::Error>
        where T: Into<U>,
              U: Serialize,
              S: Serializer
    {
        t.into().serialize(s)
    }

    pub fn deserialize<'de, T, U, D>(d: D) -> Result<T, D::Error>
        where U: Into<T> + Deserialize<'de>,
              D: Deserializer<'de>
    {
        U::deserialize(d).map(Into::into)
    }
}
