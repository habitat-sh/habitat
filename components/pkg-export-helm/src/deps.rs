use clap;
use std::{io::Write,
          path::Path,
          process::Command};

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            error::Error,
            export_docker::Result};

pub const DEFAULT_OPERATOR_VERSION: &str = "0.6.1";
pub const OPERATOR_REPO_URL: &str =
    "https://habitat-sh.github.io/habitat-operator/helm/charts/stable/";

pub struct Deps {
    _operator_version: String,
    update:            bool,
}

impl Deps {
    pub fn new_for_cli_matches(matches: &clap::ArgMatches<'_>) -> Self {
        Deps { _operator_version: matches.value_of("OPERATOR_VERSION")
                                         .unwrap_or(DEFAULT_OPERATOR_VERSION)
                                         .to_owned(),
               update:            matches.is_present("DOWNLOAD_DEPS"), }
    }

    pub fn generate(&mut self, write: &mut dyn Write) -> Result<()> {
        // TODO: Until this Helm issue is resolved or has a decent workaround, let's skip the
        //       operator dependency:
        //
        //       https://github.com/kubernetes/helm/issues/3632
        //       https://github.com/kubernetes/helm/issues/2994
        //
        // let out = self.into_string()?;
        let out = String::new();
        write.write_all(out.as_bytes())?;

        Ok(())
    }

    pub fn download<P: AsRef<Path>>(&self, dir: P, ui: &mut UI) -> Result<()> {
        if !self.update {
            return Ok(());
        }

        Command::new("helm").arg("repo")
                            .arg("add")
                            .arg("habitat-operator")
                            .arg(OPERATOR_REPO_URL)
                            .spawn()
                            .map_err(|_| Error::HelmLaunchFailed)
                            .and_then(|mut c| {
                                if c.wait().map_err(|_| Error::HelmLaunchFailed)?.success() {
                                    Ok(())
                                } else {
                                    Err(Error::HelmNotSetup(String::from("Failed to update \
                                                                          chart dependencies")))
                                }
                            })?;

        ui.status(Status::Downloading, "dependencies")?;

        Command::new("helm").arg("dep")
                            .arg("up")
                            .arg(dir.as_ref().as_os_str())
                            .spawn()?
                            .wait()
                            .map(|_| ())
                            .map_err(From::from)
    }
}
