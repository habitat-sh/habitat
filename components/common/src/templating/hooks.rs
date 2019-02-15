// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

#[cfg(unix)]
use std::os::unix::process::{CommandExt, ExitStatusExt};
#[cfg(not(windows))]
use std::process::{Child, Command, ExitStatus, Stdio};
use std::{
    ffi::OsStr,
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
    result,
};

#[cfg(windows)]
use crate::hcore::os::process::windows_child::{Child, ExitStatus};
use crate::hcore::{self, crypto, fs, outputln};
use serde::{Serialize, Serializer};

use super::{package::Pkg, TemplateRenderer};
use crate::error::{Error, Result};

#[cfg(not(windows))]
pub const HOOK_PERMISSIONS: u32 = 0o755;
static LOGKEY: &'static str = "HK";

pub fn stdout_log_path<T>(package_name: &str) -> PathBuf
where
    T: Hook,
{
    fs::svc_logs_path(package_name).join(format!("{}.stdout.log", T::file_name()))
}

pub fn stderr_log_path<T>(package_name: &str) -> PathBuf
where
    T: Hook,
{
    fs::svc_logs_path(package_name).join(format!("{}.stderr.log", T::file_name()))
}

#[derive(Debug, Copy, Clone)]
pub struct ExitCode(pub i32);

impl Default for ExitCode {
    fn default() -> ExitCode {
        ExitCode(-1)
    }
}

pub trait Hook: fmt::Debug + Sized {
    type ExitValue: Default;

    fn file_name() -> &'static str;

    /// Tries to load a hook if a (deprecated) hook file exists.
    ///
    /// Returns the hook if template file (deprecated or not) is found
    fn load<C, T>(package_name: &str, concrete_path: C, template_path: T) -> Option<Self>
    where
        C: AsRef<Path>,
        T: AsRef<Path>,
    {
        let file_name = Self::file_name();
        let deprecated_file_name = if Self::file_name().contains('-') {
            Some(Self::file_name().replace("-", "_"))
        } else {
            None
        };
        let concrete = concrete_path.as_ref().join(&file_name);
        let template = template_path.as_ref().join(&file_name);
        let deprecated_template = deprecated_file_name
            .as_ref()
            .map(|n| template_path.as_ref().join(n));

        let has_template = template.exists();
        let has_deprecated_template = deprecated_template.as_ref().map_or(false, |t| t.exists());

        let template_to_use = if has_template {
            if has_deprecated_template {
                outputln!(preamble package_name,
                    "Deprecated hook file detected along with expected one. \
                     You should remove {} and keep only {}.",
                    deprecated_file_name.unwrap(),
                    &file_name
                );
            }
            template
        } else if has_deprecated_template {
            outputln!(preamble package_name,
                "Deprecated hook file detected: {}. You should use {} instead.",
                deprecated_file_name.unwrap(),
                &file_name
            );
            deprecated_template.unwrap()
        } else {
            debug!(
                "{} not found at {}, not loading",
                &file_name,
                template.display()
            );
            return None;
        };
        match RenderPair::new(concrete, &template_to_use, Self::file_name()) {
            Ok(pair) => Some(Self::new(package_name, pair)),
            Err(err) => {
                outputln!(preamble package_name, "Failed to load hook: {}", err);
                None
            }
        }
    }

    fn new(package_name: &str, render_pair: RenderPair) -> Self;

    /// Compile a hook into its destination service directory.
    ///
    /// Returns `true` if the hook has changed.
    fn compile<T>(&self, service_group: &str, ctx: &T) -> Result<bool>
    where
        T: Serialize,
    {
        let content = self.renderer().render(Self::file_name(), ctx)?;
        // We make sure we don't use a deprecated file name
        let path = self.path().with_file_name(Self::file_name());
        if write_hook(&content, &path)? {
            outputln!(preamble service_group,
                      "Modified hook content in {}",
                      &path.display());
            Self::set_permissions(&path)?;
            Ok(true)
        } else {
            debug!(
                "{}, already compiled to {}",
                Self::file_name(),
                &path.display()
            );
            Ok(false)
        }
    }

    #[cfg(not(windows))]
    fn set_permissions<T: AsRef<Path>>(path: T) -> hcore::error::Result<()> {
        use crate::hcore::util::posix_perm;

        posix_perm::set_permissions(path.as_ref(), HOOK_PERMISSIONS)
    }

