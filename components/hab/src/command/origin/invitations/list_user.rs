use crate::{api_client::{Client,
                         Tabular},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub async fn start(ui: &mut UI, bldr_url: &str, token: &str) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Discovering,
              "member invitations sent to your account".to_string())?;

    // given a token, fetch any invitations for this user
    match api_client.list_user_invitations(token).await {
        Ok(resp) => {
            println!("Your Origin Invitations Inbox [{}]:", resp.0.len());
            match resp.as_tabbed() {
                Ok(body) => {
                    println!("{}", body);
                    Ok(())
                }
                Err(e) => {
                    ui.fatal(format!("Failed to format origin invitations under your account! \
                                      {:?}.",
                                     e))?;
                    Err(Error::from(e))
                }
            }
        }
        Err(e) => {
            ui.fatal(format!("Failed to retrieve origin invitations under your account! {:?}.",
                             e))?;
            Err(Error::from(e))
        }
    }
}
