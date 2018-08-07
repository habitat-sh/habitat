// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use handlebars::{Handlebars, Helper, HelperDef, RenderContext, RenderError};
use toml;

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToTomlHelper;

impl HelperDef for ToTomlHelper {
    fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult<()> {
        let param = h
            .param(0)
            .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toToml\""))?
            .value();
        let bytes = toml::ser::to_vec(&param)
            .map_err(|e| RenderError::new(format!("Can't serialize parameter to TOML: {}", e)))?;
        rc.writer.write_all(bytes.as_ref())?;
        Ok(())
    }
}

pub static TO_TOML: ToTomlHelper = ToTomlHelper;
