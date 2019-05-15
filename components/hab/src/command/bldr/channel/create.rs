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
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Creating, format!("channel {}.", channel))?;

    api_client.create_channel(origin, channel, token)
              .map_err(Error::APIClient)?;

    ui.status(Status::Created, format!("channel {}.", channel))?;

    Ok(())
}
