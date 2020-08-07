use super::{PUBLIC_BOX_KEY_VERSION,
            PUBLIC_KEY_SUFFIX,
            PUBLIC_SIG_KEY_VERSION,
            SECRET_BOX_KEY_SUFFIX,
            SECRET_BOX_KEY_VERSION,
            SECRET_SIG_KEY_SUFFIX,
            SECRET_SIG_KEY_VERSION,
            SECRET_SYM_KEY_SUFFIX,
            SECRET_SYM_KEY_VERSION};
use crate::{crypto::keys::util::FromSlice,
            error::{Error,
                    Result},
            fs::{Permissions,
                 DEFAULT_PUBLIC_KEY_PERMISSIONS,
                 DEFAULT_SECRET_KEY_PERMISSIONS}};
use chrono::Utc;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashSet,
          convert::TryFrom,
          fmt,
          fs::{self,
               File},
          io::{prelude::*,
               BufReader,
               BufWriter},
          ops::Deref,
          path::{Path,
                 PathBuf},
          result,
          str::FromStr};

lazy_static::lazy_static! {
    static ref NAME_WITH_REV_RE: Regex = Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\z").unwrap();
    static ref KEYFILE_RE: Regex =
        Regex::new(r"\A(?P<name>.+)-(?P<rev>\d{14})\.(?P<suffix>[a-z]+(\.[a-z]+)?)\z").unwrap();
}

#[macro_use]
mod util;
pub mod box_key_pair;
mod cache;
pub mod ring_key;
mod service_key;
pub mod sig_key_pair;
mod user_key;

pub use cache::KeyCache;
pub use service_key::{generate_service_encryption_key_pair,
                      ServicePublicEncryptionKey,
                      ServiceSecretEncryptionKey};
pub use user_key::{generate_user_encryption_key_pair,
                   UserPublicEncryptionKey,
                   UserSecretEncryptionKey};

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
enum KeyType {
    Sig,
    Box,
    Sym,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KeyType::Box => write!(f, "box"),
            KeyType::Sig => write!(f, "sig"),
            KeyType::Sym => write!(f, "sym"),
        }
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum PairType {
    Public,
    Secret,
}

impl fmt::Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PairType::Public => write!(f, "public"),
            PairType::Secret => write!(f, "secret"),
        }
    }
}

