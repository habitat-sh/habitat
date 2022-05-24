use crate::error::Result;
use habitat_common::ui::UI;
use std::{ffi::OsString,
          fs::File,
          io::Write,
          process::Command};
use tempfile::Builder;

const HAB_PLAN_BUILD_ENVIRONMENT_SOURCE: &[u8] =
    include_bytes!("../../../../plan-build/bin/environment.bash");
const HAB_PLAN_BUILD_SHARED_SOURCE: &[u8] =
    include_bytes!("../../../../plan-build/bin/shared.bash");
const HAB_PLAN_BUILD_PUBLIC_SOURCE: &[u8] =
    include_bytes!("../../../../plan-build/bin/public.bash");
const HAB_PLAN_BUILD_SOURCE: &[u8] = include_bytes!("../../../../plan-build/bin/hab-plan-build.sh");

pub fn start_native_studio(_ui: &mut UI, args: &[OsString]) -> Result<()> {
    let temp_dir = Builder::new().prefix("hab-plan-build-").tempdir()?;
    File::create(temp_dir.path().join("environment.bash"))?.write_all(HAB_PLAN_BUILD_ENVIRONMENT_SOURCE)?;
    File::create(temp_dir.path().join("shared.bash"))?.write_all(HAB_PLAN_BUILD_SHARED_SOURCE)?;
    File::create(temp_dir.path().join("public.bash"))?.write_all(HAB_PLAN_BUILD_PUBLIC_SOURCE)?;
    File::create(temp_dir.path().join("hab-plan-build.sh"))?.write_all(HAB_PLAN_BUILD_SOURCE)?;
    let mut cmd = Command::new("bash").arg(temp_dir.path().join("hab-plan-build.sh"))
                                      .arg(args.last().unwrap())
                                      .env("HAB_PLAN_FILENAME", "native-plan.sh")
                                      .spawn()?;
    let _exit_status = cmd.wait()?;
    Ok(())
}