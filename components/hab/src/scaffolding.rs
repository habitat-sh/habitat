// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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
use std::{fs,
          io,
          path::Path,
          str::FromStr};

use crate::error::Result;

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::{crypto::init,
                    package::PackageIdent}};

const SCAFFOLDING_GO_IDENT: &str = "core/scaffolding-go";
const SCAFFOLDING_GRADLE_IDENT: &str = "core/scaffolding-gradle";
const SCAFFOLDING_NODE_IDENT: &str = "core/scaffolding-node";
const SCAFFOLDING_RUBY_IDENT: &str = "core/scaffolding-ruby";

// Check to see if the --scaffolding passed matches available core scaffolding
// If not check if we've been given a pkg ident for a custom scaffolding
pub fn scaffold_check(ui: &mut UI, maybe_scaffold: Option<&str>) -> Result<Option<PackageIdent>> {
    match maybe_scaffold {
        Some(scaffold) => {
            init();
            match scaffold.to_lowercase().as_ref() {
                SCAFFOLDING_GO_IDENT | "go" => {
                    let ident = PackageIdent::from_str(SCAFFOLDING_GO_IDENT).unwrap();
                    ui.status(Status::Using, &format!("Go Scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                SCAFFOLDING_GRADLE_IDENT | "gradle" => {
                    let ident = PackageIdent::from_str(SCAFFOLDING_GRADLE_IDENT).unwrap();
                    ui.status(Status::Using, &format!("Gradle Scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                SCAFFOLDING_NODE_IDENT | "node" => {
                    let ident = PackageIdent::from_str(SCAFFOLDING_NODE_IDENT).unwrap();
                    ui.status(Status::Using, &format!("Node Scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                SCAFFOLDING_RUBY_IDENT | "ruby" => {
                    let ident = PackageIdent::from_str(SCAFFOLDING_RUBY_IDENT).unwrap();
                    ui.status(Status::Using, &format!("Ruby Scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                _ => {
                    let ident = PackageIdent::from_str(scaffold).unwrap();
                    ui.status(Status::Using, &format!("custom Scaffolding: '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
            }
        }
        // If nothing was passed try to autodiscover the codebase
        // I'm not saying it's magic. But, MAGIC.
        None => autodiscover_scaffolding(ui),
    }
}

fn autodiscover_scaffolding(ui: &mut UI) -> Result<Option<PackageIdent>> {
    // Determine if the current dir has an app that can use
    // one of our scaffoldings and use it by default
    ui.begin("Attempting autodiscovery ")?;
    ui.para("No scaffolding type was provided. Let's see if we can figure out what kind of \
             application you're planning to package.")?;
    let current_path = Path::new(".");
    if is_project_go(&current_path) {
        let ident = PackageIdent::from_str(SCAFFOLDING_GO_IDENT).unwrap();
        ui.begin("We've detected a Go codebase")?;
        ui.status(Status::Using, &format!("Scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else if is_project_gradle(&current_path) {
        let ident = PackageIdent::from_str(SCAFFOLDING_GRADLE_IDENT).unwrap();
        ui.begin("We've detected a Gradle codebase")?;
        ui.status(Status::Using, &format!("Scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else if is_project_node(&current_path) {
        let ident = PackageIdent::from_str(SCAFFOLDING_NODE_IDENT).unwrap();
        ui.begin("We've detected a Node.js codebase")?;
        ui.status(Status::Using, &format!("Scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else if is_project_ruby(&current_path) {
        let ident = PackageIdent::from_str(SCAFFOLDING_RUBY_IDENT).unwrap();
        ui.begin("We've detected a Ruby codebase")?;
        ui.status(Status::Using, &format!("Scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else {
        ui.warn("Unable to determine the type of app in your current directory")?;
        ui.para("For now, we'll generate a plan with all of the available plan variables and \
                 build phase callbacks. For more documentation on plan options try passing \
                 --withdocs or visit https://www.habitat.sh/docs/reference/plan-syntax/")?;
        Ok(None)
    }
}

fn is_project_go<T>(path: T) -> bool
    where T: AsRef<Path>
{
    path.as_ref().join("main.go").is_file()
    || path.as_ref().join("Godeps/Godeps.json").is_file()
    || path.as_ref().join("vendor/vendor.json").is_file()
    || path.as_ref().join("glide.yaml").is_file()
    || project_uses_gb(path.as_ref()).unwrap_or(false)
}

fn is_project_gradle<T>(path: T) -> bool
    where T: AsRef<Path>
{
    path.as_ref().join("build.gradle").is_file() || path.as_ref().join("settings.gradle").is_file()
}

fn is_project_node<T>(path: T) -> bool
    where T: AsRef<Path>
{
    path.as_ref().join("package.json").is_file()
}

fn is_project_ruby<T>(path: T) -> bool
    where T: AsRef<Path>
{
    path.as_ref().join("Gemfile").is_file()
}

fn project_uses_gb(dir: &Path) -> io::Result<bool> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                project_uses_gb(&path)?;
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "go" {
                        return Ok(true);
                    }
                }
            } else {
                return Ok(false);
            }
        }
    }
    Ok(false)
}
