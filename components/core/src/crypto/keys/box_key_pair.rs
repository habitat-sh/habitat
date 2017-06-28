// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path, PathBuf};
use std::str;

use base64;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey as BoxPublicKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey as BoxSecretKey;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, gen_nonce};

use error::{Error, Result};
use super::{get_key_revisions, mk_key_filename, mk_revision_string, parse_name_with_rev,
            read_key_bytes, write_keypair_files, KeyPair, KeyType};
use super::super::{BOX_FORMAT_VERSION, PUBLIC_KEY_SUFFIX, SECRET_BOX_KEY_SUFFIX};

pub type BoxKeyPair = KeyPair<BoxPublicKey, BoxSecretKey>;

impl BoxKeyPair {
    pub fn generate_pair_for_service<S1, S2, P>(
        org: S1,
        service_group: S2,
        cache_key_path: P,
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        P: AsRef<Path>,
    {
        let revision = try!(mk_revision_string());
        let keyname =
            Self::mk_key_name_for_service(org.as_ref(), service_group.as_ref(), &revision);
        debug!("new service box key name = {}", &keyname);
        let (public_key, secret_key) =
            try!(Self::generate_pair_files(&keyname, cache_key_path.as_ref()));
        let (name, _) = try!(parse_name_with_rev(&keyname));
        Ok(Self::new(
            name,
            revision,
            Some(public_key),
            Some(secret_key),
        ))
    }

    pub fn generate_pair_for_user<P: AsRef<Path> + ?Sized>(
        user: &str,
        cache_key_path: &P,
    ) -> Result<Self> {
        let revision = try!(mk_revision_string());
        let keyname = Self::mk_key_name_for_user(user, &revision);
        debug!("new user sig key name = {}", &keyname);
        let (public_key, secret_key) =
            try!(Self::generate_pair_files(&keyname, cache_key_path.as_ref()));
        let (name, _) = try!(parse_name_with_rev(&keyname));
        Ok(Self::new(
            name,
            revision,
            Some(public_key),
            Some(secret_key),
        ))
    }

    pub fn get_pairs_for<T, P>(name: T, cache_key_path: P) -> Result<Vec<Self>>
    where
        T: AsRef<str>,
        P: AsRef<Path>,
    {
        let revisions = try!(get_key_revisions(
            name.as_ref(),
            cache_key_path.as_ref(),
            None,
        ));
        let mut key_pairs = Vec::new();
        for name_with_rev in revisions {
            debug!(
                "Attempting to read key name_with_rev {} for {}",
                name_with_rev,
                name.as_ref()
            );
            let kp = try!(Self::get_pair_for(name_with_rev, cache_key_path.as_ref()));
            key_pairs.push(kp);
        }
        Ok(key_pairs)
    }

    pub fn get_pair_for<T, P>(name_with_rev: T, cache_key_path: P) -> Result<Self>
    where
        T: AsRef<str>,
        P: AsRef<Path>,
    {
        let (name, rev) = try!(parse_name_with_rev(name_with_rev.as_ref()));
        let pk = match Self::get_public_key(name_with_rev.as_ref(), cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                debug!(
                    "Can't find public key for name_with_rev {}: {}",
                    name_with_rev.as_ref(),
                    e
                );
                None
            }
        };
        let sk = match Self::get_secret_key(name_with_rev.as_ref(), cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                debug!(
                    "Can't find secret key for name_with_rev {}: {}",
                    name_with_rev.as_ref(),
                    e
                );
                None
            }
        };
        if pk == None && sk == None {
            let msg = format!(
                "No public or secret keys found for name_with_rev {}",
                name_with_rev.as_ref()
            );
            return Err(Error::CryptoError(msg));
        }
        Ok(Self::new(name, rev, pk, sk))
    }

    pub fn get_latest_pair_for<T, P>(name: T, cache_key_path: P) -> Result<Self>
    where
        T: AsRef<str>,
        P: AsRef<Path>,
    {
        let mut all = try!(Self::get_pairs_for(name.as_ref(), cache_key_path.as_ref()));
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} box key", name.as_ref());
                return Err(Error::CryptoError(msg));
            }
            _ => Ok(all.remove(0)),
        }
    }

    pub fn get_public_key_path<P: AsRef<Path> + ?Sized>(
        key_with_rev: &str,
        cache_key_path: &P,
    ) -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, PUBLIC_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(
                format!("No public key found at {}", path.display()),
            ));
        }
        Ok(path)
    }

    pub fn get_secret_key_path<P: AsRef<Path> + ?Sized>(
        key_with_rev: &str,
        cache_key_path: &P,
    ) -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, SECRET_BOX_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(
                format!("No secret key found at {}", path.display()),
            ));
        }
        Ok(path)
    }

    /// A user can encrypt data with a service as the recipient.
    /// Key names and nonce are embedded in the payload.
    pub fn encrypt(&self, data: &[u8], receiver: &Self) -> Result<Vec<u8>> {
        let nonce = gen_nonce();
        let ciphertext = box_::seal(data, &nonce, try!(receiver.public()), try!(self.secret()));
        let out = format!(
            "{}\n{}\n{}\n{}\n{}",
            BOX_FORMAT_VERSION,
            &self.name_with_rev(),
            &receiver.name_with_rev(),
            base64::encode(&nonce[..]),
            base64::encode(&ciphertext)
        );
        Ok(out.into_bytes())
    }

    /// Decrypt data from a user that was received at a service
    /// Key names are embedded in the message payload which must
    /// be present while decrypting.
    pub fn decrypt<P>(payload: &[u8], cache_key_path: P) -> Result<Vec<u8>>
    where
        P: AsRef<Path>,
    {
        debug!("Decrypt key path = {}", cache_key_path.as_ref().display());
        let mut lines = try!(str::from_utf8(payload)).lines();
        match lines.next() {
            Some(val) => {
                if val != BOX_FORMAT_VERSION {
                    return Err(Error::CryptoError(format!("Unsupported version: {}", val)));
                }
                ()
            }
            None => {
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read file version".to_string(),
                ));
            }
        }
        let sender = match lines.next() {
            Some(val) => try!(Self::get_pair_for(val, cache_key_path.as_ref())),
            None => {
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read sender key name".to_string(),
                ));
            }
        };
        let receiver = match lines.next() {
            Some(val) => try!(Self::get_pair_for(val, cache_key_path.as_ref())),
            None => {
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read receiver key name".to_string(),
                ));
            }
        };
        let nonce = match lines.next() {
            Some(val) => {
                let decoded = try!(base64::decode(val).map_err(|e| {
                    Error::CryptoError(format!("Can't decode nonce: {}", e))
                }));
                match Nonce::from_slice(&decoded) {
                    Some(nonce) => nonce,
                    None => return Err(Error::CryptoError("Invalid size of nonce".to_string())),
                }
            }
            None => {
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read nonce".to_string(),
                ));
            }
        };
        let ciphertext = match lines.next() {
            Some(val) => {
                try!(base64::decode(val).map_err(|e| {
                    Error::CryptoError(format!("Can't decode ciphertext: {}", e))
                }))
            }
            None => {
                return Err(Error::CryptoError(
                    "Corrupt payload, can't read ciphertext".to_string(),
                ));
            }
        };
        match box_::open(
            &ciphertext,
            &nonce,
            try!(sender.public()),
            try!(receiver.secret()),
        ) {
            Ok(v) => Ok(v),
            Err(_) => {
                return Err(Error::CryptoError(
                    "Secret key, public key, and nonce could not \
                                                    decrypt ciphertext"
                        .to_string(),
                ))
            }
        }
    }

    fn generate_pair_files(
        name_with_rev: &str,
        cache_key_path: &Path,
    ) -> Result<(BoxPublicKey, BoxSecretKey)> {
        let (pk, sk) = box_::gen_keypair();

        let public_keyfile = mk_key_filename(cache_key_path, name_with_rev, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(cache_key_path, name_with_rev, SECRET_BOX_KEY_SUFFIX);
        debug!("public box keyfile = {}", public_keyfile.display());
        debug!("secret box keyfile = {}", secret_keyfile.display());

        try!(write_keypair_files(
            KeyType::Box,
            &name_with_rev,
            Some(&public_keyfile),
            Some(&base64::encode(&pk[..]).into_bytes()),
            Some(&secret_keyfile),
            Some(&base64::encode(&sk[..]).into_bytes()),
        ));
        Ok((pk, sk))
    }

    fn get_public_key<T, P>(key_with_rev: T, cache_key_path: P) -> Result<BoxPublicKey>
    where
        T: AsRef<str>,
        P: AsRef<Path>,
    {
        let public_keyfile =
            mk_key_filename(cache_key_path, key_with_rev.as_ref(), PUBLIC_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&public_keyfile));
        match BoxPublicKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!(
                    "Can't read box public key for {}",
                    key_with_rev.as_ref()
                )))
            }
        }
    }

    fn get_secret_key<T, P>(key_with_rev: T, cache_key_path: P) -> Result<BoxSecretKey>
    where
        T: AsRef<str>,
        P: AsRef<Path>,
    {
        let secret_keyfile =
            mk_key_filename(cache_key_path, key_with_rev.as_ref(), SECRET_BOX_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&secret_keyfile));
        match BoxSecretKey::from_slice(&bytes) {
            Some(sk) => Ok(sk),
            None => {
                return Err(Error::CryptoError(format!(
                    "Can't read box secret key for {}",
                    key_with_rev.as_ref()
                )))
            }
        }
    }

    fn mk_key_name_for_service(org: &str, service_group: &str, revision: &str) -> String {
        format!("{}@{}-{}", service_group, org, revision)
    }

    fn mk_key_name_for_user(user: &str, revision: &str) -> String {
        format!("{}-{}", user, revision)
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::str;

    use tempdir::TempDir;

    use super::BoxKeyPair;
    use super::super::super::test_support::*;

    static VALID_KEY: &'static str = "service-key-valid.default@acme-20160509181736.box.key";
    static VALID_PUB: &'static str = "service-key-valid.default@acme-20160509181736.pub";
    static VALID_NAME_WITH_REV: &'static str = "service-key-valid.default@acme-20160509181736";

    #[test]
    fn empty_struct() {
        let pair = BoxKeyPair::new("grohl".to_string(), "201604051449".to_string(), None, None);

        assert_eq!(pair.name, "grohl");
        assert_eq!(pair.rev, "201604051449");
        assert_eq!(pair.name_with_rev(), "grohl-201604051449");

        assert_eq!(pair.public, None);
        match pair.public() {
            Ok(_) => panic!("Empty pair should not have a public key"),
            Err(_) => assert!(true),
        }
        assert_eq!(pair.secret, None);
        match pair.secret() {
            Ok(_) => panic!("Empty pair should not have a secret key"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn generated_service_pair() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        assert_eq!(pair.name, "tnt.default@acme");
        match pair.public() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a public key"),
        }
        match pair.secret() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a secret key"),
        }
        assert!(
            cache
                .path()
                .join(format!("{}.pub", pair.name_with_rev()))
                .exists()
        );
        assert!(
            cache
                .path()
                .join(format!("{}.box.key", pair.name_with_rev()))
                .exists()
        );
    }

    #[test]
    fn generated_user_pair() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();

        assert_eq!(pair.name, "wecoyote");
        match pair.public() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a public key"),
        }
        match pair.secret() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a secret key"),
        }
        assert!(
            cache
                .path()
                .join(format!("{}.pub", pair.name_with_rev()))
                .exists()
        );
        assert!(
            cache
                .path()
                .join(format!("{}.box.key", pair.name_with_rev()))
                .exists()
        );
    }

    #[test]
    fn get_pairs_for() {
        let cache = TempDir::new("key_cache").unwrap();
        let pairs = BoxKeyPair::get_pairs_for("wecoyote", cache.path()).unwrap();
        assert_eq!(pairs.len(), 0);

        let _ = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let pairs = BoxKeyPair::get_pairs_for("wecoyote", cache.path()).unwrap();
        assert_eq!(pairs.len(), 1);

        match wait_until_ok(|| {
            BoxKeyPair::generate_pair_for_user("wecoyote", cache.path())
        }) {
            Some(_) => (),
            None => panic!("Failed to generate another keypair after waiting"),
        };
        let pairs = BoxKeyPair::get_pairs_for("wecoyote", cache.path()).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should not include another named key in the count
        let _ = BoxKeyPair::generate_pair_for_user("roadrunner", cache.path()).unwrap();
        let pairs = BoxKeyPair::get_pairs_for("wecoyote", cache.path()).unwrap();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn get_pair_for() {
        let cache = TempDir::new("key_cache").unwrap();
        let p1 = BoxKeyPair::generate_pair_for_user("web", cache.path()).unwrap();
        let p2 = match wait_until_ok(|| BoxKeyPair::generate_pair_for_user("web", cache.path())) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let p1_fetched = BoxKeyPair::get_pair_for(&p1.name_with_rev(), cache.path()).unwrap();
        assert_eq!(p1.name, p1_fetched.name);
        assert_eq!(p1.rev, p1_fetched.rev);
        let p2_fetched = BoxKeyPair::get_pair_for(&p2.name_with_rev(), cache.path()).unwrap();
        assert_eq!(p2.name, p2_fetched.name);
        assert_eq!(p2.rev, p2_fetched.rev);
    }

    #[test]
    #[should_panic(expected = "No public or secret keys found for")]
    fn get_pair_for_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        BoxKeyPair::get_pair_for("nope-nope-20160405144901", cache.path()).unwrap();
    }

    #[test]
    fn get_latest_pair_for_single() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();

        let latest = BoxKeyPair::get_latest_pair_for("wecoyote", cache.path()).unwrap();
        assert_eq!(latest.name, pair.name);
        assert_eq!(latest.rev, pair.rev);
    }

    #[test]
    fn get_latest_pair_for_multiple() {
        let cache = TempDir::new("key_cache").unwrap();
        let _ = BoxKeyPair::generate_pair_for_user("web", cache.path()).unwrap();
        let p2 = match wait_until_ok(|| BoxKeyPair::generate_pair_for_user("web", cache.path())) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let latest = BoxKeyPair::get_latest_pair_for("web", cache.path()).unwrap();
        assert_eq!(latest.name, p2.name);
        assert_eq!(latest.rev, p2.rev);
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn get_latest_pair_for_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        BoxKeyPair::get_latest_pair_for("nope-nope", cache.path()).unwrap();
    }

    #[test]
    fn get_public_key_path() {
        let cache = TempDir::new("key_cache").unwrap();
        fs::copy(
            fixture(&format!("keys/{}", VALID_PUB)),
            cache.path().join(VALID_PUB),
        ).unwrap();

        let result = BoxKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_PUB));
    }

    #[test]
    #[should_panic(expected = "No public key found at")]
    fn get_public_key_path_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        BoxKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn get_secret_key_path() {
        let cache = TempDir::new("key_cache").unwrap();
        fs::copy(
            fixture(&format!("keys/{}", VALID_KEY)),
            cache.path().join(VALID_KEY),
        ).unwrap();

        let result = BoxKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_KEY));
    }

    #[test]
    #[should_panic(expected = "No secret key found at")]
    fn get_secret_key_path_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        BoxKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn encrypt_and_decrypt_from_user_to_service() {
        let cache = TempDir::new("key_cache").unwrap();
        let service = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();
        let user = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();

        let ciphertext = user.encrypt("I wish to buy more rockets".as_bytes(), &service)
            .unwrap();
        let message = BoxKeyPair::decrypt(&ciphertext, cache.path()).unwrap();
        assert_eq!(message, "I wish to buy more rockets".as_bytes());
    }

    #[test]
    fn encrypt_and_decrypt_from_service_to_user() {
        let cache = TempDir::new("key_cache").unwrap();
        let service = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();
        let user = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();

        let ciphertext = service.encrypt("Out of rockets".as_bytes(), &user).unwrap();
        let message = BoxKeyPair::decrypt(&ciphertext, cache.path()).unwrap();
        assert_eq!(message, "Out of rockets".as_bytes());
    }

    #[test]
    fn encrypt_and_decrypt_minimal_keys() {
        let full_cache = TempDir::new("full_cache").unwrap();
        let sender_cache = TempDir::new("sender_cache").unwrap();
        let receiver_cache = TempDir::new("receiver_cache").unwrap();

        // Generate the keys & prepare the sender and receiver caches with the minimal keys
        // required on each end
        {
            let sender = BoxKeyPair::generate_pair_for_user("wecoyote", full_cache.path()).unwrap();
            let receiver =
                BoxKeyPair::generate_pair_for_service("acme", "tnt.default", full_cache.path())
                    .unwrap();

            // Prepare the sender cache with sender's secret and receiver's public keys
            let secret =
                BoxKeyPair::get_secret_key_path(&sender.name_with_rev(), full_cache.path())
                    .unwrap();
            let public =
                BoxKeyPair::get_public_key_path(&receiver.name_with_rev(), full_cache.path())
                    .unwrap();
            fs::copy(
                &secret,
                sender_cache.path().join(&secret.file_name().unwrap()),
            ).unwrap();
            fs::copy(
                &public,
                sender_cache.path().join(&public.file_name().unwrap()),
            ).unwrap();

            // Prepare the receiver cache with receivers's secret and sender's public keys
            let secret =
                BoxKeyPair::get_secret_key_path(&receiver.name_with_rev(), full_cache.path())
                    .unwrap();
            let public =
                BoxKeyPair::get_public_key_path(&sender.name_with_rev(), full_cache.path())
                    .unwrap();
            fs::copy(
                &secret,
                receiver_cache.path().join(&secret.file_name().unwrap()),
            ).unwrap();
            fs::copy(
                &public,
                receiver_cache.path().join(&public.file_name().unwrap()),
            ).unwrap();

        }

        let ciphertext = {
            // Load the sender and receiver keys from sender cache to encrypt
            let sender = BoxKeyPair::get_latest_pair_for("wecoyote", sender_cache.path()).unwrap();
            let receiver = BoxKeyPair::get_latest_pair_for("tnt.default@acme", sender_cache.path())
                .unwrap();
            sender
                .encrypt("Falling hurts".as_bytes(), &receiver)
                .unwrap()
        };

        // Decrypt unpacks the ciphertext payload to read nonce , determines which secret key to
        // load for the receiver and which public key to load for the sender. We're using the
        // receiver's cache for the decrypt.
        let message = BoxKeyPair::decrypt(&ciphertext, receiver_cache.path()).unwrap();
        assert_eq!(message, "Falling hurts".as_bytes());
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn encrypt_missing_sender_secret_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        // Delete the sender's secret key
        fs::remove_file(
            BoxKeyPair::get_secret_key_path(&sender.name_with_rev(), cache.path()).unwrap(),
        ).unwrap();
        // Now reload the sender's pair which will be missing the secret key
        let sender = BoxKeyPair::get_latest_pair_for("wecoyote", cache.path()).unwrap();

        sender
            .encrypt("not going to happen".as_bytes(), &receiver)
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "Public key is required but not present for")]
    fn encrypt_missing_receiver_public_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        // Delete the receiver's public key
        fs::remove_file(
            BoxKeyPair::get_public_key_path(&receiver.name_with_rev(), cache.path()).unwrap(),
        ).unwrap();
        // Now reload the receiver's pair which will be missing the public key
        let receiver = BoxKeyPair::get_latest_pair_for("tnt.default@acme", cache.path()).unwrap();

        sender
            .encrypt("not going to happen".as_bytes(), &receiver)
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key is required but not present for")]
    fn decrypt_missing_receiver_secret_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        // Delete the receiver's secret key
        fs::remove_file(
            BoxKeyPair::get_secret_key_path(&receiver.name_with_rev(), cache.path()).unwrap(),
        ).unwrap();

        let ciphertext = sender
            .encrypt("problems ahead".as_bytes(), &receiver)
            .unwrap();
        BoxKeyPair::decrypt(&ciphertext, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Public key is required but not present for")]
    fn decrypt_missing_sender_public_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        // Delete the sender's public key
        fs::remove_file(
            BoxKeyPair::get_public_key_path(&sender.name_with_rev(), cache.path()).unwrap(),
        ).unwrap();

        let ciphertext = sender
            .encrypt("problems ahead".as_bytes(), &receiver)
            .unwrap();
        BoxKeyPair::decrypt(&ciphertext, cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn decrypt_empty_sender_key() {
        let cache = TempDir::new("key_cache").unwrap();
        BoxKeyPair::decrypt("BOX-1\n\nuhoh".as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn decrypt_invalid_sender_key() {
        let cache = TempDir::new("key_cache").unwrap();
        BoxKeyPair::decrypt("BOX-1\nnope-nope\nuhoh".as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn decrypt_empty_receiver_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();

        let payload = format!("BOX-1\n{}\n\nuhoh", sender.name_with_rev());
        BoxKeyPair::decrypt(payload.as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_name_with_rev:1 Cannot parse")]
    fn decrypt_invalid_receiver_key() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();

        let payload = format!("BOX-1\n{}\nnope-nope\nuhoh", sender.name_with_rev());
        BoxKeyPair::decrypt(payload.as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t decode nonce")]
    fn decrypt_invalid_nonce_decode() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        let payload = format!(
            "BOX-1\n{}\n{}\nnot:base64",
            sender.name_with_rev(),
            receiver.name_with_rev()
        );
        BoxKeyPair::decrypt(payload.as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid size of nonce")]
    fn decrypt_invalid_nonce() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        let payload = format!(
            "BOX-1\n{}\n{}\nuhoh",
            sender.name_with_rev(),
            receiver.name_with_rev()
        );
        BoxKeyPair::decrypt(payload.as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t decode ciphertext")]
    fn decrypt_invalid_ciphertext_decode() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        let payload = sender
            .encrypt("problems ahead".as_bytes(), &receiver)
            .unwrap();
        let mut botched = String::new();
        let mut lines = str::from_utf8(payload.as_slice()).unwrap().lines();
        botched.push_str(lines.next().unwrap()); // version
        botched.push('\n');
        botched.push_str(lines.next().unwrap()); // sender
        botched.push('\n');
        botched.push_str(lines.next().unwrap()); // receiver
        botched.push('\n');
        botched.push_str(lines.next().unwrap()); // nonce
        botched.push('\n');
        botched.push_str("not:base64");

        BoxKeyPair::decrypt(botched.as_bytes(), cache.path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Secret key, public key, and nonce could not decrypt ciphertext")]
    fn decrypt_invalid_ciphertext() {
        let cache = TempDir::new("key_cache").unwrap();
        let sender = BoxKeyPair::generate_pair_for_user("wecoyote", cache.path()).unwrap();
        let receiver = BoxKeyPair::generate_pair_for_service("acme", "tnt.default", cache.path())
            .unwrap();

        let payload = sender
            .encrypt("problems ahead".as_bytes(), &receiver)
            .unwrap();
        let mut botched = String::new();
        let mut lines = str::from_utf8(payload.as_slice()).unwrap().lines();
        botched.push_str(lines.next().unwrap()); // version
        botched.push('\n');
        botched.push_str(lines.next().unwrap()); // sender
        botched.push('\n');
        botched.push_str(lines.next().unwrap()); // receiver
        botched.push('\n');
        botched.push_str(lines.next().unwrap()); // nonce
        botched.push('\n');
        botched.push_str("uhoh");

        BoxKeyPair::decrypt(botched.as_bytes(), cache.path()).unwrap();
    }
}
