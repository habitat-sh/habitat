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

use handlebars::Handlebars;
use serde_json::Value;

use manifest::Manifest;

// Kubernetes manifest template
const MANIFESTFILE: &'static str = include_str!("../defaults/KubernetesManifest.hbs");
const BINDFILE: &'static str = include_str!("../defaults/KubernetesBind.hbs");

/// Represents the [`Manifest`] in JSON format. This is an intermediate type that can be converted
/// to the final manifest YAML file content, ready for consumption by a Kubernetes cluster.
///
/// The reason for the existence of this intermediate type is to allow users of this crate to be
/// able to modify the JSON before converting it to the final manifest string.
///
/// [`Manifest`]: ../manifest/struct.Manifest.html
pub struct ManifestJson {
    /// JSON object, holding values for the main body of the YAML content.
    pub main: Value,
    /// JSON representations of [`Bind`] instances.
    ///
    /// [`Bind`]: ../bind/struct.Bind.html
    pub binds: Vec<Value>,
}

impl ManifestJson {
    /// Create a `ManifestJson` from `manifest`.
    pub fn new(manifest: &Manifest) -> Self {
        let main = json!({
            "metadata_name": manifest.metadata_name,
            "service_name": manifest.pkg_ident.name,
            "image": manifest.image,
            "count": manifest.count,
            "service_topology": manifest.service_topology.to_string(),
            "service_group": manifest.service_group,
            "config": manifest.config,
            "ring_secret_name": manifest.ring_secret_name,
            "bind": !manifest.binds.is_empty()
        });

        let mut binds = Vec::new();
        for bind in &manifest.binds {
            let json = json!({
                "name": bind.name.clone(),
                "service": bind.service_group.service().to_owned(),
                "group": bind.service_group.group().to_owned(),
            });

            binds.push(json);
        }

        ManifestJson {
            main: main,
            binds: binds,
        }
    }
}

impl Into<String> for ManifestJson {
    /// Convert into a string. The returned string is the final manifest YAML file content, ready
    /// for consumption by a Kubernetes cluster.
    fn into(self) -> String {
        // The Result::expect() usage in this function is justied by the fact that errors can only
        // come from the crate programmer (e.g they messed-up the manifest template or don't check
        // the user input).

        let mut s = Handlebars::new()
            .template_render(MANIFESTFILE, &self.main)
            .expect("Rendering of manifest from template failed");

        for bind in &self.binds {
            s += &Handlebars::new().template_render(BINDFILE, &bind).expect(
                "Rendering of manifest from template failed",
            );
        }

        s
    }
}
