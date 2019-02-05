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
use std::{fs, io::Write, path::PathBuf};

use crate::{
    common::ui::{Status, UIWriter, UI},
    export_docker::{self, Result},
    export_k8s::{Manifest, ManifestJson, PersistentStorage},
};

use crate::{chartfile::ChartFile, deps::Deps, values::Values};

pub struct Chart<'a> {
    chartdir: PathBuf,
    chartfile: ChartFile,
    manifest_template: Option<ManifestJson>,
    values: Values,
    deps: Deps,
    ui: &'a mut UI,
}

impl<'a> Chart<'a> {
    pub fn new_for_cli_matches(ui: &'a mut UI, matches: &clap::ArgMatches<'_>) -> Result<Self> {
        let image = if !matches.is_present("NO_DOCKER_IMAGE") {
            export_docker::export_for_cli_matches(ui, &matches)?
        } else {
            None
        };
        let manifest = Manifest::new_from_cli_matches(ui, &matches, image)?;
        let chartfile = ChartFile::new_from_cli_matches(&matches, &manifest.pkg_ident)?;
        let deps = Deps::new_for_cli_matches(&matches);

        let mut chartdir = PathBuf::new();
        if let Some(o) = matches.value_of_os("OUTPUTDIR") {
            chartdir.push(o);
        }
        chartdir.push(&chartfile.name);

        Ok(Self::new_for_manifest(
            manifest, chartdir, chartfile, deps, ui,
        ))
    }

    fn new_for_manifest(
        manifest: Manifest,
        chartdir: PathBuf,
        chartfile: ChartFile,
        deps: Deps,
        ui: &'a mut UI,
    ) -> Self {
        let mut manifest_template = ManifestJson {
            value: json!({
            "metadata_name": "{{.Values.metadataName}}",
            "service_name": "{{.Values.serviceName}}",
            "image": "{{.Values.imageName}}",
            "count": "{{.Values.instanceCount}}",
            "service_topology": "{{.Values.serviceTopology}}",
            "service_group": manifest.service_group
                .as_ref()
                .map(|_| "{{.Values.serviceGroup}}"),
            "config": manifest.config
                .as_ref()
                .map(|_| "{{.Values.config}}"),
            "ring_secret_name": manifest.ring_secret_name
                .as_ref()
                .map(|_| "{{.Values.ringSecretName}}"),
            "persistent_storage": manifest.persistent_storage
                .as_ref()
                .map(|_| {
                    PersistentStorage {
                        size: "{{.Values.persistentStorageSize}}".to_string(),
                        path: "{{.Values.persistentStoragePath}}".to_string(),
                        class: "{{.Values.persistentStorageClass}}".to_string(),
                    }.to_json()
                }),
            }),
        };

        let mut values = Values::new();
        values.add_entry("metadataName", &manifest.metadata_name);
        values.add_entry("serviceName", &manifest.pkg_ident.name);
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
        if let Some(ref persistent_storage) = manifest.persistent_storage {
            values.add_entry("persistentStorageSize", &persistent_storage.size);
            values.add_entry("persistentStoragePath", &persistent_storage.path);
            values.add_entry("persistentStorageClass", &persistent_storage.class);
        }

        let mut binds = Vec::new();
        for (i, bind) in manifest.binds.iter().enumerate() {
            let name_var = format!("bindName{}", i);
            let service_var = format!("bindService{}", i);
            let group_var = format!("bindGroup{}", i);

            values.add_entry(&name_var, bind.name());
            values.add_entry(&service_var, bind.service_group().service());
            values.add_entry(&group_var, bind.service_group().group());

            let json = json!({
                "name": format!("{{{{.Values.{}}}}}", name_var),
                "service": format!("{{{{.Values.{}}}}}", service_var),
                "group": format!("{{{{.Values.{}}}}}", group_var),
            });

            binds.push(json);
        }
        manifest_template.value["binds"] = json!(binds);

        let mut environment = Vec::new();
        for (i, envvar) in manifest.environment.iter().enumerate() {
            let name_var = format!("envName{}", i);
            let value_var = format!("envValue{}", i);

            values.add_entry(&name_var, &envvar.name);
            values.add_entry(&value_var, &envvar.value);

            let json = json!({
                "name": format!("{{{{.Values.{}}}}}", name_var),
                "value": format!("{{{{.Values.{}}}}}", value_var),
            });

            environment.push(json);
        }
        manifest_template.value["environment"] = json!(environment);

        Chart {
            chartdir,
            chartfile,
            manifest_template: Some(manifest_template),
            values,
            deps,
            ui,
        }
    }

    pub fn generate(mut self) -> Result<()> {
        self.ui.status(
            Status::Creating,
            format!("directory `{}`", self.chartdir.display()),
        )?;
        fs::create_dir_all(&self.chartdir)?;

        self.generate_chartfile()?;
        self.generate_values()?;
        self.generate_manifest_template()?;
        self.generate_deps()?;

        self.download_deps()
    }

    fn generate_chartfile(&mut self) -> Result<()> {
        let mut write = self.create_file("Chart.yaml")?;
        let out = self.chartfile.into_string()?;

        write.write_all(out.as_bytes())?;

        Ok(())
    }

    fn generate_manifest_template(&mut self) -> Result<()> {
        let mut path = self.chartdir.clone();
        path.push("templates");
        self.ui
            .status(Status::Creating, format!("directory `{}`", path.display()))?;
        fs::create_dir_all(&path)?;

        path.push("habitat.yaml");
        self.ui
            .status(Status::Creating, format!("file `{}`", path.display()))?;
        let mut write = fs::File::create(path)?;
        let out: String = self
            .manifest_template
            .take()
            .expect("generate_manifest_template() called more than once")
            .into();

        write.write_all(out.as_bytes())?;

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
        self.deps.download(&self.chartdir, self.ui)
    }

    fn create_file(&mut self, name: &str) -> Result<fs::File> {
        let mut path = self.chartdir.clone();
        path.push(name);
        self.ui
            .status(Status::Creating, format!("file `{}`", path.display()))?;

        fs::File::create(path).map_err(From::from)
    }
}
