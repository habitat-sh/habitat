use crate::{crypto::keys::{NamedRevision,
                           encryption::primitives},
            error::{Error,
                    Result}};
use std::{fmt,
          str,
          str::FromStr};

/// Version identifier for signed encrypted messages.
const BOX_FORMAT_VERSION: &str = "BOX-1";

/// Version identifier for anonymous encrypted messages.
const ANONYMOUS_BOX_FORMAT_VERSION: &str = "ANONYMOUS-BOX-1";

/// An anonymously encrypted message. The message was encrypted with
/// the recipient's public key, and can only be decoded by the
/// recipient's private key. The identity of the sender cannot be
/// known.
#[derive(Debug)]
pub struct AnonymousBox {
    /// The encryption key pair that was used to encrypt, and thus
    /// must also be used to decrypt, this message.
    key_pair:   NamedRevision,
    /// The encoded ciphertext of the message.
    ciphertext: Vec<u8>,
}

impl AnonymousBox {
    /// Create a new AnonymousBox. Intentionally private to the encryption module.
    pub(super) fn new(key_pair: NamedRevision, ciphertext: Vec<u8>) -> Self {
        Self { key_pair,
               ciphertext }
    }

    pub fn key_pair(&self) -> &NamedRevision { &self.key_pair }

    pub fn ciphertext(&self) -> &[u8] { &self.ciphertext }
}

impl fmt::Display for AnonymousBox {
    /// Implements Habitat's String formatting for sending signed
    /// anonymous encrypted messages. The version and key pair
    /// identifier are in plaintext, while the ciphertext is base64
    /// encoded.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}\n{}\n{}",
               ANONYMOUS_BOX_FORMAT_VERSION,
               self.key_pair(),
               crate::base64::encode(self.ciphertext()))
    }
}

impl FromStr for AnonymousBox {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        lines.next()
             .ok_or_else(|| Error::CryptoError("Corrupt payload, can't read version".to_string()))
             .map(|line| {
                 if line == ANONYMOUS_BOX_FORMAT_VERSION {
                     Ok(line)
                 } else {
                     Err(Error::CryptoError(format!("Unsupported version: {}", line)))
                 }
             })??;

        let key_pair = lines.next()
                            .ok_or_else(|| {
                                Error::CryptoError("Corrupt payload, can't read key_pair \
                                                    identifier"
                                                               .to_string())
                            })?
                            .parse()?;

        let ciphertext =
            lines.next()
                 .ok_or_else(|| {
                     Error::CryptoError("Corrupt payload, can't read ciphertext".to_string())
                 })
                 .map(crate::base64::decode)?
                 .map_err(|e| Error::CryptoError(format!("Can't decode ciphertext: {}", e)))?;

        Ok(AnonymousBox { key_pair,
                          ciphertext })
    }
}

////////////////////////////////////////////////////////////////////////

/// An encrypted message signed by the sender (and thus traceable to
/// that sender) intended for a receiver. It was encrypted using the
/// sender's secret key and the recipient's private key, and can only
/// be decrypted with the sender's public key and the recipient's
/// secret key.
///
/// `encryptor` identifies the sender's key pair, while `decryptor`
/// identifies the recipient's key pair.
///
/// See `libsodium-rs::crypto_box` for further details.
#[derive(Debug)]
pub struct SignedBox {
    /// The identity of the keypair of the sender of this
    /// message. Only the owner of this pair's secret key could have
    /// sent this message, and its public key must be used to decrypt
    /// it.
    encryptor:  NamedRevision,
    /// The identity of the keypair of the recipient of this
    /// message. The pair's public key was used to encrypt this
    /// message (thus "addressing" it to the receiver), and only the
    /// owner of this pair's secret key may decrypt it.
    decryptor:  NamedRevision,
    /// The encrypted ciphertext of the message
    ciphertext: Vec<u8>,
    /// The cryptographic nonce used to encrypt the message.
    nonce:      primitives::Nonce,
}

