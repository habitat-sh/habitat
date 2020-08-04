use crate::{common::ui::UI,
            error::Result};
use std::ffi::OsString;

pub async fn start(ui: &mut UI,
                   args: &[OsString],
                   export_cmd_envvar: &str,
                   export_pkg_ident_envvar: &str,
                   export_cmd: &str)
                   -> Result<()> {
    inner::start(ui,
                 args,
                 export_cmd_envvar,
                 export_pkg_ident_envvar,
                 export_cmd).await
}

#[cfg(not(target_os = "macos"))]
mod inner {
    use crate::{command,
                common::ui::UI,
                error::{Error,
                        Result},
                exec,
                hcore::{crypto,
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent},
                VERSION};
    use std::{ffi::OsString,
              path::PathBuf,
              str::FromStr};

    /// Run the exporter command.
    ///
    /// The command is searched for using the following logic:
    /// 1. `export_cmd_envvar` is checked for the raw command.
    /// 2. `export_pkg_ident_envvar` is checked for an explicit habitat package.
    /// 3. The habitat package is implicitly determined using the `export_cmd` name and the current
    /// version. The package is assumed to be found in core. (eg /core/<export_cmd>/<VERSION>)
    pub async fn start(ui: &mut UI,
                       args: &[OsString],
                       export_cmd_envvar: &str,
                       export_pkg_ident_envvar: &str,
                       export_cmd: &str)
                       -> Result<()> {
        crypto::init()?;

        match henv::var(export_cmd_envvar) {
            Ok(command) => {
                let command = PathBuf::from(command);
                if let Some(command) = find_command(&command) {
                    debug!("Using export command {:?} specified with envvar `{}`",
                           command, export_cmd_envvar);
                    process::become_command(command, args)?;
                } else {
                    return Err(Error::ExecCommandNotFound(command));
                }
            }
            Err(_) => {
                let ident = match henv::var(export_pkg_ident_envvar) {
                    Ok(ref ident_str) => PackageIdent::from_str(ident_str)?,
                    Err(_) => {
                        let version: Vec<&str> = VERSION.split('/').collect();
                        PackageIdent::from_str(&format!("core/{}/{}", export_cmd, version[0]))?
                    }
                };
                debug!("Using export package `{}`", ident);
                let command = exec::command_from_min_pkg(ui, export_cmd, &ident).await?;
                command::pkg::exec::start(&ident, command, args)?;
            }
        };
        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod inner {
    use crate::{common::ui::{UIWriter,
                             UI},
                error::{Error,
                        Result}};
    use std::ffi::OsString;

    pub async fn start(ui: &mut UI,
                       _args: &[OsString],
                       _export_cmd_envvar: &str,
                       _export_pkg_ident_envvar: &str,
                       export_cmd: &str)
                       -> Result<()> {
        let cmd = export_cmd.replace("hab", "").replace("-", " ");
        ui.warn(format!("Running 'hab {}' on this operating system is not yet supported. Try \
                         running this command again on 64-bit Linux.",
                        &cmd))?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(cmd.to_string()))
    }
}
