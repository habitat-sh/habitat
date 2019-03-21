use std::ffi::OsString;

use crate::common::ui::UI;

use crate::error::Result;

const EXPORT_CMD: &str = "hab-pkg-export-tar";

pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> { inner::start(ui, args) }

#[cfg(not(target_os = "macos"))]
mod inner {
    use std::{ffi::OsString,
              path::PathBuf,
              str::FromStr};

    use crate::{common::ui::UI,
                hcore::{crypto::{default_cache_key_path,
                                 init},
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent}};

    use super::EXPORT_CMD;
    use crate::{error::{Error,
                        Result},
                exec,
                VERSION};

    const EXPORT_CMD_ENVVAR: &str = "HAB_PKG_EXPORT_TAR_BINARY";
    const EXPORT_PKG_IDENT: &str = "core/hab-pkg-export-tar";
    const EXPORT_PKG_IDENT_ENVVAR: &str = "HAB_PKG_EXPORT_TAR_PKG_IDENT";

    pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
        let command = match henv::var(EXPORT_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let ident = match henv::var(EXPORT_PKG_IDENT_ENVVAR) {
                    Ok(ref ident_str) => PackageIdent::from_str(ident_str)?,
                    Err(_) => {
                        let version: Vec<&str> = VERSION.split('/').collect();
                        PackageIdent::from_str(&format!("{}/{}", EXPORT_PKG_IDENT, version[0]))?
                    }
                };
                exec::command_from_min_pkg(ui,
                                           EXPORT_CMD,
                                           &ident,
                                           &default_cache_key_path(None),
                                           0)?
            }
        };
        if let Some(cmd) = find_command(&command) {
            process::become_command(cmd, args)?;
            Ok(())
        } else {
            Err(Error::ExecCommandNotFound(command))
        }
    }
}

#[cfg(target_os = "macos")]
mod inner {
    use std::ffi::OsString;

    use crate::common::ui::{UIWriter,
                            UI};

    use super::EXPORT_CMD;
    use crate::error::{Error,
                       Result};

    pub fn start(ui: &mut UI, _args: &[OsString]) -> Result<()> {
        let cmd = EXPORT_CMD.replace("hab", "").replace("-", " ");
        ui.warn(format!("Running 'hab {}' on this operating system is not yet supported. Try \
                         running this command again on 64-bit Linux.",
                        &cmd))?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(cmd.to_string()))
    }
}
