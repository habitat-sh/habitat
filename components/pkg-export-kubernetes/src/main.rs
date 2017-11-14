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

extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_pkg_export_docker as export_docker;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

use clap::{App, Arg};
use handlebars::Handlebars;
use std::env;
use std::fmt;
use std::result;
use std::str::FromStr;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use hcore::channel;
use hcore::PROGRAM_NAME;
use hcore::url as hurl;
use hcore::env as henv;
use hcore::package::{PackageArchive, PackageIdent};
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};

use export_docker::{Cli, Credentials, BuildSpec, Naming};
use export_docker::Error as DockerError;

// Synced with the version of the Habitat operator.
pub const VERSION: &'static str = "0.1.0";

// Kubernetes manifest template
const MANIFESTFILE: &'static str = include_str!("../defaults/KubernetesManifest.hbs");
const BINDFILE: &'static str = include_str!("../defaults/KubernetesBind.hbs");

#[derive(Debug)]
enum Error {
    Docker(DockerError),
    HabitatCore(hcore::Error),
    InvalidBindSpec(String),
    TemplateRenderError(handlebars::TemplateRenderError),
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Docker(ref err) => format!("{}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::InvalidBindSpec(ref bind_spec) => {
                format!("Invalid bind specification '{}'", bind_spec)
            }
            Error::TemplateRenderError(ref err) => format!("{}", err),
            Error::IO(ref err) => format!("{}", err),
        };
        write!(f, "{}", msg)
    }
}

impl From<DockerError> for Error {
    fn from(err: DockerError) -> Self {
        Error::Docker(err)
    }
}


impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Self {
        Error::TemplateRenderError(err)
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Self {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

fn main() {
    env_logger::init().unwrap();
    let mut ui = get_ui();
    if let Err(e) = start(&mut ui) {
        let _ = ui.fatal(e);
        std::process::exit(1)
    }
}

fn get_ui() -> UI {
    let isatty = if henv::var(NONINTERACTIVE_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    let coloring = if henv::var(NOCOLORING_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Coloring::Never
    } else {
        Coloring::Auto
    };
    UI::default_with(coloring, isatty)
}

fn start(ui: &mut UI) -> result::Result<(), Error> {
    let m = cli().get_matches();
    debug!("clap cli args: {:?}", m);

    if !m.is_present("NO_DOCKER_IMAGE") {
        gen_docker_img(ui, &m)?;
    }
    gen_k8s_manifest(ui, &m)
}

fn gen_docker_img(ui: &mut UI, matches: &clap::ArgMatches) -> result::Result<(), Error> {
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
    let spec = BuildSpec::new_from_cli_matches(&matches, &default_channel, &default_url);
    let naming = Naming::new_from_cli_matches(&matches);

    let docker_image = export_docker::export(ui, spec, &naming)?;
    docker_image.create_report(
        ui,
        env::current_dir()?.join("results"),
    )?;

    if matches.is_present("PUSH_IMAGE") {
        let credentials = Credentials::new(
            naming.registry_type,
            matches.value_of("REGISTRY_USERNAME").unwrap(),
            matches.value_of("REGISTRY_PASSWORD").unwrap(),
        )?;
        docker_image.push(ui, &credentials, naming.registry_url)?;
    }
    if matches.is_present("RM_IMAGE") {
        docker_image.rm(ui)?;
    }

    Ok(())
}

fn gen_k8s_manifest(_ui: &mut UI, matches: &clap::ArgMatches) -> result::Result<(), Error> {
    let count = matches.value_of("COUNT").unwrap_or("1");
    let topology = matches.value_of("TOPOLOGY").unwrap_or("standalone");
    let group = matches.value_of("GROUP");
    let config_secret_name = matches.value_of("CONFIG_SECRET_NAME");
    let ring_secret_name = matches.value_of("RING_SECRET_NAME");
    // clap ensures that we do have the mandatory args so unwrap() is fine here
    let pkg_ident_str = matches.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();
    let pkg_ident = if Path::new(pkg_ident_str).is_file() {
        // We're going to use the `$pkg_origin/$pkg_name`, fuzzy form of a package
        // identifier to ensure that update strategies will work if desired
        PackageArchive::new(pkg_ident_str).ident()?
    } else {
        PackageIdent::from_str(pkg_ident_str)?
    };
    let image = match matches.value_of("IMAGE_NAME") {
        Some(i) => i.to_string(),
        None => pkg_ident.origin + "/" + &pkg_ident.name,
    };
    let bind = matches.value_of("BIND");

    let json = json!({
        "metadata_name": pkg_ident.name,
        "image": image,
        "count": count,
        "service_topology": topology,
        "service_group": group,
        "config_secret_name": config_secret_name,
        "ring_secret_name": ring_secret_name,
        "bind": bind,
    });

    let mut write: Box<Write> = match matches.value_of("OUTPUT") {
        Some(o) if o != "-" => Box::new(File::create(o)?),
        _ => Box::new(io::stdout()),
    };

    let r = Handlebars::new().template_render(MANIFESTFILE, &json)?;
    let mut out = r.lines().filter(|l| *l != "").collect::<Vec<_>>().join(
        "\n",
    ) + "\n";

    if let Some(binds) = matches.values_of("BIND") {
        for bind in binds {
            let split: Vec<&str> = bind.split(":").collect();
            if split.len() < 3 {
                return Err(Error::InvalidBindSpec(bind.to_string()));
            }

            let json = json!({
                "name": split[0],
                "service": split[1],
                "group": split[2],
            });

            out += &Handlebars::new().template_render(BINDFILE, &json)?;
        }
    }

    write.write(out.as_bytes())?;

    Ok(())
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a docker image and Kubernetes manifest for a Habitat package. Habitat \
                 operator must be deployed within the Kubernetes cluster to intercept the created \
                 objects.";

    let app = Cli::new(name, about)
        .add_base_packages_args()
        .add_builder_args()
        .add_tagging_args()
        .add_publishing_args()
        .app;

    app.arg(
        Arg::with_name("OUTPUT")
            .value_name("OUTPUT")
            .long("output")
            .short("o")
            .help(
                "Name of manifest file to create. Pass '-' for stdout (default: -)",
            ),
    ).arg(
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
                    within a service group. Specify either standalone or leader \
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
        )
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
        )
        .arg(
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
        )
        .arg(
            Arg::with_name("NO_DOCKER_IMAGE")
                .long("no-docker-image")
                .short("d")
                .help(
                    "Disable creation of docker image and only create Kubernetes manifest",
                ),
        )
        .arg(
            Arg::with_name("PKG_IDENT_OR_ARTIFACT")
                .value_name("PKG_IDENT_OR_ARTIFACT")
                .required(true)
                .help("Habitat package identifier (ex: acme/redis)"),
        )
}

fn valid_natural_number(val: String) -> result::Result<(), String> {
    match val.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("{} is not a natural number", val)),
    }
}
