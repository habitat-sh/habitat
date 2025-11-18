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
                   source_channel: &ChannelIdent,
                   target_channel: &ChannelIdent)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Promoting,
              format!("Packages from channel {} to {}.",
                      source_channel, target_channel))?;

    api_client.promote_channel_packages(origin, token, source_channel, target_channel)
              .await
              .map_err(Error::APIClient)?;

    ui.status(Status::Promoted,
              format!("packages from channel {} to {}.",
                      source_channel, target_channel))?;

    Ok(())
}
