use crate::{PRODUCT,
            VERSION,
            api_client::{self,
                         API_RETRY_COUNT,
                         API_RETRY_DELAY,
                         APIFailure,
                         BuilderAPIClient,
                         Client,
                         Error::APIClientError,
                         retry_builder_api},
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};
use habitat_core::{crypto::keys::{KeyCache,
                                  NamedRevision},
                   origin::Origin};
use reqwest::StatusCode;

#[allow(clippy::too_many_arguments)]
pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   origin: &Origin,
                   revision: Option<&str>,
                   secret: bool,
                   encryption: bool,
                   token: Option<&str>,
                   key_cache: &KeyCache)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    if secret {
        handle_secret(ui, &api_client, origin, token, key_cache).await
    } else if encryption {
        handle_encryption(ui, &api_client, origin, token, key_cache).await
    } else {
        handle_public(ui, &api_client, origin, revision, token, key_cache).await
    }
}

async fn handle_public(ui: &mut UI,
                       api_client: &BuilderAPIClient,
                       origin: &Origin,
                       revision: Option<&str>,
                       token: Option<&str>,
                       key_cache: &KeyCache)
                       -> Result<()> {
    match revision {
        Some(revision) => {
            let named_revision = format!("{}-{}", origin, revision).parse()?;
            ui.begin(format!("Downloading public origin key {}", named_revision))?;
            match download_key(ui, api_client, &named_revision, token, key_cache).await {
                Ok(()) => {
                    let msg = format!("Download of {} public origin key completed.",
                                      named_revision);
                    ui.end(msg)?;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        None => {
            ui.begin(format!("Downloading public origin keys for {}", origin))?;
            match api_client.show_origin_keys(origin).await {
                Ok(ref keys) if keys.is_empty() => {
                    ui.end(format!("No public keys for {}.", origin))?;
                    Ok(())
                }
                Ok(keys) => {
                    for key in keys {
                        let named_revision = format!("{}-{}", key.origin, key.revision).parse()?;
                        download_key(ui, api_client, &named_revision, token, key_cache).await?;
                    }
                    ui.end(format!("Download of {} public origin keys completed.", &origin))?;
                    Ok(())
                }
                Err(e) => Err(Error::from(e)),
            }
        }
    }
}

async fn handle_secret(ui: &mut UI,
                       api_client: &BuilderAPIClient,
                       origin: &Origin,
                       token: Option<&str>,
                       key_cache: &KeyCache)
                       -> Result<()> {
    if token.is_none() {
        ui.end("No auth token found. You must pass a token to download secret keys.")?;
        return Ok(());
    }

    ui.begin(format!("Downloading secret origin keys for {}", origin))?;
    download_secret_key(ui, api_client, origin, token.unwrap(), key_cache).await?; // unwrap is safe because we already checked it above
    ui.end(format!("Download of {} secret origin keys completed.", origin))?;
    Ok(())
}

async fn handle_encryption(ui: &mut UI,
                           api_client: &BuilderAPIClient,
                           origin: &Origin,
                           token: Option<&str>,
                           key_cache: &KeyCache)
                           -> Result<()> {
    if token.is_none() {
        ui.end("No auth token found. You must pass a token to download secret keys.")?;
        return Ok(());
    }

    ui.begin(format!("Downloading public encryption origin key for {}", origin))?;
    download_public_encryption_key(ui, api_client, origin, token.unwrap(), key_cache).await?; // unwrap is safe because we already checked it above
    ui.end(format!("Download of {} public encryption keys completed.", &origin))?;
    Ok(())
}

pub async fn download_public_encryption_key(ui: &mut UI,
                                            api_client: &BuilderAPIClient,
                                            origin: &Origin,
                                            token: &str,
                                            key_cache: &KeyCache)
                                            -> Result<()> {
    retry_builder_api!(async {
        ui.status(Status::Downloading, "latest public encryption key")?;
        let key_path = api_client.fetch_origin_public_encryption_key(origin,
                                                                     token,
                                                                     key_cache.as_ref(),
                                                                     ui.progress())
                                 .await?;
        ui.status(Status::Cached,
                  key_path.file_name().unwrap().to_str().unwrap() /* lol */)?;
        Ok::<_, habitat_api_client::error::Error>(())
    }).await
      .map_err(|e| {
          APIClientError(APIFailure::DownloadLatestKeyFailed(API_RETRY_COUNT,
                                                             origin.to_string(),
                                                             Box::new(e)))
      })?;
    Ok(())
}

async fn download_secret_key(ui: &mut UI,
                             api_client: &BuilderAPIClient,
                             origin: &Origin,
                             token: &str,
                             key_cache: &KeyCache)
                             -> Result<()> {
    retry_builder_api!(async {
        ui.status(Status::Downloading, "latest secret key")?;
        let key_path =
            api_client.fetch_secret_origin_key(origin, token, key_cache.as_ref(), ui.progress())
                      .await?;
        ui.status(Status::Cached,
                  key_path.file_name().unwrap().to_str().unwrap() /* lol */)?;
        Ok::<_, habitat_api_client::error::Error>(())
    }).await
      .map_err(|e| {
          APIClientError(APIFailure::DownloadLatestKeyFailed(API_RETRY_COUNT,
                                                             origin.to_string(),
                                                             Box::new(e)))
      })?;
    Ok(())
}

async fn download_key(ui: &mut UI,
                      api_client: &BuilderAPIClient,
                      named_revision: &NamedRevision,
                      token: Option<&str>,
                      key_cache: &KeyCache)
                      -> Result<()> {
    if key_cache.public_signing_key(named_revision).is_ok() {
        ui.status(Status::Using,
                  format!("{} in {}", named_revision, key_cache.as_ref().display()))?;
        Ok(())
    } else {
        retry_builder_api!(async {
            ui.status(Status::Downloading, named_revision)?;
            api_client.fetch_origin_key(named_revision.name(),
                                        named_revision.revision(),
                                        token,
                                        key_cache.as_ref(),
                                        ui.progress())
                      .await?;
            ui.status(Status::Cached,
                      format!("{} to {}", named_revision, key_cache.as_ref().display()))?;
            Ok::<_, habitat_api_client::error::Error>(())
        }).await
          .map_err(|e| {
              APIClientError(APIFailure::DownloadKeyFailed(API_RETRY_COUNT,
                                                           named_revision.to_string(),
                                                           Box::new(e)))
          })?;
        Ok(())
    }
}
