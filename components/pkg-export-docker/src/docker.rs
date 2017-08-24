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

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use common::ui::{UI, Status};
use hcore::os::filesystem;
use handlebars::Handlebars;

use build::BuildRoot;
use error::{Error, Result};
use super::{Credentials, Naming};
use util;

/// The `Dockerfile` template.
const DOCKERFILE: &'static str = include_str!("../defaults/Dockerfile.hbs");
/// The entrypoint script template.
const INIT_SH: &'static str = include_str!("../defaults/init.sh.hbs");
/// The build report template.
const BUILD_REPORT: &'static str = include_str!("../defaults/last_docker_export.env.hbs");

/// A builder used to create a Docker image.
pub struct DockerBuilder<'a> {
    /// The base workdir which hosts the root file system.
    workdir: &'a Path,
    /// The name for the image.
    name: String,
    /// A list of tags for the image.
    tags: Vec<String>,
}

impl<'a> DockerBuilder<'a> {
    fn new<S>(workdir: &'a Path, name: S) -> Self
    where
        S: Into<String>,
    {
        DockerBuilder {
            workdir: workdir,
            name: name.into(),
            tags: Vec::new(),
        }
    }

    /// Adds a tag for the Docker image.
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Builds the Docker image locally and returns the corresponding `DockerImage`.
    ///
    /// # Errors
    ///
    /// * If building the Docker image fails
    pub fn build(self) -> Result<DockerImage> {
        let mut cmd = docker_cmd();
        cmd.current_dir(self.workdir)
            .arg("build")
            .arg("--force-rm")
            .arg("--no-cache");
        if self.tags.is_empty() {
            cmd.arg("--tag").arg(&self.name);
        } else {
            if self.tags.is_empty() {
                cmd.arg("--tag").arg(&self.name);
            } else {
                for tag in &self.tags {
                    cmd.arg("--tag").arg(format!("{}:{}", &self.name, tag));
                }
            }
        }
        cmd.arg(".");
        debug!("Running: {:?}", &cmd);
        cmd.spawn()?.wait()?;

        let id = match self.tags.first() {
            Some(tag) => self.image_id(&format!("{}:{}", &self.name, tag))?,
            None => self.image_id(&self.name)?,
        };

        Ok(DockerImage {
            id,
            name: self.name,
            tags: self.tags,
        })
    }

    fn image_id(&self, image_tag: &str) -> Result<String> {
        let mut cmd = docker_cmd();
        cmd.arg("images").arg("-q").arg(image_tag);
        debug!("Running: {:?}", &cmd);
        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        match stdout.lines().next() {
            Some(id) => Ok(id.to_string()),
            None => Err(Error::DockerImageIdNotFound(image_tag.to_string())),
        }
    }
}

/// A built Docker image which exists locally.
pub struct DockerImage {
    /// The image ID for this image.
    id: String,
    /// The name of this image.
    name: String,
    /// The list of tags for this image.
    tags: Vec<String>,
}

impl<'a> DockerImage {
    /// Returns a new `DockerBuilder` which is used to build the image.
    pub fn new<S>(workdir: &'a Path, name: S) -> DockerBuilder<'a>
    where
        S: Into<String>,
    {
        DockerBuilder::new(workdir, name)
    }

    /// Pushes the Docker image, with all tags, to a remote registry using the provided
    /// `Credentials`.
    ///
    /// # Errors
    ///
    /// * If a registry login is not successful
    /// * If a pushing one or more of the image tags fails
    /// * If a registry logout is not successful
    pub fn push(&self, ui: &mut UI, credentials: &Credentials) -> Result<()> {
        ui.begin(format!(
            "Pushing Docker image '{}' with all tags to remote registry",
            self.name()
        ))?;
        self.logout(ui)?;
        self.login(ui, credentials)?;
        if self.tags.is_empty() {
            self.push_image(ui, None)?;
        } else {
            for tag in &self.tags {
                self.push_image(ui, Some(tag))?;
            }
        }
        self.logout(ui)?;
        ui.end(format!(
            "Docker image '{}' published with tags: {}",
            self.name(),
            self.tags().join(", "),
        ))?;

        Ok(())
    }

    /// Removes the image from the local Docker engine along with all tags.
    ///
    /// # Errors
    ///
    /// * If one or more of the image tags cannot be removed
    pub fn rm(self, ui: &mut UI) -> Result<()> {
        ui.begin(format!(
            "Cleaning up local Docker image '{}' with all tags",
            self.name()
        ))?;
        if self.tags.is_empty() {
            self.rm_image(ui, None)?;
        } else {
            for tag in &self.tags {
                self.rm_image(ui, Some(tag))?;
            }
        }
        ui.end(format!(
            "Local Docker image '{}' with tags: {} cleaned up",
            self.name(),
            self.tags().join(", "),
        ))?;

        Ok(())
    }

    /// Returns the ID of this image.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Returns the name of this image.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the list of tags for this image.
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Create a build report with image metadata in the given path.
    ///
    /// # Errors
    ///
    /// * If the destination directory cannot be created
    /// * If the report file cannot be written
    pub fn create_report<P: AsRef<Path>>(&self, ui: &mut UI, dst: P) -> Result<()> {
        let report = dst.as_ref().join("last_docker_export.env");
        ui.status(
            Status::Creating,
            format!("build report {}", report.display()),
        )?;
        fs::create_dir_all(&dst)?;
        let name_tags: Vec<_> = self.tags
            .iter()
            .map(|t| format!("{}:{}", &self.name, t))
            .collect();
        let json = json!({
            "id": &self.id,
            "name": &self.name,
            "tags": self.tags.join(","),
            "name_tags": name_tags.join(","),
        });
        util::write_file(
            &report,
            &Handlebars::new().template_render(BUILD_REPORT, &json)?,
        )?;
        Ok(())
    }

    fn login(&self, ui: &mut UI, credentials: &Credentials) -> Result<()> {
        ui.status(
            Status::Custom('☛', "Logging into".to_string()),
            "remote registry",
        )?;
        let mut cmd = docker_cmd();
        cmd.arg("login")
            .arg("--username")
            .arg(credentials.username)
            .arg("--password")
            .arg(credentials.password);
        debug!(
            "Running: {}",
            format!("{:?}", &cmd)
                .replace(credentials.username, "<username-redacted>")
                .replace(credentials.password, "<password-redacted>")
        );
        cmd.spawn()?.wait()?;

        Ok(())
    }

    fn logout(&self, ui: &mut UI) -> Result<()> {
        ui.status(
            Status::Custom('☒', "Logging out".to_string()),
            "of remote registry",
        )?;
        let mut cmd = docker_cmd();
        cmd.arg("logout");
        debug!("Running: {:?}", &cmd);
        cmd.spawn()?.wait()?;

        Ok(())
    }

    fn push_image(&self, ui: &mut UI, tag: Option<&str>) -> Result<()> {
        let image_tag = match tag {
            Some(tag) => format!("{}:{}", &self.name, tag),
            None => self.name.to_string(),
        };
        ui.status(
            Status::Uploading,
            format!("image '{}' to remote registry", &image_tag),
        )?;
        let mut cmd = docker_cmd();
        cmd.arg("push").arg(&image_tag);
        debug!("Running: {:?}", &cmd);
        cmd.spawn()?.wait()?;
        ui.status(
            Status::Uploaded,
            format!("image '{}'", &image_tag),
        )?;

        Ok(())
    }

    fn rm_image(&self, ui: &mut UI, tag: Option<&str>) -> Result<()> {
        let image_tag = match tag {
            Some(tag) => format!("{}:{}", &self.name, tag),
            None => self.name.to_string(),
        };
        ui.status(
            Status::Deleting,
            format!("local image '{}'", &image_tag),
        )?;
        let mut cmd = docker_cmd();
        cmd.arg("rmi").arg(&image_tag);
        debug!("Running: {:?}", &cmd);
        cmd.spawn()?.wait()?;

        Ok(())
    }
}

/// A temporary file system build root for building a Docker image, based on Habitat packages.
pub struct DockerBuildRoot(BuildRoot);

impl DockerBuildRoot {
    /// Builds a completed Docker build root from a `BuildRoot`, performing any final tasks on the
    /// root file system.
    ///
    /// # Errors
    ///
    /// * If any remaining tasks cannot be performed in the build root
    pub fn from_build_root(build_root: BuildRoot, ui: &mut UI) -> Result<Self> {
        let root = DockerBuildRoot(build_root);
        root.add_users_and_groups(ui)?;
        root.create_entrypoint(ui)?;
        root.create_dockerfile(ui)?;

        Ok(root)
    }

