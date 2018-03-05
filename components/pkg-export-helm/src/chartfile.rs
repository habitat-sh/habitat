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

use clap;
use failure::SyncFailure;
use handlebars::Handlebars;

use export_docker::Result;
use hcore::package::PackageIdent;

pub const DEFAULT_VERSION: &'static str = "0.0.1";

// Helm chart file template
const CHARTFILE: &'static str = include_str!("../defaults/HelmChartFile.hbs");

pub struct ChartFile {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub app_version: Option<String>,
}

impl ChartFile {
    pub fn new_from_cli_matches(matches: &clap::ArgMatches, pkg_ident: &PackageIdent) -> Self {
        let name = matches
            .value_of("CHART")
            .unwrap_or(&pkg_ident.name)
            .to_string();
        let pkg_version = pkg_ident.version.as_ref();
        let version = matches
            .value_of("VERSION")
            .or(pkg_version.map(|s| s.as_ref()))
            .unwrap_or(DEFAULT_VERSION)
            .to_owned();
        let app_version = pkg_version.map(|v| {
            pkg_ident
                .release
                .as_ref()
                .map(|r| format!("{}-{}", v, r))
                .unwrap_or(v.to_string())
        });
        let description = matches.value_of("DESCRIPTION").map(|s| s.to_owned());

        ChartFile {
            name,
            version,
            description,
            app_version,
        }
    }

    // TODO: Implement TryInto trait instead when it's in stable std crate
    pub fn into_string(&self) -> Result<String> {
        let json = json!({
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "appVersion": self.app_version,
        });

        Handlebars::new()
            .template_render(CHARTFILE, &json)
            .map_err(SyncFailure::new)
            .map_err(From::from)
    }
}
