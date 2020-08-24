use super::{super::{hash,
                    PUBLIC_KEY_SUFFIX,
                    PUBLIC_SIG_KEY_VERSION,
                    SECRET_SIG_KEY_SUFFIX,
                    SECRET_SIG_KEY_VERSION},
            get_key_revisions,
            mk_key_filename,
            parse_name_with_rev,
            write_keypair_files,
            HabitatKey,
            KeyPair,
            KeyRevision,
            KeyType,
            PairType,
            TmpKeyfile};
use crate::error::{Error,
                   Result};
use sodiumoxide::{crypto::{sign,
                           sign::ed25519::{PublicKey as SigPublicKey,
                                           SecretKey as SigSecretKey}},
                  randombytes::randombytes};
use std::{convert::TryFrom,
          fs,
          path::{Path,
                 PathBuf}};

#[deprecated(note = "Use new key types instead")]
pub type SigKeyPair = KeyPair<SigPublicKey, SigSecretKey>;

impl SigKeyPair {
    pub fn generate_pair_for_origin(name: &str) -> Self {
        let revision = KeyRevision::new();
        let (pk, sk) = sign::gen_keypair();
        Self::new(name.to_string(), revision, Some(pk), Some(sk))
    }

    /// Return a Vec of origin keys with a given name.
    /// The newest key is listed first in the Vec.
    fn get_pairs_for<P: AsRef<Path> + ?Sized>(name: &str,
                                              cache_key_path: &P,
                                              pair_type: Option<PairType>)
                                              -> Result<Vec<Self>> {
        let revisions = get_key_revisions(name, cache_key_path.as_ref(), pair_type, KeyType::Sig)?;
        debug!("revisions = {:?}", &revisions);
        let mut key_pairs = Vec::new();
        for name_with_rev in &revisions {
            debug!("Attempting to read key name_with_rev {} for {}",
                   name_with_rev, name);
            let kp = Self::get_pair_for(name_with_rev, cache_key_path)?;
            key_pairs.push(kp);
        }
        Ok(key_pairs)
    }

