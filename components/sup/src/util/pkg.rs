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

use std::path::Path;

use common;
use common::ui::UI;
use depot_client::Client;
use hcore::fs::{self, FS_ROOT_PATH};
use hcore::package::{PackageIdent, PackageInstall};

use {PRODUCT, VERSION};
use error::Result;
use manager::ServiceSpec;

static LOGKEY: &'static str = "PK";

pub fn install(ui: &mut UI, depot_url: &str, ident: &PackageIdent) -> Result<PackageInstall> {
    let fs_root_path = Path::new(&*FS_ROOT_PATH);
    let installed_ident = common::command::package::install::start(ui,
                                                                   depot_url,
                                                                   &ident.to_string(),
                                                                   PRODUCT,
                                                                   VERSION,
                                                                   fs_root_path,
                                                                   &fs::cache_artifact_path(None),
                                                                   false)?;
    Ok(PackageInstall::load(&installed_ident, Some(&fs_root_path))?)
}

pub fn maybe_install_newer(ui: &mut UI,
                           spec: &ServiceSpec,
                           current: PackageInstall)
                           -> Result<PackageInstall> {
    let latest_ident: PackageIdent = {
        let depot_client = Client::new(&spec.depot_url, PRODUCT, VERSION, None)?;
        depot_client.show_package(&spec.ident)?.get_ident().clone().into()
    };

    if &latest_ident > current.ident() {
        outputln!("Newer version of {} detected. Installing {} from {}",
                  spec.ident,
                  latest_ident,
                  spec.depot_url);
        self::install(ui, &spec.depot_url, &latest_ident)
    } else {
        outputln!("Confirmed latest version of {} is {}",
                  spec.ident,
                  current.ident());
        Ok(current)
    }
}
