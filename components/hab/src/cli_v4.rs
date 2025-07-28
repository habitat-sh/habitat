use clap_v4 as clap;

use clap::Parser;

use habitat_common::{ui::UI,
                     FeatureFlag};

use crate::{error::Result as HabResult,
            AFTER_HELP_V4,
            VERSION};

mod bldr;
use bldr::BldrCommand;

mod cli;
use cli::CliCommand;

mod config;
use config::ConfigCommand;

mod file;
use file::FileCommand;

mod pkg;
use pkg::PkgCommand;

mod ring;
use ring::RingCommand;

mod user;
use user::UserCommand;

mod supportbundle;
use supportbundle::SupportBundleOpts;

pub(crate) mod sup;
use sup::SupCommand;

mod origin;
use origin::OriginCommand;

mod svc;
use svc::SvcCommand;

pub(crate) mod utils;
use utils::CacheKeyPath;

mod license;
use license::LicenseCommand;

mod studio;
use studio::StudioOpts;

mod plan;
use plan::PlanCommand;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Parser)]
#[command(name = "hab",
            version = VERSION,
            about = "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>",
            after_help = AFTER_HELP_V4,
            arg_required_else_help = true,
            propagate_version = true,
            term_width = 100,
            help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}{after-help}\n",
        )]
enum Hab {
    /// Commands relating to Habitat Builder
    #[clap(subcommand)]
    Bldr(BldrCommand),

    /// Commands relating to Habitat runtime config
    #[clap(subcommand)]
    Cli(CliCommand),

    /// Commands relating to a Service's runtime config
    #[clap(subcommand)]
    Config(ConfigCommand),

    /// Commands relating to Habitat files
    #[clap(subcommand)]
    File(FileCommand),

    /// Commands relating to Habitat license agreements
    #[clap(subcommand)]
    License(LicenseCommand),

    /// Commands relating to Habitat Builder origins
    #[clap(subcommand)]
    Origin(OriginCommand),

    /// Commands relating to Habitat packages
    #[clap(subcommand)]
    Pkg(PkgCommand),

    /// Commands relating to plans and other app-specific configuration
    #[clap(subcommand)]
    Plan(PlanCommand),

    /// Commands relating to Habitat rings
    #[clap(subcommand)]
    Ring(RingCommand),

    /// Commands relating to Habitat Studios
    #[cfg(any(target_os = "macos",
              any(all(target_os = "linux",
                      any(target_arch = "x86_64", target_arch = "aarch64")),
                  all(target_os = "windows", target_arch = "x86_64"))))]
    #[command(name = "studio")]
    Studio(StudioOpts),

    /// The Habitat Supervisor
    #[clap(subcommand)]
    Sup(SupCommand),

    /// Create a tarball of Habitat Supervisor data to send to support
    #[command(name = "supportbundle")]
    SupportBundle(SupportBundleOpts),

    /// Commands relating to Habitat Services
    #[clap(subcommand)]
    Svc(SvcCommand),

    /// Commands relating to Habitat users
    #[clap(subcommand)]
    User(UserCommand),

    // Aliases Below
    #[clap(hide = true)]
    Apply(ServiceConfigCommand),

    #[clap(hide = true)]
    Install(PkgInstallCommand),

    #[clap(hide = true)]
    Run(SupRunCommand),

    #[clap(hide = true)]
    Setup(CacheKeyPath),

    #[clap(hide = true)]
    Start(SvcStartCommand),

    #[clap(hide = true)]
    Stop(SvcStopCommand),

    #[clap(hide = true)]
    Term,
}

impl Hab {
    async fn do_cli_command(&self, ui: &mut UI, feature_flags: FeatureFlag) -> HabResult<()> {
        match self {
            Self::Pkg(pkg_command) => pkg_command.do_command(ui, feature_flags).await,
            Self::Sup(sup_command) => sup_command.do_command(ui, feature_flags).await,
            Self::Origin(origin_command) => origin_command.do_command(ui, feature_flags).await,
            Self::Config(config_command) => config_command.do_command(ui).await,
            Self::File(file_command) => file_command.do_command(ui).await,
            Self::User(user_command) => user_command.do_command(ui).await,
            Self::Svc(svc_command) => svc_command.do_command(ui, feature_flags).await,
            Self::License(license_command) => license_command.do_command(ui).await,
            Self::Cli(cli_command) => cli_command.do_command(ui, feature_flags).await,
            Self::Bldr(bldr_command) => bldr_command.do_command(ui).await,
            Self::Ring(ring_command) => ring_command.do_command(ui).await,
            Self::Studio(studio_command) => studio_command.do_command(ui).await,
            Self::Plan(plan_command) => plan_command.do_command(ui).await,
            Self::SupportBundle(support_bundle_command) => {
                support_bundle_command.do_command(ui).await
            }
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct ServiceConfigCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct PkgInstallCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SupRunCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStartCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStopCommand;

pub async fn cli_driver(ui: &mut UI, feature_flags: FeatureFlag) -> HabResult<()> {
    let cli = Hab::parse();
    cli.do_cli_command(ui, feature_flags).await
}
