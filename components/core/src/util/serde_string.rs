use serde;
use std::{error,
          fmt,
          marker::PhantomData,
          result,
          str::FromStr};

pub fn deserialize<'de, T, E, D>(d: D) -> result::Result<T, D::Error>
    where T: FromStr<Err = E>,
          E: error::Error,
          D: serde::Deserializer<'de>
{
    struct FromStringable<T, E>(PhantomData<T>, PhantomData<E>);

    impl<'de, T, E> serde::de::Visitor<'de> for FromStringable<T, E>
        where T: FromStr<Err = E>,
              E: error::Error
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<R>(self, value: &str) -> result::Result<T, R>
            where R: serde::de::Error
        {
            match FromStr::from_str(value) {
                Ok(t) => Ok(t),
                Err(err) => {
                    Err(R::custom(format!("string cannot be parsed: \"{}\" ({})",
                                          value,
                                          err.description())))
                }
            }
        }
    }

    d.deserialize_any(FromStringable(PhantomData, PhantomData))
}

pub fn serialize<T, S>(t: &T, s: S) -> result::Result<S::Ok, S::Error>
    where T: ToString,
          S: serde::Serializer
{
    s.serialize_str(&t.to_string())
}