    #[cfg(windows)]
    fn set_permissions<T: AsRef<Path>>(path: T) -> hcore::error::Result<()> {
        use crate::hcore::util::win_perm;

        win_perm::harden_path(path.as_ref())
    }

    /// Output a message that a hook process was terminated by a
    /// signal.
    ///
    /// This should only be called when `ExitStatus#code()` returns
    /// `None`, and this only happens on non-Windows machines.
    #[cfg(unix)]
    fn output_termination_message(service_group: &str, status: &ExitStatus) {
        outputln!(preamble service_group, "{} was terminated by signal {:?}",
                  Self::file_name(),
                  status.signal());
    }

    /// This should only be called when `ExitStatus#code()` returns
    /// `None`, and this can only happen on non-Windows machines.
    ///
    /// Thus, if this code is ever called on Windows, something has
    /// fundamentally changed in the Rust standard library.
    ///
    /// See https://doc.rust-lang.org/1.30.1/std/process/struct.ExitStatus.html#method.code
    #[cfg(windows)]
    fn output_termination_message(_: &str, _: &ExitStatus) {
        panic!(
            "ExitStatus::code should never return None on Windows; please report this to the \
             Habitat core developers"
        );
    }

    /// Run a compiled hook.
    fn run<T>(
        &self,
        service_group: &str,
        pkg: &Pkg,
        svc_encrypted_password: Option<T>,
    ) -> Self::ExitValue
    where
        T: ToString,
    {
        let mut child = match Self::exec(self.path(), &pkg, svc_encrypted_password) {
            Ok(child) => child,
            Err(err) => {
                outputln!(preamble service_group,
                    "Hook failed to run, {}, {}", Self::file_name(), err);
                return Self::ExitValue::default();
            }
        };
        let mut hook_output = HookOutput::new(self.stdout_log_path(), self.stderr_log_path());
        hook_output.stream_output::<Self>(service_group, &mut child);
        match child.wait() {
            Ok(status) => self.handle_exit(pkg, &hook_output, &status),
            Err(err) => {
                outputln!(preamble service_group,
                    "Hook failed to run, {}, {}", Self::file_name(), err);
                Self::ExitValue::default()
            }
        }
    }

    #[cfg(windows)]
    fn exec<T, S>(path: S, pkg: &Pkg, svc_encrypted_password: Option<T>) -> Result<Child>
    where
        T: ToString,
        S: AsRef<OsStr>,
    {
        let ps_cmd = format!("iex $(gc {} | out-string)", path.as_ref().to_string_lossy());
        let args = vec!["-NonInteractive", "-command", ps_cmd.as_str()];
        Ok(Child::spawn(
            "pwsh.exe",
            args,
            &pkg.env,
            &pkg.svc_user,
            svc_encrypted_password,
        )?)
    }

    #[cfg(unix)]
    fn exec<T, S>(path: S, pkg: &Pkg, _: Option<T>) -> Result<Child>
    where
        T: ToString,
        S: AsRef<OsStr>,
    {
        use crate::hcore::os::users;

        let mut cmd = Command::new(path.as_ref());
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (key, val) in pkg.env.iter() {
            cmd.env(key, val);
        }

        if users::can_run_services_as_svc_user() {
            // If we can SETUID/SETGID, then run the script as the service
            // user; otherwise, we'll just run it as ourselves.

            let uid = users::get_uid_by_name(&pkg.svc_user).ok_or_else(|| {
                Error::PermissionFailed(format!(
                    "No uid for user '{}' could be found",
                    &pkg.svc_user
                ))
            })?;
            let gid = users::get_gid_by_name(&pkg.svc_group).ok_or_else(|| {
                Error::PermissionFailed(format!(
                    "No gid for group '{}' could be found",
                    &pkg.svc_group
                ))
            })?;

            cmd.uid(uid).gid(gid);
        } else {
            debug!(
                "Current user lacks sufficient capabilites to run {:?} as \"{}\"; running as self!",
                path.as_ref(),
                &pkg.svc_user
            );
        }

        Ok(cmd.spawn()?)
    }

    fn handle_exit<'a>(
        &self,
        pkg: &Pkg,
        output: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue;

    fn path(&self) -> &Path;

    fn renderer(&self) -> &TemplateRenderer;

    fn stdout_log_path(&self) -> &Path;

    fn stderr_log_path(&self) -> &Path;
}

#[derive(Debug, Serialize)]
pub struct InstallHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl InstallHook {
    pub const STATUS_FILE: &'static str = "INSTALL_HOOK_STATUS";
}

