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

use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Deref;
use std::result;
use std::str::FromStr;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, Bucket, BasicSet, IndexSet};
use hab_core::package::{self, Identifiable};
use protobuf::Message;
use protocol::depotsrv;
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, Pipeline, PipelineCommands};

use error::{Error, Result};

pub struct DataStore {
    pub pool: Arc<ConnectionPool>,
    pub packages: PackagesTable,
    pub channels: ChannelsTable,
}

impl DataStore {
    pub fn open<C: redis::IntoConnectionInfo>(config: C) -> Result<Self> {
        // JW TODO: tune pool from config?
        let pool_cfg = Default::default();
        let manager = RedisConnectionManager::new(config).unwrap();
        let pool = Arc::new(ConnectionPool::new(pool_cfg, manager).unwrap());
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let packages = PackagesTable::new(pool1);
        let channels = ChannelsTable::new(pool2);
        Ok(DataStore {
               pool: pool,
               packages: packages,
               channels: channels,
           })
    }

    /// Truncates every database in the datastore.
    ///
    /// # Failures
    ///
    /// * If a read-write transaction could not be acquired for any of the databases in the
    ///   datastore
    pub fn clear(&self) -> Result<()> {
        try!(redis::cmd("FLUSHDB").query(self.pool
                                             .get()
                                             .unwrap()
                                             .deref()));
        Ok(())
    }

    pub fn key_count(&self) -> Result<usize> {
        let count = try!(redis::cmd("DBSIZE").query(self.pool
                                                        .get()
                                                        .unwrap()
                                                        .deref()));
        Ok(count)
    }
}

/// Contains metadata entries for each package known by the Depot
pub struct PackagesTable {
    pub index: PackagesIndex,
    pool: Arc<ConnectionPool>,
}

impl PackagesTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let index = PackagesIndex::new(pool1);
        PackagesTable {
            pool: pool,
            index: index,
        }
    }
}

impl Bucket for PackagesTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "package"
    }
}

impl BasicSet for PackagesTable {
    type Record = depotsrv::Package;

    fn write(&self, record: &depotsrv::Package) -> result::Result<bool, dbcache::Error> {
        let conn = self.pool().get().unwrap();
        let keys = [Self::key(record),
                    PackagesIndex::origin_idx(&record),
                    PackagesIndex::name_idx(&record),
                    PackagesIndex::version_idx(&record)];
        try!(redis::transaction(conn.deref(), &keys, |mut txn| {
            let body = record.write_to_bytes().unwrap();
            txn.set(Self::key(&record), body).ignore();
            PackagesIndex::write(&mut txn, &record);
            txn.query(conn.deref())
        }));
        Ok(true)
    }
}

/// Contains an index of package identifiers to easily find the latest version/release of a
/// specified package.
pub struct PackagesIndex {
    pool: Arc<ConnectionPool>,
}

