// Implemenatation of `hab origin create`

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

use habitat_core::origin::Origin;

use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use reqwest::StatusCode;

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct OriginCreateOptions {
    /// The origin to be created
    #[arg(name = "ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Origin,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,
}

impl OriginCreateOptions {
    pub(super) async fn do_create(&self, ui: &mut UI) -> HabResult<()> {
        // Build the endpoint URL string
        let endpoint = self.bldr_url.to_string();

        // Pull the token out (this will return an Err if it's missing)
        let token = self.auth_token
                        .from_cli_or_config()
                        .map_err(|e| Error::ArgumentError(e.to_string()))?;

        // Create the client
        let api_client = Client::new(endpoint, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

        ui.status(Status::Creating, format!("origin {}.", self.origin))?;

        // Call create_origin with &str for both args
        match api_client.create_origin(self.origin.as_ref(), &token).await {
            Ok(_) => {
                ui.status(Status::Created, format!("origin {}.", self.origin))?;
                Ok(())
            }
            Err(api_client::Error::APIError(StatusCode::CONFLICT, _)) => {
                ui.status(Status::Skipping,
                          format!("origin {} already exists, skipping", self.origin))?;
                Ok(())
            }
            Err(e) => {
                ui.fatal(format!("Failed to create origin {}, {:?}.", self.origin, e))?;
                Err(Error::from(e))
            }
        }
    }
}
