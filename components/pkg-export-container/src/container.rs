use crate::{build::BuildRoot,
            engine::Engine,
            error::Result,
            naming::{ImageIdentifiers,
                     Naming},
            util};
use failure::SyncFailure;
use habitat_common::ui::{Status,
                         UIWriter,
                         UI};
use habitat_core::package::PackageIdent;
use handlebars::Handlebars;
use std::{fs,
          path::{Path,
                 PathBuf},
          str::FromStr};

// This code makes heavy use of `#[cfg(unix)]` and `#[cfg(windows)]`. This should potentially be
// changed to use the various target feature flags.

/// The `Dockerfile` template.
#[cfg(unix)]
const DOCKERFILE: &str = include_str!("../defaults/Dockerfile.hbs");
#[cfg(windows)]
const DOCKERFILE: &str = include_str!("../defaults/Dockerfile_win.hbs");
/// The build report template.
const BUILD_REPORT: &str = include_str!("../defaults/last_container_export.env.hbs");

/// The file that the build report will be written to.
const BUILD_REPORT_FILE_NAME: &str = "last_container_export.env";

/// Provided only for backwards compatibility; will contain a
/// duplicate of the standard build report.
const OLD_BUILD_REPORT_FILE_NAME: &str = "last_docker_export.env";

/// A built container image which exists locally.
pub struct ContainerImage {
    /// The image ID for this image.
    id:      String,
    /// The name of this image.
    name:    String,
    /// The list of tags for this image.
    tags:    Vec<String>,
    /// The base workdir which hosts the root file system.
    workdir: PathBuf,

    /// All the identifiers for this image (in {name}:{tag} format)
    expanded_identifiers: Vec<String>,
}

impl ContainerImage {
    // TODO (CM): temporary; we shouldn't use this at all
    pub fn workdir(&self) -> &Path { self.workdir.as_path() }

    pub fn expanded_identifiers(&self) -> &[String] { &self.expanded_identifiers }

    pub fn name(&self) -> String { self.name.clone() }

    pub fn tags(&self) -> Vec<String> { self.tags.clone() }

    /// Create a build report with image metadata in the given path.
    ///
    /// # Errors
    ///
    /// * If the destination directory cannot be created
    /// * If the report file cannot be written
    pub fn create_report<P: AsRef<Path>>(&self, ui: &mut UI, dst: P) -> Result<()> {
        let report = Self::report_path(&dst);
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

        Self::create_old_report(ui, dst);

        Ok(())
    }

    fn report_path<P: AsRef<Path>>(dir: P) -> PathBuf { dir.as_ref().join(BUILD_REPORT_FILE_NAME) }

    /// When this was the "Docker exporter", we wrote the report out
    /// to "last_docker_export.env". Now that it is the "container
    /// exporter", this name makes less sense, and we instead create
    /// "last_container_export.env".
    ///
    /// For backwards compatibility, however, we'll continue to write
    /// out the same report to "last_docker_export.env", in case users
    /// have automation that depends on that specific location.
    ///
    /// This function assumes that "last_container_export.env" has
    /// already been written out in the `dst` directory, and that
    /// `dst` already exists.
    ///
    /// It intentionally does not return an error because this is a
    /// best-effort attempt. We've already done all the main work of
    /// this exporter and it makes no sense to fail the entire
    /// operation at this point.
    fn create_old_report<P: AsRef<Path>>(ui: &mut UI, dst: P) {
        let current_report = Self::report_path(&dst);
        let old_report = dst.as_ref().join(OLD_BUILD_REPORT_FILE_NAME);
        ui.status(Status::Creating,
                  format!("old build report '{}' for backwards compatibility; please favor '{}' \
                           going forward",
                          old_report.display(),
                          current_report.display()))
          .ok(); // don't care about an error here

        if let Err(e) = std::fs::copy(&current_report, &old_report) {
            error!("Failed to create '{}' for backwards-compatibility purposes; this may safely \
                    be ignored: {}",
                   old_report.display(),
                   e);
        }
    }
}

/// A build context for building a container
///
/// (i.e. the `.` in `docker build -t foo .`)
pub struct BuildContext(BuildRoot);

impl BuildContext {
    /// Builds a completed build root from a `BuildRoot`, performing any final tasks on the
    /// root file system.
    ///
    /// # Errors
    ///
    /// * If any remaining tasks cannot be performed in the build root
    #[cfg(unix)]
    pub fn from_build_root(build_root: BuildRoot, ui: &mut UI) -> Result<Self> {
        let context = BuildContext(build_root);
        context.add_users_and_groups(ui)?;
        context.create_entrypoint(ui)?;
        context.create_dockerfile(ui)?;

        Ok(context)
    }

    #[cfg(windows)]
    pub fn from_build_root(build_root: BuildRoot, ui: &mut UI) -> Result<Self> {
        let context = BuildContext(build_root);
        context.create_dockerfile(ui)?;

        Ok(context)
    }

    /// Destroys the temporary build context.
    ///
    /// Note that the build context will automatically destroy itself
    /// when it falls out of scope, so a call to this method is not
    /// required, but calling this will provide more user-facing
    /// progress and error reporting.
    ///
    /// # Errors
    ///
    /// * If the temporary work directory cannot be removed
    pub fn destroy(self, ui: &mut UI) -> Result<()> { self.0.destroy(ui) }

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
        use habitat_core::util::posix_perm;

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
            "base_image": ctx.base_image(),
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
            "multi_layer": ctx.multi_layer(),
            "primary_svc_ident": ctx.primary_svc_ident().to_string(),
            "installed_primary_svc_ident": ctx.installed_primary_svc_ident()?.to_string(),
            "environment": ctx.environment,
            "packages": self.0.graph().reverse_topological_sort().iter().map(ToString::to_string).collect::<Vec<_>>(),
        });
        util::write_file(self.0.workdir().join("Dockerfile"),
                         &Handlebars::new().template_render(DOCKERFILE, &json)
                                           .map_err(SyncFailure::new)?)?;
        Ok(())
    }

    /// Build the image locally using the provided naming policy.
    pub fn export(&self,
                  ui: &mut UI,
                  naming: &Naming,
                  memory: Option<&str>,
                  engine: &dyn Engine)
                  -> Result<ContainerImage> {
        ui.status(Status::Creating, "image")?;
        let ident = self.0.ctx().installed_primary_svc_ident()?;
        let channel = self.0.ctx().channel();

        // TODO (CM): Ideally, we'd toss this error much earlier,
        // since this error would be based on user input errors
        let ImageIdentifiers { name,
                               tags,
                               expanded_identifiers, } =
            naming.image_identifiers(&ident, &channel)?;

        let id = engine.build(self.0.workdir(), &expanded_identifiers, memory)?;

        // TODO (CM): Once ContainerImage doesn't need access to
        // workdir, we could just have Engine::build return a
        // ContainerImage directly, which is appealing.
        Ok(ContainerImage { id,
                            name,
                            tags,
                            expanded_identifiers,
                            workdir: self.0.workdir().to_path_buf() })
    }
}
