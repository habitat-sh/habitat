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

use serde_json::Value;

use crate::{hb,
            hcore::service::ServiceBind,
            manifest::Manifest};

/// Represents the [`Manifest`] in JSON format. This is an intermediate type that can be converted
/// to the final manifest YAML file content, ready for consumption by a Kubernetes cluster.
///
/// The reason for the existence of this intermediate type is to allow users of this crate to be
/// able to modify the JSON before converting it to the final manifest string.
///
/// [`Manifest`]: ../manifest/struct.Manifest.html
pub struct ManifestJson {
    /// JSON object, holding values for the YAML content.
    pub value: Value,
}

impl ManifestJson {
    /// Create a `ManifestJson` from `manifest`.
    pub fn new(manifest: &Manifest) -> Self {
        let binds = manifest.binds.iter().map(to_json).collect::<Vec<_>>();
        let environment = manifest.environment
                                  .iter()
                                  .map(|e| e.to_json())
                                  .collect::<Vec<_>>();
        let persistent_storage = manifest.persistent_storage.as_ref().map(|s| s.to_json());

        ManifestJson { value: json!({
                           "metadata_name": manifest.metadata_name,
                           "service_name": manifest.pkg_ident.name,
                           "image": manifest.image,
                           "count": manifest.count,
                           "service_topology": manifest.service_topology.to_string(),
                           "service_group": manifest.service_group,
                           "config": manifest.config,
                           "ring_secret_name": manifest.ring_secret_name,
                           "binds": binds,
                           "environment": environment,
                           "persistent_storage": persistent_storage,
                       }), }
    }
}

fn to_json(bind: &ServiceBind) -> serde_json::Value {
    json!({
        "name": bind.name(),
        "service": bind.service_group().service(),
        "group": bind.service_group().group(),
    })
}

impl Into<String> for ManifestJson {
    /// Convert into a string. The returned string is the final manifest YAML file content, ready
    /// for consumption by a Kubernetes cluster.
    fn into(self) -> String { hb::render(&self.value) }
}
