use crate::error::Result;
use habitat_common::ui::UI;
use habitat_core::{package::PackageIdent,
                   ChannelIdent};

pub mod cf;
pub mod docker;
pub mod helm;
pub mod kubernetes;
pub mod tar;

mod export_common;

#[allow(dead_code)]
pub struct ExportFormat {
    pkg_ident: PackageIdent,
    cmd:       String,
}

#[allow(dead_code)]
impl ExportFormat {
    pub fn pkg_ident(&self) -> &PackageIdent { &self.pkg_ident }

    pub fn cmd(&self) -> &str { &self.cmd }
}

pub fn start(ui: &mut UI,
             url: &str,
             channel: &ChannelIdent,
             ident: &PackageIdent,
             format: &ExportFormat)
             -> Result<()> {
    inner::start(ui, url, channel, ident, format)
}

pub fn format_for(ui: &mut UI, value: &str) -> Result<ExportFormat> { inner::format_for(ui, value) }

#[cfg(target_os = "linux")]
mod inner {
    use std::{env,
              ffi::OsString,
              str::FromStr};

    use crate::{common::ui::UI,
                hcore::{crypto::{default_cache_key_path,
                                 init},
                        env::Config,
                        fs::find_command,
                        package::PackageIdent,
                        url::BLDR_URL_ENVVAR,
                        ChannelIdent}};

    use super::ExportFormat;
    use crate::{command,
                error::{Error,
                        Result},
                exec,
                VERSION};

    pub fn format_for(_ui: &mut UI, value: &str) -> Result<ExportFormat> {
        let version: Vec<_> = VERSION.split('/').collect();
        match value {
            "aci" => {
                let format =
                    ExportFormat { pkg_ident:
                                       PackageIdent::from_str(&format!("core/hab-pkg-aci/{}",
                                                                       version[0]))?,
                                   cmd:       "hab-pkg-aci".to_string(), };
                Ok(format)
            }
            "mesos" => {
                let format = ExportFormat { pkg_ident: PackageIdent::from_str(&format!(
                    "core/hab-pkg-mesosize/{}",
                    version[0]
                ))?,
                                            cmd:       "hab-pkg-mesosize".to_string(), };
                Ok(format)
            }
            _ => Err(Error::UnsupportedExportFormat(value.to_string())),
        }
    }

    pub fn start(ui: &mut UI,
                 url: &str,
                 channel: &ChannelIdent,
                 ident: &PackageIdent,
                 format: &ExportFormat)
                 -> Result<()> {
        init();
        let command = exec::command_from_min_pkg(ui,
                                                 format.cmd(),
                                                 format.pkg_ident(),
                                                 &default_cache_key_path(None),
                                                 0)?;

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            let pkg_arg = OsString::from(&ident.to_string());
            env::set_var(BLDR_URL_ENVVAR, url);
            env::set_var(ChannelIdent::ENVVAR, channel.to_string());
            // TODO fn: Currently, the PATH-setting behavior of `hab pkg exec` is being used to put
            // dependent programs such as `docker` on `$PATH`. This is not ideal and we should be
            // using `hcore::os::process::become_command` but for the moment we'll continue to use
            // the behavior of the `pkg exec` subcommand.
            command::pkg::exec::start(format.pkg_ident(), cmd, &[pkg_arg])
        } else {
            Err(Error::ExecCommandNotFound(command))
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use super::ExportFormat;
    use crate::error::{Error,
                       Result};
    use habitat_common::ui::{UIWriter,
                             UI};
    use habitat_core::{package::PackageIdent,
                       ChannelIdent};
    use std::env;

    pub fn format_for(ui: &mut UI, value: &str) -> Result<ExportFormat> {
        ui.warn(format!("∅ Exporting {} packages from this operating system is not yet \
                         supported. Try running this command again on a 64-bit Linux operating \
                         system.\n",
                        value))?;
        ui.br()?;
        let e = Error::UnsupportedExportFormat(value.to_string());
        Err(e)
    }

    pub fn start(ui: &mut UI,
                 _url: &str,
                 _channel: &ChannelIdent,
                 _ident: &PackageIdent,
                 _format: &ExportFormat)
                 -> Result<()> {
        let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
        let subsubcmd = env::args().nth(2).unwrap_or("<unknown>".to_string());
        ui.warn("Exporting packages from this operating system is not yet supported. Try \
                 running this command again on a 64-bit Linux operating system.")?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(format!("{} {}", subcmd, subsubcmd)))
    }
}
