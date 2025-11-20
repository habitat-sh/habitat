use crate::{PRODUCT,
            VERSION,
            api_client::{self,
                         Client},
            common::ui::{Status,
                         UI,
                         UIReader,
                         UIWriter},
            error::{Error,
                    Result}};
use habitat_core::origin::{Origin,
                           OriginMemberRole};
use reqwest::StatusCode;
use url::Url;

pub async fn start(ui: &mut UI,
                   bldr_url: Url,
                   origin: Origin,
                   token: &str,
                   member_account: &str,
                   role: OriginMemberRole,
                   no_prompt: bool)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.begin(format!("Preparing to update member {}'s role to '{}' in {} origin.",
                     member_account, role, origin))?;

    if !no_prompt && !confirm_update_role(ui)? {
        return Ok(());
    }

    match api_client.update_member_role(origin.clone(), token, member_account, role)
                    .await
    {
        Ok(_) => {
            ui.status(Status::Updated, "the member role successfully!".to_string())
              .or(Ok(()))
        }
        Err(err @ api_client::Error::APIError(StatusCode::FORBIDDEN, _)) => {
            ui.fatal("Failed to update the role!")?;
            ui.fatal(format!("This situation could arise, if for example, you are not a member \
                              with sufficient privileges in the '{}' origin.",
                             origin))?;
            Err(Error::APIClient(err))
        }
        Err(err @ api_client::Error::APIError(StatusCode::UNPROCESSABLE_ENTITY, _)) => {
            ui.fatal("Failed to update the role!")?;
            ui.fatal(format!("This situation could arise, if for example, role: '{}' was no \
                              longer supported by the server API.",
                             role))?;
            Err(Error::APIClient(err))
        }
        Err(err @ api_client::Error::APIError(StatusCode::NOT_FOUND, _)) => {
            ui.fatal("Failed to update the role!")?;
            ui.fatal("This situation could arise, if for example, you passed a invalid member \
                      or origin name.")?;
            Err(Error::APIClient(err))
        }
        Err(e) => {
            ui.fatal(format!("Failed to update the role! {:?}", e))?;
            Err(Error::from(e))
        }
    }
}

fn confirm_update_role(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no("Modify the role as indicated above?", Some(true))?)
}
