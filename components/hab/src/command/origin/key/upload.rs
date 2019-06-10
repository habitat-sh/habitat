use std::path::Path;

use crate::{api_client::{self,
                         Client},
            common::{command::package::install::{RETRIES,
                                                 RETRY_WAIT},
                     ui::{Status,
                          UIWriter,
                          UI}},
            hcore::{crypto::{keys::parse_name_with_rev,
                             PUBLIC_SIG_KEY_VERSION,
                             SECRET_SIG_KEY_VERSION},
                    util::wait_for}};
use hyper::status::StatusCode;
use retry::retry;

use super::get_name_with_rev;
use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(ui: &mut UI,
             bldr_url: &str,
             token: &str,
             public_keyfile: &Path,
             secret_keyfile: Option<&Path>)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    ui.begin(format!("Uploading public origin key {}", public_keyfile.display()))?;

    let name_with_rev = get_name_with_rev(&public_keyfile, PUBLIC_SIG_KEY_VERSION)?;
    let (name, rev) = parse_name_with_rev(&name_with_rev)?;

    {
        let upload_fn = || -> Result<()> {
            ui.status(Status::Uploading, public_keyfile.display())?;
            match api_client.put_origin_key(&name, &rev, public_keyfile, token, ui.progress()) {
                Ok(()) => ui.status(Status::Uploaded, &name_with_rev)?,
                Err(api_client::Error::APIError(StatusCode::Conflict, _)) => {
                    ui.status(Status::Using,
                              format!("public key revision {} which already exists in the depot",
                                      &name_with_rev))?;
                }
                Err(err) => return Err(Error::from(err)),
            }
            Ok(())
        };

        if retry(wait_for(RETRY_WAIT, RETRIES), upload_fn).is_err() {
            return Err(Error::from(api_client::Error::UploadFailed(format!(
                "We tried {} times but could not upload {}/{} public origin key. Giving up.",
                RETRIES, &name, &rev
            ))));
        }
    }

    ui.end(format!("Upload of public origin key {} complete.", &name_with_rev))?;

    if let Some(secret_keyfile) = secret_keyfile {
        let name_with_rev = get_name_with_rev(&secret_keyfile, SECRET_SIG_KEY_VERSION)?;
        let (name, rev) = parse_name_with_rev(&name_with_rev)?;

        let upload_fn = || -> Result<()> {
            ui.status(Status::Uploading, secret_keyfile.display())?;
            match api_client.put_origin_secret_key(&name,
                                                   &rev,
                                                   secret_keyfile,
                                                   token,
                                                   ui.progress())
            {
                Ok(()) => {
                    ui.status(Status::Uploaded, &name_with_rev)?;
                    ui.end(format!("Upload of secret origin key {} complete.", &name_with_rev))?;
                    Ok(())
                }
                Err(e) => Err(Error::APIClient(e)),
            }
        };

        if retry(wait_for(RETRY_WAIT, RETRIES), upload_fn).is_err() {
            return Err(Error::from(api_client::Error::UploadFailed(format!(
                "We tried {} times but could not upload {}/{} secret origin key. Giving up.",
                RETRIES, &name, &rev
            ))));
        }
    }
    Ok(())
}
