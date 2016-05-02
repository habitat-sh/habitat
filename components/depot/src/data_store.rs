// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use dbcache::{self, ConnectionPool, Table};
use depot_core::data_object::{self, DataObject};
use hcore::package;
use r2d2_redis::RedisConnectionManager;
use redis::{self, Commands, PipelineCommands};
use rustc_serialize::json;

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

    pub fn get<T: AsRef<package::PackageIdent>>(&self, id: T) -> Result<data_object::Package> {
        let conn = self.pool().get().unwrap();
        match conn.get::<String, String>(Self::key(&id.as_ref().to_string())) {
            Ok(body) => {
                let pkg: data_object::Package = json::decode(&body).unwrap();
                Ok(pkg)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn write(&self, record: &data_object::Package) -> Result<()> {
        let conn = self.pool().get().unwrap();
        let keys = [Self::key(&record.ident.to_string()),
                    PackagesIndex::key(&record.ident.origin_idx()),
                    PackagesIndex::key(&record.ident.name_idx()),
                    PackagesIndex::key(&record.ident.version_idx().as_ref().unwrap())];
        try!(redis::transaction(conn.deref(), &keys, |txn| {
            let body = json::encode(&record).unwrap();
            txn.set(Self::key(&record.ident.to_string()), body)
               .ignore()
               .sadd(PackagesIndex::key(&record.ident.origin_idx()),
                     record.ident.clone())
               .ignore()
               .sadd(PackagesIndex::key(&record.ident.name_idx()),
                     record.ident.clone())
               .ignore()
               .sadd(PackagesIndex::key(&record.ident.version_idx().as_ref().unwrap()),
                     record.ident.clone())
               .ignore()
               .query(conn.deref())
        }));
        Ok(())
    }
}

impl Table for PackagesTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "package"
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

    pub fn all(&self, id: &str) -> Result<Vec<package::PackageIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.smembers::<String, Vec<String>>(Self::key(&id.to_string())) {
            Ok(ids) => {
                let ids = ids.iter()
                             .map(|id| package::PackageIdent::from_str(id).unwrap())
                             .collect();
                Ok(ids)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest<T: AsRef<package::PackageIdent>>(&self, id: T) -> Result<package::PackageIdent> {
        let conn = self.pool().get().unwrap();
        let key = PackagesIndex::key(&id.as_ref().to_string());
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
                Ok(ident)
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Table for PackagesIndex {
    type IdType = String;

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

    pub fn associate(&self, view: &str, pkg: &data_object::Package) -> Result<()> {
        let script = redis::Script::new(r"
            redis.call('sadd', KEYS[1], ARGV[2]);
            redis.call('zadd', KEYS[2], 0, ARGV[1]);
        ");
        try!(script.arg(pkg.ident.clone())
                   .arg(view.clone())
                   .key(PkgViewIndex::key(&pkg.ident))
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

impl Table for ViewsTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "views"
    }
}

pub struct PkgViewIndex {
    pool: Arc<ConnectionPool>,
}

impl PkgViewIndex {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        PkgViewIndex { pool: pool }
    }
}

impl Table for PkgViewIndex {
    type IdType = data_object::PackageIdent;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "pkg:view:index"
    }
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
                let set: Vec<package::PackageIdent> = set.map(|(id, _)| {
                                                             package::PackageIdent::from_str(&id)
                                                                 .unwrap()
                                                         })
                                                         .collect();
                Ok(set)
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn is_member<T: AsRef<package::PackageIdent>>(&self, view: &str, pkg: T) -> Result<bool> {
        let conn = self.pool().get().unwrap();
        match conn.sismember(Self::key(&view.to_string()), pkg.as_ref().to_string()) {
            Ok(result) => Ok(result),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn latest(&self, view: &str, pkg: &str) -> Result<package::PackageIdent> {
        match self.all(view, pkg) {
            Ok(mut ids) => {
                if let Some(id) = ids.pop() {
                    Ok(id)
                } else {
                    Err(Error::DataStore(dbcache::Error::EntityNotFound))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}

impl Table for ViewPkgIndex {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "view:pkg:index"
    }
}



pub struct OriginKeysTable {
    pool: Arc<ConnectionPool>,
}

impl OriginKeysTable {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        OriginKeysTable { pool: pool }
    }

    pub fn all(&self, origin: &str) -> Result<Vec<data_object::OriginKeyIdent>> {
        let conn = self.pool().get().unwrap();
        match conn.smembers::<String, Vec<String>>(Self::key(&origin.to_string())) {
            Ok(ids) => {
                let ids = ids.iter()
                             .map(|rev| {
                                 data_object::OriginKeyIdent::new(origin.to_string(),
                                                     rev.clone(),
                                                    format!("/origins/{}/keys/{}",
                                                            &origin, &rev))
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

impl Table for OriginKeysTable {
    type IdType = String;

    fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    fn prefix() -> &'static str {
        "origin_keys"
    }
}
