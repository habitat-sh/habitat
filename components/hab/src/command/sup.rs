use crate::{common::ui::UI,
            error::{Error,
                    Result},
            exec,
            hcore::{crypto::init,
                    env as henv,
                    fs::find_command,
                    os::process,
                    package::PackageIdent},
            VERSION};
use std::{ffi::OsString,
          path::PathBuf,
          str::FromStr};

pub const SUP_CMD: &str = "hab-sup";
pub const SUP_CMD_ENVVAR: &str = "HAB_SUP_BINARY";
pub const SUP_PKG_IDENT: &str = "chef/hab-sup";

pub async fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    let command = match henv::var(SUP_CMD_ENVVAR) {
        Ok(command) => PathBuf::from(command),
        Err(_) => {
            init()?;
            let version: Vec<&str> = VERSION.split('/').collect();
            exec::command_from_min_pkg(ui,
                                       SUP_CMD,
                                       &PackageIdent::from_str(&format!("{}/{}",
                                                                        SUP_PKG_IDENT,
                                                                        version[0]))?).await?
        }
    };
    if let Some(cmd) = find_command(&command) {
        process::become_command(cmd, args)?;
        Ok(())
    } else {
        Err(Error::ExecCommandNotFound(command))
    }
}
