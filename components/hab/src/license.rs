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
                         UIReader,
                         UIWriter,
                         UI},
            error::{Error,
                    Result},
            hcore::{fs::{am_i_root,
                         FS_ROOT_PATH},
                    users::get_current_username}};
use chrono::prelude::*;
use dirs;
use serde_yaml;
use std::{env,
          fs::{self,
               File},
          io::Write,
          path::PathBuf};

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
    pub fn new() -> Self {
        LicenseData { date_accepted: Utc::now(),
                      accepting_product: String::from("hab"),
                      accepting_product_version: super::VERSION.to_string(),
                      user: get_current_username(),
                      file_format: String::from(LICENSE_FILE_FORMAT_VERSION), }
    }
}

pub fn check_for_license_acceptance_and_prompt(ui: &mut UI) -> Result<()> {
    if license_exists() || env_var_present()? {
        return Ok(());
    }

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
        ui.info("If you do not accept this license you will not be able to use Chef products.")?;
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

pub fn accept_license(ui: &mut UI) -> Result<()> {
    if license_exists() {
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

fn license_path() -> PathBuf {
    let hab_dir = if am_i_root() {
        PathBuf::from(&*FS_ROOT_PATH).join("hab")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".hab")
    } else {
        panic!("No home directory available. Unable to find a suitable place to write a license \
                file.");
    };

    hab_dir.join("accepted-licenses")
}

fn env_var_present() -> Result<bool> {
    match env::var(LICENSE_ACCEPT_ENVVAR) {
        Ok(val) => {
            if &val == "accept" {
                write_license_file()?;
                Ok(true)
            } else if &val == "accept-no-persist" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Ok(false),
    }
}

fn write_license_file() -> Result<()> {
    let license = LicenseData::new();
    let content = serde_yaml::to_string(&license)?;
    fs::create_dir_all(license_path())?;
    let mut file = File::create(license_file())?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn license_file() -> PathBuf { license_path().join("habitat") }

pub fn license_exists() -> bool { license_file().is_file() }
