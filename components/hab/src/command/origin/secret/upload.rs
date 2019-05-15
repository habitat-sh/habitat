use super::super::key::download::download_public_encryption_key;
use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI}};

use crate::{error::{Error,
                    Result},
            hcore::crypto::BoxKeyPair,
            PRODUCT,
            VERSION};
use std::path::Path;

pub fn start(ui: &mut UI,
             bldr_url: &str,
             token: &str,
             origin: &str,
             key: &str,
             secret: &str,
             cache: &Path)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    let encryption_key = match BoxKeyPair::get_latest_pair_for(origin, cache) {
        Ok(key) => key,
        Err(_) => {
            debug!("Didn't find public encryption key in cache path");
            download_public_encryption_key(ui, &api_client, origin, token, cache)?;
            BoxKeyPair::get_latest_pair_for(origin, cache)?
        }
    };

    ui.status(Status::Encrypting, format!("value for key {}.", key))?;
    let encrypted_secret_string = encryption_key.encrypt(secret.as_bytes(), None)?;
    ui.status(Status::Encrypted, format!("{}=[REDACTED].", key))?;

    ui.status(Status::Uploading, format!("secret for key {}.", key))?;

    api_client.create_origin_secret(origin, token, key, &encrypted_secret_string)
              .map_err(Error::APIClient)?;

    ui.status(Status::Uploaded, format!("secret for {}.", key))?;

    Ok(())
}
