// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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
use std::{fs::{self,
               File},
          io::Write,
          path::PathBuf};

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
                      file_format: String::from("1.0"), }
    }
}

pub fn check_for_license_acceptance_and_prompt(ui: &mut UI) -> Result<()> {
    if license_exists() {
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

    let license = LicenseData::new();
    let content = serde_yaml::to_string(&license)?;
    fs::create_dir_all(license_path())?;
    let mut file = File::create(license_file())?;
    file.write_all(content.as_bytes())?;

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

fn license_file() -> PathBuf { license_path().join("habitat") }

fn license_exists() -> bool { license_file().is_file() }
