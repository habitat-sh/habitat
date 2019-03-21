use crate::{error::Result,
            hcore::{fs as hfs,
                    package::{list,
                              PackageIdent}}};

use std::{path::Path,
          str::FromStr};

use clap::ArgMatches;

/// There are three options for what we can list:
///   - All packages (no prefix supplied)
///   - All packages in an origin (string with no '/')
///   - A set of packages with a common package ident (e.g. /ORIGIN/NAME) with optionally
///     version/release
#[derive(Debug)]
pub enum ListingType {
    AllPackages,
    Origin(String),
    Ident(PackageIdent),
}

/// Convert a set of command line options into a ListingType
impl<'a> From<&'a ArgMatches<'a>> for ListingType {
    /// Convert clap options into a listing type.
    ///
    /// We assume that the arguments have been validated during CLI parsing i.e.
    /// ORIGIN and PKG_IDENT are a valid origin and package identifier
    fn from(m: &ArgMatches<'_>) -> Self {
        if m.is_present("ALL") {
            return ListingType::AllPackages;
        }

        if m.is_present("ORIGIN") {
            let origin = m.value_of("ORIGIN").unwrap(); // Required by clap
            return ListingType::Origin(origin.to_string());
        }

        let p = m.value_of("PKG_IDENT").unwrap(); // Required by clap
        match PackageIdent::from_str(&p) {
            Ok(ident) => ListingType::Ident(ident),
            Err(_) => unreachable!("We've already validated PackageIdent {}", &p),
        }
    }
}

pub fn start(listing: &ListingType, fs_root_path: &Path) -> Result<()> {
    let package_path = hfs::pkg_root_path(Some(&fs_root_path));

    let mut packages = match listing {
        ListingType::AllPackages => list::all_packages(&package_path)?,
        ListingType::Origin(origin) => list::package_list_for_origin(&package_path, &origin)?,
        ListingType::Ident(ident) => list::package_list_for_ident(&package_path, &ident)?,
    };

    packages.sort_unstable_by(|a, b| a.by_parts_cmp(b));
    for p in &packages {
        println!("{}", &p);
    }

    Ok(())
}
