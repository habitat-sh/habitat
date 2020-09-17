use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_core::origin::Origin;

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &Origin,
                   key: &str)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Deleting, format!("secret {}.", key))?;

    api_client.delete_origin_secret(origin, token, key)
              .await
              .map_err(Error::APIClient)?;

    ui.status(Status::Deleted, format!("secret {}.", key))?;

    Ok(())
}
