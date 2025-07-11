// Implemenatation of `hab origin delete`

use clap_v4 as clap;

use crate::{api_client::{self,
                         Client},
            cli_v4::utils::{valid_origin,
                            AuthToken,
                            BldrUrl},
            error::{Error,
                    Result as HabResult},
            PRODUCT,
            VERSION};
use clap::Parser;
use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use reqwest::StatusCode;

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct OriginDeleteOptions {
    /// The origin to be deleted
    #[arg(name = "ORIGIN", value_parser = valid_origin)]
    origin: String,

    #[command(flatten)]
    bldr_url: BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,
}

impl OriginDeleteOptions {
    pub(super) async fn do_delete(&self, ui: &mut UI) -> HabResult<()> {
        // Pull the token out of CLI / env / config
        let token = self.auth_token
                        .from_cli_or_config()
                        .map_err(|e| Error::ArgumentError(e.to_string()))?;

        // Build the endpoint URL string
        let endpoint = self.bldr_url.to_string();

        // Create the API client
        let api_client = Client::new(endpoint, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

        // Show deleting status
        ui.status(Status::Deleting, format!("origin {}.", self.origin))?;

        // Call delete_origin and handle the various outcomes
        match api_client.delete_origin(&self.origin, &token).await {
            Ok(_) => {
                ui.status(Status::Deleted, format!("origin {}.", self.origin))?;
                Ok(())
            }
            // If there are still artifacts etc, show all the fatal hints
            Err(api_client::Error::APIError(StatusCode::CONFLICT, msg)) => {
                ui.fatal(format!("Unable to delete origin {}.", self.origin))?;
                ui.fatal("Before you can delete this origin, delete all package artifacts, \
                          plan\nconnections, origin members, secrets, integrations and created \
                          channels.")?;
                ui.fatal("Delete any package artifacts with the command:\nhab pkg delete \
                          [OPTIONS] <PKG_IDENT> [PKG_TARGET]")?;
                ui.fatal("Delete any package plan connections (projects) under \"Settings\" for \
                          each package in the Builder web UI.")?;
                ui.fatal("Remove any origin members under \"Members\" in the Builder web UI.")?;
                ui.fatal("Delete any origin secrets with the command:\nhab origin secret delete \
                          [OPTIONS] <KEY_NAME>")?;
                ui.fatal("Delete any origin integrations under \"Integrations\" in the Builder \
                          web UI.")?;
                ui.fatal("Delete any user created origin channels with the command:\nhab bldr \
                          channel destroy [OPTIONS] <CHANNEL>")?;
                Err(Error::APIClient(api_client::Error::APIError(StatusCode::CONFLICT, msg)))
            }
            // Any other failure
            Err(e) => {
                ui.fatal(format!("Failed to delete origin {}, {:?}.", self.origin, e))?;
                Err(Error::from(e))
            }
        }
    }
}