    #[deprecated(note = "Use KeyCache methods instead")]
    pub fn get_pair_for<P: AsRef<Path> + ?Sized>(name_with_rev: &str,
                                                 cache_key_path: &P)
                                                 -> Result<Self> {
        let (name, rev) = parse_name_with_rev(name_with_rev)?;
        let pk = match Self::get_public_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find public key for name_with_rev {}: {}",
                       name_with_rev, e);
                None
            }
        };
        let sk = match Self::get_secret_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!("Can't find secret key for name_with_rev {}: {}",
                       name_with_rev, e);
                None
            }
        };
        if pk == None && sk == None {
            let msg = format!("No public or secret keys found for name_with_rev {}",
                              name_with_rev);
            return Err(Error::CryptoError(msg));
        }
        Ok(SigKeyPair::new(name, rev, pk, sk))
    }

    #[deprecated(note = "Use KeyCache methods instead")]
    pub fn get_latest_pair_for<P: AsRef<Path> + ?Sized>(name: &str,
                                                        cache_key_path: &P,
                                                        pair_type: Option<PairType>)
                                                        -> Result<Self> {
        let mut all = Self::get_pairs_for(name, cache_key_path, pair_type)?;
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} sig key", name);
                Err(Error::CryptoError(msg))
            }
            _ => Ok(all.remove(0)),
        }
    }

    #[deprecated]
    pub fn get_public_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str,
                                                        cache_key_path: &P)
                                                        -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, PUBLIC_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(format!("No public key found at {}", path.display())));
        }
        Ok(path)
    }

    #[deprecated]
    pub fn get_secret_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str,
                                                        cache_key_path: &P)
                                                        -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, SECRET_SIG_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(format!("No secret key found at {}", path.display())));
        }
        Ok(path)
    }

    #[deprecated(note = "Use new key types instead")]
    pub fn write_file_from_str<P: AsRef<Path> + ?Sized>(content: &str,
                                                        cache_key_path: &P)
                                                        -> Result<(Self, PairType)> {
        let key: HabitatKey = content.parse()?;
        let pair_type = key.pair_type();
        let name_with_rev = key.name_with_rev();

        let suffix = match pair_type {
            PairType::Public => PUBLIC_KEY_SUFFIX,
            PairType::Secret => SECRET_SIG_KEY_SUFFIX,
        };
        let keyfile = mk_key_filename(cache_key_path.as_ref(), &name_with_rev, &suffix);
        let tmpfile = {
            let mut t = keyfile.clone();
            t.set_file_name(format!("{}.{}",
                                    &keyfile.file_name().unwrap().to_str().unwrap(),
                                    &hex::encode(randombytes(6).as_slice())));
            TmpKeyfile { path: t }
        };

        debug!("Writing temp key file {}", tmpfile.path.display());
        match pair_type {
            PairType::Public => {
                write_keypair_files(Some((&tmpfile.path, content.to_string())), None)?;
            }
            PairType::Secret => {
                write_keypair_files(None, Some((&tmpfile.path, content.to_string())))?;
            }
        }

        if Path::new(&keyfile).is_file() {
            let existing_hash = hash::hash_file(&keyfile)?;
            let new_hash = hash::hash_file(&tmpfile.path)?;
            if existing_hash != new_hash {
                let msg = format!("Existing key file {} found but new version hash is different, \
                                   failing to write new file over existing. ({} = {}, {} = {})",
                                  keyfile.display(),
                                  keyfile.display(),
                                  existing_hash,
                                  tmpfile.path.display(),
                                  new_hash);
                return Err(Error::CryptoError(msg));
            } else {
                // Otherwise, hashes match and we can skip writing over the existing file
                debug!("New content hash matches existing file {} hash, removing temp key file \
                        {}.",
                       keyfile.display(),
                       tmpfile.path.display());
                fs::remove_file(&tmpfile.path)?;
            }
        } else {
            fs::rename(&tmpfile.path, keyfile)?;
        }
        Ok((Self::get_pair_for(&name_with_rev, cache_key_path)?, pair_type))
    }

    #[deprecated(note = "Use new key types instead")]
    pub fn to_public_string(&self) -> Result<String> {
        match self.public {
            Some(pk) => {
                Ok(format!("{}\n{}\n\n{}",
                           PUBLIC_SIG_KEY_VERSION,
                           self.name_with_rev(),
                           &base64::encode(&pk[..])))
            }
            None => {
                Err(Error::CryptoError(format!("No public key present for {}",
                                               self.name_with_rev())))
            }
        }
    }

    #[deprecated(note = "Use new key types instead")]
    pub fn to_secret_string(&self) -> Result<String> {
        match self.secret {
            Some(ref sk) => {
                Ok(format!("{}\n{}\n\n{}",
                           SECRET_SIG_KEY_VERSION,
                           self.name_with_rev(),
                           &base64::encode(&sk[..])))
            }
            None => {
                Err(Error::CryptoError(format!("No secret key present for {}",
                                               self.name_with_rev())))
            }
        }
    }

    #[deprecated(note = "Use KeyCache methods instead")]
    pub fn to_pair_files<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<()> {
        let public_keyfile = mk_key_filename(path, self.name_with_rev(), PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(path, self.name_with_rev(), SECRET_SIG_KEY_SUFFIX);
        debug!("public sig keyfile = {}", public_keyfile.display());
        debug!("secret sig keyfile = {}", secret_keyfile.display());

        write_keypair_files(Some((&public_keyfile, self.to_public_string()?)),
                            Some((&secret_keyfile, self.to_secret_string()?)))
    }

    fn get_public_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SigPublicKey> {
        let public_keyfile = mk_key_filename(cache_key_path, key_with_rev, PUBLIC_KEY_SUFFIX);
        match SigPublicKey::from_slice(HabitatKey::try_from(&public_keyfile)?.as_ref()) {
            Some(sk) => Ok(sk),
            None => {
                Err(Error::CryptoError(format!("Can't read sig public key \
                                                for {}",
                                               key_with_rev)))
            }
        }
    }

    fn get_secret_key(key_with_rev: &str, cache_key_path: &Path) -> Result<SigSecretKey> {
        let secret_keyfile = mk_key_filename(cache_key_path, key_with_rev, SECRET_SIG_KEY_SUFFIX);
        match SigSecretKey::from_slice(HabitatKey::try_from(&secret_keyfile)?.as_ref()) {
            Some(sk) => Ok(sk),
            None => {
                Err(Error::CryptoError(format!("Can't read sig secret key \
                                                for {}",
                                               key_with_rev)))
            }
        }
    }
}
