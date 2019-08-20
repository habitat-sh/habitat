use crate::error::{Error,
                   Result};
use habitat_common::{outputln,
                     util::path};
use habitat_core::fs::find_command;
use libc;
use std::{env,
          ffi::CString,
          path::PathBuf,
          ptr};

/// Our output key
static LOGKEY: &str = "SH";

/// Start a bash shell
pub fn bash() -> Result<()> {
    set_path()?;
    outputln!("Starting your bashlike shell; enjoy!");
    exec_shell("bash")
}

/// Start a sh shell
pub fn sh() -> Result<()> {
    set_path()?;
    outputln!("Starting your bourne shell; enjoy!");
    exec_shell("sh")
}

fn set_path() -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let new_path = path::append_interpreter_and_path(&mut paths)?;

    debug!("Setting the PATH to {}", &new_path);
    env::set_var("PATH", &new_path);
    Ok(())
}

fn exec_shell(cmd: &str) -> Result<()> {
    let cmd_path = match find_command(cmd) {
        Some(p) => p,
        None => return Err(Error::ExecCommandNotFound(cmd.to_string())),
    };
    let c_cmd = CString::new(cmd_path.to_string_lossy().into_owned())?;
    let mut argv = [c_cmd.as_ptr(), ptr::null()];
    debug!("Exec {:?}", &cmd_path.display());
    unsafe {
        libc::execvp(c_cmd.as_ptr(), argv.as_mut_ptr());
    }
    Ok(())
}