    /// Destroys the temporary build root.
    ///
    /// Note that the build root will automatically destroy itself when it falls out of scope, so
    /// a call to this method is not required, but calling this will provide more user-facing
    /// progress and error reporting.
    ///
    /// # Errors
    ///
    /// * If the temporary work directory cannot be removed
    pub fn destroy(self, ui: &mut UI) -> Result<()> {
        self.0.destroy(ui)
    }

    /// Build the Docker image locally using the provided naming policy.
    ///
    /// # Errors
    ///
    /// * If the Docker image cannot be created successfully
    pub fn export(&self, ui: &mut UI, naming: &Naming) -> Result<DockerImage> {
        self.build_docker_image(ui, naming)
    }

    fn add_users_and_groups(&self, ui: &mut UI) -> Result<()> {
        let ctx = self.0.ctx();
        let (users, groups) = ctx.svc_users_and_groups();
        {
            let file = "etc/passwd";
            let mut f = OpenOptions::new().append(true).open(
                ctx.rootfs().join(&file),
            )?;
            for line in users {
                let user = line.split(":").next().expect(
                    "user line contains first entry",
                );
                ui.status(
                    Status::Creating,
                    format!("user '{}' in /{}", user, &file),
                )?;
                f.write_all(line.as_bytes())?;
            }
        }
        {
            let file = "etc/group";
            let mut f = OpenOptions::new().append(true).open(
                ctx.rootfs().join(&file),
            )?;
            for line in groups {
                let group = line.split(":").next().expect(
                    "group line contains first entry",
                );
                ui.status(
                    Status::Creating,
                    format!("group '{}' in /{}", group, &file),
                )?;
                f.write_all(line.as_bytes())?;
            }
        }
        Ok(())
    }

