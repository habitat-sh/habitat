use crate::{crypto::{hash,
                     keys::{KeyFile,
                            NamedRevision,
                            OriginPublicEncryptionKey,
                            OriginSecretEncryptionKey,
                            PublicOriginSigningKey,
                            RingKey,
                            SecretOriginSigningKey,
                            ServicePublicEncryptionKey,
                            ServiceSecretEncryptionKey,
                            UserPublicEncryptionKey,
                            UserSecretEncryptionKey}},
            error::{Error,
                    Result},
            fs::AtomicWriter};
use std::{convert::TryFrom,
          io::Write,
          path::{Path,
                 PathBuf}};

/// Represents the location of all Habitat keys (user, service,
/// origin, signing, and ring) locally on disk, as well as the APIs
/// for retrieving and storing keys.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyCache(PathBuf);

impl Default for KeyCache {
    fn default() -> Self { KeyCache::new(&*crate::fs::CACHE_KEY_PATH) }
}

impl AsRef<Path> for KeyCache {
    /// Expose the path to this key cache.
    fn as_ref(&self) -> &Path { self.0.as_ref() }
}

impl KeyCache {
    pub fn new<P>(path: P) -> Self
        where P: Into<PathBuf>
    {
        KeyCache(path.into())
    }

    /// Ensure that the directory backing the cache exists on disk.
    pub fn setup(&self) -> Result<()> {
        if !self.0.is_dir() {
            std::fs::create_dir_all(&self.0)?;
        }
        Ok(())
    }

    /// Save a pair of user encryption keys to the cache. Either both are
    /// saved or neither are.
    pub fn write_user_encryption_pair(&self,
                                      public: &UserPublicEncryptionKey,
                                      secret: &UserSecretEncryptionKey)
                                      -> Result<()> {
        self.write_pair(public, secret)
    }

    /// Save a pair of service encryption keys to the cache.
    pub fn write_service_encryption_pair(&self,
                                         public: &ServicePublicEncryptionKey,
                                         secret: &ServiceSecretEncryptionKey)
                                         -> Result<()> {
        self.write_pair(public, secret)
    }

    /// Save a pair of origin encryption keys to the cache.
    pub fn write_origin_encryption_pair(&self,
                                        public: &OriginPublicEncryptionKey,
                                        secret: &OriginSecretEncryptionKey)
                                        -> Result<()> {
        self.write_pair(public, secret)
    }

    /// Save a pair of signing keys to the cache.
    pub fn write_origin_signing_pair(&self,
                                     public: &PublicOriginSigningKey,
                                     secret: &SecretOriginSigningKey)
                                     -> Result<()> {
        self.write_pair(public, secret)
    }

    /// Write a pair of keys to the cache.
    fn write_pair<P, S>(&self, public: &P, secret: &S) -> Result<()>
        where P: KeyFile,
              S: KeyFile
    {
        if public.named_revision() != secret.named_revision() {
            return Err(Error::CryptoError(format!("Not saving key pair because \
                                                   they are not actually a \
                                                   pair! public: {}, secret: {}",
                                                  public.named_revision(),
                                                  secret.named_revision())));
        }

        // TODO (CM): It would be interesting to make this an
        // all-or-nothing operation, such that both keys were written
        // or neither were. Once the errors in this part of the crate
        // are cleaned up, we can better distinguish between different
        // kinds of error conditions so we don't, say, delete a
        // previously existing public key if writing a secret key
        // fails.
        self.write_key(public)?;
        self.write_key(secret)?;
        Ok(())
    }

    /// Write a key into the cache. If the key already exists,
    /// and the content has the same hash value, nothing will be
    /// done. If the file exists and it has *different* content, an
    /// Error is returned.
    pub fn write_key<K>(&self, key: &K) -> Result<()>
        where K: KeyFile
    {
        let keyfile = self.path_in_cache(key);
        let content = key.to_key_string();

        if keyfile.is_file() {
            let existing_hash = hash::hash_file(&keyfile)?;
            let new_hash = hash::hash_string(&content);
            if existing_hash != new_hash {
                let msg = format!("Existing key file {} found but new version hash is different, \
                                   failing to write new file over existing. (existing = {}, \
                                   incoming = {})",
                                  keyfile.display(),
                                  existing_hash,
                                  new_hash);
                return Err(Error::CryptoError(msg));
            }
        } else {
            // Technically speaking, this probably doesn't really need
            // to be an atomic write process, since we just tested
            // that the file doesn't currently exist. It does,
            // however, bundle up writing with platform-independent
            // permission setting, which is *super* convenient.
            let w = AtomicWriter::new_with_permissions(&keyfile, K::permissions())?;
            w.with_writer(|f| f.write_all(content.as_ref()))?;
        }
        Ok(())
    }

