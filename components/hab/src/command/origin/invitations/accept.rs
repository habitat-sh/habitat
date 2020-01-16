use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(ui: &mut UI,
             bldr_url: &str,
             origin: &str,
             token: &str,
             invitation_id: u64)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Accepting,
              format!("invitation id {} in origin {}", invitation_id, origin))?;

    match api_client.accept_origin_invitation(origin, token, invitation_id) {
        Ok(_) => {
            ui.status(Status::Accepted, "the invitation successfully!".to_string())
              .or(Ok(()))
        }
        Err(e) => {
            ui.fatal(format!("Failed to accept invitation {} in origin {}, {:?}",
                             invitation_id, origin, e))?;
            Err(Error::from(e))
        }
    }
}
