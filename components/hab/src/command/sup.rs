use std::ffi::OsString;

use crate::common::ui::UI;

use crate::error::Result;

pub const SUP_CMD: &str = "hab-sup";
pub const SUP_CMD_ENVVAR: &str = "HAB_SUP_BINARY";
pub const SUP_PKG_IDENT: &str = "core/hab-sup";

pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> { inner::start(ui, args) }

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

    use super::{SUP_CMD,
                SUP_CMD_ENVVAR,
                SUP_PKG_IDENT};
    use crate::{error::{Error,
                        Result},
                exec,
                VERSION};

    pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
        let command = match henv::var(SUP_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split('/').collect();
                exec::command_from_min_pkg(ui,
                                           SUP_CMD,
                                           &PackageIdent::from_str(&format!("{}/{}",
                                                                            SUP_PKG_IDENT,
                                                                            version[0]))?)?
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
    use std::{env,
              ffi::OsString};

    use crate::common::ui::{UIWriter,
                            UI};

    use crate::error::{Error,
                       Result};

    pub fn start(ui: &mut UI, _args: &[OsString]) -> Result<()> {
        let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
        ui.warn("Launching a native Supervisor on this operating system is not yet supported. \
                 Try running this command again on 64-bit Linux or Windows.")?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(subcmd))
    }
}
