use crate::error::{Error,
                   Result};
use anyhow::Context;
use habitat_common::ui::UI;
use log::debug;
use std::{ffi::OsString,
          fs::File,
          io::Write,
          process::Command};
use tempfile::Builder;

const HAB_PLAN_BUILD_SOURCE_FILES: [(&str, &[u8]); 4] =
    [("environment.bash", include_bytes!("../../../../plan-build/bin/environment.bash")),
     ("shared.bash", include_bytes!("../../../../plan-build/bin/shared.bash")),
     ("public.bash", include_bytes!("../../../../plan-build/bin/public.bash")),
     ("hab-plan-build.sh", include_bytes!("../../../../plan-build/bin/hab-plan-build.sh"))];

pub fn start_native_studio(ui: &mut UI, args: &[OsString]) -> Result<()> {
    start_native_studio_impl(ui, args).map_err(Error::NativeStudioError)
}

fn start_native_studio_impl(_ui: &mut UI, args: &[OsString]) -> anyhow::Result<(), anyhow::Error> {
    let temp_dir =
        Builder::new().prefix("hab-plan-build-")
                      .tempdir()
                      .context("Failed to create a temporary directory for the habitat plan \
                                build script")?;
    for (source_file_name, source_file_data) in HAB_PLAN_BUILD_SOURCE_FILES {
        let source_file_path = temp_dir.path().join(source_file_name);
        File::create(&source_file_path).with_context(|| {
                                           format!("Failed to create plan build source file '{}'",
                                                   source_file_path.display())
                                       })?
                                       .write_all(source_file_data)
                                       .with_context(|| {
                                           format!("Failed to write data to plan build source \
                                                    file '{}'",
                                                   source_file_path.display())
                                       })?;
    }
    let mut cmd = Command::new("bash");

    cmd.arg(temp_dir.path().join("hab-plan-build.sh"))
       .arg(args.last().unwrap())
       .env("HAB_NATIVE_PACKAGE", "true");

    if let Some(position) = args.iter().position(|x| x == "-s") {
        cmd.env("HAB_SRC_PATH", &args[position + 1]); 
    }

    debug!("Executing habitat plan build script with command: [{:?}]",
           cmd);

    let mut child = cmd.spawn().context("Failed to run hab plan build script")?;
    let exit_status =
        child.wait()
             .context("Failed to wait for hab plan build script to run to completion")?;
    debug!("Habitat plan build script {}", exit_status);
    Ok(())
}
