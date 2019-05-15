use std::{env,
          ffi::OsString,
          path::PathBuf};

use crate::hcore::{fs::{find_command,
                        FS_ROOT_PATH},
                   os::process,
                   package::{PackageIdent,
                             PackageInstall}};

use crate::error::{Error,
                   Result};

pub fn start<T>(ident: &PackageIdent, command: T, args: &[OsString]) -> Result<()>
    where T: Into<PathBuf>
{
    let command = command.into();
    let pkg_install = PackageInstall::load(&ident, Some(&*FS_ROOT_PATH))?;
    let cmd_env = pkg_install.environment_for_command()?;

    for (key, value) in cmd_env.into_iter() {
        debug!("Setting: {}='{}'", key, value);
        env::set_var(key, value);
    }
    let command = match find_command(&command) {
        Some(path) => path,
        None => return Err(Error::ExecCommandNotFound(command)),
    };
    let mut display_args = command.to_string_lossy().into_owned();
    for arg in args {
        display_args.push(' ');
        display_args.push_str(arg.to_string_lossy().as_ref());
    }
    debug!("Running: {}", display_args);
    process::become_command(command, args)?;
    Ok(())
}
