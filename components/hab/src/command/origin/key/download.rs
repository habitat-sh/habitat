use std::path::Path;

use crate::{api_client::{BoxedClient,
                         Client},
            common::{self,
                     command::package::install::{RETRIES,
                                                 RETRY_WAIT},
                     ui::{Status,
                          UIWriter,
                          UI}},
            hcore::crypto::SigKeyPair};

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

use retry::retry;

#[allow(clippy::too_many_arguments)]
pub fn start(ui: &mut UI,
             bldr_url: &str,
             origin: &str,
             revision: Option<&str>,
             secret: bool,
             encryption: bool,
             token: Option<&str>,
             cache: &Path)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    if secret {
        handle_secret(ui, &api_client, origin, token, cache)
    } else if encryption {
        handle_encryption(ui, &api_client, origin, token, cache)
    } else {
        handle_public(ui, &api_client, origin, revision, cache)
    }
}

fn handle_public(ui: &mut UI,
                 api_client: &BoxedClient,
                 origin: &str,
                 revision: Option<&str>,
                 cache: &Path)
                 -> Result<()> {
    match revision {
        Some(revision) => {
            let nwr = format!("{}-{}", origin, revision);
            ui.begin(format!("Downloading public origin key {}", &nwr))?;
            match download_key(ui, api_client, &nwr, origin, revision, cache) {
                Ok(()) => {
                    let msg = format!("Download of {} public origin key completed.", nwr);
                    ui.end(msg)?;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        None => {
            ui.begin(format!("Downloading public origin keys for {}", origin))?;
            match api_client.show_origin_keys(origin) {
                Ok(ref keys) if keys.is_empty() => {
                    ui.end(format!("No public keys for {}.", origin))?;
                    Ok(())
                }
                Ok(keys) => {
                    for key in keys {
                        let nwr = format!("{}-{}", key.origin, key.revision);
                        download_key(ui, api_client, &nwr, &key.origin, &key.revision, cache)?;
                    }
                    ui.end(format!("Download of {} public origin keys completed.", &origin))?;
                    Ok(())
                }
                Err(e) => Err(Error::from(e)),
            }
        }
    }
}

fn handle_secret(ui: &mut UI,
                 api_client: &BoxedClient,
                 origin: &str,
                 token: Option<&str>,
                 cache: &Path)
                 -> Result<()> {
    if token.is_none() {
        ui.end("No auth token found. You must pass a token to download secret keys.")?;
        return Ok(());
    }

    ui.begin(format!("Downloading secret origin keys for {}", origin))?;
    download_secret_key(ui, &api_client, origin, token.unwrap(), cache)?; // unwrap is safe because we already checked it above
    ui.end(format!("Download of {} public origin keys completed.", &origin))?;
    Ok(())
}

fn handle_encryption(ui: &mut UI,
                     api_client: &BoxedClient,
                     origin: &str,
                     token: Option<&str>,
                     cache: &Path)
                     -> Result<()> {
    if token.is_none() {
        ui.end("No auth token found. You must pass a token to download secret keys.")?;
        return Ok(());
    }

    ui.begin(format!("Downloading public encryption origin key for {}", origin))?;
    download_public_encryption_key(ui, &api_client, origin, token.unwrap(), cache)?; // unwrap is safe because we already checked it above
    ui.end(format!("Download of {} public encryption keys completed.", &origin))?;
    Ok(())
}

pub fn download_public_encryption_key(ui: &mut UI,
                                      api_client: &BoxedClient,
                                      name: &str,
                                      token: &str,
                                      cache: &Path)
                                      -> Result<()> {
    let download_fn = || -> Result<()> {
        ui.status(Status::Downloading, "latest public encryption key")?;
        let key_path =
            api_client.fetch_origin_public_encryption_key(name, token, cache, ui.progress())?;
        ui.status(Status::Cached,
                  key_path.file_name().unwrap().to_str().unwrap() /* lol */)?;
        Ok(())
    };

    if retry(RETRIES, RETRY_WAIT, download_fn, Result::is_ok).is_err() {
        return Err(Error::from(common::error::Error::DownloadFailed(format!(
            "We tried {} times but could not download the latest public encryption key. Giving up.",
            RETRIES,
        ))));
    }

    Ok(())
}

fn download_secret_key(ui: &mut UI,
                       api_client: &BoxedClient,
                       name: &str,
                       token: &str,
                       cache: &Path)
                       -> Result<()> {
    let download_fn = || -> Result<()> {
        ui.status(Status::Downloading, "latest secret key")?;
        let key_path = api_client.fetch_secret_origin_key(name, token, cache, ui.progress())?;
        ui.status(Status::Cached,
                  key_path.file_name().unwrap().to_str().unwrap() /* lol */)?;
        Ok(())
    };

    if retry(RETRIES, RETRY_WAIT, download_fn, Result::is_ok).is_err() {
        return Err(Error::from(common::error::Error::DownloadFailed(format!(
            "We tried {} times but could not download the latest secret origin key. Giving up.",
            RETRIES,
        ))));
    }

    Ok(())
}

fn download_key(ui: &mut UI,
                api_client: &BoxedClient,
                nwr: &str,
                name: &str,
                rev: &str,
                cache: &Path)
                -> Result<()> {
    match SigKeyPair::get_public_key_path(&nwr, &cache) {
        Ok(_) => ui.status(Status::Using, &format!("{} in {}", nwr, cache.display()))?,
        Err(_) => {
            let download_fn = || -> Result<()> {
                ui.status(Status::Downloading, &nwr)?;
                api_client.fetch_origin_key(name, rev, cache, ui.progress())?;
                ui.status(Status::Cached, &format!("{} to {}", nwr, cache.display()))?;
                Ok(())
            };

            if retry(RETRIES, RETRY_WAIT, download_fn, Result::is_ok).is_err() {
                return Err(Error::from(common::error::Error::DownloadFailed(format!(
                    "We tried {} times but could not download {}/{} origin key. Giving up.",
                    RETRIES, &name, &rev
                ))));
            }
        }
    }
    Ok(())
}
