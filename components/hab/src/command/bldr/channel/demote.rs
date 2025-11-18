use crate::{PRODUCT,
            VERSION,
            api_client::Client,
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result},
            hcore::ChannelIdent};
use habitat_core::origin::Origin;

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &Origin,
                   source_channel: &ChannelIdent,
                   target_channel: &ChannelIdent)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Demoting,
              format!("Packages selected from channel {} that are residing in {}.",
                      source_channel, target_channel))?;

    api_client.demote_channel_packages(origin, token, source_channel, target_channel)
              .await
              .map_err(Error::APIClient)?;

    ui.status(Status::Demoted,
              format!(" Packages selected from channel {} that are residing in {}.",
                      source_channel, target_channel))?;

    Ok(())
}
