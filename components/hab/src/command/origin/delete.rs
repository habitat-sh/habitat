use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use hyper::status::StatusCode;

pub fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Deleting, format!("origin {}.", origin))?;

    match api_client.delete_origin(origin, token) {
        Ok(_) => {
            ui.status(Status::Deleted, format!("origin {}.", origin))
              .map_err(Into::into)
        }
        Err(api_client::Error::APIError(StatusCode::Conflict, msg)) => {
            ui.fatal(format!("Unable to delete origin {}", origin))?;
            ui.fatal("Origins may only be deleted if they have no packages, linked projects")?;
            ui.fatal("or other dependencies. Please check your origin and try again.")?;
            Err(Error::APIClient(api_client::Error::APIError(StatusCode::Conflict, msg)))
        }
        Err(e) => {
            ui.fatal(format!("Failed to delete origin {}, {:?}", origin, e))?;
            Err(Error::from(e))
        }
    }
}
