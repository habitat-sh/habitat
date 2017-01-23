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

use hcore::package::{PackageIdent, PackageInstall, Identifiable};
use manager::service::config::ServiceConfig;
use handlebars::{Handlebars, Helper, RenderContext, RenderError};
use serde_json;
use toml;

type RenderResult = Result<(), RenderError>;

pub fn pkg_path_for(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected a string parameter for \"pkgPathFor\"")));
    let param = try!(PackageIdent::from_str(param).map_err(|e| {
            RenderError::new(format!("Bad package identifier for \"pkgPathFor\", {}", e))
        }));
    let cfg = try!(serde_json::from_value::<ServiceConfig>(rc.context().data().clone())
        .map_err(|_| {
            RenderError::new("\"pkgPathFor\" can only be used on a template bound to a service \
                              config.")
        }));
    let pkg = cfg.pkg
        .deps
        .iter()
        .map(|pkg| PackageIdent::from_str(&pkg.ident).expect("Bad Pkg entry in ServiceConfig!"))
        .find(|ident| ident.satisfies(&param))
        .and_then(|i| PackageInstall::load(&i, None).ok())
        .and_then(|i| Some(i.installed_path().to_string_lossy().into_owned()))
        .unwrap_or("".to_string());
    try!(rc.writer.write(pkg.into_bytes().as_ref()));
    Ok(())
}

pub fn to_uppercase(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected a string parameter for \"toUppercase\"")));
    try!(rc.writer.write(param.to_uppercase().into_bytes().as_ref()));
    Ok(())
}

pub fn to_lowercase(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| RenderError::new("Expected a string parameter for \"toLowercase\"")));
    try!(rc.writer.write(param.to_lowercase().into_bytes().as_ref()));
    Ok(())
}

pub fn to_json(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
            .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toJson\"")))
        .value();
    try!(rc.writer.write(serde_json::to_string_pretty(param).unwrap().into_bytes().as_ref()));
    Ok(())
}

pub fn to_toml(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    let param = try!(h.param(0)
            .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toToml\"")))
        .value();
    try!(rc.writer.write(toml::encode_str(&param).into_bytes().as_ref()));
    Ok(())
}
