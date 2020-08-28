use super::super::{ANONYMOUS_BOX_FORMAT_VERSION,
                   BOX_FORMAT_VERSION};

use crate::{crypto::keys::encryption::{AnonymousBox,
                                       SignedBox},
            error::{Error,
                    Result}};
use serde_derive::{Deserialize,
                   Serialize};
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::Nonce;
use std::{borrow::Cow,
          str};

#[derive(Debug)]
pub struct BoxSecret<'a> {
    pub sender:     &'a str,
    pub ciphertext: Vec<u8>,
    pub receiver:   Option<&'a str>,
    pub nonce:      Option<Nonce>,
}

////////////////////////////////////////////////////////////////////////

// A sodiumoxide sealed box that has been base64-encoded together with
// metadata to indicate how it should be decrypted
#[deprecated(note = "Please new encrypted message types")]
#[derive(Serialize, Deserialize)]
pub struct WrappedSealedBox<'a>(Cow<'a, str>);

impl<'a> WrappedSealedBox<'a> {
    pub fn into_bytes(self) -> Vec<u8> { self.0.into_owned().into_bytes() }

    /// Only needed by builder due to double-base64 encoding in
    /// builder_core::integrations::encrypt
    pub fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    pub fn from_bytes(bytes: &'a [u8]) -> std::result::Result<Self, std::str::Utf8Error> {
        str::from_utf8(bytes).map(Cow::Borrowed)
                             .map(WrappedSealedBox)
    }

    // Return the metadata and encrypted text from a secret payload.
    // This is useful for services consuming an encrypted payload and need to decrypt it without
    // having keys on disk
    pub fn secret_metadata<'b>(&'b self) -> Result<BoxSecret<'b>> {
        let mut lines = self.0.lines();

        let version = Self::parse_version(lines.next())?;
        let sender = Self::parse_sender(lines.next())?;
        let receiver = if Self::is_anonymous_box(version) {
            None
        } else {
            Some(Self::parse_receiver(lines.next())?)
        };
        let nonce = if Self::is_anonymous_box(version) {
            None
        } else {
            Some(Self::parse_nonce(lines.next())?)
        };
        let ciphertext = Self::parse_ciphertext(lines.next())?;

        Ok(BoxSecret { sender,
                       receiver,
                       nonce,
                       ciphertext })
    }

    fn parse_version(line: Option<&str>) -> Result<&str> {
        line.ok_or_else(|| Error::CryptoError("Corrupt payload, can't read version".to_string()))
            .map(|line| {
                match line {
                    BOX_FORMAT_VERSION | ANONYMOUS_BOX_FORMAT_VERSION => Ok(line),
                    _ => Err(Error::CryptoError(format!("Unsupported version: {}", line))),
                }
            })?
    }

    fn parse_sender(line: Option<&str>) -> Result<&str> {
        line.ok_or_else(|| {
                Error::CryptoError("Corrupt payload, can't read sender key name".to_string())
            })
    }

    fn is_anonymous_box(version: &str) -> bool { version == ANONYMOUS_BOX_FORMAT_VERSION }

    fn parse_receiver(line: Option<&str>) -> Result<&str> {
        line.ok_or_else(|| {
                Error::CryptoError("Corrupt payload, can't read receiver key name".to_string())
            })
    }

    fn parse_nonce(line: Option<&str>) -> Result<Nonce> {
        line.ok_or_else(|| Error::CryptoError("Corrupt payload, can't read nonce".to_string()))
            .map(base64::decode)?
            .map_err(|e| Error::CryptoError(format!("Can't decode nonce: {}", e)))
            .map(|bytes| Nonce::from_slice(bytes.as_ref()))?
            .ok_or_else(|| Error::CryptoError("Invalid size of nonce".to_string()))
    }

    fn parse_ciphertext(line: Option<&str>) -> Result<Vec<u8>> {
        line.ok_or_else(|| Error::CryptoError("Corrupt payload, can't read ciphertext".to_string()))
            .map(base64::decode)?
            .map_err(|e| Error::CryptoError(format!("Can't decode ciphertext: {}", e)))
    }
}

// Temporary !!
impl<'a> From<AnonymousBox> for WrappedSealedBox<'a> {
    fn from(anon: AnonymousBox) -> Self { Self(Cow::Owned(anon.to_string())) }
}

impl<'a> From<SignedBox> for WrappedSealedBox<'a> {
    fn from(signed: SignedBox) -> Self { Self(Cow::Owned(signed.to_string())) }
}

impl<'a> From<String> for WrappedSealedBox<'a> {
    fn from(payload: String) -> Self { Self(Cow::Owned(payload)) }
}
// used in Builder to extract an encrypted value from an HTTP request.
// All these lifetimes are just unnecessary, since we end up going
// directly into a BoxSecret, and then extract named revisions AS A
// STRING.
impl<'a, 'b: 'a> From<&'b str> for WrappedSealedBox<'a> {
    fn from(payload: &'b str) -> Self { Self(Cow::Borrowed(payload)) }
}
