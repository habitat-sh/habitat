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

use handlebars::{Context, Handlebars, Helper, RenderContext, RenderError};
use rustc_serialize::Encodable;
use toml;

pub fn json_helper(_: &Context,
                   h: &Helper,
                   _: &Handlebars,
                   rc: &mut RenderContext)
                   -> Result<(), RenderError> {
    let value_to_render = try!(h.param(0)
            .ok_or_else(|| RenderError::new("Expected 1 parameter for \"json\" helper")))
        .value();
    try!(rc.writer.write(value_to_render.pretty().to_string().into_bytes().as_ref()));
    Ok(())
}

pub fn toml_helper(_: &Context,
                   h: &Helper,
                   _: &Handlebars,
                   rc: &mut RenderContext)
                   -> Result<(), RenderError> {
    let value_to_render = try!(h.param(0)
            .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toml\" helper")))
        .value();
    let mut toml_encoder = toml::Encoder::new();
    value_to_render.encode(&mut toml_encoder).unwrap();
    let table: toml::Table = toml_encoder.toml;
    try!(rc.writer.write(toml::encode_str(&table).into_bytes().as_ref()));
    Ok(())
}

pub fn to_uppercase(_: &Context,
                    h: &Helper,
                    _: &Handlebars,
                    rc: &mut RenderContext)
                    -> Result<(), RenderError> {
    let param = try!(h.param(0)
        .and_then(|v| v.value().as_string())
        .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toUppercase\" helper")));
    try!(rc.writer.write(param.to_uppercase().into_bytes().as_ref()));
    Ok(())
}

pub fn to_lowercase(_: &Context,
                    h: &Helper,
                    _: &Handlebars,
                    rc: &mut RenderContext)
                    -> Result<(), RenderError> {
    let param = try!(h.param(0)
        .and_then(|v| v.value().as_string())
        .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toLowercase\" helper")));
    try!(rc.writer.write(param.to_lowercase().into_bytes().as_ref()));
    Ok(())
}

#[cfg(test)]
mod test {
    use handlebars::{Handlebars, Template};
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn test_handlebars_json_helper() {
        let t = Template::compile("{{json x}}".to_string()).ok().unwrap();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("json", Box::new(json_helper));
        handlebars.register_template("t", t);

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = handlebars.render("t", &m);

        assert_eq!(r.ok().unwrap(),
                   r#"{
  "test": "something"
}"#
                       .to_string());
    }

    #[test]
    fn test_handlebars_toml_helper() {
        let t = Template::compile("{{toml x}}".to_string()).ok().unwrap();
        let mut data = BTreeMap::new();
        data.insert("test".into(), "something".into());

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("toml", Box::new(toml_helper));
        handlebars.register_template("t", t);

        let mut m: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        m.insert("x".into(), data);

        let r = handlebars.render("t", &m);

        assert_eq!(r.ok().unwrap(),
                   r#"test = "something"
"#
                       .to_string());
    }

    #[test]
    fn to_uppercase_helper() {
        let content = "{{toUppercase var}}".to_string();
        let template = Template::compile(content).unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("toUppercase", Box::new(to_uppercase));
        handlebars.register_template("t", template);

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "value".into());
        let rendered = handlebars.render("t", &m).unwrap();
        assert_eq!(rendered, "VALUE".to_string());
    }

    #[test]
    fn to_lowercase_helper() {
        let content = "{{toLowercase var}}".to_string();
        let template = Template::compile(content).unwrap();
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("toLowercase", Box::new(to_lowercase));
        handlebars.register_template("t", template);

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("var".into(), "VALUE".into());
        let rendered = handlebars.render("t", &m).unwrap();
        assert_eq!(rendered, "value".to_string());
    }
}
