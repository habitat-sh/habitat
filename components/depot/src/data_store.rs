// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
    pub views: ViewsTable,
    pub origin_keys: OriginKeysTable,
}

impl DataStore {
    pub fn open<C: redis::IntoConnectionInfo>(config: C) -> Result<Self> {
        // JW TODO: tune pool from config?
        let pool_cfg = Default::default();
        let manager = RedisConnectionManager::new(config).unwrap();
        let pool = Arc::new(ConnectionPool::new(pool_cfg, manager).unwrap());
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pool3 = pool.clone();
        let packages = PackagesTable::new(pool1);
        let views = ViewsTable::new(pool2);
        let origin_keys = OriginKeysTable::new(pool3);
        Ok(DataStore {
            pool: pool,
            packages: packages,
            views: views,
            origin_keys: origin_keys,
        })
    }

    /// Truncates every database in the datastore.
    ///
    /// # Failures
    ///
    /// * If a read-write transaction could not be acquired for any of the databases in the
    ///   datastore
    pub fn clear(&self) -> Result<()> {
        try!(redis::cmd("FLUSHDB").query(self.pool.get().unwrap().deref()));
        Ok(())
    }

    pub fn key_count(&self) -> Result<usize> {
        let count = try!(redis::cmd("DBSIZE").query(self.pool.get().unwrap().deref()));
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

    fn write(&self, record: &depotsrv::Package) -> result::Result<(), dbcache::Error> {
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
        Ok(())
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

    pub fn list(&self,
                id: &str,
                offset: isize,
                count: isize)
                -> Result<Vec<depotsrv::PackageIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.zrange::<String, Vec<String>>(Self::key(&id.to_string()), offset, count) {
            Ok(ids) => {
                let ids = ids.iter()
                    .map(|id| {
                        let p = package::PackageIdent::from_str(id).unwrap();
                        depotsrv::PackageIdent::from(p)
                    })
                    .collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest<T: Identifiable>(&self, id: &T) -> Result<depotsrv::PackageIdent> {
        let conn = self.pool().get().unwrap();
        let key = PackagesIndex::key(&id.to_string());
        match redis::cmd("SORT")
            .arg(key)
            .arg("LIMIT")
            .arg(0)
            .arg(1)
            .arg("ALPHA")
            .arg("DESC")
            .query::<Vec<String>>(conn.deref()) {
            Ok(ids) => {
                if ids.is_empty() {
                    return Err(Error::DataStore(dbcache::Error::EntityNotFound));
                }
                let ident = package::PackageIdent::from_str(&ids[0]).unwrap();
                Ok(depotsrv::PackageIdent::from(ident))
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Returns a vector of package identifiers matching a partial pattern.
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
                  -> Result<Vec<depotsrv::PackageIdent>> {
        let min = format!("[{}", partial);
        let max = format!("[{}{}", partial, r"xff");
        let conn = self.pool().get().unwrap();
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
                Ok(i)
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
            .ignore();
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
}

impl Bucket for PackagesIndex {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "package:ident:index"
    }
}

/// Contains a mapping of view names and the packages found within that view.
///
/// This is how packages will be "promoted" between environments without duplicating data on disk.
pub struct ViewsTable {
    pool: Arc<ConnectionPool>,
    pub pkg_view_idx: PkgViewIndex,
    pub view_pkg_idx: ViewPkgIndex,
}

impl ViewsTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pkg_view_idx = PkgViewIndex::new(pool1);
        let view_pkg_idx = ViewPkgIndex::new(pool2);

        ViewsTable {
            pool: pool,
            pkg_view_idx: pkg_view_idx,
            view_pkg_idx: view_pkg_idx,
        }
    }

    pub fn all(&self) -> Result<Vec<String>> {
        let conn = self.pool.get().unwrap();
        match conn.smembers(Self::prefix()) {
            Ok(members) => Ok(members),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn associate(&self, view: &str, pkg: &depotsrv::Package) -> Result<()> {
        let script = redis::Script::new(r"
            redis.call('sadd', KEYS[1], ARGV[2]);
            redis.call('zadd', KEYS[2], 0, ARGV[1]);
        ");
        try!(script.arg(pkg.get_ident().to_string())
            .arg(view.clone())
            .key(PkgViewIndex::key(&pkg.get_ident()))
            .key(ViewPkgIndex::key(&view.to_string()))
            .invoke(self.pool.get().unwrap().deref()));
        Ok(())
    }

    pub fn is_member(&self, view: &str) -> Result<bool> {
        let conn = self.pool.get().unwrap();
        match conn.sismember(Self::prefix(), view) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(&self, view: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.sadd(Self::prefix(), view));
        Ok(())
    }
}

impl Bucket for ViewsTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "views"
    }
}

impl BasicSet for ViewsTable {
    type Record = depotsrv::View;
}

pub struct PkgViewIndex {
    pool: Arc<ConnectionPool>,
}

impl PkgViewIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        PkgViewIndex { pool: pool }
    }
}

impl Bucket for PkgViewIndex {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "pkg:view:index"
    }
}

impl IndexSet for PkgViewIndex {
    type Key = String;
    type Value = String;
}

pub struct ViewPkgIndex {
    pool: Arc<ConnectionPool>,
}

impl ViewPkgIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        ViewPkgIndex { pool: pool }
    }

    pub fn all(&self, view: &str, pkg: &str) -> Result<Vec<package::PackageIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.zscan_match::<String, String, (String, u32)>(Self::key(&view.to_string()),
                                                                format!("{}*", pkg)) {
            Ok(set) => {
                let set: Vec<package::PackageIdent> =
                    set.map(|(id, _)| package::PackageIdent::from_str(&id).unwrap())
                        .collect();
                Ok(set)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn is_member<T: Identifiable>(&self, view: &str, pkg: &T) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        match conn.sismember(Self::key(&view.to_string()), pkg.to_string()) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest(&self, view: &str, pkg: &str) -> Result<depotsrv::PackageIdent> {
        match self.all(view, pkg) {
            Ok(mut ids) => {
                if let Some(id) = ids.pop() {
                    Ok(id.into())
                } else {
                    Err(Error::DataStore(dbcache::Error::EntityNotFound))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Bucket for ViewPkgIndex {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "view:pkg:index"
    }
}

impl IndexSet for ViewPkgIndex {
    type Key = String;
    type Value = String;
}

pub struct OriginKeysTable {
    pool: Arc<ConnectionPool>,
}

impl OriginKeysTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginKeysTable { pool: pool }
    }

    pub fn all(&self, origin: &str) -> Result<Vec<depotsrv::OriginKeyIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.smembers::<String, Vec<String>>(Self::key(&origin.to_string())) {
            Ok(ids) => {
                let ids = ids.iter()
                    .map(|rev| {
                        let mut ident = depotsrv::OriginKeyIdent::new();
                        ident.set_location(format!("/origins/{}/keys/{}", &origin, &rev));
                        ident.set_origin(origin.to_string());
                        ident.set_revision(rev.to_string());
                        ident
                    })
                    .collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(&self, origin: &str, revision: &str) -> Result<()> {
        let conn = self.pool().get().unwrap();
        try!(conn.sadd(OriginKeysTable::key(&origin.to_string()), revision));
        Ok(())
    }

    /// return the latest revision for a given origin key
    pub fn latest(&self, origin: &str) -> Result<String> {
        let conn = self.pool().get().unwrap();
        let key = OriginKeysTable::key(&origin.to_string());

        match redis::cmd("SORT")
            .arg(key)
            .arg("LIMIT")
            .arg(0)
            .arg(1)
            .arg("ALPHA")
            .arg("DESC")
            .query::<Vec<String>>(conn.deref()) {
            Ok(ids) => {
                if ids.is_empty() {
                    return Err(Error::DataStore(dbcache::Error::EntityNotFound));
                }
                Ok(ids[0].to_string())
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Bucket for OriginKeysTable {
    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin_keys"
    }
}

impl IndexSet for OriginKeysTable {
    type Key = String;
    type Value = String;
}