impl FromStr for PairType {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value {
            "public" => Ok(PairType::Public),
            "secret" => Ok(PairType::Secret),
            _ => {
                Err(Error::CryptoError(format!("Invalid PairType conversion \
                                                from {}",
                                               value)))
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////

struct TmpKeyfile {
    pub path: PathBuf,
}

impl Drop for TmpKeyfile {
    fn drop(&mut self) {
        if self.path.is_file() {
            let _ = fs::remove_file(&self.path);
        }
    }
}

////////////////////////////////////////////////////////////////////////

pub trait Key {
    const EXTENSION: &'static str;
    const VERSION_STRING: &'static str;
    const PERMISSIONS: Permissions;

    type Crypto: FromSlice<Self::Crypto> + AsRef<[u8]>;

    /// Returns the permissions with which an item should be written
    /// to the filesystem.
    fn extension() -> &'static str { Self::EXTENSION }

    /// Get a reference to the underlying bytes of actual
    /// cryptographic material.
    fn key(&self) -> &Self::Crypto;

    fn named_revision(&self) -> &NamedRevision;

    /// Returns what should be the contents of a file when rendering a
    /// key out to a file on disk.
    ///
    /// While one *could* just use `ToString` / `Display`, that choice
    /// could be a little dangerous, considering the fact that the
    /// content of private keys should never potentially make it into
    /// logging or other output.
    fn to_key_string(&self) -> String {
        let k = self.key();
        format!("{}\n{}\n\n{}",
                Self::VERSION_STRING,
                self.named_revision(),
                &base64::encode(k))
    }

    /// Returns the permissions with which an item should be written
    /// to the filesystem.
    fn permissions() -> Permissions { Self::PERMISSIONS }

    /// This is the core logic that underpins the FromStr
    /// implementation for all the keys. See the
    /// `from_str_impl_for_key!` macro for how it gets put into
    /// practice.
    fn parse_from_str(content: &str) -> Result<(String, KeyRevision, Self::Crypto)> {
        let mut lines = content.lines();

        lines.next()
             .ok_or_else(|| Error::CryptoError("Missing key version".to_string()))
             .map(|line| {
                 if line == Self::VERSION_STRING {
                     Ok(())
                 } else {
                     Err(Error::CryptoError(format!("Unsupported key version: {}", line)))
                 }
             })??;

        let named_revision: NamedRevision =
            lines.next()
                 .ok_or_else(|| Error::CryptoError("Missing name+revision".to_string()))
                 .map(str::parse)??;

        let key: Self::Crypto =
            lines.nth(1) // skip a blank line!
                      .ok_or_else(|| Error::CryptoError("Missing key material".to_string()))
                      .map(str::trim)
                      .map(base64::decode)?
                      .map_err(|_| Error::CryptoError("Invalid base64 key material".to_string()))
                      .map(|b| Self::Crypto::from_slice(&b))?
                      .ok_or_else(|| {
                          Error::CryptoError(format!("Could not parse bytes as key for {}",
                                                     named_revision))
                      })?;

        let (name, revision) = named_revision.into();
        Ok((name, revision, key))
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedRevision {
    name:     String,
    revision: KeyRevision,
}

impl NamedRevision {
    pub fn new(name: String, revision: KeyRevision) -> Self { NamedRevision { name, revision } }

    pub fn name_as_str(&self) -> &str { &self.name }

    pub fn revision_as_str(&self) -> &str { &self.revision }

    /// Returns the name of the file that would store the key of type `K`
    /// identified by this name and revision.
    ///
    /// Note: this is only the filename, not the path to that file in
    /// the key cache.
    pub fn filename<K>(&self) -> PathBuf
        where K: Key
    {
        // **DO NOT** use PathBuf::with_extension here, because it fails
        // with service keys (whose name is like "core.redis@chef");
        // `with_extension` will chop off the <group>@<org> portion of
        // that string!
        PathBuf::from(format!("{}.{}", self.to_string(), K::extension()))
    }
}

impl FromStr for NamedRevision {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let caps = match NAME_WITH_REV_RE.captures(value) {
            Some(c) => c,
            None => {
                let msg = format!("Cannot parse named revision '{}'", value);
                return Err(Error::CryptoError(msg));
            }
        };
        let name = match caps.name("name") {
            Some(r) => r.as_str().to_string(),
            None => {
                let msg = format!("Cannot parse name from '{}'", value);
                return Err(Error::CryptoError(msg));
            }
        };
        let revision = match caps.name("rev") {
            // TODO (CM): This is a bit of an awkward constructor at the
            // moment, but we'll allow it as we've already validated this
            // with the larger regex. Eventually we should harmonize all
            // this a bit more.
            Some(r) => KeyRevision(r.as_str().to_string()),
            None => {
                let msg = format!("Cannot parse revision from '{}'", value);
                return Err(Error::CryptoError(msg));
            }
        };

        Ok(NamedRevision { name, revision })
    }
}

impl fmt::Display for NamedRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.revision)
    }
}

// Think of this as a handy deconstructor. We could make the fields
// public, but that would also allow unrestricted construction of the
// struct, which I don't want right now.
impl Into<(String, KeyRevision)> for NamedRevision {
    fn into(self) -> (String, KeyRevision) { (self.name, self.revision) }
}

////////////////////////////////////////////////////////////////////////

pub struct HabitatKey {
    pair_type:     PairType, // NOT A PAIR!!!!!!
    name_with_rev: String,
    // revision:      String,
    key_bytes:     Vec<u8>,
}

impl HabitatKey {
    pub fn pair_type(&self) -> PairType { self.pair_type }

    pub fn name_with_rev(&self) -> String { self.name_with_rev.clone() }
}

impl AsRef<[u8]> for HabitatKey {
    /// Return a reference to the bytes of the underlying key material.
    fn as_ref(&self) -> &[u8] { self.key_bytes.as_ref() }
}

impl FromStr for HabitatKey {
    type Err = Error;

    /// Parses a string slice of a public or secret signature key.
    ///
    /// The return valid is a tuple consisting of:
    ///   `(PairType, name_with_rev::String, key_body::String)`
    ///
    /// # Examples
    ///
    /// With a public key:
    ///
    /// ```
    /// extern crate habitat_core;
    ///
    /// use habitat_core::crypto::keys::{HabitatKey,
    ///                                  PairType};
    ///
    /// let content = "SIG-PUB-1
    /// unicorn-20160517220007
    ///
    /// J+FGYVKgragA+dzQHCGORd2oLwCc2EvAnT9roz9BJh0=";
    /// let key: HabitatKey = content.parse().unwrap();
    /// assert_eq!(key.pair_type(), PairType::Public);
    /// assert_eq!(key.name_with_rev(), "unicorn-20160517220007");
    /// ```
    ///
    /// With a secret key:
    ///
    /// ```
    /// extern crate habitat_core;
    ///
    /// use habitat_core::crypto::keys::{HabitatKey,
    ///                                  PairType};
    ///
    /// let content = "SIG-SEC-1
    /// unicorn-20160517220007
    ///
    /// jjQaaphB5+CHw7QzDWqMMuwhWmrrHH+SzQAgRrHfQ8sn4UZhUqCtqAD53NAcIY5F3agvAJzYS8CdP2ujP0EmHQ==";
    ///
    /// let key: HabitatKey = content.parse().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * If there is a key version mismatch
    /// * If the key version is missing
    /// * If the key name with revision is missing
    /// * If the key value (the Bas64 payload) is missing
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let pair_type = match lines.next() {
            Some(val) => {
                match val {
                    PUBLIC_SIG_KEY_VERSION | PUBLIC_BOX_KEY_VERSION => PairType::Public,
                    SECRET_SIG_KEY_VERSION | SECRET_BOX_KEY_VERSION | SECRET_SYM_KEY_VERSION => {
                        PairType::Secret
                    }
                    _ => {
                        return Err(Error::CryptoError(format!("Unsupported key version: {}",
                                                              val)));
                    }
                }
            }
            None => {
                let msg = format!("Empty key string:\n({})", s);
                return Err(Error::CryptoError(msg));
            }
        };
        let name_with_rev = lines.next().ok_or_else(|| {
                                             let msg = format!("Malformed key string (missing \
                                                                name+revision):\n({})",
                                                               s);
                                             Error::CryptoError(msg)
                                         })?;

