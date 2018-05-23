// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use handlebars::{self, Handlebars};
use serde::Serialize;

// Kubernetes manifest template
const MANIFESTFILE: &'static str = include_str!("../defaults/KubernetesManifest.hbs");

pub struct Renderer {
    hb: Handlebars,
}

impl Renderer {
    fn new() -> Self {
        let mut hb = Handlebars::new();

        hb.register_escape_fn(handlebars::no_escape);

        Self { hb }
    }

    fn render<T>(&self, data: &T) -> String
    where
        T: Serialize,
    {
        // The Result::expect() usage in this function is justified by
        // the fact that errors can only come from the crate
        // programmer (e.g they messed-up the manifest template or
        // didn't check the user input).
        self.hb
            .template_render(MANIFESTFILE, &data)
            .expect("Rendering of manifest from template failed")
    }
}

pub fn render<T>(data: &T) -> String
where
    T: Serialize,
{
    Renderer::new().render(data)
}
