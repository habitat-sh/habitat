use clap_v4 as clap;

use clap::Parser;

use crate::{cli::AFTER_HELP,
            VERSION};

mod pkg;
use pkg::PkgCommand;

#[derive(Debug, Clone, Parser)]
#[command(name = "hab",
            version = VERSION,
            about = "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            after_help = AFTER_HELP,
            propagate_version = true,
            arg_required_else_help = true,
        )]
pub(crate) enum Hab {
    /// Commands relating to Habitat Builder
    Bldr(BldrCommand),

    /// Commands relating to Habitat runtime config
    Cli(CliCommand),

    Config(ConfigCommand),

    File(FileCommand),

    License(LicenseCommand),

    Origin(OriginCommand),

    /// Commands relating to Habitat packages
    #[clap(subcommand)]
    Pkg(PkgCommand),

    Plan(PlanCommand),

    Ring(RingCommand),

    Studio(StudioCommand),

    Sup(SupCommand),

    SupportBundle,

    Svc(SvcCommand),

    User(UserCommand),

    // Aliases Below
    Apply(ServiceConfigCommand),

    Install(PkgInstallCommand),

    Run(SupRunCommand),

    Setup(CacheKeyPath),

    Start(SvcStartCommand),

    Stop(SvcStopCommand),

    Term,
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct BldrCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct CliCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct ConfigCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct FileCommand;

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
pub(crate) struct SupCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct UserCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct ServiceConfigCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct PkgInstallCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SupRunCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct CacheKeyPath;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStartCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStopCommand;

pub fn cli_driver() {
    let cli = Hab::parse();
    eprintln!("Hab: {cli:#?}");
}
