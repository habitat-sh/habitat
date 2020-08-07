use super::ring_key::RingKey;
use crate::{crypto::{hash,
                     keys::{box_key_pair::{OriginPublicEncryptionKey,
                                           ServicePublicEncryptionKey,
                                           ServiceSecretEncryptionKey},
                            sig_key_pair::{PublicOriginSigningKey,
                                           SecretOriginSigningKey},
                            Key,
                            NamedRevision,
                            UserPublicEncryptionKey,
                            UserSecretEncryptionKey}},
            error::{Error,
                    Result},
            fs::AtomicWriter};
use std::{convert::TryFrom,
          io::Write,
          path::{Path,
                 PathBuf}};

pub struct KeyCache(PathBuf);

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

    /// Write a key into the cache. If the key already exists,
    /// and the content has the same hash value, nothing will be
    /// done. If the file exists and it has *different* content, an
    /// Error is returned.
    pub fn write_key<K>(&self, key: &K) -> Result<()>
        where K: AsRef<Path> + Key
    {
        self.maybe_write_key(key)
    }

    /// Note: name is just the name, not the name + revision
    pub fn latest_ring_key_revision(&self, name: &str) -> Result<RingKey> {
        self.fetch_latest_revision::<RingKey>(name)
    }

    pub fn latest_secret_origin_signing_key(&self, name: &str) -> Result<SecretOriginSigningKey> {
        self.fetch_latest_revision::<SecretOriginSigningKey>(name)
    }

    pub fn latest_public_origin_signing_key(&self, name: &str) -> Result<SecretOriginSigningKey> {
        self.fetch_latest_revision::<SecretOriginSigningKey>(name)
    }

    pub fn latest_user_secret_key(&self, user_name: &str) -> Result<UserSecretEncryptionKey> {
        self.fetch_latest_revision::<UserSecretEncryptionKey>(user_name)
    }

    pub fn latest_origin_public_encryption_key(&self,
                                               name: &str)
                                               -> Result<OriginPublicEncryptionKey> {
        self.fetch_latest_revision::<OriginPublicEncryptionKey>(name)
    }

    // TODO (CM): accept org + service_group instead? Create / accept
    // a proper type?
    /// Name should be in `"service.group@org"` format.
    pub fn latest_service_public_key(&self, name: &str) -> Result<ServicePublicEncryptionKey> {
        self.fetch_latest_revision::<ServicePublicEncryptionKey>(name)
    }

    /// Attemt to retrieve the specified signing key from the cache,
    /// if it exists and is valid.
    pub fn public_signing_key(&self,
                              named_revision: &NamedRevision)
                              -> Option<Result<PublicOriginSigningKey>> {
        self.fetch_specific_revision::<PublicOriginSigningKey>(named_revision)
    }

    pub fn user_public_encryption_key(&self,
                                      named_revision: &NamedRevision)
                                      -> Result<UserPublicEncryptionKey> {
        match self.fetch_specific_revision::<UserPublicEncryptionKey>(named_revision) {
            Some(key_result) => key_result,
            None => Err(Error::CryptoError("Key not found in cache".to_string())),
        }
    }

    pub fn service_secret_encryption_key(&self,
                                         named_revision: &NamedRevision)
                                         -> Result<ServiceSecretEncryptionKey> {
        match self.fetch_specific_revision::<ServiceSecretEncryptionKey>(named_revision) {
            Some(key_result) => key_result,
            None => Err(Error::CryptoError("Key not found in cache".to_string())),
        }
    }

    ////////////////////////////////////////////////////////////////////////

    // TODO (CM): Turn this into Option<Result<K>>; otherwise it
    // assumes that there must be at least one version of the key present
    fn fetch_latest_revision<K>(&self, name: &str) -> Result<K>
        where K: Key + TryFrom<PathBuf, Error = Error>
    {
        match self.get_latest_path_for(name, K::extension())? {
            Some(path) => <K as TryFrom<PathBuf>>::try_from(path),
            None => {
                let msg = format!("No revisions found for {}", name);
                Err(Error::CryptoError(msg))
            }
        }
    }

    /// Generic retrieval function to grab the key of the specified
    /// type `K` identified by `named_revision`
    fn fetch_specific_revision<K>(&self, named_revision: &NamedRevision) -> Option<Result<K>>
        where K: Key + TryFrom<PathBuf, Error = Error>
    {
        let path_in_cache = self.0.join(named_revision.filename::<K>());
        if path_in_cache.exists() {
            Some(<K as TryFrom<PathBuf>>::try_from(path_in_cache))
        } else {
            None
        }
    }

    ////////////////////////////////////////////////////////////////////////

    /// Provides the path at which this file would be found in the
    /// cache, if it exists (or, alternatively, where it would be
    /// written to).
    fn path_in_cache<K>(&self, key: K) -> PathBuf
        where K: AsRef<Path>
    {
        self.0.join(key.as_ref())
    }

    fn maybe_write_key<K>(&self, key: &K) -> Result<()>
        where K: AsRef<Path> + Key
    {
        let keyfile = self.path_in_cache(&key);
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
    use crate::crypto::test_support::*;

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
        assert_eq!(latest.name(), key.name());
        assert_eq!(latest.revision(), key.revision());
    }

    #[test]
    fn latest_cached_revision_multiple() {
        let (cache, _dir) = new_cache();

        let k1 = RingKey::new("beyonce");
        cache.write_key(&k1).unwrap();

        wait_1_sec();

        let k2 = RingKey::new("beyonce");
        cache.write_key(&k2).unwrap();

        assert_eq!(k1.name(), k2.name());
        assert_ne!(k1.revision(), k2.revision());

        let latest = cache.latest_ring_key_revision("beyonce").unwrap();
        assert_eq!(latest.name(), k2.name());
        assert_eq!(latest.revision(), k2.revision());
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
        assert_eq!(key.name_with_rev(), VALID_NAME_WITH_REV);
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
        assert_eq!(key.name_with_rev(), VALID_NAME_WITH_REV);
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

    // Old tests... not fully converting over to new implementation
    // yet because I think the function won't be sticking around very
    // long.

    // #[test]
    // fn cached_path() {
    //     let (cache, dir) = new_cache();
    //     fs::copy(fixture(&format!("keys/{}", VALID_KEY)),
    //              dir.path().join(VALID_KEY)).unwrap();

    //     let result = cache.ring_key_cached_path(VALID_NAME_WITH_REV).unwrap();
    //     assert_eq!(result, cache.path().join(VALID_KEY));
    // }

    // #[test]
    // #[should_panic(expected = "No secret key found at")]
    // fn get_secret_key_path_nonexistent() {
    //     let (cache, _dir) = new_cache();
    //     cache.ring_key_cached_path(VALID_NAME_WITH_REV).unwrap();
    // }
}
