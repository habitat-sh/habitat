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

use std::{fs,
          path::{Path,
                 PathBuf},
          process::Command,
          str::FromStr};

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::{fs as hfs,
                    package::PackageIdent}};
use failure::SyncFailure;
use handlebars::Handlebars;

use super::{Credentials,
            Naming};
use crate::{build::BuildRoot,
            error::{Error,
                    Result},
            util};
use serde_json;

/// The `Dockerfile` template.
#[cfg(unix)]
const DOCKERFILE: &str = include_str!("../defaults/Dockerfile.hbs");
#[cfg(windows)]
const DOCKERFILE: &str = include_str!("../defaults/Dockerfile_win.hbs");
/// The build report template.
const BUILD_REPORT: &str = include_str!("../defaults/last_docker_export.env.hbs");

lazy_static! {
    /// Absolute path to the Docker program
    static ref DOCKER_PROGRAM: PathBuf = hfs::resolve_cmd_in_pkg(
        "docker",
        include_str!(concat!(env!("OUT_DIR"), "/DOCKER_PKG_IDENT")),
    );
}

/// A builder used to create a Docker image.
pub struct DockerBuilder<'a> {
    /// The base workdir which hosts the root file system.
    workdir: &'a Path,
    /// The name for the image.
    name: String,
    /// A list of tags for the image.
    tags: Vec<String>,
    /// Optional memory limit to pass to pass to the docker build
    memory: Option<&'a str>,
}