        match lines.nth(1) {
            Some(val) => {
                let key_bytes = base64::decode(val.trim()).map_err(|_| {
                                    Error::CryptoError(format!("Malformed key string (invalid \
                                                                base64 key material):\n({})",
                                                               s))
                                })?;
                Ok(HabitatKey { pair_type,
                                name_with_rev: name_with_rev.to_string(),
                                key_bytes })
            }
            None => {
                let msg = format!("Malformed key string (missing key material):\n({})", s);
                Err(Error::CryptoError(msg))
            }
        }
    }
}

// TODO (CM): This is only needed for Builder
#[deprecated(note = "Please HabitatKey::from_str instead")]
pub fn parse_key_str(content: &str) -> Result<(PairType, String, String)> {
    let key: HabitatKey = content.parse()?;
    Ok((key.pair_type, key.name_with_rev, base64::encode(key.key_bytes)))
}

impl TryFrom<&Path> for HabitatKey {
    type Error = Error;

    fn try_from(value: &Path) -> std::result::Result<Self, Self::Error> {
        Ok(fs::read_to_string(value)?.parse()?)
    }
}

impl TryFrom<&PathBuf> for HabitatKey {
    type Error = Error;

    fn try_from(value: &PathBuf) -> std::result::Result<Self, Self::Error> {
        HabitatKey::try_from(value.as_path())
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyRevision(String);

impl KeyRevision {
    /// Generates a revision string in the form:
    /// `{year}{month}{day}{hour24}{minute}{second}`
    /// Timestamps are in UTC time.
    pub fn new() -> KeyRevision { KeyRevision(Utc::now().format("%Y%m%d%H%M%S").to_string()) }
}

// TODO (CM): Need some tests that assert that this format string and
// the regex for parsing revisions from filenames are mutually
// consistent.

impl fmt::Display for KeyRevision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) }
}

// Only used for the polymorphic `KeyPair::rev` implementation so Builder can
// use things before adopting the KeyRevison type itself.
impl From<KeyRevision> for String {
    fn from(rev: KeyRevision) -> String { rev.to_string() }
}

// As a "newtype", KeyRevision can be thought of as a kind of "smart
// container", and thus we can implement Deref for it with our heads
// held high.
impl Deref for KeyRevision {
    type Target = str;

    fn deref(&self) -> &str { self.0.as_str() }
}

#[cfg(test)]
impl KeyRevision {
    /// Unchecked constructor for testing purposes only; assumes the
    /// string being passed in is valid.
    pub(crate) fn unchecked<R>(rev: R) -> KeyRevision
        where R: AsRef<str>
    {
        KeyRevision(rev.as_ref().to_string())
    }
}

////////////////////////////////////////////////////////////////////////

/// A pair of related keys (public and secret) which have a name and revision.
///
/// Depending on the type of keypair, the public key may be empty or not apply, or one or both of
/// the keys may not be present due to the loading context. For example, the act of verifying a
/// signed message or artifact only requires the public key to be present, whereas the act of
/// signing will require the secret key to be present.
#[derive(Clone, PartialEq)]
pub struct KeyPair<P: PartialEq, S: PartialEq> {
    /// The name of the key, ex: "habitat"
    name:     String,
    /// The revision of the key, which is a timestamp, ex: "201604051449"
    // TODO (CM): This needs to be public for Builder (for now)
    #[deprecated(note = "Use KeyPair::rev() function instead")]
    pub rev:  String,
    revision: KeyRevision,
    /// The public key component, if relevant
    public:   Option<P>,
    /// The private key component, if relevant
    secret:   Option<S>,
}

impl<P: PartialEq, S: PartialEq> KeyPair<P, S> {
    /// Creates a new `KeyPair`.
    pub fn new(name: String,
               rev: KeyRevision,
               public: Option<P>,
               secret: Option<S>)
               -> KeyPair<P, S> {
        #[allow(deprecated)] // for our own internal use of the rev field
        KeyPair { name,
                  rev: rev.to_string(),
                  revision: rev,
                  public,
                  secret }
    }

    /// Returns a `String` containing the combination of the `name` and `rev` fields.
    pub fn name_with_rev(&self) -> String { format!("{}-{}", self.name, self.revision) }

    // TODO (CM): Only polymorphic for Builder right now; eventually,
    // return only KeyRevision from this (if even needed at all; once
    // Builder gets some refactorings, I think we can get rid of this
    // entirely).
    pub fn rev<T>(&self) -> T
        where T: From<KeyRevision>
    {
        self.revision.clone().into()
    }

