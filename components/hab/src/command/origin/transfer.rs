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

pub fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str, account: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Transferring,
              format!("ownership of origin {} to {}.", origin, account))?;

    match api_client.transfer_origin_ownership(origin, token, account) {
        Ok(_) => {
            ui.status(Status::Transferred, "ownership successfully!".to_string())
              .or(Ok(()))
        }
        Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
            ui.fatal("Failed to transfer origin ownership!")?;
            ui.fatal("Either you are not the current owner or the new owner is not yet a member.")?;
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
                             origin, account, e))?;
            Err(Error::from(e))
        }
    }
}
