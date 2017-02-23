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

pub mod helpers;

use std::ops::{Deref, DerefMut};
use handlebars::Handlebars;

pub struct Template(Handlebars);

impl Template {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pkgPathFor", Box::new(helpers::pkg_path_for));
        handlebars.register_helper("toUppercase", Box::new(helpers::to_uppercase));
        handlebars.register_helper("toLowercase", Box::new(helpers::to_lowercase));
        handlebars.register_helper("strReplace", Box::new(helpers::str_replace));
        handlebars.register_helper("toJson", Box::new(helpers::to_json));
        handlebars.register_helper("toToml", Box::new(helpers::to_toml));

        // JW TODO: remove these at a later date, these are an alias for toJson/toToml
        handlebars.register_helper("json", Box::new(helpers::to_json));
        handlebars.register_helper("toml", Box::new(helpers::to_toml));

        handlebars.register_escape_fn(never_escape);
        Template(handlebars)
    }
}

impl Deref for Template {
    type Target = Handlebars;

    fn deref(&self) -> &Handlebars {
        &self.0
    }
}

impl DerefMut for Template {
    fn deref_mut(&mut self) -> &mut Handlebars {
        &mut self.0
    }
}

/// Disables HTML escaping which is enabled by default in Handlebars.
fn never_escape(data: &str) -> String {
    String::from(data)
}

#[cfg(test)]
mod test {
    use toml;
    use serde_json;

    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use std::collections::BTreeMap;

    use super::*;

    use util::convert;
    use manager::ServiceConfig;

    #[test]
    fn test_handlebars_json_helper() {
        let content = "{{toJson x}}".to_string();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = template.render("t", &m);

        assert_eq!(r.ok().unwrap(),
                   r#"{
  "test": "something"
}"#
                       .to_string());
    }

    #[test]
    fn test_handlebars_toml_helper() {
        let content = "{{toToml x}}".to_string();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = template.render("t", &m);

        assert_eq!(r.ok().unwrap(),
                   r#"test = "something"
"#
                       .to_string());
    }

    #[test]
    fn to_uppercase_helper() {
        let content = "{{toUppercase var}}".to_string();
        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "value".into());
        let rendered = template.render("t", &m).unwrap();
        assert_eq!(rendered, "VALUE".to_string());
    }

    #[test]
    fn to_lowercase_helper() {
        let content = "{{toLowercase var}}".to_string();
        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "VALUE".into());
        let rendered = template.render("t", &m).unwrap();
        assert_eq!(rendered, "value".to_string());
    }

    #[test]
    fn str_replace_helper() {
        let content = "{{strReplace var old new}}".to_string();
        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "this is old".into());
        m.insert("old".into(), "old".into());
        m.insert("new".into(), "new".into());
        let rendered = template.render("t", &m).unwrap();
        assert_eq!(rendered, "this is new".to_string());
    }

    pub fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
    }

    pub fn fixtures() -> PathBuf {
        root().join("fixtures")
    }

    pub fn service_config_json_from_toml_file(filename: &str) -> serde_json::Value {
        let mut file = File::open(fixtures().join(filename)).unwrap();
        let mut config = String::new();
        let _ = file.read_to_string(&mut config).unwrap();
        let toml = toml::de::from_str(&config).unwrap();
        let data = convert::toml_to_json(toml::Value::Table(toml));
        data
    }

    #[test]
    fn pkg_path_for_helper() {
        let content = "{{pkgPathFor \"core/jq-static\"}}".to_string();
        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let data = service_config_json_from_toml_file("simple_config.toml");
        let rendered = template.render("t", &data).unwrap();
        assert_eq!(PathBuf::from(rendered),
                   PathBuf::from("/hab/pkgs/core/jq-static/1.10/20160909011845"));
    }

    #[test]
    fn deserialize_simple_config_toml() {
        let data = service_config_json_from_toml_file("simple_config.toml");
        let cfg = serde_json::from_value::<ServiceConfig>(data).unwrap();
        assert_eq!(cfg.pkg.name, "testplan");
    }

    #[test]
    fn deserialize_complex_config_toml() {
        let data = service_config_json_from_toml_file("complex_config.toml");
        let cfg = serde_json::from_value::<ServiceConfig>(data).unwrap();
        assert_eq!(cfg.pkg.name, "lsyncd");
    }

}