    /// Note: name is just the name, not the name + revision
    pub fn latest_ring_key_revision(&self, name: &str) -> Result<RingKey> {
        self.fetch_latest_revision::<RingKey>(name)
    }

    pub fn latest_secret_origin_signing_key(&self, name: &str) -> Result<SecretOriginSigningKey> {
        self.fetch_latest_revision::<SecretOriginSigningKey>(name)
    }

    pub fn latest_public_origin_signing_key(&self, name: &str) -> Result<PublicOriginSigningKey> {
        self.fetch_latest_revision::<PublicOriginSigningKey>(name)
    }

    pub fn latest_user_secret_key(&self, user_name: &str) -> Result<UserSecretEncryptionKey> {
        self.fetch_latest_revision::<UserSecretEncryptionKey>(user_name)
    }

    pub fn latest_origin_public_encryption_key(&self,
                                               name: &str)
                                               -> Result<OriginPublicEncryptionKey> {
        self.fetch_latest_revision::<OriginPublicEncryptionKey>(name)
    }

    /// Name should be in `"service.group@org"` format.
    pub fn latest_service_public_key(&self, name: &str) -> Result<ServicePublicEncryptionKey> {
        self.fetch_latest_revision::<ServicePublicEncryptionKey>(name)
    }

    /// Attemt to retrieve the specified signing key from the cache,
    /// if it exists and is valid.
    pub fn public_signing_key(&self,
                              named_revision: &NamedRevision)
                              -> Result<PublicOriginSigningKey> {
        self.fetch_specific_revision::<PublicOriginSigningKey>(named_revision)
    }

    pub fn secret_signing_key(&self,
                              named_revision: &NamedRevision)
                              -> Result<SecretOriginSigningKey> {
        self.fetch_specific_revision::<SecretOriginSigningKey>(named_revision)
    }

    pub fn user_public_encryption_key(&self,
                                      named_revision: &NamedRevision)
                                      -> Result<UserPublicEncryptionKey> {
        self.fetch_specific_revision::<UserPublicEncryptionKey>(named_revision)
    }

    pub fn service_secret_encryption_key(&self,
                                         named_revision: &NamedRevision)
                                         -> Result<ServiceSecretEncryptionKey> {
        self.fetch_specific_revision::<ServiceSecretEncryptionKey>(named_revision)
    }

    ////////////////////////////////////////////////////////////////////////

    /// Given the name and type of a key, fetch the latest revision of
    /// that key from the cache.
    ///
    /// As it happens, we currently have no real need to distinguish
    /// between "key not present" and "key present, but invalid", so
    /// we can just collapse them into an Error case.
    fn fetch_latest_revision<K>(&self, name: &str) -> Result<K>
        where K: KeyFile + TryFrom<PathBuf, Error = Error>
    {
        match self.get_latest_path_for(name, <K as KeyFile>::extension())? {
            Some(path) => <K as TryFrom<PathBuf>>::try_from(path),
            None => {
                let msg = format!("No revisions found for {}", name);
                Err(Error::CryptoError(msg))
            }
        }
    }

    /// Generic retrieval function to grab the key of the specified
    /// type `K` identified by `named_revision`
    fn fetch_specific_revision<K>(&self, named_revision: &NamedRevision) -> Result<K>
        where K: KeyFile + TryFrom<PathBuf, Error = Error>
    {
        let path_in_cache = self.0.join(<K as KeyFile>::filename(named_revision));
        if path_in_cache.exists() {
            <K as TryFrom<PathBuf>>::try_from(path_in_cache)
        } else {
            Err(Error::CryptoError(format!("Key not found in cache: {}",
                                           path_in_cache.display()).to_string()))
        }
    }

    ////////////////////////////////////////////////////////////////////////