    fn create_entrypoint(&self, ui: &mut UI) -> Result<()> {
        ui.status(Status::Creating, "entrypoint script")?;
        let ctx = self.0.ctx();
        let busybox_shell = util::pkg_path_for(&util::busybox_ident()?, ctx.rootfs())?
            .join("bin/sh");
        let json = json!({
            "busybox_shell": busybox_shell,
            "path": ctx.env_path(),
            "sup_bin": format!("{} sup", ctx.bin_path().join("hab").display()),
            "primary_svc_ident": ctx.primary_svc_ident().to_string(),
        });
        let init = ctx.rootfs().join("init.sh");
        util::write_file(&init, &Handlebars::new().template_render(INIT_SH, &json)?)?;
        filesystem::chmod(init.to_string_lossy().as_ref(), 0o0755)?;
        Ok(())
    }

    fn create_dockerfile(&self, ui: &mut UI) -> Result<()> {
        ui.status(Status::Creating, "image Dockerfile")?;
        let ctx = self.0.ctx();
        let json = json!({
            "rootfs": ctx.rootfs().file_name().expect("file_name exists").to_string_lossy().as_ref(),
            "path": ctx.env_path(),
            "volumes": ctx.svc_volumes().join(" "),
            "exposes": ctx.svc_exposes().join(" "),
            "primary_svc_ident": ctx.primary_svc_ident().to_string(),
        });
        util::write_file(
            self.0.workdir().join("Dockerfile"),
            &Handlebars::new().template_render(DOCKERFILE, &json)?,
        )?;
        Ok(())
    }

    fn build_docker_image(&self, ui: &mut UI, naming: &Naming) -> Result<DockerImage> {
        ui.status(Status::Creating, "Docker image")?;
        let ident = self.0.ctx().installed_primary_svc_ident()?;
        let version = &ident.version.expect("version exists");
        let release = &ident.release.expect("release exists");
        let json = json!({
            "pkg_origin": ident.origin,
            "pkg_name": ident.name,
            "pkg_version": &version,
            "pkg_release": &release,
            "channel": self.0.ctx().channel(),
        });
        let image_name = match naming.custom_image_name {
            Some(ref custom) => Handlebars::new().template_render(custom, &json)?,
            None => format!("{}/{}", ident.origin, ident.name),
        };

        let mut image = DockerImage::new(self.0.workdir(), image_name);
        if naming.version_release_tag {
            image = image.tag(format!("{}-{}", &version, &release));
        }
        if naming.version_tag {
            image = image.tag(version.clone());
        }
        if naming.latest_tag {
            image = image.tag("latest".to_string());
        }
        if let Some(ref custom) = naming.custom_tag {
            image = image.tag(Handlebars::new().template_render(custom, &json)?);
        }
        image.build()
    }
}

/// Returns a `Command` for the Docker program.
///
/// TODO fn: This is good enough for the moment, but it would be beneficial to support and
/// environment variable or outer CLI option to set the location for the Docker program. However,
/// future work might remove the requirement for this program so there currently isn't much bought
/// by adding work that'll be stripped away later.
fn docker_cmd() -> Command {
    Command::new("docker")
}
