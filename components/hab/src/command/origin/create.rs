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

pub fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Creating, format!("origin {}.", origin))?;

    match api_client.create_origin(origin, token) {
        Ok(_) => {
            ui.status(Status::Created, format!("origin {}.", origin))?;
            Ok(())
        }
        Err(api_client::Error::APIError(StatusCode::CONFLICT, _msg)) => {
            ui.status(Status::Skipping,
                      format!("creation of origin {}. Origin already exists!", origin))?;
            Ok(())
        }
        Err(e) => {
            ui.fatal(format!("Failed to create origin {}, {:?}.", origin, e))?;
            Err(Error::from(e))
        }
    }
}
