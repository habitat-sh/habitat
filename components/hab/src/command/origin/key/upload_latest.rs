use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            hcore::crypto::keys::{Key,
                                  KeyCache,
                                  PublicOriginSigningKey,
                                  SecretOriginSigningKey},
            PRODUCT,
            VERSION};
use reqwest::StatusCode;
use std::path::Path;

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &str,
                   with_secret: bool,
                   cache: &Path)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    ui.begin(format!("Uploading latest public origin key {}", &origin))?;

    let cache = KeyCache::new(cache);

    // Figure out the latest public key
    let public_key: PublicOriginSigningKey = cache.latest_public_origin_signing_key(origin)?;

    // The path to the key in the cache
    let public_keyfile = cache.path_in_cache(&public_key);

    ui.status(Status::Uploading, public_keyfile.display())?;

    // TODO (CM): Really, we just need to pass the key itself; it's
    // got all the information
    match api_client.put_origin_key(public_key.named_revision().name(),
                                    public_key.named_revision().revision(),
                                    &public_keyfile,
                                    token,
                                    ui.progress())
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
    ui.end(format!("Upload of public origin key {} complete.",
                   public_key.named_revision()))?;

    if with_secret {
        // get matching secret key
        let secret_key: SecretOriginSigningKey =
            cache.secret_signing_key(public_key.named_revision())?;
        let secret_keyfile = cache.path_in_cache(&secret_key);

        ui.status(Status::Uploading, secret_keyfile.display())?;
        match api_client.put_origin_secret_key(&secret_key.named_revision().name(),
                                               &secret_key.named_revision().revision(),
                                               &secret_keyfile,
                                               token,
                                               ui.progress())
                        .await
        {
            Ok(()) => {
                ui.status(Status::Uploaded, secret_key.named_revision())?;
                ui.end(format!("Upload of secret origin key {} complete.",
                               secret_key.named_revision()))?;
            }
            Err(e) => {
                return Err(Error::APIClient(e));
            }
        }
    }
    Ok(())
}
