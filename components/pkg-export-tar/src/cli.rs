use clap::Parser;

use crate::common::{cli::clap_validators::{HabPackageInstallSourceValueParser,
                                           UrlValueParser},
                    consts::{DEFAULT_BUILDER_URL,
                             DEFAULT_HAB_LAUNCHER_PKG_IDENT,
                             DEFAULT_HAB_PKG_IDENT,
                             DEFAULT_HAB_SUP_PKG_IDENT}};

#[derive(Debug, Clone, Parser)]
#[command(
    name = "hab-pkg-export-tar",
    author = concat!("\nAuthors: ", clap::crate_authors!()),
    about = "Creates a tar package from a Habitat package",
    version = crate::VERSION,
    help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}",
    max_term_width = 100)]
pub(crate) struct Cli {
    /// Habitat CLI package identifier (ex: acme/redis) or filepath to a Habitat artifact
    /// (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) to install
    #[arg(name = "HAB_PKG",
          long = "hab-pkg",
          value_name = "HAB_PKG",
          value_parser = HabPackageInstallSourceValueParser,
          default_value = DEFAULT_HAB_PKG_IDENT)]
    pub(crate) hab_pkg: String,

#[derive(Clone)]
pub struct Cli<'a, 'b>
    where 'a: 'b
{
    pub app: App<'a, 'b>,
}

impl<'a, 'b> Cli<'a, 'b> {
    pub fn new(name: &str, about: &'a str) -> Self {
        Cli { app: clap_app!(
              (name) =>
              (about: about)
              (version: VERSION)
              (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n\n")
              ), }
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
                         /home/core-hab-launcher-13829-20200527165030-x86_64-linux.hart) to \
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
                         /home/core-hab-sup-1.6.39-20200527165021-x86_64-linux.hart) to install \
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
                    .help("Install packages from the specified release channel (default: LTS-2024)"),
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
                         (default: LTS-2024)",
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

    pub fn add_pkg_ident_arg(self) -> Self {
        let help = "A Habitat package identifier (ex: acme/redis) and/or filepath to a Habitat \
                    Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)";

        let app =
            self.app
                .arg(Arg::with_name("PKG_IDENT_OR_ARTIFACT").value_name("PKG_IDENT_OR_ARTIFACT")
                                                            .required(true)
                                                            .help(help));

        Cli { app }
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_ident_or_hart(val: String) -> result::Result<(), String> {
    match InstallSource::from_str(&val) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}", e)),
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
