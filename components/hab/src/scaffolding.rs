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
use std::str::FromStr;
use std::path::Path;
use std::io;
use std::fs;

use error::Result;

use hcore::crypto::init;
use hcore::package::PackageIdent;
use common::ui::{UI, Status};

// Check to see if the --scaffolding passed matches available core scaffolding
// If not check if we've been given a pkg ident for a custom scaffolding
pub fn scaffold_check(ui: &mut UI, maybe_scaffold: Option<&str>) -> Result<Option<PackageIdent>> {
    match maybe_scaffold {
        Some(scaffold) => {
            init();
            match scaffold.to_lowercase().as_ref() {
                "core/scaffolding-ruby" |
                "ruby" |
                "rails" |
                "ruby-scaffolding" => {
                    let ident = PackageIdent::from_str("core/scaffolding-ruby").unwrap();
                    ui.status(Status::Using, &format!("ruby scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                "core/scaffolding-go" |
                "go" |
                "golang" |
                "go-scaffolding" => {
                    let ident = PackageIdent::from_str("core/scaffolding-go").unwrap();
                    ui.status(Status::Using, &format!("go scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                "core/scaffolding-node" |
                "node" |
                "node.js" |
                "javascript" |
                "js" => {
                    let ident = PackageIdent::from_str("core/scaffolding-node").unwrap();
                    ui.status(Status::Using, &format!("node scaffolding '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
                _ => {
                    let ident = PackageIdent::from_str(scaffold).unwrap();
                    ui.status(Status::Using, &format!("custom scaffolding: '{}'", ident))?;
                    ui.para("")?;
                    Ok(Some(ident))
                }
            }
        }
        // If nothing was passed try to autodiscover the codebase
        // I'm not saying it's magic. But, MAGIC.
        None => magic_function(ui),
    }
}
fn magic_function(ui: &mut UI) -> Result<Option<PackageIdent>> {
    // Determine if the current dir has an app that can use
    // one of our scaffoldings and use it by default
    ui.begin("Attempting autodiscovery ")?;
    ui.para(
        "No scaffolding type was provided. Let's see if we can figure out \
        what kind of application you're planning to package.",
    )?;
    let current_path = Path::new(".");
    if is_project_ruby(&current_path) {
        let ident = PackageIdent::from_str("core/scaffolding-ruby").unwrap();
        ui.begin("We've detected your app as Ruby")?;
        ui.status(Status::Using, &format!("scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else if is_project_golang(&current_path) {
        let ident = PackageIdent::from_str("core/scaffolding-go").unwrap();
        ui.begin("We've detected your app as Golang")?;
        ui.status(Status::Using, &format!("scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else if is_project_node(&current_path) {
        let ident = PackageIdent::from_str("core/scaffolding-node").unwrap();
        ui.begin("We've detected your app as Node.js")?;
        ui.status(Status::Using, &format!("scaffolding package: '{}'", ident))?;
        ui.para("")?;
        Ok(Some(ident))
    } else {
        ui.warn(
            "Unable to determine the type of app in your current directory",
        )?;
        ui.para(
            "For now, we'll generate a plan with all of the available plan options and callbacks. \
            For more documentation on plan options try passing --withdocs or visit \
            https://www.habitat.sh/docs/reference/plan-syntax/",
        )?;
        Ok(None)
    }
}

fn is_project_ruby<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    if path.as_ref().join("Gemfile").is_file() {
        return true;
    }
    return false;
}

fn is_project_golang<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    if path.as_ref().join("main.go").is_file() ||
        path.as_ref().join("Godeps/Godeps.json").is_file() ||
        path.as_ref().join("vendor/vendor.json").is_file() ||
        path.as_ref().join("glide.yaml").is_file() ||
        project_uses_gb(path.as_ref()).expect("Result<bool> not returned from .go file check")
    {
        return true;
    }
    return false;
}

fn is_project_node<T>(path: T) -> bool
where
    T: AsRef<Path>,
{
    if path.as_ref().join("package.json").is_file() {
        return true;
    }
    return false;
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
    return Ok(false);
}
