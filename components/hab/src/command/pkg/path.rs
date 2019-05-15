use std::path::Path;

use crate::hcore::package::{PackageIdent,
                            PackageInstall};

use crate::error::Result;

pub fn start(ident: &PackageIdent, fs_root_path: &Path) -> Result<()> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    println!("{}", pkg_install.installed_path().display());
    Ok(())
}
