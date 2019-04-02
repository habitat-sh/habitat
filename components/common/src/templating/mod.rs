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

pub mod config;
mod context;
pub mod helpers;
pub mod hooks;
pub mod package;
pub mod test_helpers;

use std::{fmt,
          ops::{Deref,
                DerefMut},
          result};

use regex::Regex;

use handlebars::{Handlebars,
                 RenderError,
                 TemplateFileError};
use serde::Serialize;
use serde_json;

use crate::{error::{Error,
                    Result},
            hcore::{fs,
                    package::PackageInstall},
            templating::hooks::{Hook,
                                InstallHook}};

pub use self::context::RenderContext;

/// A convenience method that compiles a package's install hook
/// and any configuration templates in its config_install folder
pub fn compile_for_package_install(package: &PackageInstall) -> Result<()> {
    let pkg = package::Pkg::from_install(package)?;

    fs::SvcDir::new(&pkg.name, &pkg.svc_user, &pkg.svc_group).create()?;

    let cfg = config::Cfg::new(&pkg, None)?;
    let ctx = RenderContext::new(&pkg, &cfg);
    let cfg_renderer = config::CfgRenderer::new(pkg.path.join("config_install"))?;
    cfg_renderer.compile(&pkg.name, &pkg, &pkg.svc_config_install_path, &ctx)?;

    if let Some(ref hook) = InstallHook::load(&pkg.name,
                                              &fs::svc_hooks_path(&pkg.name),
                                              &package.installed_path.join("hooks"))
    {
        hook.compile(&pkg.name, &ctx)?;
    };

    Ok(())
}

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
        where T: Serialize
    {
        let raw = serde_json::to_value(ctx).map_err(Error::RenderContextSerialization)?;
        debug!("Rendering template with context, {}, {}", template, raw);
        self.0
            .render(template, &raw)
            .map_err(|e| Error::TemplateRenderError(format!("{}", e)))
    }

    // This method is only implemented so we can intercept the call to Handlebars and display
    // a deprecation message to users. More information here https://github.com/habitat-sh/habitat/issues/6323.
    // When Handlebars is upgraded this can be safely removed.
    pub fn register_template_file<P>(&mut self,
                                     name: &str,
                                     path: P)
                                     -> result::Result<(), TemplateFileError>
        where P: AsRef<std::path::Path>
    {
        let path = path.as_ref();

        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(\{\{[^}]+[^.])(\[[^}]*\}\})").expect("Failed to compile template \
                                                                    deprecation regex");
        }

        let template_string =
            std::fs::read_to_string(path).map_err(|e| {
                                             TemplateFileError::IOError(e, name.to_owned())
                                         })?;

        fn transform(text: &str) -> String {
            let text = text.to_owned();
            if RE.is_match(&text) {
                transform(&RE.replace_all(&text, "$1.$2"))
            } else {
                text
            }
        }

        template_string.lines()
            .enumerate()
            .filter(|(_i, line)| RE.is_match(&line))
            .map(|(i, line)| (i, line, transform(&line)))
            .for_each(|(i, old_line, new_line)| {
                println!("\n\n***************************************************\n\
                          Deprecated object access syntax in handlebars template\n\
                          Use 'object.[index]' syntax instead of 'object[index]'\n\
                          See https://github.com/habitat-sh/habitat/issues/6323 for more information\n\n\
                          TEMPLATE: {}\n\
                          LINE: {}: '{}'\n\
                          Update to '{}'\n\n\
                          *******************************************************\n\n",
                         path.display(), i + 1, old_line, new_line)
            });

        // Replace the deprecated syntax with the good syntax. This should make it easier to upgrade
        // the handlebars crate without breaking folks.
        let updated_template_string = transform(&template_string);
        self.0
            .register_template_string(name, updated_template_string)?;
        Ok(())
    }
}

impl fmt::Debug for TemplateRenderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handlebars TemplateRenderer")
    }
}

impl Deref for TemplateRenderer {
    type Target = Handlebars;

    fn deref(&self) -> &Handlebars { &self.0 }
}

impl DerefMut for TemplateRenderer {
    fn deref_mut(&mut self) -> &mut Handlebars { &mut self.0 }
}

/// Disables HTML escaping which is enabled by default in Handlebars.
fn never_escape(data: &str) -> String { String::from(data) }

#[cfg(test)]
mod test {
    use super::*;
    use crate::{hcore::{fs::{pkg_root_path,
                             FS_ROOT_PATH},
                        package::PackageIdent},
                templating::test_helpers::*};
    use serde_json;
    use std::{collections::BTreeMap,
              env,
              fs::File,
              io::Read,
              path::PathBuf};
    use tempfile::TempDir;
    use toml;

