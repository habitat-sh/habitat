use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_core::origin::Origin;

pub async fn start(ui: &mut UI, bldr_url: &str, origin: &Origin) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Determining, format!("channels for {}.", origin))?;

    match api_client.list_channels(origin, false).await {
        Ok(channels) => {
            println!("{}", channels.join("\n"));
            Ok(())
        }
        Err(e) => Err(Error::APIClient(e)),
    }
}
