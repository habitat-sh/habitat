use crate::{crypto::keys::{encryption::primitives,
                           NamedRevision},
            error::{Error,
                    Result}};
use std::{fmt,
          str,
          str::FromStr};

/// Version identifier for signed encrypted messages.
const BOX_FORMAT_VERSION: &str = "BOX-1";

/// Version identifier for anonymous encrypted messages.
const ANONYMOUS_BOX_FORMAT_VERSION: &str = "ANONYMOUS-BOX-1";

/// An encrypted message sent anonymously to a recipient. The message
/// was encrypted with the recipient's public key, and can only be
/// decoded by the recipient's private key. The identity of the sender
/// cannot be known.
#[derive(Debug)]
pub struct AnonymousBox {
    // TODO (CM): if it's really anonymous, then sender is really the recipient!!!!
    /// The identity of the keypair that was used to encrypt (and
    /// thus, must be used to decrypt) this message.
    sender:     NamedRevision,
    /// The encoded ciphertext of the message.
    ciphertext: Vec<u8>,
}

impl AnonymousBox {
    /// Create a new AnonymousBox. Intentionally private to the encryption module.
    pub(super) fn new(sender: NamedRevision, ciphertext: Vec<u8>) -> Self {
        Self { sender, ciphertext }
    }

    pub fn sender(&self) -> &NamedRevision { &self.sender }

    pub fn ciphertext(&self) -> &[u8] { &self.ciphertext }
}

impl fmt::Display for AnonymousBox {
    /// Implements Habitat's String formatting for sending signed
    /// anonymous encrypted messages. The version and sender are in
    /// plaintext, while the ciphertext is base64 encoded.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}\n{}\n{}",
               ANONYMOUS_BOX_FORMAT_VERSION,
               self.sender,
               base64::encode(&self.ciphertext))
    }
}

/// An encrypted message signed by the sender (and thus traceable to
/// that sender) intended for a receiver. It was encrypted using the
/// sender's secret key and the recipient's private key, and can only
/// be decrypted with the sender's public key and the recipient's
/// secret key.
///
/// See `sodiumoxide::crypto::box_` for further details.
#[derive(Debug)]
pub struct SignedBox {
    /// The identity of the keypair of the sender of this
    /// message. Only the owner of this pair's secret key could have
    /// sent this message, and its public key must be used to decrypt
    /// it.
    sender:     NamedRevision,
    /// The identity of the keypair of the recipient of this
    /// message. The pair's public key was used to encrypt this
    /// message (thus "addressing" it to the receiver), and only the
    /// owner of this pair's secret key may decrypt it.
    receiver:   NamedRevision,
    /// The encrypted ciphertext of the message
    ciphertext: Vec<u8>,
    /// The cryptographic nonce used to encrypt the message.
    nonce:      primitives::Nonce,
}

impl SignedBox {
    /// Create a new SignedBox. Intentionally private to the encryption module.
    pub(super) fn new(sender: NamedRevision,
                      receiver: NamedRevision,
                      ciphertext: Vec<u8>,
                      nonce: primitives::Nonce)
                      -> Self {
        Self { sender,
               receiver,
               ciphertext,
               nonce }
    }

    pub fn sender(&self) -> &NamedRevision { &self.sender }

    pub fn receiver(&self) -> &NamedRevision { &self.receiver }

    pub fn ciphertext(&self) -> &[u8] { &self.ciphertext }

    pub fn nonce(&self) -> &primitives::Nonce { &self.nonce }
}

impl fmt::Display for SignedBox {
    /// Implements Habitat's String formatting for sending signed
    /// encrypted messages. The version, sender, and receiver are in
    /// plaintext, while the nonce and ciphertext are base64 encoded.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}\n{}\n{}\n{}\n{}",
               BOX_FORMAT_VERSION,
               self.sender,
               self.receiver,
               base64::encode(self.nonce),
               base64::encode(&self.ciphertext))
    }
}

// TODO (CM): THIS should be the main way to access this stuff
#[derive(Debug)]
pub enum EncryptedSecret {
    Anonymous(AnonymousBox),
    Signed(SignedBox),
}

impl EncryptedSecret {
    /// Unwraps this to an anonymous secret, if it is actually anonymous.
    pub fn anonymous(self) -> Result<AnonymousBox> {
        match self {
            Self::Anonymous(anonymous) => Ok(anonymous),
            _ => Err(Error::CryptoError("Not an anonymous secret!".to_string())),
        }
    }

    /// Unwraps this to a signed secret, if it is actually signed.
    pub fn signed(self) -> Result<SignedBox> {
        match self {
            Self::Signed(signed_box) => Ok(signed_box),
            _ => Err(Error::CryptoError("Not an signed secret!".to_string())),
        }
    }

