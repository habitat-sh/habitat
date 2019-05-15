//! Find out what channels a package belongs to.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg channels acme/redis/2.0.7/2112010203120101
//! ```
//! This will return a list of all the channels that acme/redis/2.0.7/2112010203120101
//! is in.
//!
//! Notes:
//!    The package should already have been uploaded to Builder.
//!    If the specified package does not exist, this will fail.

use crate::{api_client::Client,
            common::ui::{UIWriter,
                         UI},
            hcore::package::{PackageIdent,
                             PackageTarget}};

use crate::{error::Result,
            PRODUCT,
            VERSION};

/// Return a list of channels that a package is in.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in Builder.
pub fn start(ui: &mut UI,
             bldr_url: &str,
             (ident, target): (&PackageIdent, PackageTarget),
             token: Option<&str>)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    ui.begin(format!("Retrieving channels for {} ({})", ident, target))?;
    let channels = api_client.package_channels((ident, target), token)?;
    for channel in &channels {
        println!("{}", channel);
    }

    Ok(())
}
