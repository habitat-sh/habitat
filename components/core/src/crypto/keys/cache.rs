use super::ring_key::RingKey;
use crate::error::Result;
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
        RingKey::latest_cached_revision(name, &self.0)
    }
}
