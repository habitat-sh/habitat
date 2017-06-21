// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common::ui::UI;
use hcore::package::PackageIdent;

use error::Result;

#[allow(dead_code)]
pub struct ExportFormat {
    pkg_ident: PackageIdent,
    cmd: String,
}

#[allow(dead_code)]
impl ExportFormat {
    pub fn pkg_ident(&self) -> &PackageIdent {
        &self.pkg_ident
    }

    pub fn cmd(&self) -> &str {
        &self.cmd
    }
}

pub fn start(ui: &mut UI, ident: &PackageIdent, format: &ExportFormat) -> Result<()> {
    inner::start(ui, ident, format)
}

pub fn format_for(ui: &mut UI, value: &str) -> Result<ExportFormat> {
    inner::format_for(ui, value)
}

#[cfg(target_os = "linux")]
mod inner {
    use std::ffi::OsString;
    use std::path::Path;
    use std::str::FromStr;

    use common::command::package::install;
    use common::ui::{Status, UI};
    use hcore::fs::{cache_artifact_path, FS_ROOT_PATH};
    use hcore::package::{PackageIdent, PackageInstall};
    use hcore::url::default_depot_url;

    use {PRODUCT, VERSION};
    use command::pkg::exec;
    use error::{Error, Result};
    use super::ExportFormat;

    pub fn format_for(_ui: &mut UI, value: &str) -> Result<ExportFormat> {
        match value {
            "docker" => {
                let format = ExportFormat {
                    pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-dockerize")),
                    cmd: "hab-pkg-dockerize".to_string(),
                };
                Ok(format)
            }
            "aci" => {
                let format = ExportFormat {
                    pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-aci")),
                    cmd: "hab-pkg-aci".to_string(),
                };
                Ok(format)
            }
            "mesos" => {
                let format = ExportFormat {
                    pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-mesosize")),
                    cmd: "hab-pkg-mesosize".to_string(),
                };
                Ok(format)
            }
            "tar" => {
                let format = ExportFormat {
                    pkg_ident: try!(PackageIdent::from_str("core/hab-pkg-tarize")),
                    cmd: "hab-pkg-tarize".to_string(),
                };
                Ok(format)
            }
            _ => Err(Error::UnsupportedExportFormat(value.to_string())),
        }
    }

    pub fn start(ui: &mut UI, ident: &PackageIdent, format: &ExportFormat) -> Result<()> {
        let format_ident = format.pkg_ident();
        match PackageInstall::load(format.pkg_ident(), None) {
            Ok(_) => {}
            _ => {
                try!(ui.status(
                    Status::Missing,
                    format!("package for {}", &format_ident),
                ));
                try!(install::start(
                    ui,
                    &default_depot_url(),
                    None, // TODO: Support channels for export
                    &format_ident.to_string(),
                    PRODUCT,
                    VERSION,
                    Path::new(&*FS_ROOT_PATH),
                    &cache_artifact_path(None::<String>),
                    false,
                ));
            }
        }
        let pkg_arg = OsString::from(&ident.to_string());
        exec::start(&format_ident, &format.cmd(), vec![pkg_arg])
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use error::{Error, Result};
    use common::ui::UI;
    use hcore::package::PackageIdent;
    use std::env;
    use super::ExportFormat;

    pub fn format_for(ui: &mut UI, value: &str) -> Result<ExportFormat> {
        try!(ui.warn(format!(
            "∅ Exporting {} packages from this operating system is not yet \
                           supported. Try running this command again on a 64-bit Linux \
                           operating system.\n",
            value
        )));
        try!(ui.br());
        let e = Error::UnsupportedExportFormat(value.to_string());
        Err(e)
    }

    pub fn start(ui: &mut UI, _ident: &PackageIdent, _format: &ExportFormat) -> Result<()> {
        let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
        let subsubcmd = env::args().nth(2).unwrap_or("<unknown>".to_string());
        try!(ui.warn(
            "Exporting packages from this operating system is not yet supported. Try \
                   running this command again on a 64-bit Linux operating system.",
        ));
        try!(ui.br());
        Err(Error::SubcommandNotSupported(
            format!("{} {}", subcmd, subsubcmd),
        ))

    }
}
