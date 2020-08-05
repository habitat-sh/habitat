use super::super::key::download::download_public_encryption_key;
use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            PRODUCT,
            VERSION};
use habitat_core::crypto::keys::KeyCache;
use std::path::Path;

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   token: &str,
                   origin: &str,
                   key: &str,
                   secret: &str,
                   cache_dir: &Path)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    let cache = KeyCache::new(cache_dir);
    let encryption_key = match cache.latest_origin_public_encryption_key(origin) {
        Ok(key) => key,
        Err(_) => {
            debug!("Didn't find public encryption key in cache path");
            download_public_encryption_key(ui, &api_client, origin, token, cache_dir).await?;
            cache.latest_origin_public_encryption_key(origin)?
        }
    };

    ui.status(Status::Encrypting, format!("value for key {}.", key))?;
    let encrypted_box = encryption_key.encrypt(secret.as_bytes());
    ui.status(Status::Encrypted, format!("{}=[REDACTED].", key))?;

    ui.status(Status::Uploading, format!("secret for key {}.", key))?;

    api_client.create_origin_secret(origin, token, key, &encrypted_box)
              .await
              .map_err(Error::APIClient)?;

    ui.status(Status::Uploaded, format!("secret for {}.", key))?;

    Ok(())
}
