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
        Err(api_client::Error::APIError(StatusCode::FORBIDDEN, msg)) => {
            ui.fatal("Failed to transfer origin ownership!")?;
            // TODO: remove the "the operation has already been completed" phrase
            // when this issue is closed: https://github.com/habitat-sh/builder/issues/1273
            ui.fatal("Either you are not the current owner, the operation requested has already \
                      been completed, or the new owner is not yet a member.")?;
            Err(Error::APIClient(api_client::Error::APIError(StatusCode::FORBIDDEN, msg)))
        }
        Err(api_client::Error::APIError(StatusCode::UNAUTHORIZED, msg)) => {
            ui.fatal("Failed to transfer origin ownership!")?;
            Err(Error::APIClient(api_client::Error::APIError(StatusCode::UNAUTHORIZED, msg)))
        }
        Err(api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY, msg)) => {
            ui.fatal("Failed to transfer origin ownership!")?;
            ui.fatal("This situation could arise if, for example, you attempted to transfer \
                      ownership to yourself.")?;
            Err(Error::APIClient(api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY,
                                                             msg)))
        }
        Err(api_client::Error::APIError(StatusCode::NOT_FOUND, msg)) => {
            ui.fatal("Failed to transfer origin ownership!")?;
            ui.fatal("The origin or the new origin owner account (or both) does not exist.")?;
            Err(Error::APIClient(api_client::Error::APIError(StatusCode::NOT_FOUND, msg)))
        }
        Err(e) => {
            ui.fatal(format!("Failed to transfer origin {} ownership to {}, {:?}",
                             origin, account, e))?;
            Err(Error::from(e))
        }
    }
}
