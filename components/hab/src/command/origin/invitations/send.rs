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

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   origin: &str,
                   token: &str,
                   invitation_account: &str)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Sending,
              format!("invitation to user {} for origin {}",
                      invitation_account, origin))?;

    match api_client.send_origin_invitation(origin, token, invitation_account)
                    .await
    {
        Ok(_) => {
            ui.status(Status::Sent, "the invitation successfully!".to_string())
              .or(Ok(()))
        }
        Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
            ui.fatal("Failed to send the invitation!")?;
            ui.fatal("This situation could arise, if for example, you are not a member of the \
                      origin.")?;
            Err(Error::APIClient(err))
        }
        Err(e) => {
            ui.fatal(format!("Failed to send invitation to {} in the {} origin, {:?}",
                             invitation_account, origin, e))?;
            Err(Error::from(e))
        }
    }
}
