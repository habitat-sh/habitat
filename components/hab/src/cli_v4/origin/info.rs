// Implemenatation of `hab origin info`

use clap_v4 as clap;

use crate::{cli_v4::utils::{BldrUrl,
                            AuthToken,
                            valid_origin},
            error::{Result as HabResult,
                    Error}};
use clap::Parser;
use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use crate::{api_client::Client,
            PRODUCT,
            VERSION};
use habitat_core::util::text_render::{PortableText,
    TabularText};

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct OriginInfoOptions {
    /// The origin to be deleted
    #[arg(name = "ORIGIN", value_parser = valid_origin)]
    origin:     String,

    /// Output will be rendered in json
    #[arg(name = "TO_JSON",
          short = 'j',
          long = "json")]
    to_json: bool,

    #[command(flatten)]
    bldr_url:   BldrUrl,

    #[command(flatten)]
    auth_token: AuthToken,
}

impl OriginInfoOptions {
    pub(super) async fn do_info(&self, ui: &mut UI) -> HabResult<()> {
        // Resolve the token
        let token = self
            .auth_token
            .from_cli_or_config()
            .map_err(|e| Error::ArgumentError(e.to_string()))?;

        // Build the endpoint URL
        let endpoint = self.bldr_url.to_string();

        // Create the Habit API client
        let api_client = Client::new(endpoint, PRODUCT, VERSION, None)
            .map_err(Error::APIClient)?;

        // Fetch the origin info
        match api_client.origin_info(&token, &self.origin).await {
            Ok(resp) => {
                if self.to_json {
                    // JSON output path
                    match resp.as_json() {
                        Ok(body) => {
                            println!("{}", body);
                            Ok(())
                        }
                        Err(e) => {
                            ui.fatal(format!("Failed to serialize to JSON: {:?}.", e))?;
                            Err(Error::from(e))
                        }
                    }
                } else {
                    ui.status(Status::Discovering, "origin metadata".to_string())?;
                    println!("Origin [{}]:", self.origin);
                    match resp.as_tabbed() {
                        Ok(body) => {
                            println!("{}", body);
                            Ok(())
                        }
                        Err(e) => {
                            ui.fatal(format!("Failed to format origin metadata: {:?}.", e))?;
                            Err(Error::from(e))
                        }
                    }
                }
            }
            Err(e) => {
                ui.fatal(format!("Failed to retrieve origin metadata: {:?}.", e))?;
                Err(Error::from(e))
            }
        }
    }
}