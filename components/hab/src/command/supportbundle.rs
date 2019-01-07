// Copyright (c) 2016-2018 Chef Software Inc. and/or applicable contributors
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

use chrono::Local;
use crate::common::ui::{Status, UIWriter, UI};
use crate::error::{Error, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::hcore::fs::FS_ROOT_PATH;
use crate::hcore::os::net::hostname;
use std::env;
use std::error::Error as StdErr;
use std::fs;
use std::fs::File;
use std::path::{Path, MAIN_SEPARATOR};
use std::process;
use tar;

fn lookup_hostname() -> Result<String> {
    match hostname() {
        Ok(hostname) => Ok(hostname),
        Err(_) => Err(Error::NameLookup),
    }
}

pub fn start(ui: &mut UI) -> Result<()> {
    let dt = Local::now();
    ui.status(
        Status::Generating,
        format!("New Support Bundle at {}", dt.format("%Y-%m-%d %H:%M:%S")),
    )?;
    let host = match lookup_hostname() {
        Ok(host) => host,
        Err(e) => {
            let host = String::from("localhost");
            ui.warn(format!(
                "Hostname lookup failed; using fallback of {} ({})",
                host, e
            ))?;
            host
        }
    };
    let cwd = env::current_dir().unwrap();
    let tarball_name = format!(
        "support-bundle-{}-{}.tar.gz",
        &host,
        dt.format("%Y%m%d%H%M%S")
    );

    let sup_root = Path::new(&*FS_ROOT_PATH).join("hab").join("sup");
    let tar_gz = File::create(&tarball_name)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.follow_symlinks(false);

    if sup_root.exists() {
        ui.status(
            Status::Adding,
            format!("files from {}", &sup_root.display()),
        )?;
        match tar.append_dir_all(format!("hab{}sup", MAIN_SEPARATOR), &sup_root) {
            Err(why) => {
                ui.fatal(format!(
                    "Failed to add all files into the tarball: {:?}",
                    why.description()
                ))?;
                fs::remove_file(&tarball_name)?;
                process::exit(1)
            }
            Ok(_) => {}
        }
    } else {
        ui.fatal(format!(
            "Failed to find Supervisor root directory {}",
            &sup_root.display()
        ))?;
        process::exit(1)
    }

    ui.status(
        Status::Created,
        format!("{}{}{}", cwd.display(), MAIN_SEPARATOR, &tarball_name),
    )?;

    Ok(())
}
