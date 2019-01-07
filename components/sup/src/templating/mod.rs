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

mod context;
pub mod helpers;

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::result;

use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use serde_json;

pub use self::context::RenderContext;
use crate::error::{Error, Result};

static LOGKEY: &'static str = "TP";

pub type RenderResult<T> = result::Result<T, RenderError>;

pub struct TemplateRenderer(Handlebars);

impl TemplateRenderer {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("eachAlive", Box::new(helpers::EACH_ALIVE));
        handlebars.register_helper("pkgPathFor", Box::new(helpers::PKG_PATH_FOR));
        handlebars.register_helper("strConcat", Box::new(helpers::STR_CONCAT));
        handlebars.register_helper("strJoin", Box::new(helpers::STR_JOIN));
        handlebars.register_helper("strReplace", Box::new(helpers::STR_REPLACE));
        handlebars.register_helper("toUppercase", Box::new(helpers::TO_UPPERCASE));
        handlebars.register_helper("toLowercase", Box::new(helpers::TO_LOWERCASE));
        handlebars.register_helper("toJson", Box::new(helpers::TO_JSON));
        handlebars.register_helper("toToml", Box::new(helpers::TO_TOML));
        handlebars.register_helper("toYaml", Box::new(helpers::TO_YAML));

        handlebars.register_escape_fn(never_escape);
        TemplateRenderer(handlebars)
    }

    pub fn render<T>(&self, template: &str, ctx: &T) -> Result<String>
    where
        T: Serialize,
    {
        let raw = serde_json::to_value(ctx)
            .map_err(|e| sup_error!(Error::RenderContextSerialization(e)))?;
        debug!("Rendering template with context, {}, {}", template, raw);
        self.0
            .render(template, &raw)
            .map_err(|e| sup_error!(Error::TemplateRenderError(e)))
    }
}

impl fmt::Debug for TemplateRenderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handlebars TemplateRenderer")
    }
}

impl Deref for TemplateRenderer {
    type Target = Handlebars;

    fn deref(&self) -> &Handlebars {
        &self.0
    }
}

impl DerefMut for TemplateRenderer {
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
    use serde_json;
    use std::collections::BTreeMap;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use toml;

    use super::*;
    use crate::hcore::fs::FS_ROOT_PATH;
    use crate::util::convert;

    pub fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
    }

    pub fn fixtures() -> PathBuf {
        root().join("fixtures")
    }

    pub fn templates() -> PathBuf {
        fixtures().join("templates")
    }

    pub fn sample_configs() -> PathBuf {
        fixtures().join("sample_configs")
    }

    pub fn service_config_json_from_toml_file(filename: &str) -> serde_json::Value {
        let mut file = File::open(sample_configs().join(filename)).unwrap();
        let mut config = String::new();
        let _ = file.read_to_string(&mut config).unwrap();
        let toml = toml::de::from_str(&config).unwrap();
        let data = convert::toml_to_json(toml::Value::Table(toml));
        data
    }

    #[test]
    fn test_handlebars_json_helper() {
        let content = "{{toJson x}}".to_string();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = renderer.render("t", &m);

        assert_eq!(
            r.ok().unwrap(),
            r#"{
  "test": "something"
}"#
            .to_string()
        );
    }

    #[test]
    fn test_handlebars_toml_helper() {
        let content = "{{toToml x}}".to_string();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = renderer.render("t", &m);

        assert_eq!(
            r.ok().unwrap(),
            r#"test = "something"
"#
            .to_string()
        );
    }

    #[test]
    fn test_handlebars_yaml_helper() {
        let content = "{{toYaml x}}".to_string();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = renderer.render("t", &m);

        assert_eq!(
            r.ok().unwrap(),
            r#"---
test: something"#
                .to_string()
        );
    }

    #[test]
    fn to_uppercase_helper() {
        let content = "{{toUppercase var}}".to_string();
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "value".into());
        let rendered = renderer.render("t", &m).unwrap();
        assert_eq!(rendered, "VALUE".to_string());
    }

    #[test]
    fn to_lowercase_helper() {
        let content = "{{toLowercase var}}".to_string();
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "VALUE".into());
        let rendered = renderer.render("t", &m).unwrap();
        assert_eq!(rendered, "value".to_string());
    }

    #[test]
    fn str_replace_helper() {
        let content = "{{strReplace var old new}}".to_string();
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "this is old".into());
        m.insert("old".into(), "old".into());
        m.insert("new".into(), "new".into());
        let rendered = renderer.render("t", &m).unwrap();
        assert_eq!(rendered, "this is new".to_string());
    }

    #[test]
    fn bind_variable() {
        let content = "{{bind.foo.members[0].sys.ip}}";
        let mut renderer = TemplateRenderer::new();
        let data = service_config_json_from_toml_file("complex_config.toml");

        renderer.register_template_string("t", content).unwrap();

        let rendered = renderer.render("t", &data).unwrap();
        assert_eq!(rendered, "172.17.0.5");
    }

    #[test]
    fn pkg_path_for_helper() {
        let content = "{{pkgPathFor \"core/acl\"}}".to_string();
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("t", content).unwrap();

        let data = service_config_json_from_toml_file("complex_config.toml");
        let rendered = renderer.render("t", &data).unwrap();
        assert_eq!(
            PathBuf::from(rendered),
            PathBuf::from((&*FS_ROOT_PATH).join("/hab/pkgs/core/acl/2.2.52/20161208223311",))
        );
    }

    #[test]
    fn each_alive_helper_content() {
        let mut renderer = TemplateRenderer::new();
        // template using the new `eachAlive` helper
        renderer
            .register_template_file("each_alive", templates().join("each_alive.txt"))
            .unwrap();

        // template using an each block with a nested if block filtering on `alive`
        renderer
            .register_template_file("all_members", templates().join("all_members.txt"))
            .unwrap();

        let data = service_config_json_from_toml_file("multiple_supervisors_config.toml");

        let each_alive_render = renderer.render("each_alive", &data).unwrap();
        let each_if_render = renderer.render("all_members", &data).unwrap();

        assert_eq!(each_alive_render, each_if_render);
    }

    #[test]
    fn each_alive_helper_first_node() {
        let mut renderer = TemplateRenderer::new();
        // template using the new `eachAlive` helper
        renderer
            .register_template_file("each_alive", templates().join("each_alive.txt"))
            .unwrap();

        // template using an each block with a nested if block filtering on `alive`
        renderer
            .register_template_file("all_members", templates().join("all_members.txt"))
            .unwrap();

        let data = service_config_json_from_toml_file("one_supervisor_not_started.toml");

        let each_alive_render = renderer.render("each_alive", &data).unwrap();
        let each_if_render = renderer.render("all_members", &data).unwrap();

        assert_eq!(each_alive_render, each_if_render);
    }

    #[test]
    fn each_alive_helper_with_identifier_alias() {
        let mut renderer = TemplateRenderer::new();
        // template using the new `eachAlive` helper
        renderer
            .register_template_file(
                "each_alive",
                templates().join("each_alive_with_identifier.txt"),
            )
            .unwrap();

        // template using an each block with a nested if block filtering on `alive`
        renderer
            .register_template_file("all_members", templates().join("all_members.txt"))
            .unwrap();

        let data = service_config_json_from_toml_file("multiple_supervisors_config.toml");

        let each_alive_render = renderer.render("each_alive", &data).unwrap();
        let each_if_render = renderer.render("all_members", &data).unwrap();

        assert_eq!(each_alive_render, each_if_render);
    }
}
