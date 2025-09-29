use clap_v4 as clap;

use clap::Parser;

use habitat_common::{ui::UI,
                     FeatureFlag};

use crate::command::sup::start;

use crate::{error::Result as HabResult,
            VERSION};

use crate::license::check_for_license_acceptance_and_prompt;

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

mod license;
use license::LicenseCommand;

mod studio;
use studio::StudioOpts;

mod plan;
use plan::PlanCommand;

mod welcome;
use welcome::WelcomeOpts;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Parser)]
#[command(name = "hab",
            version = VERSION,
            about = "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>",
            arg_required_else_help = true,
            propagate_version = true,
            term_width = 100,
            help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}\n",
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

    /// Display a welcome message for Habitat
    #[command(name = "welcome")]
    Welcome(WelcomeOpts),
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
            Self::Welcome(welcome_command) => welcome_command.do_command(ui).await,
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
    let args: Vec<String> = std::env::args().collect();

    // We must manually detect a supervisor version check and call the `hab-sup` binary to get the
    // true Supervisor version.
    if args.len() >= 3
       && args.get(1).is_some_and(|arg| arg == "sup")
       && args.get(2)
              .is_some_and(|arg| arg == "--version" || arg == "-V")
    {
        let os_args: Vec<std::ffi::OsString> = std::env::args_os().skip(2).collect();
        return start(ui, &os_args).await;
    }

    // Skip license check if user is just asking for help or version
    let skip_license_check =
        args.iter()
            .any(|arg| arg == "--help" || arg == "-h" || arg == "--version" || arg == "-V");

    if !skip_license_check {
        check_for_license_acceptance_and_prompt(ui)?;
    }

    let cli = Hab::parse();
    cli.do_cli_command(ui, feature_flags).await
}
