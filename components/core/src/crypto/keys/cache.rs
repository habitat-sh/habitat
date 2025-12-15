use crate::{crypto::{hash::Blake2bHash,
                     keys::{BuilderSecretEncryptionKey,
                            KeyFile,
                            NamedRevision,
                            OriginPublicEncryptionKey,
                            OriginSecretEncryptionKey,
                            PublicOriginSigningKey,
                            RingKey,
                            SecretOriginSigningKey,
                            ServicePublicEncryptionKey,
                            ServiceSecretEncryptionKey,
                            UserPublicEncryptionKey,
                            UserSecretEncryptionKey,
                            encryption::{BUILDER_KEY_NAME,
                                         generate_origin_encryption_key_pair,
                                         generate_service_encryption_key_pair,
                                         generate_user_encryption_key_pair},
                            generate_signing_key_pair}},
            error::{Error,
                    Result},
            fs::AtomicWriter,
            origin::Origin};
use serde::Deserialize;
use std::{convert::TryFrom,
          io::Write,
          path::{Path,
                 PathBuf}};

/// Represents the location of all Habitat keys (user, service,
/// origin, signing, and ring) locally on disk, as well as the APIs
/// for retrieving and storing keys.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct KeyCache(PathBuf);

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

    ////////////////////////////////////////////////////////////////////////

    /// Generate a new ring key and save it to disk.
    pub fn new_ring_key(&self, name: &str) -> Result<RingKey> {
        let key = RingKey::new(name);
        self.write_key(&key)?;
        Ok(key)
    }

    /// Generate a new origin signing key pair and save both keys to disk.
    pub fn new_signing_pair(&self,
                            origin: &Origin)
                            -> Result<(PublicOriginSigningKey, SecretOriginSigningKey)> {
        let (public, secret) = generate_signing_key_pair(origin)?;
        self.write_pair(&public, &secret)?;
        Ok((public, secret))
    }

    /// Generate a new origin encryption key pair and save both keys to disk.
    pub fn new_origin_encryption_pair(
        &self,
        origin: &Origin)
        -> Result<(OriginPublicEncryptionKey, OriginSecretEncryptionKey)> {
        let (public, secret) = generate_origin_encryption_key_pair(origin)?;
        self.write_pair(&public, &secret)?;
        Ok((public, secret))
    }

    /// Generate a new user encryption key pair and save both keys to disk.
    pub fn new_user_encryption_pair(
        &self,
        user: &str)
        -> Result<(UserPublicEncryptionKey, UserSecretEncryptionKey)> {
        let (public, secret) = generate_user_encryption_key_pair(user)?;
        self.write_pair(&public, &secret)?;
        Ok((public, secret))
    }

    /// Generate a new service encryption key pair and save both keys to disk.
    pub fn new_service_encryption_pair(
        &self,
        org: &str,
        service_group: &str)
        -> Result<(ServicePublicEncryptionKey, ServiceSecretEncryptionKey)> {
        let (public, secret) = generate_service_encryption_key_pair(org, service_group)?;
        self.write_pair(&public, &secret)?;
        Ok((public, secret))
    }

    ////////////////////////////////////////////////////////////////////////

    /// Write a key into the cache. If the key already exists,
    /// and the content has the same hash value, nothing will be
    /// done. If the file exists and it has *different* content, an
    /// Error is returned.
    // Note that this is potentially problematic for public origin
    // encryption keys and public origin signing keys. Even though
    // they have different header values within a file, they could
    // have the same name and revision (and thus the same filename)
    // if new encryption and signing pairs were made within the same
    // second.
    //
    // That is, if you made encryption and signing key pairs for
    // "my-origin" in the same second, the public keys of both would
    // both try to be saved in the same file, which is not great.
    //
    // This is probably _highly_ unlikely to manifest outside of
    // tests, particularly since only Builder will be generating
    // origin encryption pairs, but it's worth mentioning.
    //
    // This could also cause problems trying to retrieve one of these
    // keys from the cache, either by name or named revision. If
    // you're trying to fetch a public origin signing key, but happen
    // to have a public origin *encryption* key in your cache, we'll
    // find the file, but won't be able to give you a key, because it
    // won't parse to the correct type.
    //
    // TODO (CM): Consider new filename extensions that capture what
    // kind of key we're dealing with. For example "service.pub" and
    // "service.box.key" for service encryption keys, "sig.pub"
    // and "sig.key" for signing keys, etc. This would, of course,
    // have to be done in a backwards-compatible way for all the keys
    // currently in existence.
    pub fn write_key<K>(&self, key: &K) -> Result<()>
        where K: KeyFile
    {
        let keyfile = self.path_in_cache(key);
        let content = key.to_key_string();

        if keyfile.is_file() {
            let new_hash = Blake2bHash::from_bytes(&content);
            let existing_hash = Blake2bHash::from_file(&keyfile)?;
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

    pub fn latest_secret_origin_signing_key(&self,
                                            origin: &Origin)
                                            -> Result<SecretOriginSigningKey> {
        self.fetch_latest_revision::<SecretOriginSigningKey>(origin.as_ref())
    }

    pub fn latest_public_origin_signing_key(&self,
                                            origin: &Origin)
                                            -> Result<PublicOriginSigningKey> {
        self.fetch_latest_revision::<PublicOriginSigningKey>(origin.as_ref())
    }

    pub fn latest_user_secret_key(&self, user_name: &str) -> Result<UserSecretEncryptionKey> {
        self.fetch_latest_revision::<UserSecretEncryptionKey>(user_name)
    }

    pub fn latest_origin_public_encryption_key(&self,
                                               origin: &Origin)
                                               -> Result<OriginPublicEncryptionKey> {
        self.fetch_latest_revision::<OriginPublicEncryptionKey>(origin.as_ref())
    }

    /// Name should be in `"service.group@org"` format.
    pub fn latest_service_public_key(&self, name: &str) -> Result<ServicePublicEncryptionKey> {
        self.fetch_latest_revision::<ServicePublicEncryptionKey>(name)
    }

    /// Returns the latest Builder secret encryption key. All Builder
    /// encryption keys have the same name, by definition, so we don't
    /// need to provide one to this method (in contrast with all the
    /// other key retrieval methods).
    pub fn latest_builder_key(&self) -> Result<BuilderSecretEncryptionKey> {
        self.fetch_latest_revision::<BuilderSecretEncryptionKey>(BUILDER_KEY_NAME)
    }

    /// Attempt to retrieve the specified signing key from the cache,
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

    /// Retrieve the Builder secret encryption key with the specified
    /// revision.
    pub fn builder_secret_encryption_key(&self,
                                         named_revision: &NamedRevision)
                                         -> Result<BuilderSecretEncryptionKey> {
        self.fetch_specific_revision::<BuilderSecretEncryptionKey>(named_revision)
    }

    ////////////////////////////////////////////////////////////////////////

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
            Err(Error::CryptoError(format!("Key not found in cache: {}", path_in_cache.display())))
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
                         -> Result<impl Iterator<Item = PathBuf> + use<>> {
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
    use crate::crypto::{keys::{Key,
                               KeyFile,
                               OriginSecretEncryptionKey,
                               generate_origin_encryption_key_pair,
                               generate_service_encryption_key_pair,
                               generate_signing_key_pair,
                               generate_user_encryption_key_pair},
                        test_support::*};
    static VALID_KEY: &str = "ring-key-valid-20160504220722.sym.key";
    static VALID_NAME_WITH_REV: &str = "ring-key-valid-20160504220722";

    /// Helper to call `cache.get_all_paths_for`, but return a list of
    /// the plain file names, rather than full paths in the
    /// cache. This makes testing a bit more straightforward, and less
    /// verbose.
    fn ring_key_paths(cache: &KeyCache, name: &str) -> Vec<PathBuf> {
        cache.get_all_paths_for(name, RingKey::extension())
             .unwrap()
             .map(|pb| Path::new(pb.file_name().unwrap()).to_path_buf())
             .collect()
    }

    #[test]
    fn get_all_paths_for() {
        let (cache, _dir) = new_cache();

        let ring_name = "beyonce";

        let paths = ring_key_paths(&cache, ring_name);
        assert!(paths.is_empty());

        let k1 = RingKey::new(ring_name);
        cache.write_key(&k1).unwrap();
        let paths = ring_key_paths(&cache, ring_name);
        assert_eq!(paths.len(), 1);
        assert!(paths.contains(&k1.own_filename()));

        wait_1_sec(); // ensure new revision
        // will be different.

        let k2 = RingKey::new(ring_name);
        cache.write_key(&k2).unwrap();

        let paths = ring_key_paths(&cache, ring_name);
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&k1.own_filename()));
        assert!(paths.contains(&k2.own_filename()));

        // We should not include another named key in the count
        let other = RingKey::new("jayz");
        cache.write_key(&other).unwrap();
        let paths = ring_key_paths(&cache, ring_name);
        assert_eq!(paths.len(), 2);
        assert!(!paths.contains(&other.own_filename()));
        assert!(paths.contains(&k1.own_filename()));
        assert!(paths.contains(&k2.own_filename()));
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn latest_cached_revision_nonexistent() {
        let (cache, _dir) = new_cache();
        cache.latest_ring_key_revision("nope-nope").unwrap();
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

    /// Helper macro to assert that a given key can be saved and
    /// retrieved from the cache in different ways.
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

    /// Populate a cache with multiple revisions of all kinds of our
    /// keys. This can be useful background data against which to
    /// evaluate `KeyCache::fetch_latest_revision`
    fn populate_cache(cache: &KeyCache) {
        let origin = "my-origin".parse().unwrap();
        for _ in 0..=2 {
            cache.new_user_encryption_pair("my-user").unwrap();
            cache.new_origin_encryption_pair(&origin).unwrap();
            cache.new_service_encryption_pair("my-org", "foo.default")
                 .unwrap();

            // If we're going to be using the same origin name for the
            // encryption key and the signing key, we have to wait a
            // second, because the public keys will both have the same
            // filename :/
            wait_1_sec();

            cache.new_signing_pair(&origin).unwrap();
            cache.new_ring_key("beyonce").unwrap();
            wait_1_sec();
        }
    }

    #[test]
    fn ring_key_round_trip() {
        let (cache, _dir) = new_cache();
        populate_cache(&cache);
        let key = RingKey::new("beyonce");
        assert_cache_round_trip!(RingKey, key, cache);
    }

    #[test]
    fn user_keys_round_trip() {
        let (cache, _dir) = new_cache();
        populate_cache(&cache);
        let (public, secret) = generate_user_encryption_key_pair("my-user").unwrap();
        assert_cache_round_trip!(UserPublicEncryptionKey, public, cache);
        assert_cache_round_trip!(UserSecretEncryptionKey, secret, cache);
    }

    #[test]
    fn origin_keys_round_trip() {
        let (cache, _dir) = new_cache();
        populate_cache(&cache);
        let origin = "my-origin".parse().unwrap();
        let (public, secret) = generate_origin_encryption_key_pair(&origin).unwrap();
        assert_cache_round_trip!(OriginPublicEncryptionKey, public, cache);
        assert_cache_round_trip!(OriginSecretEncryptionKey, secret, cache);
    }

    #[test]
    fn service_keys_round_trip() {
        let (cache, _dir) = new_cache();
        populate_cache(&cache);
        let (public, secret) =
            generate_service_encryption_key_pair("my-org", "foo.default").unwrap();
        assert_cache_round_trip!(ServicePublicEncryptionKey, public, cache);
        assert_cache_round_trip!(ServiceSecretEncryptionKey, secret, cache);
    }

    #[test]
    fn signing_keys_round_trip() {
        let (cache, _dir) = new_cache();
        populate_cache(&cache);
        let origin = "my-org".parse().unwrap();
        let (public, secret) = generate_signing_key_pair(&origin).unwrap();
        assert_cache_round_trip!(PublicOriginSigningKey, public, cache);
        assert_cache_round_trip!(SecretOriginSigningKey, secret, cache);
    }

    mod write_pair {
        use super::*;

        #[test]
        fn pair_must_actually_be_a_pair_in_order_to_save() {
            let (cache, _dir) = new_cache();

            let (me_public, _me_secret) = generate_user_encryption_key_pair("me").unwrap();
            let (_you_public, you_secret) = generate_user_encryption_key_pair("you").unwrap();

            let result = cache.write_pair(&me_public, &you_secret);
            assert!(result.is_err(), "Threw an error: {:?}", result);
        }
    }

    mod symlinks {
        // Keys should be able to be symlinks, not just normal
        // files. This is particularly important in environments like
        // Kubernetes that rely heavily on symlinks.
        //
        // See https://github.com/habitat-sh/habitat/issues/2939

        use super::*;
        use tempfile::Builder;

        #[test]
        fn symlinks_are_ok() {
            let (cache, _dir) = new_cache();
            let real_dir = Builder::new().prefix("symlinks_are_ok").tempdir().unwrap();
            assert_ne!(cache.as_ref(),
                       real_dir.path(),
                       "Cache directory and 'real' directory should be different");

            let key = RingKey::new("symlinks_are_ok");

            // Write the key into the "real directory"
            let path_in_real_dir = real_dir.path().join(key.own_filename());
            std::fs::write(&path_in_real_dir, key.to_key_string()).unwrap();

            // Create a symlink to the key in the actual KeyCache directory
            let path_in_cache_dir = cache.as_ref().join(key.own_filename());
            symlink_file(&path_in_real_dir, &path_in_cache_dir).expect("Could not generate \
                                                                        symlink");

            // For sanity, assert that our paths are how we expect.
            assert_ne!(path_in_real_dir, path_in_cache_dir);
            assert_eq!(path_in_real_dir.file_name(), path_in_cache_dir.file_name());

            let real_metadata = path_in_real_dir.symlink_metadata().unwrap();
            let link_metadata = path_in_cache_dir.symlink_metadata().unwrap();

            assert!(real_metadata.file_type().is_file());
            assert!(!real_metadata.file_type().is_symlink());

            assert!(!link_metadata.file_type().is_file());
            assert!(link_metadata.file_type().is_symlink());

            // When retrieving the key, we can follow symlinks!
            let retrieved_key = cache.latest_ring_key_revision(key.named_revision().name())
                                     .unwrap();
            assert_eq!(retrieved_key, key);
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
    }
}
