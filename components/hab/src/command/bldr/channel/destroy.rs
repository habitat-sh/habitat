use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_core::{origin::Origin,
                   ChannelIdent};

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &Origin,
                   channel: &ChannelIdent)
                   -> Result<()> {
    let bldr_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Deleting, format!("channel {}.", channel))?;

    bldr_client.delete_channel(origin, channel, token)
               .await
               .map_err(Error::APIClient)?;

    ui.status(Status::Deleted, format!("channel {}.", channel))?;

    Ok(())
}
