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

use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use base64;
use clap::ArgMatches;
use hcore::package::{PackageArchive, PackageIdent};
use common::ui::UI;
use rand;
use rand::Rng;

use export_docker::Result;

use habitat_sup::manager::service::Topology;
use manifestjson::ManifestJson;
use bind;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub metadata_name: String,
    pub habitat_name: String,
    pub image: String,
    pub count: u64,
    pub service_topology: Topology,
    pub service_group: Option<String>,
    pub config: Option<String>,
    pub ring_secret_name: Option<String>,
    pub binds: Vec<bind::Bind>,
}

impl Manifest {
    pub fn new_from_cli_matches(_ui: &mut UI, matches: &ArgMatches) -> Result<Self> {
        let count = matches.value_of("COUNT").unwrap_or("1").parse()?;
        let topology: Topology = matches
            .value_of("TOPOLOGY")
            .unwrap_or("standalone")
            .parse()
            .unwrap_or(Default::default());
        let group = matches.value_of("GROUP").map(|s| s.to_string());
        let config_file = matches.value_of("CONFIG");
        let ring_secret_name = matches.value_of("RING_SECRET_NAME").map(|s| s.to_string());
        // clap ensures that we do have the mandatory args so unwrap() is fine here
        let pkg_ident_str = matches.value_of("PKG_IDENT_OR_ARTIFACT").expect(
            "No package specified",
        );
        let pkg_ident = if Path::new(pkg_ident_str).is_file() {
            // We're going to use the `$pkg_origin/$pkg_name`, fuzzy form of a package
            // identifier to ensure that update strategies will work if desired
            PackageArchive::new(pkg_ident_str).ident()?
        } else {
            PackageIdent::from_str(pkg_ident_str)?
        };

        // To allow multiple instances of Habitat application in Kubernetes,
        // random suffix in metadata_name is needed.
        let metadata_name =
            format!(
            "{}-{}",
            pkg_ident.name,
            rand::thread_rng()
                .gen_ascii_chars()
                .filter(|c| c.is_lowercase() || c.is_numeric())
                .take(5)
                .collect::<String>(),
        );

        let image = match matches.value_of("IMAGE_NAME") {
            Some(i) => i.to_string(),
            None => pkg_ident.origin + "/" + &pkg_ident.name + ":latest",
        };

        let binds = bind::parse_bind_args(&matches)?;

        let config = match config_file {
            None => None,
            Some(name) => {
                let mut contents = String::new();
                File::open(name)?.read_to_string(&mut contents)?;

                Some(base64::encode(&format!("{}", contents)))
            }
        };

        Ok(Manifest {
            metadata_name: metadata_name,
            habitat_name: pkg_ident.name,
            image: image,
            count: count,
            service_topology: topology,
            service_group: group,
            config: config,
            ring_secret_name: ring_secret_name,
            binds: binds,
        })
    }

    pub fn generate(&mut self, write: &mut Write) -> Result<()> {
        let out = ManifestJson::new(&self).into_string()?;

        write.write(out.as_bytes())?;

        Ok(())
    }
}
