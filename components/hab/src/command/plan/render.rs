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
use toml::Value;

use crate::common::templating::TemplateRenderer;
use crate::common::ui::{Status, UIWriter, UI};
use crate::error::Result;

pub fn start(
    ui: &mut UI,
    template_path: String,
    default_toml_path: String,
    user_toml_path: Option<String>,
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

    // create a "data" json struct
    let mut data: Json = serde_json::from_str("{}").unwrap();

    // import default.toml values, convert to JSON
    ui.begin(format!("Importing default.toml: {}", &default_toml_path))?;
    let default_toml = read_to_string(&default_toml_path)
        .expect(&format!("Something went wrong reading: {}", &default_toml_path));
    let default_toml_value = default_toml.parse::<Value>().expect("Error parsing TOML");
    let default_toml_string = serde_json::to_string_pretty(&default_toml_value).expect("Error encoding JSON");
    let default_toml_json: Json = serde_json::from_str(&format!(r#"{{ "cfg": {} }}"#, &default_toml_string)).unwrap();

    // merge default into data struct
    merge(&mut data, default_toml_json);

    // import default.toml values, convert to JSON
    // ui.begin(format!("Importing user.toml: {}", &user_toml_path));
    let user_toml = match user_toml_path {
        Some(path) => {
            ui.begin(format!("Importing user.toml: {}", path.to_string()))?;
            read_to_string(path.to_string())
                .expect(&format!("Something went wrong reading: {}", path.to_string()))
        },
        None => "".to_string(),
    };
    // copy/paste ftw!  This could probably stand to be DRY'd up.../there's gotta be an easier way
    let user_toml_value = user_toml.parse::<Value>().expect("Error parsing TOML");
    let user_toml_string = serde_json::to_string_pretty(&user_toml_value).expect("Error encoding JSON");
    let user_toml_json: Json = serde_json::from_str(&format!(r#"{{ "cfg": {} }}"#, &user_toml_string)).unwrap();

    // merge default into data struct
    merge(&mut data, user_toml_json);

    // read mock data if provided
    // ui.begin(format!("Importing override: {}", &mock_data_path));
    let mock_data = match mock_data_path {
        Some(path) => {
            ui.begin(format!("Importing override file: {}", path.to_string()))?;
            read_to_string(path.to_string())
              .expect(&format!("Something went wrong reading: {}", path.to_string()))
        },
        None => "{}".to_string(),
    };

    // convert our mock_data into a json::Value
    let mock_data_json: Json = serde_json::from_str(&mock_data).unwrap();

    // merge mock data into
    merge(&mut data, mock_data_json);

    // create a template renderer
    let mut renderer = TemplateRenderer::new();
    // register our template 
    renderer
        .register_template_string(&template_path, &template)
        .expect("Could not register template content");
    // render our JSON override in our template.
    let rendered_template = renderer.render(&template_path, &data).ok().unwrap();
    
    if print {
        ui.warn(format!("Rendered template: {}", &template_path))?;
        println!("{}", rendered_template);
    }

    // Render our template file
    create_with_template(ui, &format!("{}/{}", render_dir, file_name), &rendered_template)?;
    ui.br()?;
    // not really sure this is correct...
    Ok(())
}

fn merge(a: &mut Json, b: Json) {
    match (a, b) {
        (a @ &mut Json::Object(_), Json::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Json::Null), v);
            }
        }
        (a, b) => *a = b,
    }
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
