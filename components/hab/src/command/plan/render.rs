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

use std::collections::HashMap;
// use std::env;
use std::fs::create_dir_all;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use serde::Serialize;
use serde_json::{self, Value as Json};

use handlebars::Handlebars;

use crate::common::templating::*;
use crate::common::ui::{Status, UIWriter, UI};
use crate::error::Result;

pub fn start(
    ui: &mut UI,
    template_path: String,
    default_toml_path: String,
    mock_data_path: Option<String>,
    render_dir: String,
) -> Result<()> {
    // create necessary vars
    let handlebars = Handlebars::new();
    // let mut data = HashMap::new();

    // let mut new_data = convert_to_json(&data);
    // Strip the file name out of our passed template
    let file_name = match Path::new(&template_path).file_name() {
        Some(name) => name.to_str().clone().unwrap(),
        None => panic!(format!("Something went wrong getting filename of {}", &template_path)),
    }; 

    ui.begin(format!("Rendering: {} into: {}/ as: {}", template_path, render_dir, file_name))?;
    ui.br()?;

    // Build out the variables passed.
    // data.insert("pkg_name".to_string(), "test".to_string());

    // read our template from file
    let template = read_to_string(&template_path)
        .expect(&format!("something went wrong reading: {}", template_path)); 

    let mock_data = match mock_data_path {
        Some(path) => read_to_string(path.to_string())
            .expect(&format!("Something went wrong reading: {}", path.to_string())),
        None => "{}".to_string(),
    };


    let json: Json = serde_json::from_str(&mock_data).unwrap();
    // println!("mock_data: {}", json);
    // println!("{}", template);
    //let content = render(template, json)?;

    let mut renderer = TemplateRenderer::new();
    renderer
        .register_template_string("testing", &template)
        .expect("Could not register template content");
    renderer
        .render("testing", &mock_data)
        .expect("Could not render template");


    // We want to render the configured variables.
    // let rendered_template = handlebars.template_render(&template, &json)?;
    
    // println!("#################\nRendered template:\n{}\n#############", rendered_template);
    // println!("######\nRendered template:\n{}\n#######", renderer);
    create_with_template(ui, &format!("{}/{}", render_dir, file_name), &renderer.to_string())?;
    Ok(())
}

fn create_with_template(ui: &mut UI, location: &str, template: &str) -> Result<()> {
    let path = Path::new(&location);
    ui.status(Status::Creating, format!("file: {}", location))?;
    // If the directory doesn't exist we need to make it.
    if let Some(directory) = path.parent() {
        create_dir_all(directory)?;
    }
    // Create and then render the template with Handlebars
    File::create(path).and_then(|mut file| file.write(template.as_bytes()))?;
    Ok(())
}