impl SignedBox {
    /// Create a new SignedBox. Intentionally private to the encryption module.
    pub(super) fn new(encryptor: NamedRevision,
                      decryptor: NamedRevision,
                      ciphertext: Vec<u8>,
                      nonce: primitives::Nonce)
                      -> Self {
        Self { encryptor,
               decryptor,
               ciphertext,
               nonce }
    }

    pub fn encryptor(&self) -> &NamedRevision { &self.encryptor }

    pub fn decryptor(&self) -> &NamedRevision { &self.decryptor }

    pub fn ciphertext(&self) -> &[u8] { &self.ciphertext }

    pub fn nonce(&self) -> &primitives::Nonce { &self.nonce }

    /// Helper function to parse a `SignedBox` from raw bytes.
    pub fn from_bytes<B>(bytes: B) -> Result<Self>
        where B: AsRef<[u8]>
    {
        str::from_utf8(bytes.as_ref())?.parse()
    }
}

impl fmt::Display for SignedBox {
    /// Implements Habitat's String formatting for sending signed
    /// encrypted messages. The version, encryptor identifier, and
    /// decryptor identifier are in plaintext, while the nonce and
    /// ciphertext are base64 encoded.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}\n{}\n{}\n{}\n{}",
               BOX_FORMAT_VERSION,
               self.encryptor,
               self.decryptor,
               crate::base64::encode(self.nonce.clone()),
               crate::base64::encode(&self.ciphertext))
    }
}

impl FromStr for SignedBox {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        lines.next()
             .ok_or_else(|| Error::CryptoError("Corrupt payload, can't read version".to_string()))
             .map(|line| {
                 if line == BOX_FORMAT_VERSION {
                     Ok(line)
                 } else {
                     Err(Error::CryptoError(format!("Unsupported version: {}", line)))
                 }
             })??;

        let encryptor = lines.next()
                             .ok_or_else(|| {
                                 Error::CryptoError("Corrupt payload, can't read encryptor \
                                                     identifier"
                                                                .to_string())
                             })?
                             .parse()?;

        let decryptor = lines.next()
                             .ok_or_else(|| {
                                 Error::CryptoError("Corrupt payload, can't read decryptor key \
                                                     name"
                                                          .to_string())
                             })?
                             .parse()?;

        let nonce =
            lines.next()
                 .ok_or_else(|| Error::CryptoError("Corrupt payload, can't read nonce".to_string()))
                 .map(crate::base64::decode)?
                 .map_err(|e| Error::CryptoError(format!("Can't decode nonce: {}", e)))
                 .and_then(|bytes| {
                     let array: [u8; 24] = bytes.try_into()
                         .map_err(|_| Error::CryptoError("Invalid size of nonce".to_string()))?;
                     Ok(primitives::Nonce::from_bytes_exact(array))
                 })?;

        let ciphertext =
            lines.next()
                 .ok_or_else(|| {
                     Error::CryptoError("Corrupt payload, can't read ciphertext".to_string())
                 })
                 .map(crate::base64::decode)?
                 .map_err(|e| Error::CryptoError(format!("Can't decode ciphertext: {}", e)))?;

