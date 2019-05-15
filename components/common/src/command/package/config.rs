//! Prints the default configuration options for a service.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg config core/redis
//! ```
//!
//! Will show the `default.toml`.

use std::{io::{self,
               Write},
          path::Path};

use crate::hcore::package::{install::DEFAULT_CFG_FILE,
                            PackageIdent,
                            PackageInstall};
use toml;

use crate::error::Result;

pub fn start<P>(ident: &PackageIdent, fs_root_path: P) -> Result<()>
    where P: AsRef<Path>
{
    let package = PackageInstall::load(ident, Some(fs_root_path.as_ref()))?;
    match package.default_cfg() {
        Some(cfg) => println!("{}", toml::ser::to_string(&cfg)?),
        None => {
            writeln!(&mut io::stderr(),
                     "No '{}' found for {}",
                     DEFAULT_CFG_FILE,
                     package.ident()).expect("Failed printing to stderr")
        }
    }
    Ok(())
}
