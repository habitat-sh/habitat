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

use crate::{export_docker as docker,
            hcore::service::ServiceBind};
use clap::{App,
           Arg};
use std::{result,
          str::FromStr};

/// A Kubernetes-specific clap:App wrapper
///
/// The API here is provided to make it possible to reuse the CLI code of the Kubernetes exporter.
/// The CLI argument addition is divided between multiple methods to allow you to only pick the
/// parts of the CLI that you need.
#[derive(Clone)]
pub struct Cli<'a, 'b>
where
    'a: 'b,
{
    pub app: App<'a, 'b>,
}

impl<'a, 'b> Cli<'a, 'b> {
    /// Create a `Cli`
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the CLI application to show in `--help' output.
    /// * `about` - The long description of the CLi to show in `--help' output.
    pub fn new(name: &str, about: &'a str) -> Self {
        let app = docker::Cli::new(name, about).app;

        Cli { app }
    }

    /// Convenient method to add all known arguments to the CLI.
    pub fn add_all_args(self) -> Self {
        self.add_docker_args()
            .add_output_args()
            .add_runtime_args()
            .add_secret_names_args()
            .add_bind_args()
    }

    pub fn add_docker_args(self) -> Self {
        let cli = docker::Cli { app: self.app };
        let app = cli
            .add_base_packages_args()
            .add_builder_args()
            .add_tagging_args()
            .add_publishing_args()
            .add_pkg_ident_arg(docker::PkgIdentArgOptions { multiple: false })
            .app
            .arg(
                Arg::with_name("NO_DOCKER_IMAGE")
                    .long("no-docker-image")
                    .short("d")
                    .help(
                        "Disable creation of the Docker image and only create a Kubernetes \
                         manifest",
                    ),
            );

        Cli { app }
    }

    pub fn add_output_args(self) -> Self {
        Cli {
            app: self.app.arg(
                Arg::with_name("OUTPUT")
                    .value_name("OUTPUT")
                    .long("output")
                    .short("o")
                    .help("Name of manifest file to create. Pass '-' for stdout (default: -)"),
            ),
        }
    }

    /// Add Habitat (operator) runtime arguments to the CLI.
    pub fn add_runtime_args(self) -> Self {
        Cli {
            app: self
                .app
                .arg(
                    Arg::with_name("K8S_NAME")
                        .value_name("K8S_NAME")
                        .long("k8s-name")
                        .help(
                            "The Kubernetes resource name (default: \
                             {{pkg_name}}-{{pkg_version}}-{{pkg_release}})",
                        ),
                )
                .arg(
                    Arg::with_name("COUNT")
                        .value_name("COUNT")
                        .long("count")
                        .validator(valid_natural_number)
                        .help("Count is the number of desired instances"),
                )
                .arg(
                    Arg::with_name("TOPOLOGY")
                        .value_name("TOPOLOGY")
                        .long("topology")
                        .short("t")
                        .possible_values(&["standalone", "leader"])
                        .help(
                            "A topology describes the intended relationship between peers within \
                             a Habitat service group. Specify either standalone or leader \
                             topology (default: standalone)",
                        ),
                )
                .arg(
                    Arg::with_name("GROUP")
                        .value_name("GROUP")
                        .long("service-group")
                        .short("g")
                        .help(
                            "Group is a logical grouping of services with the same package and \
                             topology type connected together in a ring (default: default)",
                        ),
                )
                .arg(
                    Arg::with_name("CONFIG")
                        .value_name("CONFIG")
                        .long("config")
                        .short("n")
                        .help(
                            "The path to Habitat configuration file in user.toml format. Habitat \
                             will use it for initial configuration of the service running in a \
                             Kubernetes cluster",
                        ),
                )
                .arg(
                    Arg::with_name("ENVIRONMENT")
                        .value_name("ENVIRONMENT")
                        .long("env")
                        .short("e")
                        .multiple(true)
                        .number_of_values(1)
                        .help("Additional environment variables to set for the service"),
                )
                .arg(
                    Arg::with_name("PERSISTENT_STORAGE")
                        .value_name("PERSISTENT_STORAGE")
                        .long("storage")
                        .help(
                            "Storage specification in form of <size>:<path>:<storage class name>. \
                             <size> uses the same format as Kubernetes' size field (e.g. 10Gi). \
                             <path> describes where the storage will be mounted. <storage class \
                             name> is the name of the storage class that will be used as a \
                             backing store; it is a Kubernetes platform-specific thing (GCE has \
                             its own classes, Azure - its own).",
                        ),
                ),
        }
    }

    pub fn add_secret_names_args(self) -> Self {
        Cli {
            app: self.app.arg(
                Arg::with_name("RING_SECRET_NAME")
                    .value_name("RING_SECRET_NAME")
                    .long("ring-secret-name")
                    .short("r")
                    .help(
                        "Name of the Kubernetes Secret that contains the ring key, which encrypts \
                         the communication between Habitat supervisors",
                    ),
            ),
        }
    }

    pub fn add_bind_args(self) -> Self {
        Cli {
            app: self.app.arg(
                Arg::with_name("BIND")
                    .value_name("BIND")
                    .long("bind")
                    .short("b")
                    .multiple(true)
                    .number_of_values(1)
                    .validator(valid_bind)
                    .help(
                        "Bind to another service to form a producer/consumer relationship, \
                         specified as name:service.group",
                    ),
            ),
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_natural_number(val: String) -> result::Result<(), String> {
    match val.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{} is not a natural number", val)),
    }
}

// TODO (JC) This might be worth moving to core if it could be used
// elsewhere.  `ServiceGroup` already has a similar `validate` fn
#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_bind(val: String) -> result::Result<(), String> {
    if let Err(e) = ServiceBind::from_str(&val) {
        Err(e.to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_natural_number() {
        valid_natural_number("99".to_owned()).unwrap();

        for &s in ["x", "", "#####", "0x11", "ab"].iter() {
            assert!(valid_natural_number(s.to_owned()).is_err());
        }
    }

    #[test]
    fn test_valid_bind_ok() {
        assert!(valid_bind("foo:service.group".to_owned()).is_ok());
    }

    #[test]
    fn test_valid_bind_err() {
        assert!(valid_bind("foo:service".to_owned()).is_err());
    }
}
