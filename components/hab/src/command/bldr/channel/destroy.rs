use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::ChannelIdent};

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(ui: &mut UI,
             bldr_url: &str,
             token: &str,
             origin: &str,
             channel: &ChannelIdent)
             -> Result<()> {
    let bldr_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Deleting, format!("channel {}.", channel))?;

    bldr_client.delete_channel(origin, channel, token)
               .map_err(Error::APIClient)?;

    ui.status(Status::Deleted, format!("channel {}.", channel))?;

    Ok(())
}
