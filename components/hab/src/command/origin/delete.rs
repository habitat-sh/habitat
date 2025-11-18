use crate::{PRODUCT,
            VERSION,
            api_client::{self,
                         Client},
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};
use reqwest::StatusCode;

pub async fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Deleting, format!("origin {}.", origin))?;

    match api_client.delete_origin(origin, token).await {
        Ok(_) => {
            ui.status(Status::Deleted, format!("origin {}.", origin))
              .map_err(Error::from)
        }
        Err(api_client::Error::APIError(StatusCode::CONFLICT, msg)) => {
            ui.fatal(format!("Unable to delete origin {}", origin))?;
            ui.fatal("Before you can delete this origin, delete all package artifacts, \
                      plan\nconnections, origin members, integrations and created channels.")?;
            ui.fatal("Delete any package artifacts with the command:\nhab pkg delete [OPTIONS] \
                      <PKG_IDENT> [PKG_TARGET]")?;
            ui.fatal("Delete any package plan connections (projects) under \"Settings\" for \
                      each package in the Builder web UI.")?;
            ui.fatal("Remove any origin members under \"Members\" in the Builder web UI.")?;
            ui.fatal("Delete any origin integrations under \"Integrations\" in the Builder web \
                      UI.")?;
            ui.fatal("Delete any user created origin channels with the command:\nhab bldr \
                      channel destroy [OPTIONS] <CHANNEL>")?;
            Err(Error::APIClient(api_client::Error::APIError(StatusCode::CONFLICT, msg)))
        }
        Err(e) => {
            ui.fatal(format!("Failed to delete origin {}, {:?}", origin, e))?;
            Err(Error::from(e))
        }
    }
}
