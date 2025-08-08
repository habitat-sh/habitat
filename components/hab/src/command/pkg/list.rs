use crate::{error::Result,
            hcore::{fs::{self as hfs,
                         FS_ROOT_PATH},
                    package::{list,
                              PackageIdent}}};

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

impl From<PackageIdent> for ListingType {
    fn from(ident: PackageIdent) -> Self { ListingType::Ident(ident) }
}

pub fn package_list(listing: &ListingType) -> Result<Vec<PackageIdent>> {
    let package_path = hfs::pkg_root_path(Some(&*FS_ROOT_PATH));

    let mut packages = match listing {
        ListingType::AllPackages => list::all_packages(&package_path)?,
        ListingType::Origin(origin) => list::package_list_for_origin(&package_path, origin)?,
        ListingType::Ident(ident) => list::package_list_for_ident(&package_path, ident)?,
    };

    packages.sort_unstable_by(habitat_core::package::PackageIdent::by_parts_cmp);
    Ok(packages)
}

pub fn start(listing: &ListingType) -> Result<()> {
    let packages = package_list(listing)?;
    for p in &packages {
        println!("{}", &p);
    }

    Ok(())
}
