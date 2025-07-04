use clap_v4 as clap;

use crate::error::Result as HabResult;
use clap::Subcommand;
use habitat_common::{ui::UI,
                     FeatureFlag};

mod create;
mod delete;
mod depart;
mod info;
mod invitations;
mod key;
mod rbac;
mod secret;
mod transfer;

#[derive(Clone, Debug, Subcommand)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          about = "Commands relating to Habitat Builder origins",
          arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]

pub(super) enum OriginCommand {
    /// Creates a new Builder origin
    Create(create::OriginCreateOptions),

    /// Removes an unused/empty origin
    Delete(delete::OriginDeleteOptions),

    /// Departs membership from selected origin
    Depart(depart::OriginDepartOptions),

    /// Displays general information about an origin
    Info(info::OriginInfoOptions),

    /// Manage origin member invitations
    #[command(subcommand)]
    Invitations(invitations::OriginInvitationsOptions),

    /// Commands relating to Habitat origin key maintenance
    #[command(subcommand)]
    Key(key::OriginKeyOptions),

    /// Role Based Access Control for origin members
    #[command(subcommand)]
    Rbac(rbac::OriginRbacOptions),

    /// Commands related to secret management
    #[command(subcommand)]
    Secret(secret::OriginSecretOptions),

    /// Transfers ownership of an origin to another member of that origin
    Transfer(transfer::OriginTransferOptions),
}

impl OriginCommand {
    pub(crate) async fn do_command(&self,
                                   ui: &mut UI,
                                   _feature_flags: FeatureFlag)
                                   -> HabResult<()> {
        match self {
            Self::Create(opts) => opts.do_create(ui).await,
            Self::Delete(opts) => opts.do_delete(ui).await,
            Self::Depart(opts) => opts.do_depart(ui).await,
            Self::Info(opts) => opts.do_info(ui).await,
            Self::Invitations(opts) => opts.execute(ui).await,
            Self::Key(opts) => opts.execute(ui).await,
            Self::Rbac(opts) => opts.execute(ui).await,
            Self::Secret(opts) => opts.execute(ui).await,
            Self::Transfer(opts) => opts.do_transfer(ui).await,
        }
    }
}
