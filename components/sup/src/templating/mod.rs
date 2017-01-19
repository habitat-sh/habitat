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
        handlebars.register_helper("json", Box::new(helpers::json_helper));
        handlebars.register_helper("toml", Box::new(helpers::toml_helper));
        handlebars.register_helper("toUppercase", Box::new(helpers::to_uppercase));
        handlebars.register_helper("toLowercase", Box::new(helpers::to_lowercase));
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
