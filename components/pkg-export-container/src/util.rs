use anyhow::Result;
use habitat_core::package::{PackageIdent,
                            PackageInstall};
use std::{fs::{self,
               File},
          io::Write,
          path::{Path,
                 PathBuf}};
const BIN_PATH: &str = "/bin";

/// Returns the `bin` path used for symlinking programs.
pub(crate) fn bin_path() -> &'static Path { Path::new(BIN_PATH) }

/// Returns the Package Identifier for a Busybox package.
#[cfg(unix)]
pub(crate) fn busybox_ident() -> Result<PackageIdent> {
    use super::BUSYBOX_IDENT;
    use std::str::FromStr;

    Ok(PackageIdent::from_str(BUSYBOX_IDENT)?)
}

/// Returns the path to a package prefix for the provided Package Identifier in a root file system.
///
/// # Errors
///
/// * If a package cannot be loaded from in the root file system
pub(crate) fn pkg_path_for<P: AsRef<Path>>(ident: &PackageIdent, rootfs: P) -> Result<PathBuf> {
    let pkg_install = PackageInstall::load(ident, Some(rootfs.as_ref()))?;
    Ok(Path::new("/").join(pkg_install.installed_path()
                                      .strip_prefix(rootfs.as_ref())
                                      .expect("installed path contains rootfs path")))
}

/// Writes a truncated/new file at the provided path with the provided content.
///
/// # Errors
///
/// * If an `IO` error occurs while creating, tuncating, writing, or closing the file
pub(crate) fn write_file<T>(file: T, content: &str) -> Result<()>
    where T: AsRef<Path>
{
    fs::create_dir_all(file.as_ref().parent().expect("Parent directory exists"))?;
    let mut f = File::create(file)?;
    f.write_all(content.as_bytes())?;
    Ok(())
}