    /// Provides the path at which this file would be found in the
    /// cache, if it exists (or, alternatively, where it would be
    /// written to).
    // TODO (CM): Only making this public temporarily
    pub fn path_in_cache<K>(&self, key: &K) -> PathBuf
        where K: KeyFile
    {
        self.0.join(key.own_filename())
    }

    /// Search the key cache for all files that are revisions of the
    /// given key. Returns the full paths to those files within the
    /// cache.
    fn get_all_paths_for(&self,
                         name: &str,
                         key_extension: &str)
                         -> Result<impl Iterator<Item = PathBuf>> {
        // Ideally, we'd want that `*` to be `\d{14}` to match the
        // structure of our revisions... perhaps that can be an
        // additional filter later on with an actual regex?
        let pattern = self.0.join(format!("{}-*.{}", name, key_extension));
        let pattern = pattern.to_string_lossy();

        // TODO (CM): this is a bogus error
        Ok(glob::glob(&pattern).map_err(|_e| Error::CryptoError("Couldn't glob!".to_string()))?
                               .filter_map(std::result::Result::ok)
                               .filter(|p| p.metadata().map(|m| m.is_file()).unwrap_or(false)))
    }

    /// Given a key name and extension, find the path that corresponds
    /// to the most recent revision of that key in the cache, if it
    /// exists.
    fn get_latest_path_for(&self, name: &str, key_extension: &str) -> Result<Option<PathBuf>> {
        Ok(self.get_all_paths_for(name, key_extension)?.max())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::crypto::{keys::{generate_origin_encryption_key_pair,
                               generate_service_encryption_key_pair,
                               generate_signing_key_pair,
                               generate_user_encryption_key_pair,
                               Key,
                               KeyFile,
                               OriginSecretEncryptionKey},
                        test_support::*};

    static VALID_KEY: &str = "ring-key-valid-20160504220722.sym.key";
    static VALID_NAME_WITH_REV: &str = "ring-key-valid-20160504220722";

    #[test]
    fn get_all_paths_for() {
        let (cache, _dir) = new_cache();

        let paths = cache.get_all_paths_for("beyonce", RingKey::extension())
                         .unwrap();
        assert_eq!(paths.count(), 0);

        cache.write_key(&RingKey::new("beyonce")).unwrap();
        let paths = cache.get_all_paths_for("beyonce", RingKey::extension())
                         .unwrap();
        assert_eq!(paths.count(), 1);

        wait_1_sec(); // ensure new revision
                      // will be different.
        cache.write_key(&RingKey::new("beyonce")).unwrap();

        let paths = cache.get_all_paths_for("beyonce", RingKey::extension())
                         .unwrap();
        assert_eq!(paths.count(), 2);

        // We should not include another named key in the count
        cache.write_key(&RingKey::new("jayz")).unwrap();
        let paths = cache.get_all_paths_for("beyonce", RingKey::extension())
                         .unwrap();
        assert_eq!(paths.count(), 2);
    }

    #[test]
    fn latest_cached_revision_single() {
        let (cache, _dir) = new_cache();

        let key = RingKey::new("beyonce");
        cache.write_key(&key).unwrap();

        let latest = cache.latest_ring_key_revision("beyonce").unwrap();
        assert_eq!(latest.named_revision().name(), key.named_revision().name());
        assert_eq!(latest.named_revision().revision(),
                   key.named_revision().revision());
    }

    #[test]
    fn latest_cached_revision_multiple() {
        let (cache, _dir) = new_cache();

        let k1 = RingKey::new("beyonce");
        cache.write_key(&k1).unwrap();

        wait_1_sec();

        let k2 = RingKey::new("beyonce");
        cache.write_key(&k2).unwrap();

        assert_eq!(k1.named_revision().name(), k2.named_revision().name());
        assert_ne!(k1.named_revision().revision(),
                   k2.named_revision().revision());

        let latest = cache.latest_ring_key_revision("beyonce").unwrap();
        assert_eq!(latest.named_revision(), k2.named_revision());
        assert_eq!(latest.named_revision(), k2.named_revision());
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn latest_cached_revision_nonexistent() {
        let (cache, _dir) = new_cache();
        cache.latest_ring_key_revision("nope-nope").unwrap();
    }

    #[test]
    fn writing_ring_key() {
        let (cache, dir) = new_cache();

        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = dir.path().join(VALID_KEY);
        assert_eq!(new_key_file.is_file(), false);

        let key: RingKey = content.parse().unwrap();
        assert_eq!(key.named_revision().to_string(), VALID_NAME_WITH_REV);
        cache.write_key(&key).unwrap();
        assert!(new_key_file.is_file());

        let new_content = std::fs::read_to_string(new_key_file).unwrap();
        assert_eq!(new_content, content);
    }

    #[test]
    fn write_key_with_existing_identical() {
        let (cache, dir) = new_cache();
        let content = fixture_as_string(&format!("keys/{}", VALID_KEY));
        let new_key_file = dir.path().join(VALID_KEY);

        // install the key into the cache
        std::fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &new_key_file).unwrap();

        let key: RingKey = content.parse().unwrap();
        cache.write_key(&key).unwrap();
        assert_eq!(key.named_revision().to_string(), VALID_NAME_WITH_REV);
        assert!(new_key_file.is_file());
    }

    #[test]
    #[should_panic(expected = "Existing key file")]
    fn write_key_exists_but_hashes_differ() {
        let (cache, dir) = new_cache();
        let old_content = fixture_as_string("keys/ring-key-valid-20160504220722.sym.key");

        std::fs::write(dir.path().join("ring-key-valid-20160504220722.sym.key"),
                       &old_content).unwrap();

        #[rustfmt::skip]
        let new_content = "SYM-SEC-1\nring-key-valid-20160504220722\n\nkA+c03Ly5qEoOZIjJ5zCD2vHI05pAW59PfCOb8thmZw=";

        assert_ne!(old_content, new_content);

        let new_key: RingKey = new_content.parse().unwrap();
        // this should fail
        cache.write_key(&new_key).unwrap();
    }

    macro_rules! assert_cache_round_trip {
        ($t:ty, $key:expr, $cache:expr) => {
            $cache.write_key::<$t>(&$key).unwrap();
            let fetched_latest: $t = $cache.fetch_latest_revision($key.named_revision().name())
                                           .unwrap();
            assert_eq!(fetched_latest, $key,
                       "Expected to retrieve the latest key by name");

            let fetched_specific: $t = $cache.fetch_specific_revision($key.named_revision())
                                             .unwrap();
            assert_eq!(fetched_specific, $key,
                       "Expected to retrieve the key by specific revision");
        };
    }

    #[test]
    fn ring_key_round_trip() {
        let (cache, _dir) = new_cache();
        let key = RingKey::new("beyonce");
        assert_cache_round_trip!(RingKey, key, cache);
    }

    #[test]
    fn user_keys_round_trip() {
        let (cache, _dir) = new_cache();
        let (public, secret) = generate_user_encryption_key_pair("my-user");
        assert_cache_round_trip!(UserPublicEncryptionKey, public, cache);
        assert_cache_round_trip!(UserSecretEncryptionKey, secret, cache);
    }

    #[test]
    fn origin_keys_round_trip() {
        let (cache, _dir) = new_cache();
        let (public, secret) = generate_origin_encryption_key_pair("my-origin");
        assert_cache_round_trip!(OriginPublicEncryptionKey, public, cache);
        assert_cache_round_trip!(OriginSecretEncryptionKey, secret, cache);
    }

    #[test]
    fn service_keys_round_trip() {
        let (cache, _dir) = new_cache();
        let (public, secret) = generate_service_encryption_key_pair("my-org", "foo.default");
        assert_cache_round_trip!(ServicePublicEncryptionKey, public, cache);
        assert_cache_round_trip!(ServiceSecretEncryptionKey, secret, cache);
    }

    #[test]
    fn signing_keys_round_trip() {
        let (cache, _dir) = new_cache();
        let (public, secret) = generate_signing_key_pair("my-org");
        assert_cache_round_trip!(PublicOriginSigningKey, public, cache);
        assert_cache_round_trip!(SecretOriginSigningKey, secret, cache);
    }

    mod write_pair {
        use super::*;

        #[test]
        fn pair_must_actually_be_a_pair_in_order_to_save() {
            let (cache, _dir) = new_cache();

            let (me_public, _me_secret) = generate_user_encryption_key_pair("me");
            let (_you_public, you_secret) = generate_user_encryption_key_pair("you");

            let result = cache.write_user_encryption_pair(&me_public, &you_secret);
            assert!(result.is_err(), "Threw an error: {:?}", result);
        }
    }
}
