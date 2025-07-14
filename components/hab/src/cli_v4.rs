use clap_v4 as clap;

use clap::Parser;

use habitat_common::{ui::UI,
                     FeatureFlag};

use crate::{error::Result as HabResult,
            AFTER_HELP_V4,
            VERSION};

mod config;
mod file;
mod pkg;
mod user;

use config::ConfigCommand;
use file::FileCommand;
use pkg::PkgCommand;
use user::UserCommand;

pub(crate) mod sup;
use sup::SupCommand;

mod utils;
use utils::CacheKeyPath;

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
    Bldr(BldrCommand),

    /// Commands relating to Habitat runtime config
    Cli(CliCommand),

    /// Commands relating to a Service's runtime config
    #[clap(subcommand)]
    Config(ConfigCommand),

    /// Commands relating to Habitat files
    #[clap(subcommand)]
    File(FileCommand),

    License(LicenseCommand),

    Origin(OriginCommand),

    /// Commands relating to Habitat packages
    #[clap(subcommand)]
    Pkg(PkgCommand),

    Plan(PlanCommand),

    Ring(RingCommand),

    Studio(StudioCommand),

    /// The Habitat Supervisor
    #[clap(subcommand)]
    Sup(SupCommand),

    SupportBundle,

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
            Self::Config(config_command) => config_command.do_command(ui).await,
            Self::File(file_command) => file_command.do_command(ui).await,
            Self::User(user_command) => user_command.do_command(ui).await,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct BldrCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct CliCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct LicenseCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct OriginCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct PlanCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct RingCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct StudioCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcCommand;

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
