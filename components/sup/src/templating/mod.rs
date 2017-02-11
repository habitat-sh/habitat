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
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::collections::BTreeMap;
    use super::*;

    use config::{gcache, Config};
    use hcore::package::{PackageIdent, PackageInstall};
    use error::Result;
    use manager::service::config::ServiceConfig;
    use manager::service::ServiceSpec;
    use supervisor::RuntimeConfig;
    use util::convert;

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

    fn gen_ident() -> PackageIdent {
        PackageIdent::from_str("neurosis/redis/2000/20160222201258").unwrap()
    }

    fn gen_pkg_install() -> PackageInstall {
        PackageInstall::new_from_parts(
            gen_ident(),
            PathBuf::from("/"),
            PathBuf::from("/fakeo"),
            PathBuf::from("/fakeo/here"))
    }

    fn setup_service_config() -> Result<ServiceConfig> {
        gcache(Config::default());
        let spec = ServiceSpec::default_for(gen_ident());
        let pkg = gen_pkg_install();
        ServiceConfig::new(&pkg, &RuntimeConfig::default(), spec.config_from, &spec.binds)
    }

    // This test replicates the logic in manager::service::config::ServiceConfig::write
    // to convert the ServiceConfig to json.
    // The pkgPathFor helper takes the render context (which is the config converted from toml to json)
    // and attempts to deserialize it to find the package path for one of the services dependencies.
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn pkg_path_for_helper() {
        let content = "{{pkgPathFor \"neurosis/jq-static\"}}".to_string();
        let mut template = Template::new();
        template.register_template_string("t", content).unwrap();

        let sc = setup_service_config().unwrap();
        let toml = sc.to_toml().unwrap();
        let data = convert::toml_to_json(toml);

        let rendered = template.render("t", &data).unwrap();
        assert_eq!(rendered,
                   "/fakeo/here/neurosis/jq-static/1000/20160222201259".to_string());
    }
}
