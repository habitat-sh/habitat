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

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::ops::Deref;

use time;

/// Time from which we begin issuing identifiers. This number can be used to determine how old
/// an entity is by subtracting it's timestamp from this value.
pub const EPOCH_MS: u64 = 1460499133628;
/// Maximum shard count. This number cannot exceed the value of `MAX_SHARD_ID`. If this number
/// is altered then the entities in the data store must all be issued new identifiers.
pub const SHARD_COUNT: u32 = 128;
/// Maximum value allowed for a Shard ID (2^13).
pub const MAX_SHARD_ID: u16 = 8192;

const ID_MASK: u64 = 0x3FF;
const SHARD_MASK: u64 = 0x1FFF;

pub type ShardId = u32;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct InstaId(pub u64);

impl InstaId {
    pub fn generate(auto_id: u64) -> Self {
        let time = Self::since_epoch();
        let id = auto_id % 1024;
        let shard_id = id % SHARD_COUNT as u64;
        let mut iid: u64 = time << 23;
        iid |= id << 13;
        iid |= shard_id;
        InstaId(iid)
    }

    pub fn since_epoch() -> u64 {
        let timespec = time::get_time();
        let sec: u64 = timespec.sec as u64 * 1000;
        let nsec: u64 = timespec.nsec as u64 / 1000 / 1000;
        (sec + nsec) - EPOCH_MS
    }

    pub fn inner_id(&self) -> u64 {
        (self.0 >> 13) & ID_MASK
    }

    pub fn shard(&self) -> u64 {
        self.0 & SHARD_MASK
    }

    pub fn timestamp(&self) -> u64 {
        self.0 >> 23
    }
}

impl Default for InstaId {
    fn default() -> Self {
        InstaId(0)
    }
}

impl Deref for InstaId {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl From<u64> for InstaId {
    fn from(id: u64) -> InstaId {
        InstaId(id)
    }
}

impl fmt::Display for InstaId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialOrd for InstaId {
    fn partial_cmp(&self, other: &InstaId) -> Option<Ordering> {
        match self.timestamp().cmp(&other.timestamp()) {
            Ordering::Equal => {
                match self.inner_id().cmp(&other.inner_id()) {
                    Ordering::Equal => None,
                    ordering => Some(ordering),
                }
            }
            ordering => Some(ordering),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn extract_shard_id_from_insta_id() {
        let auto_id = 1984198419841984;
        let insta_id = InstaId::generate(auto_id);
        assert_eq!(insta_id.shard(), 64);
    }

    #[test]
    fn extract_timestamp_from_insta_id() {
        let auto_id = 1984198419841984;
        let gen_time = InstaId::since_epoch();
        let insta_id = InstaId::generate(auto_id);
        assert!(insta_id.timestamp() >= gen_time);
    }

    #[test]
    fn ordering_of_insta_id() {
        // validate when generated within the same millisecond
        let id1 = InstaId::generate(1);
        let id2 = InstaId::generate(2);
        let id3 = InstaId::generate(3);
        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id2 > id1);
        assert!(id2 < id3);
        assert!(id3 > id1);
        assert!(id3 > id2);
        // validated when generated with same id at different time
        thread::sleep(Duration::from_millis(1));
        let id1 = InstaId::generate(1);
        thread::sleep(Duration::from_millis(1));
        let id2 = InstaId::generate(1);
        thread::sleep(Duration::from_millis(1));
        let id3 = InstaId::generate(1);
        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id2 > id1);
        assert!(id2 < id3);
        assert!(id3 > id1);
        assert!(id3 > id2);
    }

    #[test]
    fn extract_inner_id_from_insta_id() {
        let inner_id = 298674917348924;
        let insta_id = InstaId::generate(inner_id);
        assert_eq!(insta_id.inner_id(), inner_id % 1024);
    }
}
