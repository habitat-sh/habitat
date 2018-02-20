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
use std::fs;
use std::io::Write;

use common::ui::{UI, Status};
use export_docker;
use export_docker::Result;
use export_k8s::{Manifest, ManifestJson};

use chartfile::ChartFile;
use values::Values;
use deps::Deps;

pub struct Chart<'a> {
    name: String,
    chartfile: ChartFile,
    manifest_template: Option<ManifestJson>,
    values: Values,
    deps: Deps,
    ui: &'a mut UI,
}

impl<'a> Chart<'a> {
    pub fn new_for_cli_matches(ui: &'a mut UI, matches: &clap::ArgMatches) -> Result<Self> {
        let image = if !matches.is_present("NO_DOCKER_IMAGE") {
            export_docker::export_for_cli_matches(ui, &matches)?
        } else {
            None
        };
        let manifest = Manifest::new_from_cli_matches(ui, &matches, image)?;

        let name = matches
            .value_of("CHART")
            .unwrap_or(&manifest.service_name)
            .to_string();
        let version = matches.value_of("VERSION");
        let description = matches.value_of("DESCRIPTION");
        let chartfile = ChartFile::new(&name, version, description);
        let deps = Deps::new_for_cli_matches(&matches);

        Ok(Self::new_for_manifest(manifest, name, chartfile, deps, ui))
    }

    fn new_for_manifest(
        manifest: Manifest,
        name: String,
        chartfile: ChartFile,
        deps: Deps,
        ui: &'a mut UI,
    ) -> Self {
        let main = json!({
            "metadata_name": "{{.Values.metadataName}}",
            "service_name": "{{.Values.serviceName}}",
            "image": "{{.Values.imageName}}",
            "count": "{{.Values.instanceCount}}",
            "service_topology": "{{.Values.serviceTopology}}",
            "service_group": manifest.service_group.clone().map(|_| "{{.Values.serviceGroup}}"),
            "config": manifest.config
                .clone()
                .map(|_| "{{.Values.config}}"),
            "ring_secret_name": manifest.ring_secret_name
                .clone()
                .map(|_| "{{.Values.ringSecretName}}"),
            "bind": !manifest.binds.is_empty(),
        });

        let mut values = Values::new();
        values.add_entry("metadataName", &manifest.metadata_name);
        values.add_entry("serviceName", &manifest.service_name);
        values.add_entry("imageName", &manifest.image);
        values.add_entry("instanceCount", &manifest.count.to_string());
        values.add_entry("serviceTopology", &manifest.service_topology.to_string());
        if let Some(ref group) = manifest.service_group {
            values.add_entry("serviceGroup", group);
        }
        if let Some(ref config) = manifest.config {
            values.add_entry("config", config);
        }
        if let Some(ref name) = manifest.ring_secret_name {
            values.add_entry("ringSecretName", name);
        }

        let mut binds = Vec::new();
        let mut i = 0;
        for bind in &manifest.binds {
            let name_var = format!("bindName{}", i);
            let service_var = format!("bindService{}", i);
            let group_var = format!("bindGroup{}", i);
            i = i + 1;

            values.add_entry(&name_var, &bind.name);
            values.add_entry(&service_var, &bind.service_group.service());
            values.add_entry(&group_var, &bind.service_group.group());

            let json = json!({
                "name": format!("{{{{.Values.{}}}}}", name_var),
                "service": format!("{{{{.Values.{}}}}}", service_var),
                "group": format!("{{{{.Values.{}}}}}", group_var),
            });

            binds.push(json);
        }

        let manifest_template = Some(ManifestJson {
            main: main,
            binds: binds,
        });

        Chart {
            name,
            chartfile,
            manifest_template,
            values,
            deps,
            ui,
        }
    }

    pub fn generate(mut self) -> Result<()> {
        self.ui.status(
            Status::Creating,
            format!("directory `{}`", self.name),
        )?;
        fs::create_dir_all(&self.name)?;

        self.generate_chartfile()?;
        self.generate_values()?;
        self.generate_manifest_template()?;
        self.generate_deps()?;

        self.download_deps()
    }

    fn generate_chartfile(&mut self) -> Result<()> {
        let mut write = self.create_file("Chart.yaml")?;
        let out = self.chartfile.into_string()?;

        write.write(out.as_bytes())?;

        Ok(())
    }

    fn generate_manifest_template(&mut self) -> Result<()> {
        let template_path = format!("{}/{}", self.name, "templates");
        self.ui.status(
            Status::Creating,
            format!("directory `{}`", template_path),
        )?;
        fs::create_dir_all(&template_path)?;

        let manifest_path = format!("{}/{}.yaml", template_path, self.name);
        self.ui.status(
            Status::Creating,
            format!("file `{}`", manifest_path),
        )?;
        let mut write = fs::File::create(manifest_path)?;
        let out: String = self.manifest_template
            .take()
            .expect("generate_manifest_template() called more than once")
            .into();

        write.write(out.as_bytes())?;

        Ok(())
    }

    fn generate_values(&mut self) -> Result<()> {
        let mut write = self.create_file("values.yaml")?;
        self.values.generate(&mut write)?;

        Ok(())
    }

    fn generate_deps(&mut self) -> Result<()> {
        let mut write = self.create_file("requirements.yaml")?;
        self.deps.generate(&mut write)?;

        Ok(())
    }

    fn download_deps(&mut self) -> Result<()> {
        self.deps.download(&self.name, self.ui)
    }

    fn create_file(&mut self, name: &str) -> Result<fs::File> {
        let path = format!("{}/{}", self.name, name);
        self.ui.status(Status::Creating, format!("file `{}`", path))?;

        fs::File::create(path).map_err(From::from)
    }
}
