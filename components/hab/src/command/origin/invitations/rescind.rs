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

pub fn start(ui: &mut UI,
             bldr_url: &str,
             origin: &str,
             token: &str,
             invitation_id: u64)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Rescinding,
              format!("invitation id {} for origin {}", invitation_id, origin))?;

    match api_client.rescind_origin_invitation(origin, token, invitation_id) {
        Ok(_) => {
            ui.status(Status::Rescinded,
                      "the invitation successfully!".to_string())
              .or(Ok(()))
        }
        Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
            ui.fatal("Failed to rescind the invitation!")?;
            ui.fatal("This situation could arise, if for example, you are not a member of the \
                      origin.")?;
            Err(Error::APIClient(err))
        }
        Err(err @ api_client::Error::APIError(StatusCode::UNAUTHORIZED, _)) => {
            ui.fatal("Failed to rescind the invitation!")?;
            Err(Error::APIClient(err))
        }
        Err(err @ api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
            ui.fatal("Failed to rescind the invitation!")?;
            ui.fatal("The origin or invitation id does not exist.")?;
            Err(Error::APIClient(err))
        }
        Err(e) => {
            ui.fatal(format!("Failed to rescind the invitation id {} in the {} origin, {:?}",
                             invitation_id, origin, e))?;
            Err(Error::from(e))
        }
    }
}
