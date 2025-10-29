use crate::error::Result;
use futures::StreamExt;
use habitat_common::{types::ResolvedListenCtlAddr,
                     ui::{Status,
                          UIWriter}};
use habitat_core::{crypto::keys::{Key,
                                  KeyCache},
                   service::ServiceGroup};
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol::{self as sup_proto,
                           butterfly::MAX_FILE_PUT_SIZE_BYTES,
                           net::ErrCode};
use std::{convert::TryFrom,
          fs::File,
          io::{self,
               Read},
          path::PathBuf,
          process};

pub(crate) async fn sub_file_put<U>(service_group: &str,
                                    version: u64,
                                    file_path: &PathBuf,
                                    user_opt: Option<String>,
                                    remote_sup: &ResolvedListenCtlAddr,
                                    key_path: PathBuf,
                                    ui: &mut U)
                                    -> Result<()>
    where U: UIWriter
{
    let grp = service_group.parse::<ServiceGroup>()?;

    let mut msg = sup_proto::ctl::SvcFilePut { service_group: Some(grp.clone().into()),
                                               version: Some(version),
                                               ..Default::default() };

    let file = PathBuf::from(file_path);
    if file.metadata()?.len() > MAX_FILE_PUT_SIZE_BYTES as u64 {
        ui.fatal(format!("File too large. Maximum size allowed is {} bytes.",
                         MAX_FILE_PUT_SIZE_BYTES))?;
        process::exit(1);
    }

    msg.filename = Some(file.file_name().unwrap().to_string_lossy().into_owned());

    let key_cache = KeyCache::new(key_path);
    key_cache.setup()?;

    ui.begin(format!("Uploading file {} to {} incarnation {}",
                     file.display(),
                     msg.version
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "UNKNOWN".to_string()),
                     msg.service_group
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "UNKNOWN".to_string()),))?;
    ui.status(Status::Creating, "service file")?;

    let mut buf = Vec::with_capacity(MAX_FILE_PUT_SIZE_BYTES);
    File::open(&file)?.read_to_end(&mut buf)?;

    match (grp.org(), user_opt) {
        (Some(_org), Some(username)) => {
            let user_key = key_cache.latest_user_secret_key(&username)?;
            let service_key = key_cache.latest_service_public_key(&grp)?;
            ui.status(Status::Encrypting,
                      format!("file as {} for {}",
                              user_key.named_revision(),
                              service_key.named_revision()))?;
            msg.content = Some(user_key.encrypt_for_service(&buf, &service_key)?
                                       .to_string()
                                       .into_bytes());
            msg.is_encrypted = Some(true);
        }
        _ => {
            msg.content = Some(buf.clone());
        }
    }

    ui.status(Status::Applying, format!("via peer {}", remote_sup))?;

    let mut response = SrvClient::request(remote_sup, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                match ErrCode::try_from(m.code) {
                    Ok(ErrCode::InvalidPayload) => {
                        ui.warn(m)?;
                    }
                    _ => return Err(SrvClientError::from(m).into()),
                }
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }

    ui.end("Uploaded file")?;
    Ok(())
}
