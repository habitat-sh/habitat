//! This module provides the logic necessary to implement a license prompt into
//! the hab CLI. When a user runs hab for the first time, they will be prompted
//! to accept the Chef license agreement. If they choose not to, then hab will
//! exit and they will be unable to use the tool.
//!
//! Choosing to accept the license will write a file to either
//! /hab/accepted-licenses/habitat (if run as root) or
//! ~/.hab/accepted-licenses/habitat. The presence of this file will
//! prevent further license prompts. The file itself contains various metadata
//! that we collect, but it's important to note that the contents of this file
//! are not strictly important. The important thing is the presence of the file.
//!
//! If a user was to read our code and pre-emptively create one of the above
//! files and leave its contents completely empty, that is sufficient for the
//! acceptance of the license. We will not overwrite their empty file, nor
//! will we care that it is empty.
//!
//! Because of this, the metadata that we store is not strictly necessary and
//! is more of a best effort scenario. If we can't collect all of it for
//! whatever reason, it's fine.
//!
//! Additionally, there are 2 environment variables that are supported to
//! constitute license acceptance. Setting HAB_LICENSE to "accept" will
//! accept the license and persist the file and setting HAB_LICENSE to
//! "accept-no-persist" will accept the license but not persist the
//! acceptance.
//!
//! More detailed information on the license spec is available at
//! https://github.com/chef/license-acceptance

use crate::{common::ui::{self,
                         UI,
                         UIReader,
                         UIWriter},
            error::{Error,
                    Result},
            hcore::{fs::{FS_ROOT_PATH,
                         am_i_root},
                    users::get_current_username}};
use chrono::{DateTime,
             Utc};
use serde::{Deserialize,
            Serialize};
use std::{env,
          fs::{self,
               File},
          io::Write,
          path::{Path,
                 PathBuf}};

const LICENSE_FILE_FORMAT_VERSION: &str = "1.0";
const LICENSE_ACCEPT_ENVVAR: &str = "HAB_LICENSE";

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseData {
    pub date_accepted: DateTime<Utc>,
    pub accepting_product: String,
    pub accepting_product_version: String,
    pub user: Option<String>,
    pub file_format: String,
}

impl LicenseData {
    pub fn new() -> Result<Self> {
        Ok(LicenseData { date_accepted: Utc::now(),
                         accepting_product: String::from("hab"),
                         accepting_product_version: super::VERSION.to_string(),
                         user: get_current_username()?,
                         file_format: String::from(LICENSE_FILE_FORMAT_VERSION), })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LicenseAcceptance {
    Accepted,
    /// Explicitly deny the license and do not prompt for license acceptance. This is useful for
    /// testing.
    Denied,
    NotYetAccepted,
}

impl LicenseAcceptance {
    pub fn accepted(self) -> bool { self == Self::Accepted }
}

impl Default for LicenseAcceptance {
    fn default() -> Self { Self::NotYetAccepted }
}

pub fn check_for_license_acceptance() -> Result<LicenseAcceptance> {
    match (acceptance_from_env_var()?, license_exists()) {
        // The environment variable takes precedence regardless of the existence of the license
        // file
        (l @ LicenseAcceptance::Accepted, _) | (l @ LicenseAcceptance::Denied, _) => Ok(l),
        (_, true) => Ok(LicenseAcceptance::Accepted),
        (..) => Ok(LicenseAcceptance::NotYetAccepted),
    }
}

pub fn check_for_license_acceptance_and_prompt(ui: &mut UI) -> Result<()> {
    match check_for_license_acceptance()? {
        LicenseAcceptance::Accepted => Ok(()),
        LicenseAcceptance::Denied => Err(Error::LicenseNotAccepted),
        LicenseAcceptance::NotYetAccepted => {
            ui.heading("+---------------------------------------------+")?;
            ui.heading("            Chef License Acceptance")?;
            ui.br()?;
            ui.info("Before you can continue, 1 product license must be accepted.")?;
            ui.info("View the license at https://www.chef.io/end-user-license-agreement")?;
            ui.br()?;
            ui.info("License that needs accepting:")?;
            ui.br()?;
            ui.info("  * Habitat")?;
            ui.br()?;

            if ui.prompt_yes_no("Do you accept the 1 product license?", Some(false))? {
                accept_license(ui)
            } else {
                ui.br()?;
                ui.info("If you do not accept this license you will not be able to use Chef \
                         products.")?;
                ui.br()?;

                if ui.prompt_yes_no("Do you accept the 1 product license?", Some(false))? {
                    accept_license(ui)
                } else {
                    ui.br()?;
                    ui.heading("+---------------------------------------------+")?;
                    Err(Error::LicenseNotAccepted)
                }
            }
        }
    }
}

pub fn accept_license(ui: &mut UI) -> Result<()> {
    if license_exists_for_current_user() {
        ui.info("You have already accepted the license.")?;
        return Ok(());
    }

    ui.br()?;
    ui.info("Accepting 1 product license...")?;

    write_license_file()?;

    ui.status(ui::Status::Custom(ui::Glyph::CheckMark, String::from("")),
              "1 product license accepted.")?;
    ui.br()?;
    ui.heading("+---------------------------------------------+")?;

    Ok(())
}

fn superuser_license_root() -> PathBuf { PathBuf::from(&*FS_ROOT_PATH).join("hab") }

fn user_license_root() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".hab")
    } else {
        superuser_license_root()
    }
}

fn license_path(root_path: &Path) -> PathBuf { root_path.join("accepted-licenses") }

fn license_file(license_path: &Path) -> PathBuf { license_path.join("habitat") }

fn writeable_license_path() -> PathBuf {
    let root_dir = if am_i_root() {
        superuser_license_root()
    } else {
        user_license_root()
    };

    license_path(&root_dir)
}

fn acceptance_from_env_var() -> Result<LicenseAcceptance> {
    match env::var(LICENSE_ACCEPT_ENVVAR) {
        Ok(val) => {
            if &val == "accept" {
                write_license_file()?;
                Ok(LicenseAcceptance::Accepted)
            } else if &val == "accept-no-persist" {
                Ok(LicenseAcceptance::Accepted)
            } else if &val == "deny" {
                Ok(LicenseAcceptance::Denied)
            } else {
                Ok(LicenseAcceptance::NotYetAccepted)
            }
        }
        Err(_) => Ok(LicenseAcceptance::NotYetAccepted),
    }
}

fn write_license_file() -> Result<()> {
    let license = LicenseData::new()?;
    let content = serde_yaml::to_string(&license)?;
    fs::create_dir_all(writeable_license_path())?;
    let mut file = File::create(license_file(&writeable_license_path()))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn license_exists() -> bool {
    license_file(&license_path(&user_license_root())).is_file()
    || license_file(&license_path(&superuser_license_root())).is_file()
}

pub fn license_exists_for_current_user() -> bool {
    if am_i_root() {
        license_file(&license_path(&superuser_license_root())).is_file()
    } else {
        license_file(&license_path(&user_license_root())).is_file()
    }
}
