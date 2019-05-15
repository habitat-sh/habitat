use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI}};

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(ui: &mut UI, bldr_url: &str, token: &str, origin: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Determining, format!("secrets for {}.", origin))?;

    match api_client.list_origin_secrets(origin, token) {
        Ok(secrets) => {
            println!("{}", secrets.join("\n"));
            Ok(())
        }
        Err(e) => Err(Error::APIClient(e)),
    }
}
