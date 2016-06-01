// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Wire level message encoding and decoding with optional symmetric encryption.
//!
//! This module contains a `WireMessage` which represents an arbitrary serializable message which
//! can be sent or received when communicating between 2 systems. In Habitat's use case, this is
//! over a UTP socket between Supervisors or from a CLI to one or more Supervisors. The message can
//! optionally be encrypted or decrypted with a symmetric encryption key. The message itself
//! contains extra information if encryption is used, and blank if the message is plaintext.

use std::str;

use hcore::crypto::SymKey;
use rustc_serialize::{Decodable, Encodable, json};

use error::{Error, Result};

const WIRE_VERSION: &'static str = "WIRE-1";

/// The types of valid messages: currently `Plain` or `Encrypted`.
#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub enum MessageFormat {
    /// A plaintext or unencrypted message.
    Plain,
    /// An encrypted message.
    Encrypted,
}

/// A message to be serialized and deserialized for the purposes of communication.
///
/// A message can be encrypted, denoted by the `format` field being set to
/// `MessageFormat::Encrypted`, or plaintext, denoted by the `format` field being set to
/// `MessageFormat::Plain`.
#[derive(RustcEncodable, RustcDecodable)]
pub struct WireMessage {
    /// The format of the message.
    pub format: MessageFormat,
    /// The wire format identifier. A lexically sortable string which will be used for any furture
    /// breaking changes are required.
    pub version: String,
    /// The name with revision of the key which encrypted this message, if this message is
    /// encrypted.
    key: Option<String>,
    /// A byte vector containing the cryptographic
    /// [nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce) which is associated with the
    /// encrypted message. This will not be set if the message is unencrypted.
    nonce: Option<Vec<u8>>,
    /// A byte vector containing the raw message if plaintext or the the encrypted message if
    /// encrypted.
    msg_bytes: Vec<u8>,
}

impl WireMessage {
    /// Creates a new plaintext (unencrypted) `WireMessage`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_common;
    /// extern crate rustc_serialize;
    ///
    /// use habitat_common::wire_message::{MessageFormat, WireMessage};
    /// use rustc_serialize::{Decodable, Encodable};
    ///
    /// #[derive(RustcEncodable, RustcDecodable)]
    /// pub struct Person {
    ///     pub given_name: String,
    ///     pub surname: String,
    /// }
    ///
    /// fn main() {
    ///     let cash = Person { given_name: "Johnny".to_string(), surname: "Cash".to_string() };
    ///     let plain = WireMessage::plain(&cash).unwrap();
    ///
    ///     // The message is plaintext
    ///     assert_eq!(plain.format, MessageFormat::Plain);
    ///     // The message format has a version
    ///     assert_eq!(plain.version, "WIRE-1".to_string());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If the `msg` cannot be encoded into bytes
    pub fn plain<T: Encodable>(msg: &T) -> Result<WireMessage> {
        Ok(WireMessage {
            format: MessageFormat::Plain,
            version: WIRE_VERSION.to_string(),
            key: None,
            nonce: None,
            msg_bytes: try!(json::encode(&msg)).into_bytes(),
        })
    }

