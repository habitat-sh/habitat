use std::ffi::OsString;

use crate::common::ui::UI;

use crate::{command::studio,
            error::Result};

#[allow(clippy::too_many_arguments)]
pub fn start(ui: &mut UI,
             plan_context: &str,
             root: Option<&str>,
             src: Option<&str>,
             keys: Option<&str>,
             reuse: bool,
             windows: bool,
             docker: bool)
             -> Result<()> {
    let mut args: Vec<OsString> = Vec::new();
    if let Some(root) = root {
        args.push("-r".into());
        args.push(root.into());
    }
    if let Some(src) = src {
        args.push("-s".into());
        args.push(src.into());
    }
    if let Some(keys) = keys {
        args.push("-k".into());
        args.push(keys.into());
    }
    args.push("build".into());
    if studio::native_studio_support() && reuse {
        args.push("-R".into());
    }
    args.push(plan_context.into());
    if cfg!(target_os = "windows") && windows {
        args.push("-w".into());
    }
    if studio::native_studio_support() && docker {
        args.push("-D".into());
    }
    studio::enter::start(ui, &args)
}
