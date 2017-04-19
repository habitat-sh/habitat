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

use std::str::FromStr;
use std::string::ToString;
use std::collections::HashMap;

use hcore::package::{PackageIdent, Identifiable};
use hcore::fs;
use manager::service::Pkg;
use handlebars::{Handlebars, Helper, Renderable, RenderContext, RenderError, Context};
use serde_json;
use serde_json::map::Map;
use toml;


type RenderResult = Result<(), RenderError>;

pub fn each_alive(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let value =
        try!(h.param(0)
                 .ok_or_else(|| RenderError::new("Param not found for helper \"eachAlive\"")))
                .value();

    let template =
        try!(h.template()
        .ok_or_else(|| RenderError::new("No content to render inside the helper \"eachAlive\"")));
    let default_array = Vec::default();
    let value_array = value.as_array().unwrap_or(&default_array);
    for i in 0..value_array.len() {
        let member = try!(value_array[i]
            .as_object()
            .ok_or_else(|| {
                RenderError::new(format!("Param value is not a valid census member.  Parameter \
                                          content is: {:?}",
                                         value_array[i]))
            }));
        if member.contains_key("alive") && member["alive"].as_bool().unwrap_or(false) {
            debug!("Alive! {:?}", value_array[i]);
            let mut map = HashMap::default();

            // Check for those tricky people who use as |blah| inside their helpers.
            // I'm looking at you Smith!
            let local_context_data = match h.block_param() {
                Some(name) => {
                    let mut json_map = Map::new();
                    json_map.insert(name.to_string(), value_array[i].clone());
                    serde_json::Value::Object(json_map)
                }
                None => value_array[i].clone(),
            };
            let mut local_context = Context::wraps(&local_context_data);
            let mut writer = rc.writer();
            let mut local_rc = RenderContext::new(&mut local_context, &mut map, &mut writer);
            try!(template.render(r, &mut local_rc));
        } else {
            debug!("Dead! {:?}", value_array[i]);
        }
    }
    Ok(())
}

pub fn pkg_path_for(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param =
        try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected a string parameter for \"pkgPathFor\"")));
    let param = try!(PackageIdent::from_str(param).map_err(|e| {
            RenderError::new(format!("Bad package identifier for \"pkgPathFor\", {}", e))
        }));

    let pkg_data = {
        rc.context().data()["pkg"].clone()
    };
    let pkg = try!(serde_json::from_value::<Pkg>(pkg_data)
        .map_err(|_| {
            RenderError::new("\"pkgPathFor\" requires valid package metadata. \
                              Please check the config.toml in the service directory.")
        }));
    let target_pkg = pkg.deps
        .iter()
        .find(|ident| ident.satisfies(&param))
        .and_then(|i| {
                      Some(fs::pkg_install_path(&i, None)
                               .to_string_lossy()
                               .into_owned())
                  })
        .unwrap_or("".to_string());
    try!(rc.writer.write(target_pkg.into_bytes().as_ref()));
    Ok(())
}

pub fn to_uppercase(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param =
        try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected a string parameter for \"toUppercase\"")));
    try!(rc.writer
             .write(param.to_uppercase().into_bytes().as_ref()));
    Ok(())
}

pub fn to_lowercase(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param =
        try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected a string parameter for \"toLowercase\"")));
    try!(rc.writer
             .write(param.to_lowercase().into_bytes().as_ref()));
    Ok(())
}

pub fn str_replace(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param =
        try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected 3 string parameters for \"strReplace\"")));
    let old =
        try!(h.param(1)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected 3 string parameters for \"strReplace\"")));
    let new =
        try!(h.param(2)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected 3 string parameters for \"strReplace\"")));
    try!(rc.writer
             .write(param.replace(old, new).into_bytes().as_ref()));
    Ok(())
}

pub fn to_json(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
                         .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toJson\"")))
            .value();
    let json =
        try!(serde_json::to_string_pretty(param)
        .map_err(|e| RenderError::new(format!("Can't serialize parameter to JSON: {}", e))));
    try!(rc.writer.write(json.into_bytes().as_ref()));
    Ok(())
}

pub fn to_toml(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
                         .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toToml\"")))
            .value();
    let bytes =
        try!(toml::ser::to_vec(&param)
        .map_err(|e| RenderError::new(format!("Can't serialize parameter to TOML: {}", e))));
    try!(rc.writer.write_all(bytes.as_ref()));
    Ok(())
}
