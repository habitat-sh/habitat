//! Demote a package from a specified channel.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg demote acme/redis/2.0.7/2112010203120101 stable
//! ```
//! This will demote the acme package specified from the stable channel, removing it.
//!
//! Notes:
//!    The package should already have been uploaded to Builder.
//!    If the specified channel does not exist, this will fail.

use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::{package::{PackageIdent,
                              PackageTarget},
                    ChannelIdent}};
use reqwest::StatusCode;

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

/// Demote a package from the specified channel.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in Builder.
/// * Fails if the channel specified does not exist.
/// * Fails if "unstable" is the channel specified.
pub fn start(ui: &mut UI,
             bldr_url: &str,
             (ident, target): (&PackageIdent, PackageTarget),
             channel: &ChannelIdent,
             token: &str)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    ui.begin(format!("Demoting {} ({}) from {}", ident, target, channel))?;

    if channel == &ChannelIdent::unstable() {
        return Err(Error::CannotRemoveFromChannel((ident.to_string(), channel.to_string())));
    }

    match api_client.demote_package((ident, target), channel, token) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to demote '{}': {:?}", ident, e);
            if let api_client::Error::APIError(StatusCode::NOT_FOUND, _) = e {
                println!("You may need to specify a platform target argument");
            }
            return Err(Error::from(e));
        }
    }

    ui.status(Status::Demoted, format!("{} ({})", ident, target))?;

    Ok(())
}