    pub fn public(&self) -> Result<&P> {
        self.public.as_ref().ok_or_else(|| {
                                let msg = format!("Public key is required but not present for {}",
                                                  self.name_with_rev());
                                Error::CryptoError(msg)
                            })
    }

    pub fn secret(&self) -> Result<&S> {
        self.secret.as_ref().ok_or_else(|| {
                                let msg = format!("Secret key is required but not present for {}",
                                                  self.name_with_rev());
                                Error::CryptoError(msg)
                            })
    }
}

////////////////////////////////////////////////////////////////////////

/// If a key "belongs" to a filename revision, then add the full stem of the
/// file (without path, without .suffix) to the set. This function doesn't
/// return an error on a "bad" file, the bad file key name just doesn't get
/// added to the set.
fn check_filename(keyname: &str,
                  filename: &str,
                  candidates: &mut HashSet<String>,
                  pair_type: Option<PairType>) {
    let caps = match KEYFILE_RE.captures(&filename) {
        Some(c) => c,
        None => {
            debug!("check_filename: Cannot parse {}", &filename);
            return;
        }
    };
    let name = match caps.name("name") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse name from {}", &filename);
            return;
        }
    };

    let rev = match caps.name("rev") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse rev from {}", &filename);
            return;
        }
    };

    let suffix = match caps.name("suffix") {
        Some(r) => r.as_str(),
        None => {
            debug!("check_filename: Cannot parse suffix from {}", &filename);
            return;
        }
    };

    if suffix == PUBLIC_KEY_SUFFIX
       || suffix == SECRET_SIG_KEY_SUFFIX
       || suffix == SECRET_BOX_KEY_SUFFIX
       || suffix == SECRET_SYM_KEY_SUFFIX
    {
        debug!("valid key suffix");
    } else {
        debug!("check_filename: Invalid key suffix from {}", &filename);
        return;
    };

    if name == keyname {
        let thiskey = format!("{}-{}", name, rev);

        let do_insert = match pair_type {
            Some(PairType::Secret) => {
                suffix == SECRET_SIG_KEY_SUFFIX
                || suffix == SECRET_BOX_KEY_SUFFIX
                || suffix == SECRET_SYM_KEY_SUFFIX
            }
            Some(PairType::Public) => suffix == PUBLIC_KEY_SUFFIX,
            None => true,
        };

        if do_insert {
            candidates.insert(thiskey);
        }
    }
}

/// Take a key name (ex "habitat"), and find all revisions of that
/// keyname in the `cache_key_path`.
fn get_key_revisions<P>(keyname: &str,
                        cache_key_path: P,
                        pair_type: Option<PairType>,
                        key_type: KeyType)
                        -> Result<Vec<String>>
    where P: AsRef<Path>
{
    // accumulator for files that match
    let mut candidates = HashSet::new();

    let dir_entries = fs::read_dir(cache_key_path.as_ref()).map_err(|e| {
                          Error::CryptoError(format!("Error reading key directory {}: {}",
                                                     cache_key_path.as_ref().display(),
                                                     e))
                      })?;

    for result in dir_entries {
        let dir_entry = result.map_err(|e| {
                                  debug!("Error reading path {}", e);
                                  Error::CryptoError(format!("Error reading key path {}", e))
                              })?;

        // NB: this metadata() call traverses symlinks, which is
        // exactly what we want.
        match dir_entry.path().metadata() {
            Ok(md) => {
                if !md.is_file() {
                    continue;
                }
            }
            Err(e) => {
                debug!("Error checking file metadata {}", e);
                continue;
            }
        };

        match file_is_valid_key_for_type(dir_entry.path(), key_type) {
            Ok(true) => {} // we're good; keep processing
            Ok(false) => continue,
            Err(e) => {
                debug!("Error reading key file: {:?}: {:?}", dir_entry.path(), e);
                continue;
            }
        }

        let filename = match dir_entry.file_name().into_string() {
            Ok(f) => f,
            Err(e) => {
                // filename is still an OsString, so print it as debug output
                debug!("Invalid filename {:?}", e);
                return Err(Error::CryptoError("Invalid filename in key path".to_string()));
            }
        };
        debug!("checking file: {}", &filename);
        check_filename(keyname, &filename, &mut candidates, pair_type);
    }

    let mut candidate_vec = candidates.into_iter().collect::<Vec<String>>();
    candidate_vec.sort();
    candidate_vec.reverse(); // newest key first
    Ok(candidate_vec)
}

/// Attempt to read the file at `path` to see if it is a valid
/// instance of the given `key_type`.
///
/// If the file cannot be read or processed an Error will be
/// returned.
// TODO (CM): It would be better to read the contents of the entire
// file to make sure it's consistent overall, rather than just hitting
// the first line (actually, just the first 3 characters of the first
// line!)
fn file_is_valid_key_for_type<P>(path: P, key_type: KeyType) -> Result<bool>
    where P: AsRef<Path>
{
    let file = File::open(path.as_ref())?;
    if let Some(first_line) = BufReader::new(file).lines().next() {
        if first_line?.starts_with(&key_type.to_string().to_uppercase()) {
            return Ok(true);
        }
    }
    debug!("Invalid key content in {:?} for type {}",
           path.as_ref(),
           key_type);
    Ok(false)
}

