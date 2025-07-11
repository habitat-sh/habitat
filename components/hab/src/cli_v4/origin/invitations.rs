// Implemenatation of `hab origin invitations`

use clap_v4 as clap;

use crate::{cli_v4::utils::{valid_origin,
                            AuthToken,
                            BldrUrl},
            command::origin::invitations,
            error::{Error,
                    Result as HabResult}};
use clap::Parser;

use habitat_common::ui::UI;

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum OriginInvitationsCommand {
    /// Accept an origin member invitation
    Accept {
        /// The origin name the invitation applies to
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin: String,

        /// The id of the invitation to accept
        #[arg(name = "INVITATION_ID")]
        invitation_id: u64,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },

    /// Ignore an origin member invitation
    Ignore {
        /// The origin name the invitation applies to
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin: String,

        /// The id of the invitation to ignore
        #[arg(name = "INVITATION_ID")]
        invitation_id: u64,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },

    /// List origin invitations sent to your account
    List {
        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },

    /// List pending invitations for a particular origin. Requires that you are the origin owner
    Pending {
        /// The name of the origin you wish to list invitations for
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin: String,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },

    /// Rescind an existing origin member invitation
    Rescind {
        /// The origin name the invitation applies to
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin: String,

        /// The id of the invitation to rescind
        #[arg(name = "INVITATION_ID")]
        invitation_id: u64,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },

    /// Send an origin member invitation
    Send {
        /// The origin name the invitation applies to
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin: String,

        /// The account name to invite into the origin
        #[arg(name = "INVITEE_ACCOUNT")]
        invitee_account: String,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },
}

impl OriginInvitationsCommand {
    pub(super) async fn execute(&self, ui: &mut UI) -> HabResult<()> {
        // Helper to extract token+url
        fn get_token_and_endpoint(bldr: &BldrUrl,
                                  auth: &AuthToken)
                                  -> Result<(String, String), Error> {
            let url = bldr.to_string();
            let token = auth.from_cli_or_config()
                            .map_err(|e| Error::ArgumentError(e.to_string()))?;
            Ok((url, token))
        }

        match self {
            OriginInvitationsCommand::Accept { origin,
                                               invitation_id,
                                               bldr_url,
                                               auth_token, } => {
                let (url, token) = get_token_and_endpoint(bldr_url, auth_token)?;
                invitations::accept::start(ui, &url, origin, &token, *invitation_id).await
            }
            OriginInvitationsCommand::Ignore { origin,
                                               invitation_id,
                                               bldr_url,
                                               auth_token, } => {
                let (url, token) = get_token_and_endpoint(bldr_url, auth_token)?;
                invitations::ignore::start(ui, &url, origin, &token, *invitation_id).await
            }
            OriginInvitationsCommand::List { bldr_url,
                                             auth_token, } => {
                let (url, token) = get_token_and_endpoint(bldr_url, auth_token)?;
                invitations::list_user::start(ui, &url, &token).await
            }
            OriginInvitationsCommand::Pending { origin,
                                                bldr_url,
                                                auth_token, } => {
                let (url, token) = get_token_and_endpoint(bldr_url, auth_token)?;
                invitations::list_pending_origin::start(ui, &url, origin, &token).await
            }
            OriginInvitationsCommand::Rescind { origin,
                                                invitation_id,
                                                bldr_url,
                                                auth_token, } => {
                let (url, token) = get_token_and_endpoint(bldr_url, auth_token)?;
                invitations::rescind::start(ui, &url, origin, &token, *invitation_id).await
            }
            OriginInvitationsCommand::Send { origin,
                                             invitee_account,
                                             bldr_url,
                                             auth_token, } => {
                let (url, token) = get_token_and_endpoint(bldr_url, auth_token)?;
                invitations::send::start(ui, &url, origin, &token, invitee_account).await
            }
        }
    }
}
