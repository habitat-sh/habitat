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
pub struct AnonymousBox {
    // TODO (CM): if it's really anonymous, then sender is really the recipient!!!!
    /// The identity of the keypair that was used to encrypt (and
    /// thus, must be used to decrypt) this message.
    sender:     NamedRevision,
    /// The encoded ciphertext of the message.
    ciphertext: Vec<u8>,
}

// TODO (CM): make constructors crate-private

impl AnonymousBox {
    pub fn new(sender: NamedRevision, ciphertext: Vec<u8>) -> Self { Self { sender, ciphertext } }

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
pub struct SignedBox {
    /// The identity of the keypair of the sender of this
    /// message. Only the owner of this pair's secret key could have
    /// sent this message, and its public key must be used to decrypt
    /// it.
    sender:     NamedRevision,
    /// The identity of the keypair of the recipient of this
    /// message. The pair's public key was used to encrypt this
    /// message (thus "addressing" it to the receiver), and only the
    /// owner of this pair's secret key man decrypt it.
    receiver:   NamedRevision,
    /// The encrypted ciphertext of the message
    ciphertext: Vec<u8>,
    /// The cryptographic nonce used to encrypt the message.
    nonce:      primitives::Nonce,
}

impl SignedBox {
    pub fn new(sender: NamedRevision,
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
