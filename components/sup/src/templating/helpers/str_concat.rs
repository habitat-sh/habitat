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

use handlebars::{Handlebars, Helper, HelperDef, RenderContext};

use super::super::RenderResult;

#[derive(Clone, Copy)]
pub struct StrConcatHelper;

impl HelperDef for StrConcatHelper {
    fn call(&self, h: &Helper<'_>, _: &Handlebars, rc: &mut RenderContext<'_>) -> RenderResult<()> {
        let list: Vec<String> = h
            .params()
            .iter()
            .map(|v| v.value())
            .filter(|v| !v.is_object())
            .map(|v| v.to_string().replace("\"", ""))
            .collect();

        rc.writer.write(list.concat().into_bytes().as_ref())?;
        Ok(())
    }
}

pub static STR_CONCAT: StrConcatHelper = StrConcatHelper;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_concat_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("strConcat", Box::new(STR_CONCAT));
        let expected = "foobarbaz";
        assert_eq!(
            expected,
            handlebars
                .template_render("{{strConcat \"foo\" \"bar\" \"baz\"}}", &json!({}))
                .unwrap()
        );
    }
}
