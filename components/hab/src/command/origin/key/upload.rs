use crate::{api_client::{self,
                         Client},
            common::{command::package::install::{RETRIES,
                                                 RETRY_WAIT},
                     ui::{Status,
                          UIWriter,
                          UI}},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_core::crypto::keys::{Key,
                                 PublicOriginSigningKey,
                                 SecretOriginSigningKey};
use reqwest::StatusCode;
use retry::delay;
use std::{convert::TryFrom,
          path::Path};

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   public_keyfile: &Path,
                   secret_keyfile: Option<&Path>)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    ui.begin(format!("Uploading public origin key {}", public_keyfile.display()))?;

    let public_key: PublicOriginSigningKey = TryFrom::try_from(public_keyfile)?;
    let name = public_key.named_revision().name();
    let rev = public_key.named_revision().revision();

    {
        retry::retry_future!(delay::Fixed::from(RETRY_WAIT).take(RETRIES), async {
            ui.status(Status::Uploading, public_keyfile.display())?;
            match api_client.put_origin_key(&name, &rev, public_keyfile, token, ui.progress())
                            .await
            {
                Ok(()) => ui.status(Status::Uploaded, public_key.named_revision())?,
                Err(api_client::Error::APIError(StatusCode::CONFLICT, _)) => {
                    ui.status(Status::Using,
                              format!("public key revision {} which already exists in the depot",
                                      public_key.named_revision()))?;
                }
                Err(err) => return Err(Error::from(err)),
            }
            Ok::<_, Error>(())
        }).await
          .map_err(|_| {
              Error::from(api_client::Error::UploadFailed(format!("We tried {} times but could \
                                                                   not upload {}/{} public \
                                                                   origin key. Giving up.",
                                                                  RETRIES, &name, &rev)))
          })?;
    }

    ui.end(format!("Upload of public origin key {} complete.",
                   public_key.named_revision()))?;

    if let Some(secret_keyfile) = secret_keyfile {
        let secret_key: SecretOriginSigningKey = TryFrom::try_from(secret_keyfile)?;
        let name = secret_key.named_revision().name();
        let rev = secret_key.named_revision().name();

        retry::retry_future!(delay::Fixed::from(RETRY_WAIT).take(RETRIES), async {
            ui.status(Status::Uploading, secret_keyfile.display())?;
            match api_client.put_origin_secret_key(&name,
                                                   &rev,
                                                   secret_keyfile,
                                                   token,
                                                   ui.progress())
                            .await
            {
                Ok(()) => {
                    ui.status(Status::Uploaded, secret_key.named_revision())?;
                    ui.end(format!("Upload of secret origin key {} complete.",
                                   secret_key.named_revision()))?;
                    Ok(())
                }
                Err(e) => Err(Error::APIClient(e)),
            }
        }).await
          .map_err(|_| {
              Error::from(api_client::Error::UploadFailed(format!("We tried {} times but could \
                                                                   not upload {}/{} secret \
                                                                   origin key. Giving up.",
                                                                  RETRIES, &name, &rev)))
          })?;
    }
    Ok(())
}
