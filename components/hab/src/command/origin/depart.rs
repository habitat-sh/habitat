use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use reqwest::StatusCode;

pub async fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Departing,
              format!("membership from origin {}.", origin))?;

    match api_client.depart_origin(origin, token).await {
        Ok(_) => {
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
            ui.fatal(format!("Failed to depart membership from origin {}, {:?}",
                             origin, e))?;
            Err(Error::from(e))
        }
    }
}
