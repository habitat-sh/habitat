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

use std::{path::Path, result, str::FromStr};

use crate::hcore::package::PackageIdent;
use clap::{App, Arg};
use url::Url;

use crate::RegistryType;

/// The version of this library and program when built.
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// A Docker-specific clap:App wrapper
#[derive(Clone)]
pub struct Cli<'a, 'b>
where
    'a: 'b,
{
    pub app: App<'a, 'b>,
}

pub struct PkgIdentArgOptions {
    pub multiple: bool,
}

impl<'a, 'b> Cli<'a, 'b> {
    pub fn new(name: &str, about: &'a str) -> Self {
        Cli {
            app: clap_app!(
            (name) =>
            (about: about)
            (version: VERSION)
            (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n\n")
            (@arg IMAGE_NAME: --("image-name") -i +takes_value
                "Image name (default: \"{{pkg_origin}}/{{pkg_name}}\" supports: \
                 {{pkg_origin}}, {{pkg_name}}, {{pkg_version}}, {{pkg_release}}, {{channel}})")
            ),
        }
    }

    pub fn add_base_packages_args(self) -> Self {
        let app = self
            .app
            .arg(
                Arg::with_name("HAB_PKG")
                    .long("hab-pkg")
                    .value_name("HAB_PKG")
                    .validator(valid_ident_or_hart)
                    .help(
                        "Habitat CLI package identifier (ex: acme/redis) or filepath to a Habitat \
                         artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) \
                         to install (default: core/hab)",
                    ),
            )
            .arg(
                Arg::with_name("HAB_LAUNCHER_PKG")
                    .long("launcher-pkg")
                    .value_name("HAB_LAUNCHER_PKG")
                    .validator(valid_ident_or_hart)
                    .help(
                        "Launcher package identifier (ex: core/hab-launcher) or filepath to a \
                         Habitat artifact (ex: \
                         /home/core-hab-launcher-6083-20171101045646-x86_64-linux.hart) to \
                         install (default: core/hab-launcher)",
                    ),
            )
            .arg(
                Arg::with_name("HAB_SUP_PKG")
                    .long("sup-pkg")
                    .value_name("HAB_SUP_PKG")
                    .validator(valid_ident_or_hart)
                    .help(
                        "Supervisor package identifier (ex: core/hab-sup) or filepath to a \
                         Habitat artifact (ex: \
                         /home/ore-hab-sup-0.39.1-20171118011657-x86_64-linux.hart) to install \
                         (default: core/hab-sup)",
                    ),
            );

        Cli { app }
    }

    pub fn add_builder_args(self) -> Self {
        let app = self
            .app
            .arg(
                Arg::with_name("BLDR_URL")
                    .long("url")
                    .short("u")
                    .value_name("BLDR_URL")
                    .validator(valid_url)
                    .help(
                        "Install packages from Builder at the specified URL \
                         (default: https://bldr.habitat.sh)",
                    ),
            )
            .arg(
                Arg::with_name("CHANNEL")
                    .long("channel")
                    .short("c")
                    .value_name("CHANNEL")
                    .help("Install packages from the specified release channel (default: stable)"),
            )
            .arg(
                Arg::with_name("BASE_PKGS_BLDR_URL")
                    .long("base-pkgs-url")
                    .value_name("BASE_PKGS_BLDR_URL")
                    .validator(valid_url)
                    .help(
                        "Install base packages from Builder at the specified URL \
                         (default: https://bldr.habitat.sh)",
                    ),
            )
            .arg(
                Arg::with_name("BASE_PKGS_CHANNEL")
                    .long("base-pkgs-channel")
                    .value_name("BASE_PKGS_CHANNEL")
                    .help(
                        "Install base packages from the specified release channel \
                         (default: stable)",
                    ),
            )
            .arg(
                Arg::with_name("BLDR_AUTH_TOKEN")
                    .long("auth")
                    .short("z")
                    .value_name("BLDR_AUTH_TOKEN")
                    .help("Provide a Builder auth token for private pkg export"),
            );

        Cli { app }
    }

    pub fn add_tagging_args(self) -> Self {
        let app = self
            .app
            .arg(
                Arg::with_name("TAG_VERSION_RELEASE")
                    .long("tag-version-release")
                    .conflicts_with("NO_TAG_VERSION_RELEASE")
                    .help("Tag image with :\"{{pkg_version}}-{{pkg_release}}\" (default: yes)"),
            )
            .arg(
                Arg::with_name("NO_TAG_VERSION_RELEASE")
                    .long("no-tag-version-release")
                    .conflicts_with("TAG_VERSION_RELEASE")
                    .help(
                        "Do not tag image with :\"{{pkg_version}}-{{pkg_release}}\" (default: no)",
                    ),
            )
            .arg(
                Arg::with_name("TAG_VERSION")
                    .long("tag-version")
                    .conflicts_with("NO_TAG_VERSION")
                    .help("Tag image with :\"{{pkg_version}}\" (default: yes)"),
            )
            .arg(
                Arg::with_name("NO_TAG_VERSION")
                    .long("no-tag-version")
                    .conflicts_with("TAG_VERSION")
                    .help("Do not tag image with :\"{{pkg_version}}\" (default: no)"),
            )
            .arg(
                Arg::with_name("TAG_LATEST")
                    .long("tag-latest")
                    .conflicts_with("NO_TAG_LATEST")
                    .help("Tag image with :\"latest\" (default: yes)"),
            )
            .arg(
                Arg::with_name("NO_TAG_LATEST")
                    .long("no-tag-latest")
                    .conflicts_with("TAG_LATEST")
                    .help("Do not tag image with :\"latest\" (default: no)"),
            )
            .arg(
                Arg::with_name("TAG_CUSTOM")
                    .long("tag-custom")
                    .value_name("TAG_CUSTOM")
                    .help(
                        "Tag image with additional custom tag (supports: {{pkg_origin}}, \
                         {{pkg_name}}, {{pkg_version}}, {{pkg_release}}, {{channel}})",
                    ),
            );

        Cli { app }
    }

    pub fn add_publishing_args(self) -> Self {
        let app = self
            .app
            .arg(
                Arg::with_name("PUSH_IMAGE")
                    .long("push-image")
                    .conflicts_with("NO_PUSH_IMAGE")
                    .requires_all(&["REGISTRY_USERNAME", "REGISTRY_PASSWORD"])
                    .help("Push image to remote registry (default: no)"),
            )
            .arg(
                Arg::with_name("NO_PUSH_IMAGE")
                    .long("no-push-image")
                    .conflicts_with("PUSH_IMAGE")
                    .help("Do not push image to remote registry (default: yes)"),
            )
            .arg(
                Arg::with_name("REGISTRY_USERNAME")
                    .long("username")
                    .short("U")
                    .value_name("REGISTRY_USERNAME")
                    .requires("REGISTRY_PASSWORD")
                    .help(
                        "Remote registry username, required for pushing image to remote registry",
                    ),
            )
            .arg(
                Arg::with_name("REGISTRY_PASSWORD")
                    .long("password")
                    .short("P")
                    .value_name("REGISTRY_PASSWORD")
                    .requires("REGISTRY_USERNAME")
                    .help(
                        "Remote registry password, required for pushing image to remote registry",
                    ),
            )
            .arg(
                Arg::with_name("REGISTRY_TYPE")
                    .possible_values(RegistryType::variants())
                    .long("registry-type")
                    .short("R")
                    .value_name("REGISTRY_TYPE")
                    .help("Remote registry type (default: docker)"),
            )
            .arg(
                Arg::with_name("REGISTRY_URL")
                    // This is not strictly a requirement but will keep someone from
                    // making a mistake when inputing an ECR URL
                    .required_if("REGISTRY_TYPE", "amazon")
                    .required_if("REGISTRY_TYPE", "azure")
                    .long("registry-url")
                    .short("G")
                    .value_name("REGISTRY_URL")
                    .help("Remote registry url"),
            )
            // Cleanup
            .arg(
                Arg::with_name("RM_IMAGE")
                    .long("rm-image")
                    .help("Remove local image from engine after build and/or push (default: no)"),
            );

        Cli { app }
    }

    pub fn add_pkg_ident_arg(self, options: PkgIdentArgOptions) -> Self {
        let help = if options.multiple {
            "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a \
             Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)"
        } else {
            "A Habitat package identifier (ex: acme/redis) and/or filepath to a Habitat Artifact \
             (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)"
        };

        let app = self.app.arg(
            Arg::with_name("PKG_IDENT_OR_ARTIFACT")
                .value_name("PKG_IDENT_OR_ARTIFACT")
                .required(true)
                .multiple(options.multiple)
                .help(help),
        );

        Cli { app }
    }

    pub fn add_memory_arg(self) -> Self {
        let app = self.app.arg(
            Arg::with_name("MEMORY_LIMIT")
                .value_name("MEMORY_LIMIT")
                .long("memory")
                .short("m")
                .help("Memory limit passed to docker build's --memory arg (ex: 2bg)"),
        );

        Cli { app }
    }
}

fn valid_ident_or_hart(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else if val.ends_with(".hart") {
        Err(format!("Habitat artifact file: '{}' not found", &val))
    } else {
        match PackageIdent::from_str(&val) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
