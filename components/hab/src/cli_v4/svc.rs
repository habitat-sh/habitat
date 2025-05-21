// Implementation of `hab svc` subcommand

use clap_v4 as clap;

use clap::Subcommand;

use habitat_common::{ui::UI,
                     FeatureFlag};

use crate::error::Result as HabResult;

mod bulk_load;
use bulk_load::BulkLoadCommand;

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
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum SvcCommand {
    BulkLoad(BulkLoadCommand),

    #[clap(subcommand)]
    Key(KeyCommand),

    Load(LoadCommand),

    Update(UpdateCommand),

    Start(StartCommand),

    Status(StatusCommand),

    Stop(StopCommand),

    Unload(UnloadCommand),
}

impl SvcCommand {
    pub(crate) async fn do_command(&self,
                                   _ui: &mut UI,
                                   _feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        Ok(())
    }
}