impl PackagesIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        PackagesIndex { pool: pool }
    }

    pub fn count(&self, id: &str) -> Result<u64> {
        let conn = self.pool().get().unwrap();
        let val = try!(conn.zcount(Self::key(&id.to_string()), 0, 0));
        Ok(val)
    }

    pub fn count_unique(&self, id: &str) -> Result<u64> {
        let conn = self.pool().get().unwrap();
        let val = try!(conn.zcount(Self::unique_key(&id.to_string()), 0, 0));
        Ok(val)
    }

    pub fn list(&self, id: &str, start: isize, stop: isize) -> Result<Vec<depotsrv::PackageIdent>> {
        let conn = self.pool().get().unwrap();

        // Note: start and stop are INCLUSIVE ranges
        match conn.zrange::<String, Vec<String>>(Self::key(&id.to_string()), start, stop) {
            Ok(ids) => {
                // JW TODO: This in-memory sorting logic can be removed once the Redis sorted set
                // is pre-sorted on write. For now, we'll do it on read each time.
                let mut ids: Vec<package::PackageIdent> =
                    ids.iter().map(|id| package::PackageIdent::from_str(id).unwrap()).collect();
                ids.sort();
                let ids = ids.into_iter().map(|id| depotsrv::PackageIdent::from(id)).collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn unique(&self,
                  id: &str,
                  start: isize,
                  stop: isize)
                  -> Result<Vec<depotsrv::PackageIdent>> {
        let conn = self.pool().get().unwrap();

        // Note: start and stop are INCLUSIVE ranges
        match conn.zrange::<String, Vec<String>>(Self::unique_key(&id.to_string()), start, stop) {
            Ok(ids) => {
                // JW TODO: This in-memory sorting logic can be removed once the Redis sorted set
                // is pre-sorted on write. For now, we'll do it on read each time.
                let mut idz: Vec<package::PackageIdent> = ids.iter()
                    .map(|iz| {
                             package::PackageIdent::from_str(&format!("{}/{}", &id, &iz)).unwrap()
                         })
                    .collect();
                idz.sort();
                let new_ids = idz.into_iter().map(|zd| depotsrv::PackageIdent::from(zd)).collect();
                Ok(new_ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest<T: Identifiable>(&self, id: &T, target: &str) -> Result<depotsrv::PackageIdent> {
        let conn = self.pool().get().unwrap();
        match conn.zrange::<String, Vec<String>>(PackagesIndex::key(&format!("{}/{}",
                                                                             &target,
                                                                             &id)),
                                                 0,
                                                 -1) {
            Ok(ref ids) if ids.len() <= 0 => {
                return Err(Error::DataStore(dbcache::Error::EntityNotFound))
            }
            Ok(ids) => {
                // JW TODO: This in-memory sorting logic can be removed once the Redis sorted set
                // is pre-sorted on write. For now, we'll do it on read each time.
                let mut ids: Vec<package::PackageIdent> = ids.iter()
                    .map(|id| package::PackageIdent::from_str(id).unwrap())
                    .filter(|p| p.fully_qualified())
                    .collect();
                ids.sort();
                ids.reverse();
                Ok(depotsrv::PackageIdent::from(ids.remove(0)))
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Returns a tuple with a vector of package identifiers matching a partial pattern
    /// (up to the passed in count values), and a value indicating the total count of all the
    ///  values that match the query.
    ///
    /// This search behaves as an "auto-complete" search by returning package identifiers that
    /// contain a match for the pattern. The match is applied to each of the four parts of a package
    /// identifier so typing "cor" will return a list of package identifiers whose name or origin
    /// begin with "cor". A string containing integers is also allowed and will allow searching on
    /// version numbers or releases.
    pub fn search(&self,
                  partial: &str,
                  offset: isize,
                  count: isize)
                  -> Result<(Vec<depotsrv::PackageIdent>, isize)> {
        let min = format!("[{}", partial);
        let max = format!("[{}{}", partial, r"xff");
        let conn = self.pool().get().unwrap();

        let total_count: isize = try!(conn.zlexcount(Self::prefix(), min.clone(), max.clone()));

        match conn.zrangebylex_limit::<&'static str, String, String, Vec<String>>(Self::prefix(),
                                                                                  min,
                                                                                  max,
                                                                                  offset,
                                                                                  count) {
            Ok(ids) => {
                let i = ids.iter()
                    .map(|i| {
                             let id = i.split(":").last().unwrap();
                             let p = package::PackageIdent::from_str(id).unwrap();
                             depotsrv::PackageIdent::from(p)
                         })
                    .collect();

                Ok((i, total_count))
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(pipe: &mut Pipeline, record: &depotsrv::Package) {
        pipe.zadd(Self::origin_idx(record), record.to_string(), 0)
            .ignore()
            .zadd(Self::name_idx(record), record.to_string(), 0)
            .ignore()
            .zadd(Self::version_idx(record), record.to_string(), 0)
            .ignore()
            .zadd(Self::target_name_idx(record), record.to_string(), 0)
            .ignore()
            .zadd(Self::target_version_idx(record), record.to_string(), 0)
            .ignore()
            .zadd(Self::prefix(),
                  format!("{}:{}", record.get_ident().get_origin(), record.to_string()),
                  0)
            .ignore()
            .zadd(Self::prefix(),
                  format!("{}:{}", record.get_ident().get_name(), record.to_string()),
                  0)
            .ignore()
            .zadd(Self::prefix(),
                  format!("{}:{}",
                          record.get_ident().get_release(),
                          record.to_string()),
                  0)
            .ignore()
            .zadd(Self::prefix(),
                  format!("{}:{}",
                          record.get_ident().get_version(),
                          record.to_string()),
                  0)
            .ignore()
            .zadd(Self::unique_prefix(),
                  format!("{}:{}",
                          record.get_ident().get_origin(),
                          record.get_ident().get_name()),
                  0)
            .ignore()
            .zadd(Self::unique_idx(record), record.get_ident().get_name(), 0)
            .ignore();
    }

    fn unique_prefix() -> &'static str {
        "package:ident:unique:index"
    }

    fn unique_key<K: fmt::Display>(id: K) -> String {
        format!("{}:{}", Self::unique_prefix(), id).to_lowercase()
    }

    fn unique_idx(package: &depotsrv::Package) -> String {
        Self::unique_key(package.get_ident().get_origin())
    }

    fn origin_idx(package: &depotsrv::Package) -> String {
        Self::key(package.get_ident().get_origin())
    }

    fn name_idx(package: &depotsrv::Package) -> String {
        let ident = package.get_ident();
        Self::key(format!("{}/{}", ident.get_origin(), ident.get_name()))
    }

    fn version_idx(package: &depotsrv::Package) -> String {
        let ident = package.get_ident();
        Self::key(format!("{}/{}/{}",
                          ident.get_origin(),
                          ident.get_name(),
                          ident.get_version()))
    }

    fn target_name_idx(package: &depotsrv::Package) -> String {
        let ident = package.get_ident();
        Self::key(format!("{}/{}/{}",
                          package.get_target(),
                          ident.get_origin(),
                          ident.get_name()))
    }

    fn target_version_idx(package: &depotsrv::Package) -> String {
        let ident = package.get_ident();
        Self::key(format!("{}/{}/{}/{}",
                          package.get_target(),
                          ident.get_origin(),
                          ident.get_name(),
                          ident.get_version()))
    }
}

impl Bucket for PackagesIndex {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "package:ident:index"
    }
}

/// Contains a mapping of channel names and the packages found within that channel.
///
/// This is how packages will be "promoted" between environments without duplicating data on disk.
pub struct ChannelsTable {
    pool: Arc<ConnectionPool>,
    channel_package_map: HashMap<String, Vec<depotsrv::PackageIdent>>,
    origin_channel_map: HashMap<String, Vec<String>>,
    pub pkg_channel_idx: PkgChannelIndex,
    pub channel_pkg_idx: ChannelPkgIndex,
}

// It's worth noting that these two new HashMaps are a temporary measure, put in place to avoid
// having to deal with persisting things to Redis while the work to convert our persistence layer
// to Postgres is in progress. Once the transition to PG is complete, these two HashMaps
// should go away and all of the methods below that use them should be changed to use PG instead.

impl ChannelsTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pkg_channel_idx = PkgChannelIndex::new(pool1);
        let channel_pkg_idx = ChannelPkgIndex::new(pool2);
        let channel_package_map = HashMap::<String, Vec<depotsrv::PackageIdent>>::new();
        let origin_channel_map = HashMap::<String, Vec<String>>::new();

        ChannelsTable {
            pool: pool,
            channel_package_map: channel_package_map,
            origin_channel_map: origin_channel_map,
            pkg_channel_idx: pkg_channel_idx,
            channel_pkg_idx: channel_pkg_idx,
        }
    }

    pub fn all(&mut self, origin: &str) -> Vec<String> {
        let channels = self.origin_channel_map
            .entry(origin.to_string())
            .or_insert(vec!["stable".to_string(), "unstable".to_string()]);
        channels.clone()
    }

    pub fn create(&mut self, origin: String, channel: String) -> Result<()> {
        let mut vec = match self.origin_channel_map.get(&origin) {
            Some(channels) => channels.clone(),
            None => Vec::new(),
        };

        if self.channel_exists(&origin, &channel) {
            return Err(Error::ChannelAlreadyExists(channel.to_string()));
        }

        vec.push(channel);
        self.origin_channel_map.insert(origin, vec);
        Ok(())
    }

    pub fn latest(&self,
                  origin: &str,
                  channel: &str,
                  ident: &str)
                  -> Option<&depotsrv::PackageIdent> {
        let key = format!("{}/{}", origin, channel);

        if let Some(packages) = self.channel_package_map.get(&key) {
            let mut pkgs: Vec<&depotsrv::PackageIdent> =
                packages.iter().filter(|pkg| pkg.to_string().contains(ident)).collect();

            if pkgs.is_empty() {
                return None;
            }

            pkgs.sort_by(|a, b| match a.get_version().cmp(b.get_version()) {
                             Ordering::Equal => a.get_release().cmp(b.get_release()),
                             other => other,
                         });
            let pkg = pkgs.last().unwrap();
            Some(*pkg)
        } else {
            None
        }
    }

    pub fn all_packages(&self,
                        origin: &str,
                        channel: &str,
                        ident: &str,
                        start: isize,
                        stop: isize)
                        -> Vec<&depotsrv::PackageIdent> {
        let key = format!("{}/{}", origin, channel);

        if let Some(packages) = self.channel_package_map.get(&key) {
            packages.iter()
                .enumerate()
                .filter(|&(i, pkg)| if ident == origin {
                            i >= start as usize && i <= stop as usize
                        } else {
                            i >= start as usize && i <= stop as usize &&
                            pkg.to_string().contains(ident)
                        })
                .map(|(_, e)| e)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn remove(&mut self, origin: &str, channel: &str) {
        let vec = match self.origin_channel_map.get_mut(origin) {
            Some(channels) => channels,
            None => return,
        };

        if let Some(index) = vec.iter().position(|x| x == channel) {
            vec.remove(index);
        }
    }


    pub fn channel_exists(&mut self, origin: &str, channel: &str) -> bool {
        let vec = self.origin_channel_map
            .entry(origin.to_string())
            .or_insert(vec!["stable".to_string(), "unstable".to_string()]);

        vec.contains(&channel.to_string())
    }

    pub fn package_exists(&self, key: &str, pkg: &depotsrv::PackageIdent) -> bool {
        let vec = match self.channel_package_map.get(key) {
            Some(packages) => packages,
            None => return false,
        };

        vec.contains(&pkg)
    }

    pub fn associate(&mut self, channel: &str, pkg: &depotsrv::Package) -> Result<()> {
        let ident = pkg.get_ident();
        let key = format!("{}/{}", ident.get_origin(), channel);
        let mut vec = match self.channel_package_map.get(&key) {
            Some(packages) => packages.clone(),
            None => Vec::new(),
        };

        if self.package_exists(&key, &ident) {
            return Err(Error::PackageIsAlreadyInChannel(pkg.to_string(), channel.to_string()));
        }

        vec.push(pkg.get_ident().clone());
        self.channel_package_map.insert(key, vec);
        Ok(())
    }
}

impl Bucket for ChannelsTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "views"
    }
}

impl BasicSet for ChannelsTable {
    type Record = depotsrv::View;
}

pub struct PkgChannelIndex {
    pool: Arc<ConnectionPool>,
}

impl PkgChannelIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        PkgChannelIndex { pool: pool }
    }
}

impl Bucket for PkgChannelIndex {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "pkg:view:index"
    }
}

impl IndexSet for PkgChannelIndex {
    type Key = String;
    type Value = String;
}

pub struct ChannelPkgIndex {
    pool: Arc<ConnectionPool>,
}

impl ChannelPkgIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        ChannelPkgIndex { pool: pool }
    }

    pub fn all(&self, channel: &str, pkg: &str) -> Result<Vec<package::PackageIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.zscan_match::<String, String, (String, u32)>(Self::key(&channel.to_string()),
                                                                format!("{}*", pkg)) {
            Ok(set) => {
                // JW TODO: This in-memory sorting logic can be removed once the Redis sorted set
                // is pre-sorted on write. For now, we'll do it on read each time.
                let mut set: Vec<package::PackageIdent> =
                    set.map(|(id, _)| package::PackageIdent::from_str(&id).unwrap()).collect();
                set.sort();
                set.reverse();
                Ok(set)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn is_member<T: Identifiable>(&self, channel: &str, pkg: &T) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        match conn.sismember(Self::key(&channel.to_string()), pkg.to_string()) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest(&self, channel: &str, pkg: &str) -> Result<depotsrv::PackageIdent> {
        match self.all(channel, pkg) {
            Ok(ref ids) if ids.len() <= 0 => Err(Error::DataStore(dbcache::Error::EntityNotFound)),
            Ok(mut ids) => Ok(depotsrv::PackageIdent::from(ids.remove(0))),
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Bucket for ChannelPkgIndex {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "view:pkg:index"
    }
}

impl IndexSet for ChannelPkgIndex {
    type Key = String;
    type Value = String;
}
