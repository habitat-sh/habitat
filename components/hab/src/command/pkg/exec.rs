use crate::{error::{Error,
                    Result},
            hcore::{fs::{find_command,
                         FS_ROOT_PATH},
                    os::process,
                    package::{PackageIdent,
                              PackageInstall}}};
use std::{env,
          ffi::OsString,
          io,
          path::PathBuf};

const PATH_KEY: &str = "PATH";

pub fn start<T>(ident: &PackageIdent, command: T, args: &[OsString]) -> Result<()>
    where T: Into<PathBuf>
{
    let command = command.into();
    let pkg_install = PackageInstall::load(&ident, Some(&*FS_ROOT_PATH))?;
    let mut cmd_env = pkg_install.environment_for_command()?;

    if let Some(path) = cmd_env.get(PATH_KEY) {
        if let Some(val) = env::var_os(PATH_KEY) {
            let mut paths: Vec<PathBuf> = env::split_paths(&path).collect();
            let mut os_paths = env::split_paths(&val).collect();
            paths.append(&mut os_paths);
            let joined = env::join_paths(paths)?;
            let path_str =
                joined.into_string()
                      .map_err(|s| {
                          io::Error::new(io::ErrorKind::InvalidData, s.to_string_lossy())
                      })?;
            cmd_env.insert(PATH_KEY.to_string(), path_str);
        }
    }

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