impl Hook for InstallHook {
    type ExitValue = bool;

    fn file_name() -> &'static str {
        "install"
    }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        InstallHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(package_name),
            stderr_log_path: stderr_log_path::<Self>(package_name),
        }
    }

    fn handle_exit<'a>(
        &self,
        pkg: &Pkg,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        let name = &pkg.name;
        if let Some(code) = status.code() {
            let path = pkg.path.join(InstallHook::STATUS_FILE);
            if let Ok(mut f) = File::create(&path) {
                if let Err(err) = write!(f, "{}", code) {
                    outputln!(
                        preamble name,
                        "failed to write install hook status file to {:?}: {}",
                        path,
                        err
                    );
                }
            } else {
                outputln!(
                    preamble name,
                    "failed to write install hook status file to {:?}",
                    path
                );
            }
        }
        match status.code() {
            Some(0) => true,
            Some(code) => {
                outputln!(
                    preamble name,
                    "Installation failed! '{}' exited with \
                     status code {}",
                    Self::file_name(),
                    code
                );
                false
            }
            None => {
                Self::output_termination_message(name, status);
                false
            }
        }
    }

    fn path(&self) -> &Path {
        &self.render_pair.path
    }

    fn renderer(&self) -> &TemplateRenderer {
        &self.render_pair.renderer
    }

    fn stdout_log_path(&self) -> &Path {
        &self.stdout_log_path
    }

    fn stderr_log_path(&self) -> &Path {
        &self.stderr_log_path
    }
}

/// Cryptographically hash the contents of the compiled hook
/// file.
///
/// If the file does not exist, an empty string is returned.
fn hash_content<T>(path: T) -> Result<String>
where
    T: AsRef<Path>,
{
    if path.as_ref().exists() {
        crypto::hash::hash_file(path).map_err(Error::from)
    } else {
        Ok(String::new())
    }
}

fn write_hook<T>(content: &str, path: T) -> Result<bool>
where
    T: AsRef<Path>,
{
    let content_hash = crypto::hash::hash_string(&content);
    let existing_hash = hash_content(path.as_ref())?;

    if existing_hash == content_hash {
        Ok(false)
    } else {
        let mut file = File::create(path.as_ref())?;
        file.write_all(&content.as_bytes())?;
        Ok(true)
    }
}

pub struct RenderPair {
    pub path: PathBuf,
    pub renderer: TemplateRenderer,
}

impl RenderPair {
    pub fn new<C, T>(concrete_path: C, template_path: T, name: &'static str) -> Result<Self>
    where
        C: Into<PathBuf>,
        T: AsRef<Path>,
    {
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_file(&name, template_path.as_ref())?;
        Ok(RenderPair {
            path: concrete_path.into(),
            renderer,
        })
    }
}

impl fmt::Debug for RenderPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path: {}", self.path.display())
    }
}

impl Serialize for RenderPair {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.path.as_os_str().to_string_lossy().into_owned())
    }
}

pub struct HookOutput<'a> {
    stdout_log_file: &'a Path,
    stderr_log_file: &'a Path,
}

impl<'a> HookOutput<'a> {
    fn new(stdout_log: &'a Path, stderr_log: &'a Path) -> Self {
        HookOutput {
            stdout_log_file: stdout_log,
            stderr_log_file: stderr_log,
        }
    }

    pub fn stdout(&self) -> Option<BufReader<File>> {
        match File::open(&self.stdout_log_file) {
            Ok(f) => Some(BufReader::new(f)),
            Err(_) => None,
        }
    }

    pub fn stderr(&self) -> Option<BufReader<File>> {
        match File::open(&self.stderr_log_file) {
            Ok(f) => Some(BufReader::new(f)),
            Err(_) => None,
        }
    }

    fn stream_output<H: Hook>(&mut self, service_group: &str, process: &mut Child) {
        let mut stdout_log =
            File::create(&self.stdout_log_file).expect("couldn't create log output file");
        let mut stderr_log =
            File::create(&self.stderr_log_file).expect("couldn't create log output file");

        let preamble_str = self.stream_preamble::<H>(service_group);
        if let Some(ref mut stdout) = process.stdout {
            for line in BufReader::new(stdout).lines() {
                if let Ok(ref l) = line {
                    outputln!(preamble preamble_str, l);
                    stdout_log
                        .write_fmt(format_args!("{}\n", l))
                        .expect("couldn't write line");
                }
            }
        }
        if let Some(ref mut stderr) = process.stderr {
            for line in BufReader::new(stderr).lines() {
                if let Ok(ref l) = line {
                    outputln!(preamble preamble_str, l);
                    stderr_log
                        .write_fmt(format_args!("{}\n", l))
                        .expect("couldn't write line");
                }
            }
        }
    }

