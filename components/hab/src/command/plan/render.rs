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
use serde_json::{self, json, Value as Json};
use toml::Value;

use crate::common::templating::TemplateRenderer;
use crate::common::ui::{Status, UIWriter, UI};
use crate::error::Result;

pub fn start(
    ui: &mut UI,
    template_path: &Path,
    default_toml_path: &Path,
    user_toml_path: Option<&Path>,
    mock_data_path: Option<&Path>,
    print: bool,
    no_render_dir: bool,
    render_dir: &Path,
    quiet: bool,
) -> Result<()> {
    // Strip the file name out of our passed template
    let file_name = match Path::new(&template_path).file_name() {
        Some(name) => name.to_str().clone().unwrap(),
        None => panic!(format!("Something went wrong getting filename of {:?}", &template_path)),
    }; 

    if !(quiet) {
      ui.begin(format!("Rendering: {:?} into: {:?} as: {:?}", template_path, render_dir, file_name))?;
      ui.br()?;
    }

    // read our template from file
    let template = read_to_string(&template_path)?;

    // create a "data" json struct
    let mut data = json!({});

    if !(quiet) {
        // import default.toml values, convert to JSON
        ui.begin(format!("Importing default.toml: {:?}", &default_toml_path))?;
    }

    // we should always have a default.toml, would be nice to "autodiscover" based on package name,
    // for now assume we're working in the plan dir if --default-toml not passed
    let default_toml = read_to_string(&default_toml_path)?;

    // merge default into data struct
    merge(&mut data, toml_to_json(&default_toml));

    // import default.toml values, convert to JSON
    let user_toml = match user_toml_path {
        Some(path) => {
            if !(quiet) {
              // print helper message, maybe only print if '--verbose'? how?
              ui.begin(format!("Importing user.toml: {:?}", path))?;
            }
            read_to_string(path)?
        },
        None => "".to_string(),
    };
    // merge default into data struct
    merge(&mut data, toml_to_json(&user_toml));

    // read mock data if provided
    let mock_data = match mock_data_path {
        Some(path) => {
            if !(quiet) {
                // print helper message, maybe only print if '--verbose'? how?
                ui.begin(format!("Importing override file: {:?}", path))?;
            }
            read_to_string(path)?
        },
        // return an empty json block if '--mock-data' isn't defined.
        // this allows us to merge an empty JSON block
        None => "{}".to_string(),
    };
    // merge mock data into data
    merge(&mut data, serde_json::from_str(&mock_data).unwrap());

    // create a template renderer
    let mut renderer = TemplateRenderer::new();
    // register our template 
    renderer
        .register_template_string(&template, &template)
        .expect("Could not register template content");
    // render our JSON override in our template.
    let rendered_template = renderer.render(&template, &data).ok().unwrap();
    
    if print {
        if !(quiet) {
          ui.br()?;
          ui.warn(format!("###======== Rendered template: {:?}", &template_path))?;
        }

        println!("{}", rendered_template);

        if !(quiet) {
          ui.warn(format!("========### End rendered template: {:?}", &template_path))?;
        }
    }

    // if not no render dir (aka "unless no_render_dir == true")
    if !(no_render_dir) {
      // Render our template file
      create_with_template(ui, &Path::new(render_dir).join(file_name), &rendered_template, quiet)?;
    }

    if !(quiet) {
      ui.br()?;
    }
    // not really sure this is correct...
    Ok(())
}

fn toml_to_json(cfg: &str) -> Json {
    // parse TOML string to Value
    let toml_value = cfg.parse::<Value>().expect("Error parsing TOML");
    // convert toml to json string
    let toml_string = serde_json::to_string(&toml_value).expect("Error encoding JSON");
    // convert to Json::Value
    serde_json::from_str(&format!(r#"{{ "cfg": {} }}"#, &toml_string)).unwrap()
}

// merge two Json structs
fn merge(a: &mut Json, b: Json) {
    match (a, b) {
        // not sure I understand this... 
        (a @ &mut Json::Object(_), Json::Object(b)) => {
            // not sure I understand why we unwrap this
            let a = a.as_object_mut().unwrap();
            // Iterate through key/values in Json object b, 
            // merge with Json object b
            for (k, v) in b {
                merge(a.entry(k).or_insert(Json::Null), v);
            }
        }
        // or this...
        (a, b) => *a = b,
    }
}

// This is almost a dupe of the method in plan/init, except we don't care if the file exists and go
// ahead and overwite it.  I feel like maybe a different name would be good?
fn create_with_template(ui: &mut UI, location: &std::path::PathBuf, template: &str, quiet: bool) -> Result<()> {
    let path = Path::new(&location);
    if !(quiet) {
        ui.status(Status::Creating, format!("file: {:?}", location))?;
    }
    // If the directory doesn't exist we need to make it.
    if let Some(directory) = path.parent() {
        create_dir_all(directory)?;
    }
    // Write file to disk
    File::create(path).and_then(|mut file| file.write(template.as_bytes()))?;
    Ok(())
}
