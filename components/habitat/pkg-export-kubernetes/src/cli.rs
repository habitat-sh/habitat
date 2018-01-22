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

use clap::{App, Arg};
use std::result;

use export_docker as docker;

/// A Kubernetes-specific clap:App wrapper
#[derive(Clone)]
pub struct Cli<'a, 'b>
where
    'a: 'b,
{
    pub app: App<'a, 'b>,
}

impl<'a, 'b> Cli<'a, 'b> {
    pub fn new(name: &str, about: &'a str) -> Self {
        let app = docker::Cli::new(name, about).app;

        Cli { app: app }
    }

    pub fn add_all_args(self) -> Self {
        self.add_docker_args()
            .add_output_args()
            .add_runtime_args()
            .add_secret_names_args()
            .add_bind_args()
    }

    pub fn add_docker_args(self) -> Self {
        let cli = docker::Cli { app: self.app };
        let app = cli.add_base_packages_args()
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

        Cli { app: app }
    }

    pub fn add_output_args(self) -> Self {
        Cli {
            app: self.app.arg(
                Arg::with_name("OUTPUT")
                    .value_name("OUTPUT")
                    .long("output")
                    .short("o")
                    .help(
                        "Name of manifest file to create. Pass '-' for stdout (default: -)",
                    ),
            ),
        }
    }

    pub fn add_runtime_args(self) -> Self {
        Cli {
            app: self.app
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
                            "A topology describes the intended relationship between peers \
                         within a Habitat service group. Specify either standalone or leader \
                         topology (default: standalone)",
                        ),
                )
                .arg(
                    Arg::with_name("GROUP")
                        .value_name("GROUP")
                        .long("service-group")
                        .short("g")
                        .help(
                            "group is a logical grouping of services with the same package and \
                         topology type connected together in a ring (default: default)",
                        ),
                ),
        }
    }

    pub fn add_secret_names_args(self) -> Self {
        Cli {
            app: self.app
                .arg(
                    Arg::with_name("CONFIG_SECRET_NAME")
                        .value_name("CONFIG_SECRET_NAME")
                        .long("config-secret-name")
                        .short("n")
                        .help(
                            "name of the Kubernetes Secret containing the config file - \
                         user.toml - that the user has previously created. Habitat will \
                         use it for initial configuration of the service",
                        ),
                )
                .arg(
                    Arg::with_name("RING_SECRET_NAME")
                        .value_name("RING_SECRET_NAME")
                        .long("ring-secret-name")
                        .short("r")
                        .help(
                            "name of the Kubernetes Secret that contains the ring key, which \
                         encrypts the communication between Habitat supervisors",
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
                    .help(
                        "Bind to another service to form a producer/consumer relationship, \
                         specified as name:service:group",
                    ),
            ),
        }
    }
}

fn valid_natural_number(val: String) -> result::Result<(), String> {
    match val.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{} is not a natural number", val)),
    }
}
