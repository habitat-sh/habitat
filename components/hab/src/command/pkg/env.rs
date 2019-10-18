use crate::{error::Result,
            hcore::package::{PackageIdent,
                             PackageInstall}};
use std::{collections::BTreeMap,
          path::Path};

pub fn start(ident: &PackageIdent, fs_root_path: &Path) -> Result<()> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    let env = pkg_install.environment_for_command()?;
    render_environment(env);
    Ok(())
}

#[cfg(unix)]
fn render_environment(env: BTreeMap<String, String>) {
    for (key, value) in env.into_iter() {
        println!("export {}=\"{}\"", key, value);
    }
}

#[cfg(windows)]
fn render_environment(env: BTreeMap<String, String>) {
    for (key, value) in env.into_iter() {
        println!("$env:{}=\"{}\"", key, value);
    }
}
