use crate::{PRODUCT,
            VERSION,
            api_client::Client,
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};
use habitat_core::{ChannelIdent,
                   origin::Origin};

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &Origin,
                   channel: &ChannelIdent)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Creating, format!("channel {}.", channel))?;

    api_client.create_channel(origin, channel, token)
              .await
              .map_err(Error::APIClient)?;

    ui.status(Status::Created, format!("channel {}.", channel))?;

    Ok(())
}
