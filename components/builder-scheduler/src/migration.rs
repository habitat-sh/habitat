// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::path::PathBuf;

use protocol::scheduler as proto;
use bldr_core::file_walker::FileWalker;

use config::Config;
use data_store::DataStore;
use error::SrvResult;

struct DataMigrator {
    datastore: DataStore,
    packages_path: PathBuf,
}

impl DataMigrator {
    pub fn new(datastore: DataStore, packages_path: PathBuf) -> Self {
        DataMigrator {
            datastore: datastore,
            packages_path: packages_path,
        }
    }

    fn run(self) -> SrvResult<()> {
        println!(
            "DataMigrator running, packages path is: {:?}",
            self.packages_path
        );

        let packages = FileWalker::new(&self.packages_path);

        for p in packages {
            let msg = proto::PackageCreate::from(p);
            println!("Migrating package: {}", msg.get_ident());
            self.datastore.create_package(&msg)?;
        }

        Ok(())
    }
}

pub fn run(config: Config) -> SrvResult<()> {
    let datastore = {
        DataStore::new(&config)?
    };
    datastore.setup()?;

    let migrator = DataMigrator::new(datastore, PathBuf::from(config.migration_path));
    migrator.run()
}
