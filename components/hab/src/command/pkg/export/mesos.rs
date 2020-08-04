use crate::{common::ui::UI,
            error::Result};
use std::ffi::OsString;

// It would be more consistent naming to use "export mesos" instead of "mesoize", but for backwards
// compatibility we keep "mesoize"
const EXPORT_CMD_ENVVAR: &str = "HAB_PKG_MESOSIZE_BINARY";
const EXPORT_PKG_IDENT_ENVVAR: &str = "HAB_PKG_MESOSIZE_PKG_IDENT";
const EXPORT_CMD: &str = "hab-pkg-mesosize";

pub async fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    crate::command::pkg::export::export_common::start(ui,
                                                      args,
                                                      EXPORT_CMD_ENVVAR,
                                                      EXPORT_PKG_IDENT_ENVVAR,
                                                      EXPORT_CMD).await
}
