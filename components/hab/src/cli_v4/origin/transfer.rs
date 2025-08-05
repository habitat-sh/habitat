// Implemenatation of `hab origin transfer`

use clap_v4 as clap;

use crate::{api_client::{self,
                         Client},
            cli_v4::utils::{AuthToken,
                            BldrUrl},
            error::{Error,
                    Result as HabResult},
            PRODUCT,
            VERSION};
use clap::Parser;
use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use habitat_core::origin::Origin;

use reqwest::StatusCode;

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct OriginTransferOptions {
    /// The origin name
    #[arg(name = "ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Origin,

    #[arg(name = "NEW_OWNER_ACCOUNT")]
    new_owner_account: String,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,
}

impl OriginTransferOptions {
    pub(super) async fn do_transfer(&self, ui: &mut UI) -> HabResult<()> {
        // Resolve token from CLI / env / config
        let token = self.auth_token
                        .from_cli_or_config()
                        .map_err(|e| Error::ArgumentError(e.to_string()))?;

        // Build the Builder endpoint URL
        let endpoint = self.bldr_url.to_string();

        // Instantiate the API client
        let api_client = Client::new(endpoint, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

        // Show transferring status
        ui.status(Status::Transferring,
                  format!("ownership of origin {} to {}.",
                          self.origin, self.new_owner_account))?;

        // Call the transfer API and handle outcomes
        match api_client.transfer_origin_ownership(self.origin.as_ref(),
                                                   &token,
                                                   &self.new_owner_account)
                        .await
        {
            Ok(_) => {
                ui.status(Status::Transferred, "ownership successfully!".to_string())
                  .or(Ok(()))
            }
            Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
                ui.fatal("Failed to transfer origin ownership!")?;
                ui.fatal("Either you are not the current owner or the new owner is not yet a \
                          member.")?;
                Err(Error::APIClient(err))
            }
            Err(err @ api_client::Error::APIError(StatusCode::UNAUTHORIZED, _)) => {
                ui.fatal("Failed to transfer origin ownership!")?;
                Err(Error::APIClient(err))
            }
            Err(err @ api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY, _)) => {
                ui.fatal("Failed to transfer origin ownership!")?;
                ui.fatal("This situation could arise if, for example, you attempted to transfer \
                          ownership to yourself.")?;
                Err(Error::APIClient(err))
            }
            Err(err @ api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
                ui.fatal("Failed to transfer origin ownership!")?;
                ui.fatal("The origin or the new origin owner account (or both) does not exist.")?;
                Err(Error::APIClient(err))
            }
            Err(e) => {
                ui.fatal(format!("Failed to transfer origin {} ownership to {}, {:?}",
                                 self.origin, self.new_owner_account, e))?;
                Err(Error::from(e))
            }
        }
    }
}