        Ok(SignedBox { encryptor,
                       decryptor,
                       ciphertext,
                       nonce })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod anonymous {
        use super::*;

        /// An encrypted anonymous box message.
        #[rustfmt::skip]
        const ANONYMOUS_TEXT: &str = "ANONYMOUS-BOX-1\nfhloston-paradise-20200813211603\nDHcXh6dIhfpOg2jtwHFhNzDl19FsHnmERjlKucsAWiG9yTnu7DORBE/nxTrhzoU0ZxPkjNn2VL4xCDlb4EPEUJM=";

        /// The raw ciphertext of the `ANONYMOUS_TEXT` message above.
        const CIPHERTEXT: [u8; 65] =
            [12u8, 119u8, 23u8, 135u8, 167u8, 72u8, 133u8, 250u8, 78u8, 131u8, 104u8, 237u8,
             192u8, 113u8, 97u8, 55u8, 48u8, 229u8, 215u8, 209u8, 108u8, 30u8, 121u8, 132u8, 70u8,
             57u8, 74u8, 185u8, 203u8, 0u8, 90u8, 33u8, 189u8, 201u8, 57u8, 238u8, 236u8, 51u8,
             145u8, 4u8, 79u8, 231u8, 197u8, 58u8, 225u8, 206u8, 133u8, 52u8, 103u8, 19u8, 228u8,
             140u8, 217u8, 246u8, 84u8, 190u8, 49u8, 8u8, 57u8, 91u8, 224u8, 67u8, 196u8, 80u8,
             147u8];

        #[test]
        fn parse() {
            let anonymous = ANONYMOUS_TEXT.parse::<AnonymousBox>().unwrap();
            let expected_key_pair: NamedRevision =
                "fhloston-paradise-20200813211603".parse().unwrap();

            assert_eq!(anonymous.key_pair(), &expected_key_pair);
            assert_eq!(anonymous.ciphertext().to_vec(), CIPHERTEXT.to_vec());
        }

        #[test]
        fn to_string() {
            let anonymous = AnonymousBox { key_pair:   "fhloston-paradise-20200813211603".parse()
                                                                                         .unwrap(),
                                           ciphertext: CIPHERTEXT.to_vec(), };

            assert_eq!(anonymous.to_string(), ANONYMOUS_TEXT);
        }
    }

    mod signed {
        use super::*;

        /// An encrypted signed box message.
        #[rustfmt::skip]
        const SIGNED_TEXT: &str = "BOX-1\nruby-rhod-20200813204159\nservice-key-valid.default@acme-20160509181736\nB7e4QAO4aKZFNbiBQAOm+8kZhqVDtOwo\n+faMzEwgX1EDcjl9ab4MVBR8klaQnLegxSmGygWOHU+Y8PUz";

        /// The bytes of the nonce used in `SIGNED_TEXT` above.
        const NONCE: [u8; 24] = [7u8, 183u8, 184u8, 64u8, 3u8, 184u8, 104u8, 166u8, 69u8, 53u8,
                                 184u8, 129u8, 64u8, 3u8, 166u8, 251u8, 201u8, 25u8, 134u8, 165u8,
                                 67u8, 180u8, 236u8, 40u8];

        /// The bytes of the ciphertext used in `SIGNED_TEXT` above.
        const CIPHERTEXT: [u8; 36] = [249u8, 246u8, 140u8, 204u8, 76u8, 32u8, 95u8, 81u8, 3u8,
                                      114u8, 57u8, 125u8, 105u8, 190u8, 12u8, 84u8, 20u8, 124u8,
                                      146u8, 86u8, 144u8, 156u8, 183u8, 160u8, 197u8, 41u8, 134u8,
                                      202u8, 5u8, 142u8, 29u8, 79u8, 152u8, 240u8, 245u8, 51u8];

        #[test]
        fn parse() {
            let signed = SIGNED_TEXT.parse::<SignedBox>().unwrap();

            let expected_encryptor: NamedRevision = "ruby-rhod-20200813204159".parse().unwrap();
            let expected_receiver: NamedRevision =
                "service-key-valid.default@acme-20160509181736".parse()
                                                               .unwrap();

            assert_eq!(signed.encryptor(), &expected_encryptor);
            assert_eq!(signed.decryptor(), &expected_receiver);
            assert_eq!(signed.nonce(), &primitives::Nonce::from_bytes_exact(NONCE));
            assert_eq!(signed.ciphertext().to_vec(), CIPHERTEXT.to_vec());
        }

        #[test]
        fn to_string() {
            let encryptor = "ruby-rhod-20200813204159".parse().unwrap();
            let decryptor = "service-key-valid.default@acme-20160509181736".parse()
                                                                           .unwrap();
            let nonce = primitives::Nonce::from_bytes_exact(NONCE);
            let ciphertext = CIPHERTEXT.to_vec();

            let signed = SignedBox { encryptor,
                                     decryptor,
                                     ciphertext,
                                     nonce };

            assert_eq!(signed.to_string(), SIGNED_TEXT);
        }
    }
}