    fn stream_preamble<H: Hook>(&self, service_group: &str) -> String {
        format!("{} hook[{}]:", service_group, H::file_name())
    }
}

#[cfg(test)]
mod tests {
    use crate::hcore::{
        package::{PackageIdent, PackageInstall},
        service::ServiceGroup,
    };
    use tempfile::TempDir;

    use super::*;
    use crate::templating::{config::Cfg, context::RenderContext, package::Pkg, test_helpers::*};

    // Turns out it's useful for Hooks to implement AsRef<Path>, at
    // least for these tests. Ideally, this would be useful to use
    // outside of the tests as well, but some additional refactoring
    // will be necessary.
    macro_rules! as_ref_path_impl {
        ($($t:ty)*) => ($(
            impl AsRef<Path> for $t {
                fn as_ref(&self) -> &Path {
                    &self.render_pair.path
                }
            }
        )*)
    }

    as_ref_path_impl!(InstallHook);

    #[cfg(not(windows))]
    fn hook_fixtures_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("hooks")
    }

    fn hook_templates_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("hooks")
            .join("hook_templates")
    }

    fn rendered_hooks_path() -> TempDir {
        TempDir::new().expect("create temp dir")
    }

    fn service_group() -> ServiceGroup {
        ServiceGroup::new(None, "test_service", "test_group", None)
            .expect("couldn't create ServiceGroup")
    }

    ////////////////////////////////////////////////////////////////////////

    #[test]
    fn hashing_a_hook_that_already_exists_returns_a_hash_of_the_file() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InstallHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing install hook");

        let content = r#"
#!/bin/bash

echo "The message is Hello World"
"#;
        create_with_content(&hook, content);

        assert_eq!(
            hash_content(hook.path()).unwrap(),
            "1cece41b2f4d5fddc643fc809d80c17d6658634b28ec1c5ceb80e512e20d2e72"
        );
    }

    #[test]
    fn hashing_a_hook_that_does_not_already_exist_returns_an_empty_string() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();
        let hook = InstallHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing install hook");

        assert_eq!(hash_content(hook.path()).unwrap(), "");
    }

    #[test]
    fn updating_a_hook_with_the_same_content_is_a_noop() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InstallHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing install hook");

        // Since we're trying to update a file that should already
        // exist, we need to actually create it :P
        let content = r#"
#!/bin/bash

echo "The message is Hello World"
"#;
        create_with_content(&hook, content);

        let pre_change_content = file_content(&hook);

        // In the real world, we'd be templating something with this
        // content, but for the purposes of detecting changes, feeding
        // it the final text works well enough, and doesn't tie this
        // test to the templating machinery.
        assert_eq!(write_hook(&content, hook.path()).unwrap(), false);

        let post_change_content = file_content(&hook);
        assert_eq!(post_change_content, pre_change_content);
    }

    #[test]
    fn updating_a_hook_that_creates_the_file_works() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InstallHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing install hook");

        // In this test, we'll start with *no* rendered content.
        assert_eq!(hook.as_ref().exists(), false);

        let updated_content = r#"
#!/bin/bash

echo "The message is Hello World"
"#;
        // Since there was no compiled hook file before, this should
        // create it, returning `true` to reflect that
        assert_eq!(write_hook(&updated_content, hook.path()).unwrap(), true);

        // The content of the file should now be what we just changed
        // it to.
        let post_change_content = file_content(&hook);
        assert_eq!(post_change_content, updated_content);
    }

    #[test]
    fn truly_updating_a_hook_works() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InstallHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing install hook");

        let initial_content = r#"
#!/bin/bash

echo "The message is Hello World"
"#;
        create_with_content(&hook, initial_content);

        // Again, we're not templating anything here (as would happen
        // in the real world), but just passing the final content that
        // we'd like to update the hook with.
        let updated_content = r#"
#!/bin/bash

