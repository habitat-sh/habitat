// Implemenatation of `hab origin depart`

use clap_v4 as clap;

use crate::{PRODUCT,
            VERSION,
            api_client::{self,
                         Client},
            cli_v4::utils::{AuthToken,
                            BldrUrl},
            error::{Error,
                    Result as HabResult}};
use clap::Parser;

use habitat_core::origin::Origin;

use habitat_common::ui::{Status,
                         UI,
                         UIWriter};
use reqwest::StatusCode;

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct OriginDepartOptions {
    /// The origin name
    #[arg(name = "ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Origin,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,
}

impl OriginDepartOptions {
    pub(super) async fn do_depart(&self, ui: &mut UI) -> HabResult<()> {
        // Resolve token
        let token = self.auth_token
                        .from_cli_or_config()
                        .map_err(|e| Error::ArgumentError(e.to_string()))?;

        // Build endpoint
        let endpoint = self.bldr_url.to_string();

        // Create client
        let api_client = Client::new(endpoint, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

        // Show departing status
        ui.status(Status::Departing,
                  format!("membership from origin {}.", self.origin))?;

        // Call the API
        match api_client.depart_origin(self.origin.as_ref(), &token).await {
            Ok(_) => {
                // on success, we swallow any error from ui.status
                ui.status(Status::Departed, "membership successfully!".to_string())
                  .or(Ok(()))
            }
            Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
                ui.fatal("Failed to depart origin membership!")?;
                ui.fatal("You are the current owner and cannot depart the origin.")?;
                Err(Error::APIClient(err))
            }
            Err(err @ api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY, _)) => {
                ui.fatal("Failed to depart origin membership!")?;
                ui.fatal("You don't appear to be a member of the origin you're trying to depart \
                          from.")?;
                Err(Error::APIClient(err))
            }
            Err(e) => {
                ui.fatal(format!("Failed to depart membership from origin {}, {:?}.",
                                 self.origin, e))?;
                Err(Error::from(e))
            }
        }
    }
}
