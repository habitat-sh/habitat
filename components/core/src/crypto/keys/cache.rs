use super::{get_key_revisions,
            parse_name_with_rev,
            ring_key::RingKey,
            KeyType};
use crate::error::{Error,
                   Result};
use std::path::PathBuf;

pub struct KeyCache(PathBuf);

impl<P> From<P> for KeyCache where P: Into<PathBuf>
{
    fn from(path: P) -> KeyCache { KeyCache(path.into()) }
}

impl KeyCache {
    pub fn write_ring_key(&self, key: &RingKey) -> Result<()> { key.write_to_cache(&self.0) }

    pub fn ring_key_cached_path(&self, key: &RingKey) -> Result<PathBuf> {
        RingKey::cached_path(&key.name_with_rev(), &self.0)
    }

    /// Note: name is just the name, not the name + revision
    pub fn latest_ring_key_revision(&self, name: &str) -> Result<RingKey> {
        let mut all = self.get_pairs_for(name)?;
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} sym key", name);
                Err(Error::CryptoError(msg))
            }
            _ => Ok(all.remove(0)),
        }
    }
}

// for RingKey implementations
impl KeyCache {
    // TODO (CM): Really, we just need to find all the files that
    // pertain to this named key, sort them by revision, and then read
    // the last one into a RingKey.

    fn get_pairs_for(&self, name: &str) -> Result<Vec<RingKey>> {
        let revisions = get_key_revisions(name, &self.0, None, KeyType::Sym)?;
        let mut key_pairs = Vec::new();
        for name_with_rev in &revisions {
            debug!("Attempting to read key name_with_rev {} for {}",
                   name_with_rev, name);
            let kp = self.get_pair_for(name_with_rev)?;
            key_pairs.push(kp);
        }
        Ok(key_pairs)
    }

    fn get_pair_for(&self, name_with_rev: &str) -> Result<RingKey> {
        let (name, rev) = parse_name_with_rev(&name_with_rev)?;

        // reading the secret key here is really about parsing the base64 bytes into an actual key.
        // That truly should be part of the "from string"
        let sk = match RingKey::get_secret_key(name_with_rev, &self.0) {
            Ok(k) => Some(k),
            Err(e) => {
                let msg = format!("No secret keys found for name_with_rev {}: {}",
                                  name_with_rev, e);
                return Err(Error::CryptoError(msg));
            }
        };
        Ok(RingKey::from_raw(name, rev, sk))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{thread,
              time::Duration};
    use tempfile::{Builder,
                   TempDir};

    /// Returns the `TempDir` that backs the cache to prevent it from
    /// getting `Drop`ped too early; feel free to ignore it.
    fn new_cache() -> (KeyCache, TempDir) {
        let dir = Builder::new().prefix("key_cache").tempdir().unwrap();
        let cache: KeyCache = dir.path().into();
        (cache, dir)
    }

    fn wait_1_sec() { thread::sleep(Duration::from_secs(1)); }

    #[test]
    fn get_pairs_for() {
        let (cache, _dir) = new_cache();

        let pairs = cache.get_pairs_for("beyonce").unwrap();
        assert_eq!(pairs.len(), 0);

        cache.write_ring_key(&RingKey::new("beyonce")).unwrap();
        let pairs = cache.get_pairs_for("beyonce").unwrap();
        assert_eq!(pairs.len(), 1);

        wait_1_sec(); // ensure new revision
                      // will be different.
        cache.write_ring_key(&RingKey::new("beyonce")).unwrap();

        let pairs = cache.get_pairs_for("beyonce").unwrap();
        assert_eq!(pairs.len(), 2);

        // We should not include another named key in the count
        cache.write_ring_key(&RingKey::new("jayz")).unwrap();
        let pairs = cache.get_pairs_for("beyonce").unwrap();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn latest_cached_revision_single() {
        let (cache, _dir) = new_cache();

        let key = RingKey::new("beyonce");
        cache.write_ring_key(&key).unwrap();

        let latest = cache.latest_ring_key_revision("beyonce").unwrap();
        assert_eq!(latest.name(), key.name());
        assert_eq!(latest.revision(), key.revision());
    }

    #[test]
    fn latest_cached_revision_multiple() {
        let (cache, _dir) = new_cache();

        let k1 = RingKey::new("beyonce");
        cache.write_ring_key(&k1).unwrap();

        wait_1_sec();

        let k2 = RingKey::new("beyonce");
        cache.write_ring_key(&k2).unwrap();

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
}
