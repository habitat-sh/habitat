// Implementation of `hab svc` subcommand

use clap_v4 as clap;

use clap::Subcommand;

use habitat_common::{ui::UI,
                     FeatureFlag};

use crate::error::Result as HabResult;

mod key;
use key::KeyCommand;

mod load;
use load::LoadCommand;

mod update;
use update::UpdateCommand;

mod start;
use start::StartCommand;

mod status;
use status::StatusCommand;

mod stop;
use stop::StopCommand;

mod unload;
use unload::UnloadCommand;

#[derive(Clone, Debug, Subcommand)]
#[command(rename_all = "kebab-case",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) enum SvcCommand {
    #[clap(subcommand)]
    Key(KeyCommand),

    Load(LoadCommand),

    Start(StartCommand),

    Status(StatusCommand),

    Stop(StopCommand),

    Unload(UnloadCommand),

    Update(UpdateCommand),
}

impl SvcCommand {
    pub(crate) async fn do_command(&self,
                                   ui: &mut UI,
                                   _feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        match self {
            Self::Load(load_cmd) => load_cmd.do_command().await,
            Self::Unload(unload_cmd) => unload_cmd.clone().do_command().await,
            Self::Key(KeyCommand::Generate(key_generate_cmd)) => {
                key_generate_cmd.do_command(ui).await
            }
            Self::Update(update_cmd) => update_cmd.do_command().await,
            Self::Start(start_cmd) => start_cmd.do_command().await,
            Self::Stop(stop_cmd) => stop_cmd.do_command().await,
            Self::Status(status_cmd) => status_cmd.do_command().await,
        }
    }
}
