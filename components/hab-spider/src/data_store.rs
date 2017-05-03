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

use config::Config;
use db::pool::Pool;
use postgres;
use protocol::scheduler::*;
use protobuf::RepeatedField;
use error::{Result, Error};

// DataStore inherits Send + Sync by virtue of having only one member, the pool itself.
#[derive(Debug, Clone)]
pub struct DataStore {
    pool: Pool,
}

// Sample connection_url: "postgresql://hab@127.0.0.1/builder_scheduler"

impl DataStore {
    /// Create a new DataStore.
    ///
    /// * Can fail if the pool cannot be created
    /// * Blocks creation of the datastore on the existince of the pool; might wait indefinetly.
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore, vec![0])?;
        Ok(DataStore { pool: pool })
    }

    /// Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        Ok(DataStore { pool: pool })
    }

    /// Setup the datastore.
    ///
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<()> {
        Ok(())
    }

    pub fn insert_package(&self, msg: &Package) -> Result<()> {
        let conn = self.pool.get_shard(0)?;

        conn.execute("SELECT insert_package_v1($1, $2)",
                     &[&msg.get_ident(), &msg.get_deps()])
            .map_err(Error::PackageInsert)?;

        debug!("Package inserted: {}", msg.get_ident());

        Ok(())
    }

    pub fn get_packages(&self) -> Result<RepeatedField<Package>> {
        let mut packages = RepeatedField::new();

        let conn = self.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_packages_v1()", &[])
                        .map_err(Error::PackagesGet)?;

        if rows.is_empty() {
            warn!("No packages found");
            return Ok(packages);
        }

        for row in rows {
            let package = self.row_to_package(&row)?;
            packages.push(package);
        }

        Ok(packages)
    }

    fn row_to_package(&self, row: &postgres::rows::Row) -> Result<Package> {
        let mut package = Package::new();

        let name: String = row.get("ident");
        package.set_ident(name);

        let deps: Vec<String> = row.get("deps");

        let mut pb_deps = RepeatedField::new();

        for dep in deps {
            pb_deps.push(dep);
        }

        package.set_deps(pb_deps);

        Ok(package)
    }
}
