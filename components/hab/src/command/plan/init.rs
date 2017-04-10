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

use std::env;
use std::fs::create_dir_all;
use std::fs::{File, canonicalize};
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;

use handlebars::Handlebars;

use common::ui::{UI, Status};
use error::Result;

const PLAN_TEMPLATE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                                         "/static/template_plan.sh"));
const DEFAULT_TOML_TEMPLATE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
                                                                 "/static/template_default.toml"));

pub fn start(ui: &mut UI,
             origin: String,
             include_callbacks: bool,
             maybe_name: Option<String>)
             -> Result<()> {
    try!(ui.begin("Constructing a cozy habitat for your app..."));
    try!(ui.br());

    let (root, name) = match maybe_name {
        Some(name) => (name.clone(), name.clone()),
        // The name of the current working directory.
        None => {
            ("habitat".into(),
             canonicalize(".")
                 .ok()
                 .and_then(|path| {
                               path.components()
                                   .last()
                                   .and_then(|val| {
                                                 // Type gymnastics!
                                                 val.as_os_str().to_os_string().into_string().ok()
                                             })
                           })
                 .unwrap_or("unnamed".into()))
        }
    };

    // Build out the variables passed.
    let handlebars = Handlebars::new();
    let mut data = HashMap::new();
    data.insert("pkg_name".to_string(), name);
    data.insert("pkg_origin".to_string(), origin);
    if include_callbacks {
        data.insert("include_callbacks".to_string(), "true".to_string());
    }

    // Add all environment variables that start with "pkg_" as variables in
    // the template.
    for (key, value) in env::vars() {
        if key.starts_with("pkg_") {
            data.insert(key, value);
        }
    }

    // We want to render the configured variables.
    let rendered_plan = try!(handlebars.template_render(PLAN_TEMPLATE, &data));
    try!(create_with_template(ui, &format!("{}/plan.sh", root), &rendered_plan));
    try!(ui.para("The `plan.sh` is the foundation of your new habitat. You can \
        define core metadata, dependencies, and tasks. More documentation here: \
        https://www.habitat.sh/docs/reference/plan-syntax/"));

    let rendered_default_toml = try!(handlebars.template_render(DEFAULT_TOML_TEMPLATE, &data));
    try!(create_with_template(ui,
                              &format!("{}/default.toml", root),
                              &rendered_default_toml));
    try!(ui.para("The `default.toml` allows you to declare default values for `cfg` prefixed
        variables. For more information see here:  \
        https://www.habitat.sh/docs/reference/plan-syntax/#runtime-configuration-settings"));

    let config_path = format!("{}/config/", root);
    match Path::new(&config_path).exists() {
        true => {
            try!(ui.status(Status::Using,
                           format!("existing directory: {}", config_path)))
        }
        false => {
            try!(ui.status(Status::Creating, format!("directory: {}", config_path)));
            try!(create_dir_all(&config_path));
        }
    };
    try!(ui.para("The `config` directory is where you can set up configuration files for your app. \
               They are influenced by `default.toml`. For more information see here: \
               https://www.habitat.sh/docs/reference/plan-syntax/#runtime-configuration-settings"));

    let hooks_path = format!("{}/hooks/", root);
    match Path::new(&hooks_path).exists() {
        true => try!(ui.status(Status::Using, format!("existing directory: {}", hooks_path))),
        false => {
            try!(ui.status(Status::Creating, format!("directory: {}", hooks_path)));
            try!(create_dir_all(&hooks_path));
        }
    };
    try!(ui.para("The `hooks` directory is where you can create a number of automation hooks into \
               your habitat. There are several hooks to create and tweak! See the full list \
               with info here: https://www.habitat.sh/docs/reference/plan-syntax/#hooks"));

    try!(ui.end("A happy abode for your code has been initialized! Now it's time to explore!"));
    Ok(())
}

fn create_with_template(ui: &mut UI, location: &str, template: &str) -> Result<()> {
    let path = Path::new(&location);
    match path.exists() {
        false => {
            try!(ui.status(Status::Creating, format!("file: {}", location)));
            // If the directory doesn't exist we need to make it.
            if let Some(directory) = path.parent() {
                try!(create_dir_all(directory));
            }
            // Create and then render the template with Handlebars
            try!(File::create(path).and_then(|mut file| file.write(template.as_bytes())));
        }
        true => {
            // If the user has already configured a file overwriting would be impolite.
            try!(ui.status(Status::Using, format!("existing file: {}", location)));
        }
    };
    Ok(())
}