    /// Creates a new encrypted `WireMessage`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_common;
    /// extern crate habitat_core;
    /// extern crate rustc_serialize;
    /// extern crate tempdir;
    ///
    /// use habitat_core::crypto::SymKey;
    /// use habitat_common::wire_message::{MessageFormat, WireMessage};
    /// use rustc_serialize::{Decodable, Encodable};
    /// use tempdir::TempDir;
    ///
    /// #[derive(RustcEncodable, RustcDecodable)]
    /// pub struct Person {
    ///     pub given_name: String,
    ///     pub surname: String,
    /// }
    ///
    /// fn main() {
    ///     let cache = TempDir::new("key_cache").unwrap();
    ///     let sym_key = SymKey::generate_pair_for_ring("beyonce", cache.path()).unwrap();
    ///
    ///     let tabor = Person { given_name: "Ty".to_string(), surname: "Tabor".to_string() };
    ///     let encrypted = WireMessage::encrypted(&tabor, &sym_key).unwrap();
    ///
    ///     // The message is encrypted
    ///     assert_eq!(encrypted.format, MessageFormat::Encrypted);
    ///     // The message format has a version
    ///     assert_eq!(encrypted.version, "WIRE-1".to_string());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If the message can't be encoded to bytes
    /// * If a crypto error occurs when encrypting
    pub fn encrypted<T: Encodable>(msg: &T, sym_key: &SymKey) -> Result<WireMessage> {
        let (nonce, ciphertext) = try!(sym_key.encrypt(try!(json::encode(&msg)).as_bytes()));
        Ok(WireMessage {
            format: MessageFormat::Encrypted,
            version: WIRE_VERSION.to_string(),
            key: Some(sym_key.name_with_rev()),
            nonce: Some(nonce),
            msg_bytes: ciphertext,
        })
    }

    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// extern crate habitat_common;
    /// extern crate habitat_core;
    /// extern crate rustc_serialize;
    /// extern crate tempdir;
    ///
    /// use habitat_core::crypto::SymKey;
    /// use habitat_common::wire_message::{MessageFormat, WireMessage};
    /// use rustc_serialize::{Decodable, Encodable};
    /// use tempdir::TempDir;
    ///
    /// #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    /// pub struct Person {
    ///     pub given_name: String,
    ///     pub surname: String,
    /// }
    ///
    /// fn main() {
    ///     let cache = TempDir::new("key_cache").unwrap();
    ///     let sym_key = SymKey::generate_pair_for_ring("beyonce", cache.path()).unwrap();
    ///
    ///     let bonham = Person { given_name: "John".to_string(), surname: "Bonham".to_string() };
    ///     let encrypted = WireMessage::encrypted(&bonham, &sym_key).unwrap();
    ///     let result: Person = encrypted.msg(Some(&sym_key)).unwrap();
    ///
    ///     // The result message matches the input message
    ///     assert_eq!(result, bonham);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * If the message can't be decoded from bytes
    /// * If the wire message is malformed, that is, missing fields required for encrypted messages
    /// * If a required key with revision is not present for decrypting
    pub fn msg<T: Decodable>(&self, sym_key: Option<&SymKey>) -> Result<T> {
        match self.format {
            MessageFormat::Plain => {
                let msg_str = try!(str::from_utf8(&self.msg_bytes));
                let decoded: T = try!(json::decode(msg_str));
                Ok(decoded)
            }
            MessageFormat::Encrypted => {
                let key_name_with_rev = match self.key.as_ref() {
                    Some(k) => k,
                    None => {
                        return Err(Error::WireDecode("Key not present for encrypted message"
                            .to_string()))
                    }
                };
                let nonce = match self.nonce.as_ref() {
                    Some(n) => n,
                    None => {
                        return Err(Error::WireDecode("Nonce not present for encrypted message"
                            .to_string()))
                    }
                };
                let sym_key = match sym_key.as_ref() {
                    Some(k) => k,
                    None => {
                        let msg = format!("Message is encrypted with key {} but \
                                          no key is loaded to decrypt.",
                                          key_name_with_rev);
                        return Err(Error::WireDecode(msg));
                    }
                };
                if key_name_with_rev != &sym_key.name_with_rev() {
                    let msg = format!("Loaded key {} does not match message encrypted with key {}",
                                      &sym_key.name_with_rev(),
                                      key_name_with_rev);
                    return Err(Error::WireDecode(msg));
                }
                let msg = try!(sym_key.decrypt(&nonce[..], &self.msg_bytes[..]));
                let msg_str = try!(str::from_utf8(&msg));
                let decoded: T = try!(json::decode(msg_str));
                Ok(decoded)
            }
        }
    }
}
