//! Delete a package from Builder.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg delete acme/redis/2.0.7/2112010203120101
//! ```
//! //! This will delete the acme package specified from Builder
//! //! if certain conditions apply - for example, the package is not
//! //! in the stable channel, and does not have any other packages that
//! //! depend on it.
//!
//! //! Note: This command does not remove the package from disk

use crate::{api_client::{self,
                         Client},
            common::ui::{Status,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            hcore::package::{PackageIdent,
                             PackageTarget},
            PRODUCT,
            VERSION};
use hyper::status::StatusCode;

/// Delete a package from Builder.
///
/// # Failures
///
/// * Fails if it cannot find the specified package in Builder
pub fn start(ui: &mut UI,
             bldr_url: &str,
             (ident, target): (&PackageIdent, PackageTarget),
             token: &str)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;

    ui.begin(format!("Deleting {} ({}) from Builder", ident, target))?;

    if let Err(err) = api_client.delete_package((ident, target), token) {
        println!("Failed to delete '{}': {:?}", ident, err);
        if let api_client::Error::APIError(StatusCode::NotFound, _) = err {
            println!("You may need to specify a platform target argument");
        }
        return Err(Error::from(err));
    }

    ui.status(Status::Deleted, format!("{} ({})", ident, target))?;

    Ok(())
}