fn mk_key_filename<P, S1, S2>(path: P, keyname: S1, suffix: S2) -> PathBuf
    where P: AsRef<Path>,
          S1: AsRef<str>,
          S2: AsRef<str>
{
    path.as_ref()
        .join(format!("{}.{}", keyname.as_ref(), suffix.as_ref()))
}

// TODO (CM): replace with NamedRevision code directly
pub fn parse_name_with_rev<T>(name_with_rev: T) -> Result<(String, KeyRevision)>
    where T: AsRef<str>
{
    Ok(name_with_rev.as_ref().parse::<NamedRevision>()?.into())
}

fn write_keypair_files<P>(public: Option<(P, String)>, secret: Option<(P, String)>) -> Result<()>
    where P: AsRef<Path>
{
    if let Some((public_keyfile, public_content)) = public {
        let public_keyfile = public_keyfile.as_ref();

        if let Some(pk_dir) = public_keyfile.parent() {
            fs::create_dir_all(pk_dir)?;
        } else {
            return Err(Error::BadKeyPath(public_keyfile.to_string_lossy().into_owned()));
        }
        if public_keyfile.exists() {
            return Err(Error::CryptoError(format!("Public keyfile or a \
                                                   directory already exists {}",
                                                  public_keyfile.display())));
        }
        let public_file = File::create(public_keyfile)?;
        let mut public_writer = BufWriter::new(&public_file);
        public_writer.write_all(public_content.as_bytes())?;
        set_permissions(public_keyfile, &DEFAULT_PUBLIC_KEY_PERMISSIONS)?;
    }

    if let Some((secret_keyfile, secret_content)) = secret {
        let secret_keyfile = secret_keyfile.as_ref();

        if let Some(sk_dir) = secret_keyfile.parent() {
            fs::create_dir_all(sk_dir)?;
        } else {
            return Err(Error::BadKeyPath(secret_keyfile.to_string_lossy().into_owned()));
        }
        if secret_keyfile.exists() {
            return Err(Error::CryptoError(format!("Secret keyfile or a \
                                                   directory already exists {}",
                                                  secret_keyfile.display())));
        }
        let secret_file = File::create(secret_keyfile)?;
        let mut secret_writer = BufWriter::new(&secret_file);
        secret_writer.write_all(secret_content.as_bytes())?;
        set_permissions(secret_keyfile, &DEFAULT_SECRET_KEY_PERMISSIONS)?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn set_permissions<T: AsRef<Path>>(path: T, perms: &Permissions) -> Result<()> {
    use crate::util::posix_perm;

    if let Permissions::Explicit(permissions) = perms {
        posix_perm::set_permissions(path.as_ref(), *permissions)?;
    }
    Ok(())
}

#[cfg(windows)]
fn set_permissions<T: AsRef<Path>>(path: T, _perms: &Permissions) -> Result<()> {
    use crate::util::win_perm;

    win_perm::harden_path(path.as_ref())
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;
    use crate::crypto::{keys::{box_key_pair::BoxKeyPair,
                               ring_key::RingKey,
                               sig_key_pair::SigKeyPair},
                        test_support::*};
    use std::{thread,
              time::Duration};
    use tempfile::Builder;

    static VALID_KEY: &str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &str = "\
         44215a3bce23e351a6af359d77131db17a46767de2b88cbb330df162b8cf2ec1";

    mod named_revision {
        use super::*;
        use crate::crypto::keys::{box_key_pair::{OriginPublicEncryptionKey,
                                                 OriginSecretEncryptionKey},
                                  sig_key_pair::{PublicOriginSigningKey,
                                                 SecretOriginSigningKey},
                                  ServicePublicEncryptionKey,
                                  ServiceSecretEncryptionKey,
                                  UserPublicEncryptionKey,
                                  UserSecretEncryptionKey};
        #[test]
        fn parse_valid_named_revisions() {
            let result: NamedRevision = "foo-20160504220722".parse().unwrap();
            assert_eq!("foo", result.name);
            assert_eq!(KeyRevision::unchecked("20160504220722"), result.revision);

            let result: NamedRevision = "foo-stuff-20160504220722".parse().unwrap();
            assert_eq!("foo-stuff", result.name);
            assert_eq!(KeyRevision::unchecked("20160504220722"), result.revision);
        }

        #[test]
        fn parse_invalid_named_revisions() {
            let result = "barf".parse::<NamedRevision>();
            assert!(result.is_err());

            let result = "barf-20160504220722-wheeeeeeeee".parse::<NamedRevision>();
            assert!(result.is_err());

            let result = "barf-123".parse::<NamedRevision>();
            assert!(result.is_err());
        }

        #[test]
        fn to_string() {
            let nr = NamedRevision { name:     "foo".to_string(),
                                     revision: KeyRevision::unchecked("20160504220722"), };
            assert_eq!(nr.to_string(), "foo-20160504220722");
        }

        /// These key names have a different structure!
        #[test]
        fn to_string_for_service_key_names() {
            let nr = NamedRevision { name:     "core.redis@chef".to_string(),
                                     revision: KeyRevision::unchecked("20160504220722"), };
            assert_eq!(nr.to_string(), "core.redis@chef-20160504220722");
        }

        #[test]
        fn string_roundtrip() {
            let input = "foo-20160504220722";
            assert_eq!(input.parse::<NamedRevision>().unwrap().to_string(), input);
        }

        #[test]
        fn as_path_testing() {
            let source = "foo-20160504220722".parse::<NamedRevision>().unwrap();
            let service_source = "redis.default@chef-20160504220722".parse::<NamedRevision>()
                                                                    .unwrap();

            assert_eq!(source.filename::<RingKey>(),
                       PathBuf::from("foo-20160504220722.sym.key"));

            assert_eq!(source.filename::<PublicOriginSigningKey>(),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(source.filename::<SecretOriginSigningKey>(),
                       PathBuf::from("foo-20160504220722.sig.key"));

            assert_eq!(source.filename::<UserPublicEncryptionKey>(),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(source.filename::<UserSecretEncryptionKey>(),
                       PathBuf::from("foo-20160504220722.box.key"));

            assert_eq!(source.filename::<OriginPublicEncryptionKey>(),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(source.filename::<OriginSecretEncryptionKey>(),
                       PathBuf::from("foo-20160504220722.box.key"));

            assert_eq!(service_source.filename::<ServicePublicEncryptionKey>(),
                       PathBuf::from("redis.default@chef-20160504220722.pub"));
            assert_eq!(service_source.filename::<ServiceSecretEncryptionKey>(),
                       PathBuf::from("redis.default@chef-20160504220722.box.key"));

            // NOTE: Nothing yet explicitly prevents a named revision
            // that does not really belong to a service key from being
            // pathed as though it were.
            assert_eq!(source.filename::<ServicePublicEncryptionKey>(),
                       PathBuf::from("foo-20160504220722.pub"));
            assert_eq!(source.filename::<ServiceSecretEncryptionKey>(),
                       PathBuf::from("foo-20160504220722.box.key"));
        }
    }

    mod tmpkeyfile {
        use super::*;

        #[test]
        fn tmp_keyfile_delete_on_drop() {
            let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
            let path = cache.path().join("mykey");

            {
                let tmp_keyfile = TmpKeyfile { path: path.clone() };
                File::create(&tmp_keyfile.path).unwrap();
                assert!(tmp_keyfile.path.is_file());
            }
            assert_eq!(path.is_file(), false);
        }

        #[test]
        fn tmp_keyfile_no_file_on_drop() {
            let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
            let path = cache.path().join("mykey");

            {
                let tmp_keyfile = TmpKeyfile { path: path.clone() };
                assert_eq!(tmp_keyfile.path.is_file(), false);
            }
            assert_eq!(path.is_file(), false);
        }
    }

    mod habitat_key {
        use super::*;

        // TODO (CM): These tests need to be recast purely in terms of
        // HabitatKey; keeping this here for now, though.
        fn read_key_bytes(keyfile: &Path) -> Result<Vec<u8>> {
            Ok(HabitatKey::try_from(keyfile)?.as_ref().into())
        }

        #[test]
        fn test_read_key_bytes() {
            let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
            let keyfile = cache.path().join(VALID_KEY);
            fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &keyfile).unwrap();
            println!("keyfile {:?}", keyfile);
            let result = HabitatKey::try_from(keyfile.as_path()).unwrap();
            assert_eq!(hex::encode(result.as_ref()), VALID_KEY_AS_HEX);
        }

        #[test]
        #[should_panic(expected = "Empty key string")]
        fn read_key_bytes_empty_file() {
            let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
            let keyfile = cache.path().join("not-much-here");
            let _ = File::create(&keyfile).unwrap();

            read_key_bytes(keyfile.as_path()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Unsupported key version: SOMETHING")]
        fn read_key_bytes_missing_newlines() {
            let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
            let keyfile = cache.path().join("missing-newlines");
            let mut f = File::create(&keyfile).unwrap();
            f.write_all(b"SOMETHING\nELSE\n").unwrap();

            read_key_bytes(keyfile.as_path()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Unsupported key version: header")]
        fn read_key_bytes_malformed_base64() {
            let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
            let keyfile = cache.path().join("missing-newlines");
            let mut f = File::create(&keyfile).unwrap();
            f.write_all(b"header\nsomething\n\nI am not base64 content")
             .unwrap();

            read_key_bytes(keyfile.as_path()).unwrap();
        }
    }

    #[test]
    fn parse_name_with_rev() {
        let (name, rev) = super::parse_name_with_rev("an-origin-19690114010203").unwrap();
        assert_eq!(name, "an-origin");
        assert_eq!(rev, KeyRevision::unchecked("19690114010203"));

        let (name, rev) = super::parse_name_with_rev("user-19480531051223").unwrap();
        assert_eq!(name, "user");
        assert_eq!(rev, KeyRevision::unchecked("19480531051223"));

        let (name, rev) = super::parse_name_with_rev("tnt.default@acme-19480531051223").unwrap();
        assert_eq!(name, "tnt.default@acme");
        assert_eq!(rev, KeyRevision::unchecked("19480531051223"));

        let (name, rev) = super::parse_name_with_rev("--20160420042001").unwrap();
        assert_eq!(name, "-");
        assert_eq!(rev, KeyRevision::unchecked("20160420042001"));
    }

    #[test]
    fn get_key_revisions_can_return_everything() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(Duration::from_millis(1000));
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        let revs = super::get_key_revisions("foo", cache.path(), None, KeyType::Sig).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_key_revisions_can_only_return_keys_of_specified_type() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        let revs = super::get_key_revisions("foo", cache.path(), None, KeyType::Sig).unwrap();
        assert_eq!(1, revs.len());
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(Duration::from_millis(1000));
        let pair = BoxKeyPair::generate_pair_for_user("foo-user");
        pair.unwrap().to_pair_files(cache.path()).unwrap();
        let revs = super::get_key_revisions("foo-user", cache.path(), None, KeyType::Sig).unwrap();
        assert_eq!(0, revs.len());
    }

    #[test]
    fn get_key_revisions_can_return_secret_keys() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(Duration::from_millis(1000));
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        let revs = super::get_key_revisions("foo",
                                            cache.path(),
                                            Some(PairType::Secret),
                                            KeyType::Sig).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_key_revisions_can_return_public_keys() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        // we need to wait at least 1 second between generating keypairs to ensure uniqueness
        thread::sleep(Duration::from_millis(1000));
        SigKeyPair::generate_pair_for_origin("foo").to_pair_files(cache.path())
                                                   .unwrap();
        let revs = super::get_key_revisions("foo",
                                            cache.path(),
                                            Some(PairType::Public),
                                            KeyType::Sig).unwrap();
        assert_eq!(2, revs.len());
    }

    #[test]
    fn get_user_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();
        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = BoxKeyPair::generate_pair_for_user("wecoyote")?;
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }
        BoxKeyPair::generate_pair_for_user("wecoyote-foo").unwrap()
                                                          .to_pair_files(cache.path())
                                                          .unwrap();

        // we shouldn't see wecoyote-foo as a 4th revision
        let revisions =
            super::get_key_revisions("wecoyote", cache.path(), None, KeyType::Box).unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("wecoyote-foo", cache.path(), None, KeyType::Box).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_service_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = BoxKeyPair::generate_pair_for_service("acme", "tnt.default")?;
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }

        BoxKeyPair::generate_pair_for_service("acyou", "tnt.default").unwrap()
                                                                     .to_pair_files(cache.path())
                                                                     .unwrap();

        let revisions =
            super::get_key_revisions("tnt.default@acme", cache.path(), None, KeyType::Box).unwrap();
        assert_eq!(3, revisions.len());

        let revisions = super::get_key_revisions("tnt.default@acyou",
                                                 cache.path(),
                                                 None,
                                                 KeyType::Box).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_ring_key_revisions() {
        let (cache, dir) = new_cache();

        for _ in 0..3 {
            let key = RingKey::new("acme");
            cache.write_key(&key).unwrap();
            wait_1_sec();
        }

        let key = RingKey::new("acme-you");
        cache.write_key(&key).unwrap();

        let revisions = super::get_key_revisions("acme", dir.path(), None, KeyType::Sym).unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("acme-you", dir.path(), None, KeyType::Sym).unwrap();
        assert_eq!(1, revisions.len());
    }

    #[test]
    fn get_origin_key_revisions() {
        let cache = Builder::new().prefix("key_cache").tempdir().unwrap();

        for _ in 0..3 {
            wait_until_ok(|| {
                let pair = SigKeyPair::generate_pair_for_origin("mutants");
                pair.to_pair_files(cache.path())?;
                Ok(())
            });
        }

        SigKeyPair::generate_pair_for_origin("mutants-x").to_pair_files(cache.path())
                                                         .unwrap();

        let revisions =
            super::get_key_revisions("mutants", cache.path(), None, KeyType::Sig).unwrap();
        assert_eq!(3, revisions.len());

        let revisions =
            super::get_key_revisions("mutants-x", cache.path(), None, KeyType::Sig).unwrap();
        assert_eq!(1, revisions.len());
    }

    /// Keys should be able to be symlinks, not just normal
    /// files. This is particularly important in environments like
    /// Kubernetes that rely heavily on symlinks.
    ///
    /// See https://github.com/habitat-sh/habitat/issues/2939
    #[test]
    fn keys_that_are_symlinks_can_still_be_found() {
        let temp_dir = Builder::new().prefix("symlinks_are_ok").tempdir().unwrap();
        let key = RingKey::new("symlinks_are_ok");

        let key_name = format!("{}.sym.key", key.name_with_rev());
        let key_path = temp_dir.path().join(&key_name);
        fs::write(&key_path, key.to_key_string()).unwrap();

        // Create a directory in our temp directory; this will serve
        // as the cache directory in which we look for keys.
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir(&cache_dir).expect("Could not create cache_dir");

        // Create a symlink to the key INTO that new dir
        let dest = cache_dir.join(&key_name);
        symlink_file(&key_path, &dest).expect("Could not generate symlink");

        // For sanity, confirm that we are indeed dealing with a symlink
        let sym_meta = dest.symlink_metadata()
                           .expect("Could not get file metadata");
        assert!(sym_meta.file_type().is_symlink());

        let revisions =
            super::get_key_revisions("symlinks_are_ok",
                                     &cache_dir, // <-- THIS IS THE KEY PART OF THE TEST
                                     None,
                                     KeyType::Sym).expect("Could not fetch key revisions!");

        assert_eq!(1, revisions.len());
        assert_eq!(revisions[0], key.name_with_rev());
    }

    // Windows and Linux platforms handle symlinking differently; this
    // abstracts that for the purposes of our tests here.
    #[cfg(target_os = "windows")]
    fn symlink_file<P, Q>(src: P, dest: Q) -> ::std::io::Result<()>
        where P: AsRef<Path>,
              Q: AsRef<Path>
    {
        ::std::os::windows::fs::symlink_file(src.as_ref(), dest.as_ref())
    }

    #[cfg(not(target_os = "windows"))]
    fn symlink_file<P, Q>(src: P, dest: Q) -> ::std::io::Result<()>
        where P: AsRef<Path>,
              Q: AsRef<Path>
    {
        ::std::os::unix::fs::symlink(src.as_ref(), dest.as_ref())
    }

    #[test]
    fn check_filename_for_secret_keys() {
        // only look for secret keys
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              Some(PairType::Secret));
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.pub",
                              &mut candidates,
                              Some(PairType::Secret));
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.sig.key",
                              &mut candidates,
                              Some(PairType::Secret));
        assert_eq!(1, candidates.len());
    }

    #[test]
    fn check_filename_for_public_keys() {
        // only look for public keys
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              Some(PairType::Public));
        super::check_filename("wecoyote",
                              "wecoyote-20160519203611.pub",
                              &mut candidates,
                              Some(PairType::Public));
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.sig.key",
                              &mut candidates,
                              Some(PairType::Public));
        assert_eq!(2, candidates.len());
    }

    #[test]
    fn check_filename_key_without_dash() {
        // look for a keyname that doesn't include a dash
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote",
                              "wecoyote-20160519203610.box.key",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote",
                              "wecoyote-foo-20160519203610.box.key",
                              &mut candidates,
                              None);
        assert_eq!(1, candidates.len());
    }

    #[test]
    fn check_filename_key_with_dash() {
        // look for a keyname that includes a dash
        let mut candidates = HashSet::new();
        super::check_filename("wecoyote-foo",
                              "wecoyote-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote-foo",
                              "wecoyote-foo-20160519203610.pub",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote-foo",
                              "wecoyote-20160519203610.box.key",
                              &mut candidates,
                              None);
        super::check_filename("wecoyote-foo",
                              "wecoyote-foo-20160519203610.box.key",
                              &mut candidates,
                              None);
        assert_eq!(1, candidates.len());
    }

    fn create_file_with_content(content: &[u8]) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().expect("couldn't generate tempfile");
        file.write_all(content)
            .expect("couldn't write content to file");
        file
    }

    /// Creates a file with the content of the secret key of the given
    /// type.
    // TODO (CM): Doesn't currently generate public keys, or all the
    // possible varieties of the various key types (e.g., service
    // keys).
    fn key_file(key_type: KeyType, name: &str) -> tempfile::NamedTempFile {
        let content = match key_type {
            KeyType::Sym => RingKey::new(name).to_key_string(),
            KeyType::Sig => {
                SigKeyPair::generate_pair_for_origin(name).to_secret_string()
                                                          .unwrap()
            }
            KeyType::Box => {
                BoxKeyPair::generate_pair_for_user(name).unwrap()
                                                        .to_secret_string()
                                                        .unwrap()
            }
        };
        create_file_with_content(content.as_bytes())
    }

    #[test]
    fn test_file_is_valid_key_for_type() {
        let file = key_file(KeyType::Sym, "foo");
        assert!(super::file_is_valid_key_for_type(file.path(), KeyType::Sym).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sig).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Box).unwrap());

        let file = key_file(KeyType::Sig, "foo");
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sym).unwrap());
        assert!(super::file_is_valid_key_for_type(file.path(), KeyType::Sig).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Box).unwrap());

        let file = key_file(KeyType::Box, "foo");
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sym).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sig).unwrap());
        assert!(super::file_is_valid_key_for_type(file.path(), KeyType::Box).unwrap());
    }

    #[test]
    fn test_file_is_valid_key_for_type_with_bogus_content() {
        let file = create_file_with_content(b"LOLWUT-NOT-A-KEY\nNOPE\n\nGO AWAY");
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sym).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Box).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sig).unwrap());

        let file = create_file_with_content(b"");
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sym).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Box).unwrap());
        assert!(!super::file_is_valid_key_for_type(file.path(), KeyType::Sig).unwrap());
    }
}
