use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_core::PortableText;
use reqwest::StatusCode;

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   origin: &str,
                   token: &str,
                   member_account: &str,
                   to_json: bool)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    match api_client.get_member_role(origin, token, member_account)
                    .await
    {
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
                ui.status(Status::Discovering, "origin member role".to_string())?;
                println!("Member {} has the '{}' role in the {} origin.",
                         member_account, resp.role, origin);
                Ok(())
            }
        }
        Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
            ui.fatal("Failed to get origin member's role!")?;
            ui.fatal(format!("This situation could arise, if for example, you are not a member \
                              with sufficient privileges in the '{}' origin.",
                             origin))?;
            Err(Error::APIClient(err))
        }
        Err(err @ api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
            ui.fatal("Failed to get origin member's role!")?;
            ui.fatal("This situation could arise, if for example, you passed a invalid member \
                      or origin name.")?;
            Err(Error::APIClient(err))
        }
        Err(e) => {
            ui.fatal(format!("Failed to retrieve origin member's role! {:?}.", e))?;
            Err(Error::from(e))
        }
    }
}
