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
//!

use crate::{api_client::Client,
            common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::{package::PackageIdent,
                    ChannelIdent}};

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
             ident: &PackageIdent,
             channel: &ChannelIdent,
             token: &str)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    ui.begin(format!("Demoting {} from {}", ident, channel))?;

    if channel == &ChannelIdent::unstable() {
        return Err(Error::CannotRemoveFromChannel((ident.to_string(), channel.to_string())));
    }

    match api_client.demote_package(ident, channel, token) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to demote '{}': {:?}", ident, e);
            return Err(Error::from(e));
        }
    }

    ui.status(Status::Demoted, ident)?;

    Ok(())
}
