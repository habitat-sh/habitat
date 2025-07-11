// Implemenatation of `hab origin rbac`

use clap_v4 as clap;

use crate::{cli_v4::utils::{AuthToken,
                            BldrOrigin,
                            BldrUrl},
            command::origin::rbac,
            error::{Error,
                    Result as HabResult}};
use clap::Parser;

use url::Url;

use habitat_common::ui::UI;

use habitat_core::origin::{Origin,
                           OriginMemberRole};

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum OriginRbacCommand {
    /// Display an origin member's current role
    Show {
        #[command(flatten)]
        origin: BldrOrigin,

        /// The account name of the role to display
        #[arg(value_name = "MEMBER_ACCOUNT")]
        member_account: String,

        /// Output will be rendered in json
        #[arg(name = "TO_JSON", short = 'j', long = "json")]
        to_json: bool,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },

    /// Change an origin member's role
    Set {
        #[command(flatten)]
        origin: BldrOrigin,

        /// The account name whose role will be changed
        #[arg(value_name = "MEMBER_ACCOUNT")]
        member_account: String,

        /// The role name to enforce for the member account
        #[arg(value_enum,
              ignore_case = false,
              value_name = "ROLE",
              help = "[possible values: READONLY_MEMBER, MEMBER, MAINTAINER, ADMINISTRATOR, \
                      OWNER]")]
        role: OriginMemberRole,

        #[command(flatten)]
        bldr_url: BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,

        /// Do not prompt for confirmation
        #[arg(short = 'n', long = "no-prompt")]
        no_prompt: bool,
    },
}

impl OriginRbacCommand {
    pub(super) async fn execute(&self, ui: &mut UI) -> HabResult<()> {
        fn get_args(bldr_url: &BldrUrl,
                    auth_token: &AuthToken,
                    origin: &BldrOrigin)
                    -> Result<(Url, String, Origin), Error> {
            // URL → Url
            let url = bldr_url.resolve()?;
            // Token → String
            let token = auth_token.resolve()?;
            // Origin → Origin
            let origin = origin.inner.clone();
            Ok((url, token, origin))
        }

        match self {
            OriginRbacCommand::Show { origin,
                                      member_account,
                                      to_json,
                                      bldr_url,
                                      auth_token, } => {
                let (url, token, origin_obj) = get_args(bldr_url, auth_token, origin)?;
                rbac::show_role::start(ui, url, origin_obj, &token, member_account, *to_json).await
            }

            OriginRbacCommand::Set { origin,
                                     member_account,
                                     role,
                                     bldr_url,
                                     auth_token,
                                     no_prompt, } => {
                let (url, token, origin_obj) = get_args(bldr_url, auth_token, origin)?;
                rbac::set_role::start(ui,
                                      url,
                                      origin_obj,
                                      &token,
                                      member_account,
                                      *role,
                                      *no_prompt).await
            }
        }
    }
}
