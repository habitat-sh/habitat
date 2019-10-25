use std::path::Path;

use crate::error::Result;

/// Creates a root file system under the given path.
///
/// # Errors
///
/// * If files and/or directories cannot be created
/// * If permissions for files and/or directories cannot be set
#[cfg(unix)]
pub fn create<T>(root: T) -> Result<()>
    where T: AsRef<Path>
{
    use std::fs;

    use crate::hcore::util;

    let root = root.as_ref();
    fs::create_dir_all(root)?;
    util::posix_perm::set_permissions(root.to_str().unwrap(), 0o0750)?;
    Ok(())
}

#[cfg(windows)]
pub fn create<T>(_root: T) -> Result<()>
    where T: AsRef<Path>
{
    Ok(())
}
