use crate::{command::sup::{SUP_CMD,
                           SUP_CMD_ENVVAR,
                           SUP_PKG_IDENT},
            common::ui::UI,
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

#[cfg(feature = "v2")]
use crate::cli::hab::sup::SupRun;

const LAUNCH_CMD: &str = "hab-launch";
const LAUNCH_CMD_ENVVAR: &str = "HAB_LAUNCH_BINARY";
const LAUNCH_PKG_IDENT: &str = "core/hab-launcher";

#[cfg(feature = "v2")]
pub async fn start(ui: &mut UI, sup_run: SupRun, args: &[OsString]) -> Result<()> {
    init()?;
    let channel = sup_run.shared_load.channel;
    if henv::var(SUP_CMD_ENVVAR).is_err() {
        let version: Vec<&str> = VERSION.split('/').collect();
        exec::command_from_min_pkg_with_channel(ui,
                                                SUP_CMD,
                                                &PackageIdent::from_str(&format!("{}/{}",
                                                                                 SUP_PKG_IDENT,
                                                                                 version[0]))?,
                                                channel.clone()).await?;
    }
    let command = match henv::var(LAUNCH_CMD_ENVVAR) {
        Ok(command) => PathBuf::from(command),
        Err(_) => {
            init()?;
            exec::command_from_min_pkg_with_channel(ui,
                                                    LAUNCH_CMD,
                                                    &PackageIdent::from_str(LAUNCH_PKG_IDENT)?,
                                                    channel).await?
        }
    };
    if let Some(cmd) = find_command(&command) {
        process::become_command(cmd, args)?;
        Ok(())
    } else {
        Err(Error::ExecCommandNotFound(command))
    }
}
