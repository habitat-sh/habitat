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

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct ToUppercaseHelper;

impl HelperDef for ToUppercaseHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let param = h
            .param(0)
            .and_then(|v| v.value().as_str())
            .ok_or_else(|| RenderError::new("Expected a string parameter for \"toUppercase\""))?;
        rc.writer
            .write(param.to_uppercase().into_bytes().as_ref())?;
        Ok(())
    }
}

pub static TO_UPPERCASE: ToUppercaseHelper = ToUppercaseHelper;