impl<'a> DockerBuilder<'a> {
    fn new<S>(workdir: &'a Path, name: S) -> Self
        where S: Into<String>
    {
        DockerBuilder { workdir,
                        name: name.into(),
                        tags: Vec::new(),
                        memory: None }
    }

    /// Adds a tag for the Docker image.
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Specifies an amount of memory to allocate to build
    pub fn memory(mut self, memory: &'a str) -> Self {
        self.memory = Some(memory);
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
        if let Some(mem) = self.memory {
            cmd.arg("--memory").arg(mem);
        }
        if self.tags.is_empty() {
            cmd.arg("--tag").arg(&self.name);
        } else {
            for tag in &self.tags {
                cmd.arg("--tag").arg(format!("{}:{}", &self.name, tag));
            }
        }
        cmd.arg(".");
        debug!("Running: {:?}", &cmd);
        let exit_status = cmd.spawn()?.wait()?;
        if !exit_status.success() {
            return Err(Error::BuildFailed(exit_status))?;
        }

        let id = match self.tags.first() {
            Some(tag) => self.image_id(&format!("{}:{}", &self.name, tag))?,
            None => self.image_id(&self.name)?,
        };

        Ok(DockerImage { id,
                         name: self.name,
                         tags: self.tags,
                         workdir: self.workdir.to_owned() })
    }

    fn image_id(&self, image_tag: &str) -> Result<String> {
        let mut cmd = docker_cmd();
        cmd.arg("images").arg("-q").arg(image_tag);
        debug!("Running: {:?}", &cmd);
        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        match stdout.lines().next() {
            Some(id) => Ok(id.to_string()),
            None => Err(Error::DockerImageIdNotFound(image_tag.to_string()))?,
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
    /// The base workdir which hosts the root file system.
    workdir: PathBuf,
}

impl<'a> DockerImage {
    /// Pushes the Docker image, with all tags, to a remote registry using the provided
    /// `Credentials`.
    ///
    /// # Errors
    ///
    /// * If a registry login is not successful
    /// * If a pushing one or more of the image tags fails
    /// * If a registry logout is not successful
    pub fn push(&self,
                ui: &mut UI,
                credentials: &Credentials,
                registry_url: Option<&str>)
                -> Result<()> {
        ui.begin(format!("Pushing Docker image '{}' with all tags to remote registry",
                         self.name()))?;
        self.create_docker_config_file(credentials, registry_url)
            .unwrap();
        if self.tags.is_empty() {
            self.push_image(ui, None)?;
        } else {
            for tag in &self.tags {
                self.push_image(ui, Some(tag))?;
            }
        }
        ui.end(format!("Docker image '{}' published with tags: {}",
                       self.name(),
                       self.tags().join(", "),))?;

        Ok(())
    }

    /// Removes the image from the local Docker engine along with all tags.
    ///
    /// # Errors
    ///
    /// * If one or more of the image tags cannot be removed
    pub fn rm(self, ui: &mut UI) -> Result<()> {
        ui.begin(format!("Cleaning up local Docker image '{}' with all tags",
                         self.name()))?;
        if self.tags.is_empty() {
            self.rm_image(ui, None)?;
        } else {
            for tag in &self.tags {
                self.rm_image(ui, Some(tag))?;
            }
        }
        ui.end(format!("Local Docker image '{}' with tags: {} cleaned up",
                       self.name(),
                       self.tags().join(", "),))?;

        Ok(())
    }

    /// Returns the ID of this image.
    pub fn id(&self) -> &str { self.id.as_str() }

    /// Returns the name of this image.
    pub fn name(&self) -> &str { self.name.as_str() }

    /// Returns the list of tags for this image.
    pub fn tags(&self) -> &[String] { &self.tags }

    /// Create a build report with image metadata in the given path.
    ///
    /// # Errors
    ///
    /// * If the destination directory cannot be created
    /// * If the report file cannot be written
    pub fn create_report<P: AsRef<Path>>(&self, ui: &mut UI, dst: P) -> Result<()> {
        let report = dst.as_ref().join("last_docker_export.env");
        ui.status(Status::Creating,
                  format!("build report {}", report.display()))?;
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
        util::write_file(&report,
                         &Handlebars::new().template_render(BUILD_REPORT, &json)
                                           .map_err(SyncFailure::new)?)?;
        Ok(())
    }

    pub fn create_docker_config_file(&self,
                                     credentials: &Credentials,
                                     registry_url: Option<&str>)
                                     -> Result<()> {
        let config = self.workdir.join("config.json");
        fs::create_dir_all(&self.workdir)?;
        let registry = match registry_url {
            Some(url) => url,
            None => "https://index.docker.io/v1/",
        };
        debug!("Using registry: {:?}", registry);
        let json = json!({
            "auths": {
                registry: {
                    "auth": credentials.token
                }
            }
        });
        util::write_file(&config, &serde_json::to_string(&json).unwrap())?;
        Ok(())
    }

    fn push_image(&self, ui: &mut UI, tag: Option<&str>) -> Result<()> {
        let image_tag = match tag {
            Some(tag) => format!("{}:{}", &self.name, tag),
            None => self.name.to_string(),
        };
        ui.status(Status::Uploading,
                  format!("image '{}' to remote registry", &image_tag))?;
        let mut cmd = docker_cmd();
        cmd.arg("--config");
        cmd.arg(self.workdir.to_str().unwrap());
        cmd.arg("push").arg(&image_tag);
        debug!("Running: {:?}", &cmd);
        let exit_status = cmd.spawn()?.wait()?;
        if !exit_status.success() {
            return Err(Error::PushImageFailed(exit_status))?;
        }
        ui.status(Status::Uploaded, format!("image '{}'", &image_tag))?;

        Ok(())
    }

    fn rm_image(&self, ui: &mut UI, tag: Option<&str>) -> Result<()> {
        let image_tag = match tag {
            Some(tag) => format!("{}:{}", &self.name, tag),
            None => self.name.to_string(),
        };
        ui.status(Status::Deleting, format!("local image '{}'", &image_tag))?;
        let mut cmd = docker_cmd();
        cmd.arg("rmi").arg(&image_tag);
        debug!("Running: {:?}", &cmd);
        let exit_status = cmd.spawn()?.wait()?;
        if !exit_status.success() {
            return Err(Error::RemoveImageFailed(exit_status))?;
        }

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
    #[cfg(unix)]
    pub fn from_build_root(build_root: BuildRoot, ui: &mut UI) -> Result<Self> {
        let root = DockerBuildRoot(build_root);
        root.add_users_and_groups(ui)?;
        root.create_entrypoint(ui)?;
        root.create_dockerfile(ui)?;

        Ok(root)
    }

    #[cfg(windows)]
    pub fn from_build_root(build_root: BuildRoot, ui: &mut UI) -> Result<Self> {
        let root = DockerBuildRoot(build_root);
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
    pub fn destroy(self, ui: &mut UI) -> Result<()> { self.0.destroy(ui) }

    /// Build the Docker image locally using the provided naming policy.
    ///
    /// # Errors
    ///
    /// * If the Docker image cannot be created successfully
    #[cfg(unix)]
    pub fn export(&self,
                  ui: &mut UI,
                  naming: &Naming,
                  memory: Option<&str>)
                  -> Result<DockerImage> {
        self.build_docker_image(ui, naming, memory)
    }

    #[cfg(windows)]
    pub fn export(&self,
                  ui: &mut UI,
                  naming: &Naming,
                  memory: Option<&str>)
                  -> Result<DockerImage> {
        let mut cmd = docker_cmd();
        cmd.arg("version").arg("--format='{{.Server.Os}}'");
        debug!("Running command: {:?}", cmd);
        let result = cmd.output().expect("Docker command failed to spawn");
        let os = String::from_utf8_lossy(&result.stdout);
        if !os.contains("windows") {
            return Err(Error::DockerNotInWindowsMode(os.to_string()))?;
        }

        self.build_docker_image(ui, naming, memory)
    }

    #[cfg(unix)]
    fn add_users_and_groups(&self, ui: &mut UI) -> Result<()> {
        use std::{fs::OpenOptions,
                  io::Write};

        let ctx = self.0.ctx();
        let (users, groups) = ctx.svc_users_and_groups()?;
        {
            let file = "etc/passwd";
            let mut f = OpenOptions::new().append(true)
                                          .open(ctx.rootfs().join(&file))?;
            for user in users {
                ui.status(Status::Creating,
                          format!("user '{}' in /{}", user.name, &file))?;
                writeln!(f, "{}", user)?;
            }
        }
        {
            let file = "etc/group";
            let mut f = OpenOptions::new().append(true)
                                          .open(ctx.rootfs().join(&file))?;
            for group in groups {
                ui.status(Status::Creating,
                          format!("group '{}' in /{}", group.name, &file))?;
                writeln!(f, "{}", group)?;
            }
        }
        Ok(())
    }

    #[cfg(unix)]
    fn create_entrypoint(&self, ui: &mut UI) -> Result<()> {
        use crate::hcore::util::posix_perm;

        /// The entrypoint script template.
        const INIT_SH: &str = include_str!("../defaults/init.sh.hbs");

        ui.status(Status::Creating, "entrypoint script")?;
        let ctx = self.0.ctx();
        let busybox_shell =
            util::pkg_path_for(&util::busybox_ident()?, ctx.rootfs())?.join("bin/sh");
        let json = json!({
            "busybox_shell": busybox_shell,
            "path": ctx.env_path(),
            "sup_bin": format!("{} sup", ctx.bin_path().join("hab").display()),
            "primary_svc_ident": ctx.primary_svc_ident().to_string(),
        });
        let init = ctx.rootfs().join("init.sh");
        util::write_file(&init,
                         &Handlebars::new().template_render(INIT_SH, &json)
                                           .map_err(SyncFailure::new)?)?;
        posix_perm::set_permissions(init.to_string_lossy().as_ref(), 0o0755)?;
        Ok(())
    }

    fn create_dockerfile(&self, ui: &mut UI) -> Result<()> {
        ui.status(Status::Creating, "image Dockerfile")?;
        let ctx = self.0.ctx();
        let json = json!({
            "rootfs": ctx.rootfs().file_name().expect("file_name exists")
                .to_string_lossy()
                .as_ref(),
            "path": ctx.env_path(),
            "hab_path": util::pkg_path_for(
                &PackageIdent::from_str("core/hab")?,
                ctx.rootfs())?.join("bin/hab")
                .to_string_lossy()
                .replace("\\", "/"),
            "exposes": ctx.svc_exposes().join(" "),
            "primary_svc_ident": ctx.primary_svc_ident().to_string(),
            "installed_primary_svc_ident": ctx.installed_primary_svc_ident()?.to_string(),
            "environment": ctx.environment,
        });
        util::write_file(self.0.workdir().join("Dockerfile"),
                         &Handlebars::new().template_render(DOCKERFILE, &json)
                                           .map_err(SyncFailure::new)?)?;
        Ok(())
    }

    fn build_docker_image(&self,
                          ui: &mut UI,
                          naming: &Naming,
                          memory: Option<&str>)
                          -> Result<DockerImage> {
        ui.status(Status::Creating, "Docker image")?;
        let ident = self.0.ctx().installed_primary_svc_ident()?;
        let version = &ident.version.expect("version exists");
        let release = &ident.release.expect("release exists");
        let json = json!({
            "pkg_origin": ident.origin,
            "pkg_name": ident.name,
            "pkg_version": &version,
            "pkg_release": &release,
            "channel": self.0.ctx().channel().as_str(),
        });
        let image_name = match naming.custom_image_name {
                             Some(ref custom) => {
                                 Handlebars::new().template_render(custom, &json)
                                                  .map_err(SyncFailure::new)?
                             }
                             None => format!("{}/{}", ident.origin, ident.name),
                         }.to_lowercase();

        let image_name = match naming.registry_url {
                             Some(ref url) => format!("{}/{}", url, image_name),
                             None => image_name,
                         }.to_lowercase();

        let mut builder = DockerBuilder::new(self.0.workdir(), image_name);
        if naming.version_release_tag {
            builder = builder.tag(format!("{}-{}", &version, &release));
        }
        if naming.version_tag {
            builder = builder.tag(version.clone());
        }
        if naming.latest_tag {
            builder = builder.tag("latest".to_string());
        }
        if let Some(memory) = memory {
            builder = builder.memory(memory);
        }
        if let Some(ref custom) = naming.custom_tag {
            builder = builder.tag(Handlebars::new().template_render(custom, &json)
                                                   .map_err(SyncFailure::new)?
                                                   .to_lowercase());
        }
        builder.build()
    }
}

/// Returns a `Command` for the Docker program.
fn docker_cmd() -> Command { Command::new(&*DOCKER_PROGRAM) }
