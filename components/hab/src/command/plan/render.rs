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

use serde_json::{self,
                 json,
                 Value as Json};
use std::{fs::{create_dir_all,
               read_to_string,
               File},
          io::Write,
          path::Path};
use toml::Value;

use crate::{common::{templating::TemplateRenderer,
                     ui::{Status,
                          UIWriter,
                          UI}},
            error::Result};

pub fn start(ui: &mut UI,
             template_path: &Path,
             default_toml_path: &Path,
             user_toml_path: Option<&Path>,
             mock_data_path: Option<&Path>,
             print: bool,
             render: bool,
             render_dir: &Path,
             quiet: bool)
             -> Result<()> {
    // Strip the file name out of our passed template
    let file_name = Path::new(template_path.file_name().expect("valid template file"));

    if !quiet {
        ui.begin(format!("Rendering: {} into: {} as: {}",
                         template_path.display(), render_dir.display(), file_name.display()))?;
        ui.br()?;
    }

    // read our template from file
    let template = read_to_string(&template_path)?;

    // create a "data" json struct
    let mut data = json!({});

    if !quiet {
        // import default.toml values, convert to JSON
        ui.begin(format!("Importing default.toml: {}", &default_toml_path.display()))?;
    }

    // we should always have a default.toml, would be nice to "autodiscover" based on package name,
    // for now assume we're working in the plan dir if --default-toml not passed
    let default_toml = read_to_string(&default_toml_path)?;

    // merge default into data struct
    merge(&mut data, toml_to_json(&default_toml)?);

    // import default.toml values, convert to JSON
    let user_toml = match user_toml_path {
        Some(path) => {
            if !quiet {
                // print helper message, maybe only print if '--verbose'? how?
                ui.begin(format!("Importing user.toml: {}", path.display()))?;
            }
            read_to_string(path)?
        }
        None => String::new(),
    };
    // merge default into data struct
    merge(&mut data, toml_to_json(&user_toml)?);

    // read mock data if provided
    let mock_data = match mock_data_path {
        Some(path) => {
            if !quiet {
                // print helper message, maybe only print if '--verbose'? how?
                ui.begin(format!("Importing override file: {}", path.display()))?;
            }
            read_to_string(path)?
        }
        // return an empty json block if '--mock-data' isn't defined.
        // this allows us to merge an empty JSON block
        None => "{}".to_string(),
    };
    // merge mock data into data
    merge(&mut data, serde_json::from_str(&mock_data)?);

    // create a template renderer
    let mut renderer = TemplateRenderer::new();
    // register our template
    renderer.register_template_string(&template, &template)
            .expect("Could not register template content");
    // render our JSON override in our template.
    let rendered_template = renderer.render(&template, &data)?;

    if print {
        if !quiet {
            ui.br()?;
            ui.warn(format!("###======== Rendered template: {}", &template_path.display()))?;
        }

        println!("{}", rendered_template);

        if !quiet {
            ui.warn(format!("========### End rendered template: {}", &template_path.display()))?;
        }
    }

    if render {
        // Render our template file
        create_with_template(ui, &render_dir, &file_name, &rendered_template, quiet)?;
    }

    if !quiet {
        ui.br()?;
    }
    Ok(())
}

fn toml_to_json(cfg: &str) -> Result<Json> {
    let toml_value = cfg.parse::<Value>()?;
    let toml_string = serde_json::to_string(&toml_value)?;
    let json = serde_json::from_str(&format!(r#"{{ "cfg": {} }}"#, &toml_string))?;
    Ok(json)
}

// merge two Json structs
fn merge(a: &mut Json, b: Json) {
    if let Json::Object(a_map) = a {
        if let Json::Object(b_map) = b {
            for (k, v) in b_map {
                merge(a_map.entry(k).or_insert(Json::Null), v);
            }
            return;
        }
    }
    *a = b;
}

fn create_with_template(ui: &mut UI,
                        render_dir: &std::path::Path,
                        file_name: &std::path::Path,
                        template: &str,
                        quiet: bool)
                        -> Result<()> {
    let path = Path::new(&render_dir).join(&file_name);
    if !quiet {
        ui.status(Status::Creating, format!("file: {}", path.display()))?;
    }

    create_dir_all(render_dir)?;

    // Write file to disk
    File::create(path).and_then(|mut file| file.write(template.as_bytes()))?;
    Ok(())
}
