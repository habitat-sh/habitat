use crate::{api_client::{self,
                         Client,
                         Tabular},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use reqwest::StatusCode;

pub fn start(ui: &mut UI, bldr_url: &str, origin: &str, token: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Discovering,
              format!("pending member invitations in origin {}", origin))?;

    // given an origin, list its pending invitations
    match api_client.list_pending_origin_invitations(origin, token) {
        Ok(resp) => {
            println!("Pending Origin ({}) Member Invitations [{}]:",
                     origin,
                     resp.invitations.len());
            match resp.as_tabbed() {
                Ok(body) => {
                    println!("{}", body);
                    Ok(())
                }
                Err(e) => {
                    ui.fatal(format!("Failed to format pending origin invitations! {:?}.", e))?;
                    Err(Error::from(e))
                }
            }
        }
        Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
            ui.fatal("Failed to list pending invitations!")?;
            ui.fatal("This situation could arise, if for example, you are not the owner of the \
                      origin.")?;
            Err(Error::APIClient(err))
        }
        Err(e) => {
            ui.fatal(format!("Failed to retrieve pending origin invitations! {:?}.", e))?;
            Err(Error::from(e))
        }
    }
}