echo "The message is Hola Mundo"
"#;
        assert_eq!(write_hook(&updated_content, hook.path()).unwrap(), true);

        let post_change_content = file_content(&hook);
        assert_ne!(post_change_content, initial_content);
        assert_eq!(post_change_content, updated_content);
    }

    /// Avert your eyes, children; avert your eyes!
    ///
    /// All I wanted was a simple RenderContext so I could compile a
    /// hook. With the type signatures as they are, though, I don't
    /// know if that's possible. So, in the functions that follow, a
    /// minimal fake RenderContext is created within this function,
    /// and we pass it into the relevant compilation functions to test
    ///
    /// A `RenderContext` could _almost_ be anything that's
    /// JSON-serializable, in which case we wouldn't have to jump
    /// through _nearly_ as many hoops as we do here. Unfortunately,
    /// the compilation call also pulls things out of the context's
    /// package struct, which is more than just a blob of JSON
    /// data. We can probably do something about that, though.
    ///
    /// The context that these functions ends up making has a lot of
    /// fake data around the ring membership, the package, etc. We
    /// don't really need all that just to make compilation actually
    /// change a file or not.
    ///
    /// Due to how a RenderContext is currently set up, though, I
    /// couldn't sort out the relevant Rust lifetimes and type
    /// signatures needed to have a helper function that just handed
    /// back a RenderContext. It may be possible, or we may want to
    /// refactor that code to make it possible. In the meantime, copy
    /// and paste of the code is how we're going to do it :(
    #[test]
    fn compile_a_hook() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InstallHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing install hook");

        ////////////////////////////////////////////////////////////////////////
        // BEGIN RENDER CONTEXT SETUP
        // (See comment above)

        let pg_id = PackageIdent::new(
            "testing",
            &service_group.service(),
            Some("1.0.0"),
            Some("20170712000000"),
        );

        let pkg_install = PackageInstall::new_from_parts(
            pg_id.clone(),
            PathBuf::from("/tmp"),
            PathBuf::from("/tmp"),
            PathBuf::from("/tmp"),
        );
        let pkg = Pkg::from_install(&pkg_install).expect("Could not create package!");

        // This is gross, but it actually works
        let cfg_path = concrete_path.as_ref().join("default.toml");
        create_with_content(cfg_path, &String::from("message = \"Hello\""));

        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_ref().to_path_buf()))
            .expect("Could not create config");

        let ctx = RenderContext::new(&pkg, &cfg);

        // END RENDER CONTEXT SETUP
        ////////////////////////////////////////////////////////////////////////

        assert_eq!(hook.compile(&service_group, &ctx).unwrap(), true);

        let post_change_content = file_content(&hook);
        let expected = r#"#!/bin/bash

echo "The message is Hello"
"#;
        assert_eq!(post_change_content, expected);

        // Compiling again should result in no changes
        assert_eq!(hook.compile(&service_group, &ctx).unwrap(), false);
        let post_second_change_content = file_content(&hook);
        assert_eq!(post_second_change_content, post_change_content);
    }

    ////////////////////////////////////////////////////////////////////////

    #[test]
    #[cfg(not(windows))]
    fn hook_output() {
        use std::{
            fs as stdfs,
            fs::DirBuilder,
            process::{Command, Stdio},
        };

        let tmp_dir = TempDir::new().expect("create temp dir");
        let logs_dir = tmp_dir.path().join("logs");
        DirBuilder::new()
            .recursive(true)
            .create(logs_dir)
            .expect("couldn't create logs dir");
        let mut cmd = Command::new(hook_fixtures_path().join(InstallHook::file_name()));
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd.spawn().expect("couldn't run hook");
        let stdout_log = tmp_dir
            .path()
            .join("logs")
            .join(format!("{}.stdout.log", InstallHook::file_name()));
        let stderr_log = tmp_dir
            .path()
            .join("logs")
            .join(format!("{}.stderr.log", InstallHook::file_name()));
        let mut hook_output = HookOutput::new(&stdout_log, &stderr_log);
        let service_group = ServiceGroup::new(None, "dummy", "service", None)
            .expect("couldn't create ServiceGroup");

        hook_output.stream_output::<InstallHook>(&service_group, &mut child);

        let mut stdout = String::new();
        hook_output
            .stdout()
            .unwrap()
            .read_to_string(&mut stdout)
            .expect("couldn't read stdout");
        assert_eq!(stdout, "This is stdout\n");

        let mut stderr = String::new();
        hook_output
            .stderr()
            .unwrap()
            .read_to_string(&mut stderr)
            .expect("couldn't read stderr");
        assert_eq!(stderr, "This is stderr\n");

        stdfs::remove_dir_all(tmp_dir).expect("remove temp dir");
    }
}
