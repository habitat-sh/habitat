// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

use std::{collections::HashMap,
          env,
          fs::{canonicalize,
               create_dir_all,
               File,
               OpenOptions},
          io::{BufRead,
               BufReader,
               Write},
          path::Path};

use crate::hcore::package::PackageIdent;
use handlebars::Handlebars;

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            error::Result};

const DEFAULT_PLAN_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/default_template_plan.sh"
));
const FULL_PLAN_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/full_template_plan.sh"
));
const DEFAULT_PLAN_PS1_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/default_template_plan.ps1"
));
const FULL_PLAN_PS1_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/full_template_plan.ps1"
));
const DEFAULT_TOML_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/template_default.toml"
));
const GITIGNORE_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/template_gitignore"
));
const README_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/template_README.md"
));

const DEFAULT_PKG_VERSION: &str = "0.1.0";

pub fn start(
    ui: &mut UI,
    origin: String,
    with_docs: bool,
    with_callbacks: bool,
    with_all: bool,
    windows: bool,
    scaffolding_ident: Option<PackageIdent>,
    maybe_name: Option<String>,
) -> Result<()> {
    ui.begin("Constructing a cozy habitat for your app...")?;
    ui.br()?;

    let (root, name) = match maybe_name {
        Some(name) => (name.clone(), name.clone()),
        // The name of the current working directory.
        None => {
            (
                "habitat".into(),
                canonicalize(".")
                    .ok()
                    .and_then(|path| {
                        path.components().last().and_then(|val| {
                            // Type gymnastics!
                            val.as_os_str().to_os_string().into_string().ok()
                        })
                    })
                    .unwrap_or_else(|| "unnamed".into()),
            )
        }
    };

    // Build out the variables passed.
    let handlebars = Handlebars::new();
    let mut data = HashMap::new();
    data.insert("pkg_name".to_string(), name);
    data.insert("pkg_origin".to_string(), origin);
    data.insert("pkg_version".to_string(), DEFAULT_PKG_VERSION.to_string());

    let scaffold = match scaffolding_ident {
        Some(ident) => Some(data.insert("scaffolding_ident".to_string(), ident.to_string())),
        None => None,
    };

    if with_callbacks {
        data.insert("with_callbacks".to_string(), "true".to_string());
    }
    if with_docs {
        data.insert("with_docs".to_string(), "true".to_string());
    }
    // Add all environment variables that start with "pkg_" as variables in
    // the template.
    for (key, value) in env::vars() {
        if key.starts_with("pkg_") {
            data.insert(key, value);
        }
    }

    // We want to render the configured variables.
    if windows {
        if with_all || scaffold.is_none() {
            let rendered_plan = handlebars.template_render(FULL_PLAN_PS1_TEMPLATE, &data)?;
            create_with_template(ui, &format!("{}/plan.ps1", root), &rendered_plan)?;
        } else {
            let rendered_plan = handlebars.template_render(DEFAULT_PLAN_PS1_TEMPLATE, &data)?;
            create_with_template(ui, &format!("{}/plan.ps1", root), &rendered_plan)?;
        }
    } else if with_all || scaffold.is_none() {
        let rendered_plan = handlebars.template_render(FULL_PLAN_TEMPLATE, &data)?;
        create_with_template(ui, &format!("{}/plan.sh", root), &rendered_plan)?;
    } else {
        let rendered_plan = handlebars.template_render(DEFAULT_PLAN_TEMPLATE, &data)?;
        create_with_template(ui, &format!("{}/plan.sh", root), &rendered_plan)?;
    }
    ui.para(
        "`plan.sh` is the foundation of your new habitat. It contains metadata, dependencies, and \
         tasks.",
    )?;
    let rendered_default_toml = handlebars.template_render(DEFAULT_TOML_TEMPLATE, &data)?;
    create_with_template(
        ui,
        &format!("{}/default.toml", root),
        &rendered_default_toml,
    )?;
    ui.para("`default.toml` contains default values for `cfg` prefixed variables.")?;

    let rendered_readme_md = handlebars.template_render(README_TEMPLATE, &data)?;
    create_with_template(ui, &format!("{}/README.md", root), &rendered_readme_md)?;
    ui.para("`README.md` contains a basic README document which you should update.")?;

    let config_path = format!("{}/config/", root);
    if Path::new(&config_path).exists() {
        ui.status(
            Status::Using,
            format!("existing directory: {}", config_path),
        )?;
    } else {
        ui.status(Status::Creating, format!("directory: {}", config_path))?;
        create_dir_all(&config_path)?;
    }
    ui.para("`/config/` contains configuration files for your app.")?;

    let hooks_path = format!("{}/hooks/", root);
    if Path::new(&hooks_path).exists() {
        ui.status(Status::Using, format!("existing directory: {}", hooks_path))?;
    } else {
        ui.status(Status::Creating, format!("directory: {}", hooks_path))?;
        create_dir_all(&hooks_path)?;
    }
    ui.para("`/hooks/` contains automation hooks into your habitat.")?;

    ui.para(
        "For more information on any of the files: \
         https://www.habitat.sh/docs/reference/plan-syntax/",
    )?;

    render_ignorefile(ui, &root)?;

    ui.end("An abode for your code is initialized!")?;
    Ok(())
}

fn render_ignorefile(ui: &mut UI, root: &str) -> Result<()> {
    let parent = format!("{}/..", root);
    let expanded = canonicalize(&parent)?;
    let current_path = Path::new(&expanded);

    if is_git_managed(&current_path) {
        let target = format!("{}/.gitignore", parent);
        let target_path = Path::new(&target);

        if !target_path.exists() {
            create_with_template(ui, &target, GITIGNORE_TEMPLATE)?
        } else {
            let file = OpenOptions::new()
                .read(true)
                .append(true)
                .open(target_path)?;

            let entries: Vec<String> = BufReader::new(&file)
                .lines()
                .map(|l| l.expect("Failed to parse line"))
                .collect();

            let mut appended = 0;

            for line in GITIGNORE_TEMPLATE.lines() {
                let s = line.to_string();

                if !entries.contains(&s) {
                    writeln!(&file, "{}", s)?;
                    appended += 1;
                }
            }

            ui.status(
                Status::Using,
                format!("existing file: {} ({} lines appended)", &target, appended),
            )?;
        }
    }
    Ok(())
}

fn is_git_managed(path: &Path) -> bool {
    path.join(".git").is_dir()
        || path
            .parent()
            .map_or(false, |parent| is_git_managed(&parent))
}

fn create_with_template(ui: &mut UI, location: &str, template: &str) -> Result<()> {
    let path = Path::new(&location);
    if path.exists() {
        // If the user has already configured a file overwriting would be impolite.
        ui.status(Status::Using, format!("existing file: {}", location))?;
    } else {
        ui.status(Status::Creating, format!("file: {}", location))?;
        // If the directory doesn't exist we need to make it.
        if let Some(directory) = path.parent() {
            create_dir_all(directory)?;
        }
        // Create and then render the template with Handlebars
        File::create(path).and_then(|mut file| file.write(template.as_bytes()))?;
    }
    Ok(())
}
