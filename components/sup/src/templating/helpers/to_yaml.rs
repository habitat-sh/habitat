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
use serde_yaml;

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToYamlHelper;

impl HelperDef for ToYamlHelper {
    fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult<()> {
        let param = h
            .param(0)
            .ok_or_else(|| RenderError::new("Expected 1 parameter for \"toYaml\""))?
            .value();
        let yaml = serde_yaml::to_string(param)
            .map_err(|e| RenderError::new(format!("Can't serialize parameter to YAML: {}", e)))?;
        rc.writer.write(yaml.into_bytes().as_ref())?;
        Ok(())
    }
}

pub static TO_YAML: ToYamlHelper = ToYamlHelper;