    /// Helper function to parse an `EncryptedSecret` from raw bytes.
    pub fn from_bytes<B>(bytes: B) -> Result<EncryptedSecret>
        where B: AsRef<[u8]>
    {
        str::from_utf8(bytes.as_ref())?.parse()
    }
}

/// Encapsulates parsing logic for all variants of `EncryptedSecret`,
/// since they overlap a great deal.
impl FromStr for EncryptedSecret {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let version =
            lines.next()
                 .ok_or_else(|| {
                     Error::CryptoError("Corrupt payload, can't read version".to_string())
                 })
                 .map(|line| {
                     match line {
                         BOX_FORMAT_VERSION | ANONYMOUS_BOX_FORMAT_VERSION => Ok(line),
                         _ => Err(Error::CryptoError(format!("Unsupported version: {}", line))),
                     }
                 })??;

        let is_anonymous = version == ANONYMOUS_BOX_FORMAT_VERSION;

        let sender =
            lines.next()
                 .ok_or_else(|| {
                     Error::CryptoError("Corrupt payload, can't read sender key name".to_string())
                 })?
                 .parse()?;

        let receiver = if is_anonymous {
            None
        } else {
            let r = lines.next()
                         .ok_or_else(|| {
                             Error::CryptoError("Corrupt payload, can't read receiver key \
                                                         name"
                                                              .to_string())
                         })?
                         .parse()?;
            Some(r)
        };
        let nonce = if is_anonymous {
            None
        } else {
            let n =
                lines.next()
                     .ok_or_else(|| {
                         Error::CryptoError("Corrupt payload, can't read nonce".to_string())
                     })
                     .map(base64::decode)?
                     .map_err(|e| Error::CryptoError(format!("Can't decode nonce: {}", e)))
                     .map(|bytes| primitives::Nonce::from_slice(bytes.as_ref()))?
                     .ok_or_else(|| Error::CryptoError("Invalid size of nonce".to_string()))?;
            Some(n)
        };

        let ciphertext =
            lines.next()
                 .ok_or_else(|| {
                     Error::CryptoError("Corrupt payload, can't read ciphertext".to_string())
                 })
                 .map(base64::decode)?
                 .map_err(|e| Error::CryptoError(format!("Can't decode ciphertext: {}", e)))?;

        if is_anonymous {
            Ok(EncryptedSecret::Anonymous(AnonymousBox { sender, ciphertext }))
        } else {
            Ok(EncryptedSecret::Signed(SignedBox { sender,
                                                   receiver:
                                                       receiver.unwrap(),
                                                   ciphertext,
                                                   nonce:
                                                       nonce.unwrap() }))
        }
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
            let encrypted = ANONYMOUS_TEXT.parse::<EncryptedSecret>().unwrap();
            let anonymous = encrypted.anonymous().unwrap();

            let expected_sender: NamedRevision =
                "fhloston-paradise-20200813211603".parse().unwrap();

            assert_eq!(anonymous.sender(), &expected_sender);
            assert_eq!(anonymous.ciphertext().to_vec(), CIPHERTEXT.to_vec());
        }

        #[test]
        fn to_string() {
            let anonymous = AnonymousBox { sender:     "fhloston-paradise-20200813211603".parse()
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
            let encrypted = SIGNED_TEXT.parse::<EncryptedSecret>().unwrap();
            let signed = encrypted.signed().unwrap();

            let expected_sender: NamedRevision = "ruby-rhod-20200813204159".parse().unwrap();
            let expected_receiver: NamedRevision =
                "service-key-valid.default@acme-20160509181736".parse()
                                                               .unwrap();

            assert_eq!(signed.sender(), &expected_sender);
            assert_eq!(signed.receiver(), &expected_receiver);
            assert_eq!(signed.nonce(),
                       &primitives::Nonce::from_slice(&NONCE).unwrap());
            assert_eq!(signed.ciphertext().to_vec(), CIPHERTEXT.to_vec());
        }

        #[test]
        fn to_string() {
            let sender = "ruby-rhod-20200813204159".parse().unwrap();
            let receiver = "service-key-valid.default@acme-20160509181736".parse()
                                                                          .unwrap();
            let nonce = primitives::Nonce::from_slice(&NONCE).unwrap();
            let ciphertext = CIPHERTEXT.to_vec();

            let signed = SignedBox { sender,
                                     receiver,
                                     nonce,
                                     ciphertext };

            assert_eq!(signed.to_string(), SIGNED_TEXT);
        }
    }
}
