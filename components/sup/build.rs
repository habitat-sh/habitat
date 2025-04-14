// Inline common build behavior
include!("../libbuild.rs");

use handlebars::Handlebars;
use serde_json;
use serde_yaml;
use std::{env,
          fs::{self,
               File},
          io::Write,
          path::Path};

fn main() {
    habitat::common();
    generate_apidocs();
    generate_event_protobufs();
}

fn generate_apidocs() {
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("api.html");

    match env::var("CARGO_FEATURE_APIDOCS") {
        Ok(_) => {
            let src_yaml = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("doc/api.yaml");
            let template =
                Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("doc/template.hbs");

            let html = render_with_handlebars(&src_yaml, &template).expect("Failed to render API \
                                                                            docs from YAML");

            fs::write(&dst, html).expect("Failed to write api.html");
        }
        Err(_) => {
            let mut file = File::create(dst).unwrap();
            file.write_all(b"No API docs provided at build").unwrap();
        }
    };
}

fn render_with_handlebars(yaml_path: &Path,
                          template_path: &Path)
                          -> Result<String, Box<dyn std::error::Error>> {
    let yaml = fs::read_to_string(yaml_path)?;
    let value: serde_yaml::Value = serde_yaml::from_str(&yaml)?;
    let json = serde_json::to_string_pretty(&value)?;

    let template = fs::read_to_string(template_path)?;

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("api", template)?;

    let mut data = std::collections::BTreeMap::new();
    data.insert("spec", json);

    let html = handlebars.render("api", &data)?;
    Ok(html)
}

fn generate_event_protobufs() {
    let mut config = prost_build::Config::new();
    config.compile_protos(&["protocols/event.proto"], &["protocols/"])
          .expect("Couldn't compile protobufs!")
}
