use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI}};

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str, key: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Deleting, format!("secret {}.", key))?;

    api_client.delete_origin_secret(origin, token, key)
              .map_err(Error::APIClient)?;

    ui.status(Status::Deleted, format!("secret {}.", key))?;

    Ok(())
}
