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

use std::fs::create_dir_all;
use std::fs::{File, read_to_string};
use std::io::{Write};
use std::path::Path;
use serde_json::{self, Value as Json};

use crate::common::templating::TemplateRenderer;
use crate::common::ui::{Status, UIWriter, UI};
use crate::error::Result;

// TODO:
//  * Need to figure out how to merge TOML and JSON
//  * Need to figure out how to load multiple files


pub fn start(
    ui: &mut UI,
    template_path: String,
    default_toml_path: String,
    mock_data_path: Option<String>,
    print: bool,
    render_dir: String,
) -> Result<()> {
    // Strip the file name out of our passed template
    let file_name = match Path::new(&template_path).file_name() {
        Some(name) => name.to_str().clone().unwrap(),
        None => panic!(format!("Something went wrong getting filename of {}", &template_path)),
    }; 

    ui.begin(format!("Rendering: {} into: {}/ as: {}", template_path, render_dir, file_name))?;
    ui.br()?;

    // read our template from file
    let template = read_to_string(&template_path)
        .expect(&format!("something went wrong reading: {}", template_path)); 

    let mock_data = match mock_data_path {
        Some(path) => read_to_string(path.to_string())
            .expect(&format!("Something went wrong reading: {}", path.to_string())),
        None => "{}".to_string(),
    };

    // convert our mock_data into a string(?)
    let json: Json = serde_json::from_str(&mock_data).unwrap();


    // create a template renderer
    let mut renderer = TemplateRenderer::new();
    // register our template 
    renderer
        .register_template_string(&template_path, &template)
        .expect("Could not register template content");
    // render our JSON override in our template.
    let rendered_template = renderer.render(&template_path, &json).ok().unwrap();
    
    if print {
        ui.warn(format!("Rendered template: {}", &template_path))?;

        println!("{}", rendered_template);
    }
    // Render our template file
    create_with_template(ui, &format!("{}/{}", render_dir, file_name), &rendered_template)?;
    // not really sure this is correct...
    Ok(())
}

fn create_with_template(ui: &mut UI, location: &str, template: &str) -> Result<()> {
    let path = Path::new(&location);
    ui.status(Status::Creating, format!("file: {}", location))?;
    // If the directory doesn't exist we need to make it.
    if let Some(directory) = path.parent() {
        create_dir_all(directory)?;
    }
    // Write file to disk
    File::create(path).and_then(|mut file| file.write(template.as_bytes()))?;
    Ok(())
}
