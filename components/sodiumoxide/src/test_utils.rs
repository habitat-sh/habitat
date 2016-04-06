#![cfg(all(test, feature = "default"))]

use rustc_serialize::{Decodable, Encodable, json};

// Encodes then decodes `value` using JSON
pub fn round_trip<T>(value: T) where T: Decodable + Encodable + Eq {
    let encoded_value = json::encode(&value).unwrap();
    let decoded_value = json::decode(&encoded_value).unwrap();
    assert!(value == decoded_value);
}
