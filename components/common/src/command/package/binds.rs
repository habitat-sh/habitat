//! Prints the binds for a service.
//!
//! # Examples
//!
//! ```bash
//! $ hab pkg binds core/redis
//! ```
//!
//! Will show all available binds.

use std::{io::{self,
               Write},
          path::Path};

use crate::hcore::{self,
                   package::{metadata::Bind,
                             PackageIdent,
                             PackageInstall}};

use crate::error::Result;

pub fn start<P>(ident: &PackageIdent, fs_root_path: P) -> Result<()>
    where P: AsRef<Path>
{
    let package = PackageInstall::load(ident, Some(fs_root_path.as_ref()))?;
    println!("Showing binds for {}", package.ident());
    print_binds(package.binds(), true, package.ident());
    print_binds(package.binds_optional(), false, package.ident());
    Ok(())
}

fn print_binds(package_binds: hcore::error::Result<Vec<Bind>>,
               required: bool,
               package_ident: &PackageIdent) {
    let bind_type = if required { "required" } else { "optional" };
    match package_binds {
        Ok(binds) => {
            let binds_as_strings = binds.iter()
                                        .map(ToString::to_string)
                                        .collect::<Vec<String>>();
            if !binds_as_strings.is_empty() {
                println!("{}:\n    {}", bind_type, binds_as_strings.join("    \n"))
            } else {
                println!("{}: none", bind_type)
            }
        }
        Err(_) => {
            writeln!(&mut io::stderr(),
                     "Error while querying {} binds for {}",
                     bind_type,
                     package_ident).expect("Failed printing to stderr")
        }
    }
}
