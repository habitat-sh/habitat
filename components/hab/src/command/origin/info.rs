use crate::{PRODUCT,
            VERSION,
            api_client::Client,
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};

use habitat_core::util::text_render::{PortableText,
                                      TabularText};

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &str,
                   to_json: bool)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    match api_client.origin_info(token, origin).await {
        Ok(resp) => {
            if to_json {
                match resp.as_json() {
                    Ok(body) => {
                        println!("{}", body);
                        Ok(())
                    }
                    Err(e) => {
                        ui.fatal(format!("Failed to deserialize into json! {:?}.", e))?;
                        Err(Error::from(e))
                    }
                }
            } else {
                ui.status(Status::Discovering, "origin metadata".to_string())?;
                println!("Origin [{}]:", origin);
                match resp.as_tabbed() {
                    Ok(body) => {
                        println!("{}", body);
                        Ok(())
                    }
                    Err(e) => {
                        ui.fatal(format!("Failed to format origin metadata! {:?}.", e))?;
                        Err(Error::from(e))
                    }
                }
            }
        }
        Err(e) => {
            ui.fatal(format!("Failed to retrieve origin metadata! {:?}.", e))?;
            Err(Error::from(e))
        }
    }
}
