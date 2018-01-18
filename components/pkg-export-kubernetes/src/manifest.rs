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

use export_docker::Result;

use habitat_sup::manager::service::{Topology, ServiceBind};
use manifestjson::ManifestJson;
use bind;

/// Represents a Kubernetes manifest.
#[derive(Debug, Clone)]
pub struct Manifest {
    /// The Habitat service name.
    pub metadata_name: String,
    /// The docker image.
    pub image: String,
    /// The number of desired instances in the service group.
    pub count: u64,
    /// The relationship of a service with peers in the same service group.
    pub service_topology: Topology,
    /// The logical group of services in the service group.
    pub service_group: Option<String>,
    /// The config file content (in base64 encoded format).
    pub config: Option<String>,
    /// The name of the Kubernetes secret that contains the ring key, which encrypts the
    /// communication between Habitat supervisors.
    pub ring_secret_name: Option<String>,

    /// Any binds, as `ServiceBind` instances.
    pub binds: Vec<ServiceBind>,
}

impl Manifest {
    ///
    /// Create a Manifest instance from command-line arguments passed as [`clap::ArgMatches`].
    ///
    /// [`clap::ArgMatches`]: https://kbknapp.github.io/clap-rs/clap/struct.ArgMatches.html
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

        let version_suffix = match pkg_ident.version {
            Some(v) => {
                pkg_ident
                    .release
                    .map(|r| format!("{}-{}", v, r))
                    .unwrap_or(v)
            }
            None => "latest".to_owned(),
        };
        let name = format!("{}-{}", pkg_ident.name, version_suffix);

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
            metadata_name: name,
            image: image,
            count: count,
            service_topology: topology,
            service_group: group,
            config: config,
            ring_secret_name: ring_secret_name,
            binds: binds,
        })
    }

    /// Generates the manifest as a string and writes it to `write`.
    pub fn generate(&mut self, write: &mut Write) -> Result<()> {
        let out: String = ManifestJson::new(&self).into();

        write.write(out.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_generation() {
        let mut m = Manifest {
            metadata_name: "nginx-latest".to_owned(),
            image: "core/nginx:latest".to_owned(),
            count: 3,
            service_topology: Default::default(),
            service_group: Some("group1".to_owned()),
            config: Some(base64::encode(&format!("{}", "port = 4444"))),
            ring_secret_name: Some("deltaechofoxtrot".to_owned()),
            binds: vec![],
        };

        let expected = include_str!("../tests/KubernetesManifestTest.yaml");

        let mut o = vec![];
        m.generate(&mut o).unwrap();

        let out = String::from_utf8(o).unwrap();

        assert_eq!(out, expected);
    }

    #[test]
    fn test_manifest_generation_binds() {
        let mut m = Manifest {
            metadata_name: "nginx-latest".to_owned(),
            image: "core/nginx:latest".to_owned(),
            count: 3,
            service_topology: Default::default(),
            service_group: Some("group1".to_owned()),
            config: None,
            ring_secret_name: Some("deltaechofoxtrot".to_owned()),
            binds: vec!["name1:service1.group1".parse().unwrap()],
        };

        let expected = include_str!("../tests/KubernetesManifestTestBinds.yaml");

        let mut o = vec![];
        m.generate(&mut o).unwrap();

        let out = String::from_utf8(o).unwrap();

        assert_eq!(out, expected);
    }
}
