use std::ffi::OsString;

use crate::common::ui::UI;

use crate::error::Result;

pub fn start(ui: &mut UI,
             args: &[OsString],
             export_cmd: &str,
             export_cmd_envvar: &str,
             export_pkg_ident: &str,
             export_pkg_ident_envvar: &str)
             -> Result<()> {
    inner::start(ui,
                 args,
                 export_cmd,
                 export_cmd_envvar,
                 export_pkg_ident,
                 export_pkg_ident_envvar)
}

#[cfg(not(target_os = "macos"))]
mod inner {
    use std::{ffi::OsString,
              path::PathBuf,
              str::FromStr};

    use crate::{common::ui::UI,
                hcore::{crypto::init,
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent}};

    use crate::{error::{Error,
                        Result},
                exec,
                VERSION};

    pub fn start(ui: &mut UI,
                 args: &[OsString],
                 export_cmd: &str,
                 export_cmd_envvar: &str,
                 export_pkg_ident: &str,
                 export_pkg_ident_envvar: &str)
                 -> Result<()> {
        let command = match henv::var(export_cmd_envvar) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let ident = match henv::var(export_pkg_ident_envvar) {
                    Ok(ref ident_str) => PackageIdent::from_str(ident_str)?,
                    Err(_) => {
                        let version: Vec<&str> = VERSION.split('/').collect();
                        PackageIdent::from_str(&format!("{}/{}", export_pkg_ident, version[0]))?
                    }
                };
                exec::command_from_min_pkg(ui, export_cmd, &ident)?
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

    use crate::error::{Error,
                       Result};

    pub fn start(ui: &mut UI,
                 _args: &[OsString],
                 export_cmd: &str,
                 _export_cmd_envvar: &str,
                 _export_pkg_ident: &str,
                 _export_pkg_ident_envvar: &str)
                 -> Result<()> {
        let cmd = export_cmd.replace("hab", "").replace("-", " ");
        ui.warn(format!("Running 'hab {}' on this operating system is not yet supported. Try \
                         running this command again on 64-bit Linux.",
                        &cmd))?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(cmd.to_string()))
    }
}
