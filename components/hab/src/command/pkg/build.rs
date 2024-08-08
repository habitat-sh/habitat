use crate::{command::studio,
            common::ui::UI,
            error::Result};
use habitat_core::origin::Origin;
use std::ffi::OsString;

/// origins - origins whose secret signing keys should be made
///           available in the build
#[allow(clippy::too_many_arguments)]
pub async fn start(ui: &mut UI,
                   plan_context: &str,
                   root: Option<&str>,
                   src: Option<&str>,
                   origins: &[Origin],
                   native_package: bool,
                   reuse: bool,
                   docker: bool,
                   refresh_channel: Option<&str>)
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
    if let Some(refresh_channel) = refresh_channel {
        args.push("-f".into());
        args.push(refresh_channel.into());
    }

    if !origins.is_empty() {
        let signing_key_names = origins.iter()
                                       .map(AsRef::as_ref)
                                       .collect::<Vec<&str>>()
                                       .join(",");
        args.push("-k".into());
        args.push(signing_key_names.into());
    }

    args.push("build".into());

    if native_package {
        args.push("-N".into());
        args.push(plan_context.into());
    } else {
        if studio::docker_studio_support() && reuse {
            args.push("-R".into());
        }
        args.push(plan_context.into());
        if studio::docker_studio_support() && docker {
            args.push("-D".into());
        }
    }
    studio::enter::start(ui, &args).await
}