    pub fn root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests") }

    pub fn fixtures() -> PathBuf { root().join("fixtures") }

    pub fn templates() -> PathBuf { fixtures().join("templates") }

    pub fn sample_configs() -> PathBuf { fixtures().join("sample_configs") }

    pub fn service_config_json_from_toml_file(filename: &str) -> serde_json::Value {
        let mut file = File::open(sample_configs().join(filename)).unwrap();
        let mut config = String::new();
        let _ = file.read_to_string(&mut config).unwrap();
        let toml = toml::de::from_str(&config).unwrap();
        toml_to_json(toml::Value::Table(toml))
    }

    fn toml_vec_to_json(toml: Vec<toml::Value>) -> serde_json::Value {
        serde_json::Value::Array(toml.into_iter().map(toml_to_json).collect())
    }

    // Translates a toml table to a mustache data structure.
    fn toml_table_to_json(toml: toml::value::Table) -> serde_json::Value {
        serde_json::Value::Object(toml.into_iter()
                                      .map(|(k, v)| (k, toml_to_json(v)))
                                      .collect())
    }

    pub fn toml_to_json(value: toml::Value) -> serde_json::Value {
        match value {
            toml::Value::String(s) => serde_json::Value::String(s.to_string()),
            toml::Value::Integer(i) => serde_json::Value::from(i as i64),
            toml::Value::Float(i) => serde_json::Value::from(i as f64),
            toml::Value::Boolean(b) => serde_json::Value::Bool(b),
            toml::Value::Datetime(s) => serde_json::Value::String(s.to_string()),
            toml::Value::Array(a) => toml_vec_to_json(a),
            toml::Value::Table(t) => toml_table_to_json(t),
        }
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
}"#.to_string()
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
"#.to_string()
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
        assert_eq!(PathBuf::from(rendered),
                   pkg_root_path(Some(&*FS_ROOT_PATH)).join("core/acl/2.2.52/20161208223311",));
    }

    #[test]
    fn each_alive_helper_content() {
        let mut renderer = TemplateRenderer::new();
        // template using the new `eachAlive` helper
        renderer.register_template_file("each_alive", templates().join("each_alive.txt"))
                .unwrap();

        // template using an each block with a nested if block filtering on `alive`
        renderer.register_template_file("all_members", templates().join("all_members.txt"))
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
        renderer.register_template_file("each_alive", templates().join("each_alive.txt"))
                .unwrap();

        // template using an each block with a nested if block filtering on `alive`
        renderer.register_template_file("all_members", templates().join("all_members.txt"))
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
        renderer.register_template_file("each_alive",
                                        templates().join("each_alive_with_identifier.txt"))
                .unwrap();

        // template using an each block with a nested if block filtering on `alive`
        renderer.register_template_file("all_members", templates().join("all_members.txt"))
                .unwrap();

        let data = service_config_json_from_toml_file("multiple_supervisors_config.toml");

        let each_alive_render = renderer.render("each_alive", &data).unwrap();
        let each_if_render = renderer.render("all_members", &data).unwrap();

        assert_eq!(each_alive_render, each_if_render);
    }

    #[test]
    fn render_package_install() {
        let root = TempDir::new().expect("create temp dir").into_path();
        env::set_var(fs::FS_ROOT_ENVVAR, &root);
        let pg_id = PackageIdent::new("testing", "test", Some("1.0.0"), Some("20170712000000"));

        let pkg_install =
            PackageInstall::new_from_parts(pg_id.clone(), root.clone(), root.clone(), root.clone());

        let toml_path = root.join("default.toml");
        create_with_content(toml_path, "message = \"Hello\"");
        let hooks_path = root.join("hooks");
        std::fs::create_dir_all(&hooks_path).unwrap();
        create_with_content(hooks_path.join("install"),
                            "install message is {{cfg.message}}");
        let config_path = root.join("config_install");
        std::fs::create_dir_all(&config_path).unwrap();
        create_with_content(config_path.join("config.txt"),
                            "config message is {{cfg.message}}");

        compile_for_package_install(&pkg_install).expect("compile package");

        assert_eq!(
            file_content(fs::svc_config_install_path(&pkg_install.ident().name).join("config.txt")),
            "config message is Hello"
        );
        assert_eq!(file_content(fs::svc_hooks_path(&pkg_install.ident().name).join("install")),
                   "install message is Hello");

        env::remove_var(fs::FS_ROOT_ENVVAR);
        std::fs::remove_dir_all(root).expect("removing temp root");
    }
}
