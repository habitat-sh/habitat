use crate::error::Result;
use habitat_common::{types::ResolvedListenCtlAddr, ui::{UIWriter, Status}};
use habitat_core::{
    fs::{cache_key_path, FS_ROOT_PATH},
    service::ServiceGroup,
    package::PackageIdent,
    crypto::keys::{KeyCache, Key},
};
use habitat_sup_client::{SrvClient, SrvClientError};
use habitat_sup_protocol::{self as sup_proto, butterfly::MAX_SVC_CFG_SIZE, net::ErrCode};
use futures::StreamExt;
use std::{convert::TryFrom, fs::File, io::{self, Read}, path::Path, process};

pub(crate) async fn sub_svc_set<U>(
    ui: &mut U,
    grp: ServiceGroup,
    cfg_path: &Path,
    version: u64,
    user_opt: Option<String>,
    remote_sup: Option<ResolvedListenCtlAddr>,
) -> Result<()>
where
    U: UIWriter,
{
    let mut buf = Vec::with_capacity(MAX_SVC_CFG_SIZE);
    let len = if cfg_path.as_os_str() == "-" {
        io::stdin().read_to_end(&mut buf)?
    } else {
        let mut f = File::open(cfg_path)?;
        f.read_to_end(&mut buf)?
    };
    if len > MAX_SVC_CFG_SIZE {
        ui.fatal(format!(
            "Configuration too large. Maximum allowed is {} bytes.",
            MAX_SVC_CFG_SIZE
        ))?;
        process::exit(1);
    }

    let mut validate = sup_proto::ctl::SvcValidateCfg {
        service_group: Some(grp.clone().into()),
        ..Default::default()
    };
    validate.cfg = Some(buf.clone());

    let key_cache = KeyCache::new(cache_key_path(FS_ROOT_PATH.as_path()));
    key_cache.setup()?;

    let mut set_msg = sup_proto::ctl::SvcSetCfg::default();
    if let Some(username) = user_opt {
        let user_key = key_cache.latest_user_secret_key(&username)?;
        let svc_key = key_cache.latest_service_public_key(&grp)?;
        ui.status(
            Status::Encrypting,
            format!(
                "Encrypting config (user rev = {}, service rev = {})",
                user_key.named_revision(),
                svc_key.named_revision()
            ),
        )?;
        set_msg.cfg = Some(
            user_key
                .encrypt_for_service(&buf, &svc_key)
                .to_string()
                .into_bytes(),
        );
        set_msg.is_encrypted = Some(true);
    } else {
        set_msg.cfg = Some(buf.clone());
    }

    set_msg.service_group = Some(grp.clone().into());
    set_msg.version = Some(version);

    ui.begin(format!("Setting new configuration version {} for {}", version, grp))?;
    ui.status(Status::Creating, "service configuration")?;
    let mut resp = SrvClient::request(remote_sup.as_ref(), validate).await?;
    while let Some(msg) = resp.next().await {
        let reply = msg?;
        match reply.message_id() {
            "NetOk" => {}
            "NetErr" => {
                let net_err =
                    reply.parse::<sup_proto::net::NetErr>().map_err(SrvClientError::Decode)?;
                if ErrCode::try_from(net_err.code) == Ok(ErrCode::InvalidPayload) {
                    ui.warn(net_err)?;
                } else {
                    return Err(SrvClientError::from(net_err).into());
                }
            }
            _ => {
                return Err(
                    SrvClientError::from(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Unexpected reply",
                    ))
                    .into(),
                )
            }
        }
    }

    ui.status(Status::Applying, "applying...")?;
    let mut resp = SrvClient::request(remote_sup.as_ref(), set_msg).await?;
    while let Some(msg) = resp.next().await {
        let reply = msg?;
        match reply.message_id() {
            "NetOk" => {}
            "NetErr" => {
                let net_err =
                    reply.parse::<sup_proto::net::NetErr>().map_err(SrvClientError::Decode)?;
                return Err(SrvClientError::from(net_err).into());
            }
            _ => {
                return Err(
                    SrvClientError::from(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Unexpected reply",
                    ))
                    .into(),
                )
            }
        }
    }

    ui.end("Applied configuration")?;

    Ok(())
}

pub(crate) async fn sub_svc_config(
    ident: PackageIdent,
    remote_sup_addr: Option<ResolvedListenCtlAddr>,
) -> Result<()> {
    let msg = sup_proto::ctl::SvcGetDefaultCfg {
        ident: Some(ident.into()),
    };
    let mut resp = SrvClient::request(remote_sup_addr.as_ref(), msg).await?;
    while let Some(msg) = resp.next().await {
        let reply = msg?;
        match reply.message_id() {
            "ServiceCfg" => {
                reply
                    .parse::<sup_proto::types::ServiceCfg>()
                    .map_err(SrvClientError::Decode)?;
            }
            "NetErr" => {
                let net_err = reply
                    .parse::<sup_proto::net::NetErr>()
                    .map_err(SrvClientError::Decode)?;
                return Err(SrvClientError::from(net_err).into());
            }
            _ => {
                return Err(
                    SrvClientError::from(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "Unexpected reply",
                    ))
                    .into(),
                )
            }
        }
    }
    Ok(())
}
