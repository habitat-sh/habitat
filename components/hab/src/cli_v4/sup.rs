use clap_v4 as clap;

use crate::error::Result as HabResult;
use clap::Subcommand;
use habitat_common::{FeatureFlag,
                     ui::UI};

mod bash;
mod depart;
mod restart;
mod secret;
mod sh;
pub(crate) mod sup_run;
mod term;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(super) enum SupCommand {
    /// Depart a Supervisor from the gossip ring; kicking and banning the target from joining again
    /// with the same member-id
    Depart(depart::SupDepartOptions),

    /// Restart a Supervisor without restarting its services
    Restart(restart::SupRestartOptions),

    /// Commands relating to a Habitat Supervisor's Control Gateway secret
    #[command(subcommand)]
    Secret(secret::SupSecretOptions),

    // Basic Commands (sh, bash , terminate)
    /// Start an interactive Bourne-like shell
    Sh(sh::SupShellCommand),

    /// Start an interactive Bash-like shell
    Bash(bash::SupBashCommand),

    /// Gracefully terminate the Habitat Supervisor and all of its running services
    Term(term::SupTermCommand),

    /// Run the supervisor (load config and start services)
    Run(sup_run::SupRunOptions),
}

impl SupCommand {
    pub(crate) async fn do_command(&self,
                                   ui: &mut UI,
                                   _feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        match self {
            Self::Depart(opts) => opts.do_depart(ui).await,
            Self::Restart(opts) => opts.do_restart().await,
            Self::Secret(opts) => opts.execute().await,
            Self::Sh(opts) => opts.execute(ui).await,
            Self::Bash(opts) => opts.execute(ui).await,
            Self::Term(opts) => opts.execute(ui).await,
            Self::Run(opts) => opts.do_run(ui).await,
        }
    }
}
