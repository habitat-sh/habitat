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

use std::sync::Arc;

use config::Config;
use db::pool::Pool;
use postgres;
use protocol::jobsrv;
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
    pub fn from_pool(pool: Pool, _: Arc<String>) -> Result<DataStore> {
        Ok(DataStore { pool: pool })
    }

    /// Setup the datastore.
    ///
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<()> {
        Ok(())
    }

    pub fn get_job_graph_packages(&self) -> Result<RepeatedField<jobsrv::JobGraphPackage>> {
        let mut packages = RepeatedField::new();

        let conn = self.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_graph_packages_v1()", &[])
            .map_err(Error::JobGraphPackagesGet)?;

        if rows.is_empty() {
            warn!("No packages found");
            return Ok(packages);
        }

        for row in rows {
            let package = self.row_to_job_graph_package(&row)?;
            packages.push(package);
        }

        Ok(packages)
    }

    pub fn get_job_graph_package(&self, ident: &str) -> Result<jobsrv::JobGraphPackage> {
        let conn = self.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_graph_package_v1($1)", &[&ident])
            .map_err(Error::JobGraphPackagesGet)?;

        if rows.is_empty() {
            error!("No package found");
            return Err(Error::UnknownJobGraphPackage);
        }

        assert!(rows.len() == 1);
        let package = self.row_to_job_graph_package(&rows.get(0))?;
        Ok(package)
    }

    fn row_to_job_graph_package(
        &self,
        row: &postgres::rows::Row,
    ) -> Result<jobsrv::JobGraphPackage> {
        let mut package = jobsrv::JobGraphPackage::new();

        let name: String = row.get("ident");
        package.set_ident(name);

        if let Some(Ok(target)) = row.get_opt::<&str, String>("target") {
            package.set_target(target);
        }

        let deps: Vec<String> = row.get("deps");

        let mut pb_deps = RepeatedField::new();

        for dep in deps {
            pb_deps.push(dep);
        }

        package.set_deps(pb_deps);

        Ok(package)
    }
}
