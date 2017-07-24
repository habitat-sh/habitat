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

use std;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
#[cfg(not(windows))]
use std::process::{Child, ExitStatus};
#[cfg(windows)]
use hcore::os::process::windows_child::{Child, ExitStatus};
use std::result;

use ansi_term::Colour;
use hcore;
use hcore::crypto;
use hcore::service::ServiceGroup;
use serde::{Serialize, Serializer};

use super::{health, Pkg};
use error::{Result, SupError};
use fs;
use templating::{RenderContext, TemplateRenderer};
use util::exec;

pub const HOOK_PERMISSIONS: u32 = 0o755;
static LOGKEY: &'static str = "HK";

pub fn stdout_log_path<T>(service_group: &ServiceGroup) -> PathBuf
where
    T: Hook,
{
    fs::svc_logs_path(service_group.service()).join(format!("{}.stdout.log", T::file_name()))
}

pub fn stderr_log_path<T>(service_group: &ServiceGroup) -> PathBuf
where
    T: Hook,
{
    fs::svc_logs_path(service_group.service()).join(format!("{}.stderr.log", T::file_name()))
}

#[derive(Debug, Copy, Clone)]
pub struct ExitCode(i32);

impl Default for ExitCode {
    fn default() -> ExitCode {
        ExitCode(-1)
    }
}

pub trait Hook: fmt::Debug + Sized {
    type ExitValue: Default;

    fn file_name() -> &'static str;

    fn load<C, T>(service_group: &ServiceGroup, concrete_path: C, template_path: T) -> Option<Self>
    where
        C: AsRef<Path>,
        T: AsRef<Path>,
    {
        let concrete = concrete_path.as_ref().join(Self::file_name());
        let template = template_path.as_ref().join(Self::file_name());
        match std::fs::metadata(&template) {
            Ok(_) => {
                let pair = match RenderPair::new(concrete, &template) {
                    Ok(pair) => pair,
                    Err(err) => {
                        outputln!(preamble service_group, "Failed to load hook: {}", err);
                        return None;
                    }
                };
                Some(Self::new(service_group, pair))
            }
            Err(_) => {
                info!(
                    "{} not found at {}, not loading",
                    Self::file_name(),
                    template.display()
                );
                None
            }
        }
    }

    fn new(service_group: &ServiceGroup, render_pair: RenderPair) -> Self;

    /// Compile a hook into its destination service directory.
    fn compile(&self, ctx: &RenderContext) -> Result<bool> {
        let content = self.renderer().render("hook", ctx)?;
        if write_hook(&content, self.path())? {
            info!(
                "{}, compiled to {}",
                Self::file_name(),
                self.path().display()
            );
            hcore::util::perm::set_owner(self.path(), &ctx.pkg.svc_user, &ctx.pkg.svc_group)?;
            hcore::util::perm::set_permissions(self.path(), HOOK_PERMISSIONS)?;
            Ok(true)
        } else {
            info!(
                "{}, already compiled to {}",
                Self::file_name(),
                self.path().display()
            );
            Ok(false)
        }
    }

    /// Run a compiled hook.
    fn run<T>(
        &self,
        service_group: &ServiceGroup,
        pkg: &Pkg,
        svc_encrypted_password: Option<T>,
    ) -> Self::ExitValue
    where
        T: ToString,
    {
        let mut child = match exec::run(self.path(), &pkg, svc_encrypted_password) {
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
            Ok(status) => self.handle_exit(service_group, &hook_output, &status),
            Err(err) => {
                outputln!(preamble service_group,
                    "Hook failed to run, {}, {}", Self::file_name(), err);
                Self::ExitValue::default()
            }
        }
    }

    fn handle_exit<'a>(
        &self,
        group: &ServiceGroup,
        output: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue;

    fn path(&self) -> &Path;

    fn renderer(&self) -> &TemplateRenderer;

    fn stdout_log_path(&self) -> &Path;

    fn stderr_log_path(&self) -> &Path;
}

#[derive(Debug, Serialize)]
pub struct FileUpdatedHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for FileUpdatedHook {
    type ExitValue = bool;

    fn file_name() -> &'static str {
        "file_updated"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        FileUpdatedHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        _: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        status.success()
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

#[derive(Debug, Serialize)]
pub struct HealthCheckHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for HealthCheckHook {
    type ExitValue = health::HealthCheck;

    fn file_name() -> &'static str {
        "health_check"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        HealthCheckHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(0) => health::HealthCheck::Ok,
            Some(1) => health::HealthCheck::Warning,
            Some(2) => health::HealthCheck::Critical,
            Some(3) => health::HealthCheck::Unknown,
            Some(code) => {
                outputln!(preamble service_group,
                    "Health check exited with an unknown status code, {}", code);
                health::HealthCheck::default()
            }
            None => {
                outputln!(preamble service_group,
                    "{} exited without a status code", Self::file_name());
                health::HealthCheck::default()
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

#[derive(Debug, Serialize)]
pub struct InitHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for InitHook {
    type ExitValue = bool;

    fn file_name() -> &'static str {
        "init"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        InitHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(0) => true,
            Some(code) => {
                outputln!(preamble service_group, "Initialization failed! '{}' exited with \
                    status code {}", Self::file_name(), code);
                false
            }
            None => {
                outputln!(preamble service_group, "Initialization failed! '{}' exited without a \
                    status code", Self::file_name());
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

#[derive(Debug, Serialize)]
pub struct RunHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for RunHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "run"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        RunHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn run<T>(&self, _: &ServiceGroup, _: &Pkg, _: Option<T>) -> Self::ExitValue
    where
        T: ToString,
    {
        panic!(
            "The run hook is a an exception to the lifetime of a service. It should only be \
             run by the supervisor module!"
        );
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(code) => ExitCode(code),
            None => {
                outputln!(preamble service_group,
                    "{} exited without a status code", Self::file_name());
                ExitCode::default()
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

#[derive(Debug, Serialize)]
pub struct PostRunHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for PostRunHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "post-run"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        PostRunHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(code) => ExitCode(code),
            None => {
                outputln!(preamble service_group,
                    "{} exited without a status code", Self::file_name());
                ExitCode::default()
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

#[derive(Debug, Serialize)]
pub struct ReloadHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for ReloadHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "reload"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        ReloadHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(0) => ExitCode(0),
            Some(code) => {
                outputln!(preamble service_group, "Reload failed! '{}' exited with \
                    status code {}", Self::file_name(), code);
                ExitCode(code)
            }
            None => {
                outputln!(preamble service_group, "Reload failed! '{}' exited without a \
                    status code", Self::file_name());
                ExitCode::default()
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

#[derive(Debug, Serialize)]
pub struct ReconfigureHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for ReconfigureHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "reconfigure"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        ReconfigureHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(code) => ExitCode(code),
            None => {
                outputln!(preamble service_group,
                    "{} exited without a status code", Self::file_name());
                ExitCode::default()
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

#[derive(Debug, Serialize)]
pub struct SmokeTestHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for SmokeTestHook {
    type ExitValue = health::SmokeCheck;

    fn file_name() -> &'static str {
        "smoke_test"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        SmokeTestHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        _: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(0) => health::SmokeCheck::Ok,
            Some(code) => health::SmokeCheck::Failed(code),
            None => {
                outputln!(preamble service_group,
                    "{} exited without a status code", Self::file_name());
                health::SmokeCheck::Failed(-1)
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

#[derive(Debug, Serialize)]
pub struct SuitabilityHook {
    render_pair: RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for SuitabilityHook {
    type ExitValue = Option<u64>;

    fn file_name() -> &'static str {
        "suitability"
    }

    fn new(service_group: &ServiceGroup, pair: RenderPair) -> Self {
        SuitabilityHook {
            render_pair: pair,
            stdout_log_path: stdout_log_path::<Self>(service_group),
            stderr_log_path: stderr_log_path::<Self>(service_group),
        }
    }

    fn handle_exit<'a>(
        &self,
        service_group: &ServiceGroup,
        hook_output: &'a HookOutput,
        status: &ExitStatus,
    ) -> Self::ExitValue {
        match status.code() {
            Some(0) => {
                if let Some(reader) = hook_output.stdout() {
                    if let Some(line_reader) = reader.lines().last() {
                        match line_reader {
                            Ok(line) => {
                                match line.trim().parse::<u64>() {
                                    Ok(suitability) => {
                                        outputln!(preamble service_group, "Reporting suitability \
                                    of: {}", Colour::Green.bold().paint(format!("{}",suitability)));
                                        return Some(suitability);
                                    }
                                    Err(err) => {
                                        outputln!(preamble service_group,
                                            "Parsing suitability failed: {}", err);
                                    }
                                };
                            }
                            Err(err) => {
                                outputln!(preamble service_group,
                                    "Failed to read last line of stdout: {}", err);
                            }
                        };
                    } else {
                        outputln!(preamble service_group,
                                  "{} did not print anything to stdout", Self::file_name());
                    }
                }
            }
            Some(code) => {
                outputln!(preamble service_group,
                    "{} exited with status code {}", Self::file_name(), code);
            }
            None => {
                outputln!(preamble service_group,
                    "{} exited without a status code", Self::file_name());
            }
        }
        None
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
        crypto::hash::hash_file(path).map_err(|e| SupError::from(e))
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

#[derive(Debug, Default, Serialize)]
pub struct HookTable {
    pub health_check: Option<HealthCheckHook>,
    pub init: Option<InitHook>,
    pub file_updated: Option<FileUpdatedHook>,
    pub reload: Option<ReloadHook>,
    pub reconfigure: Option<ReconfigureHook>,
    pub suitability: Option<SuitabilityHook>,
    pub run: Option<RunHook>,
    pub post_run: Option<PostRunHook>,
    pub smoke_test: Option<SmokeTestHook>,
}

impl HookTable {
    /// Read all available hook templates from the table's package directory into the table.
    pub fn load<T>(service_group: &ServiceGroup, templates: T) -> Self
    where
        T: AsRef<Path>,
    {
        let mut table = HookTable::default();
        let hooks = fs::svc_hooks_path(service_group.service());
        if let Some(meta) = std::fs::metadata(templates.as_ref()).ok() {
            if meta.is_dir() {
                table.file_updated = FileUpdatedHook::load(service_group, &hooks, &templates);
                table.health_check = HealthCheckHook::load(service_group, &hooks, &templates);
                table.suitability = SuitabilityHook::load(service_group, &hooks, &templates);
                table.init = InitHook::load(service_group, &hooks, &templates);
                table.reload = ReloadHook::load(service_group, &hooks, &templates);
                table.reconfigure = ReconfigureHook::load(service_group, &hooks, &templates);
                table.run = RunHook::load(service_group, &hooks, &templates);
                table.post_run = PostRunHook::load(service_group, &hooks, &templates);
                table.smoke_test = SmokeTestHook::load(service_group, &hooks, &templates);
            }
        }
        info!(
            "{}, Hooks loaded, destination={}, templates={}",
            service_group,
            hooks.display(),
            templates.as_ref().display()
        );
        table
    }

    /// Compile all loaded hooks from the table into their destination service directory.
    ///
    /// Returns `true` if compiling any of the hooks resulted in new
    /// content being written to the hook scripts on disk.
    pub fn compile(&self, service_group: &ServiceGroup, ctx: &RenderContext) -> bool {
        debug!("{:?}", self);
        let mut changed = false;
        if let Some(ref hook) = self.file_updated {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.health_check {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.init {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.reload {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.reconfigure {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.suitability {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.run {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.post_run {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        if let Some(ref hook) = self.smoke_test {
            changed = self.compile_one(hook, service_group, ctx) || changed;
        }
        info!("{}, Hooks compiled", service_group);
        changed
    }

    fn compile_one<H>(&self, hook: &H, service_group: &ServiceGroup, ctx: &RenderContext) -> bool
    where
        H: Hook,
    {
        match hook.compile(ctx) {
            Ok(status) => status,
            Err(e) => {
                outputln!(preamble service_group,
                          "Failed to compile {} hook: {}", H::file_name(), e);
                false
            }
        }
    }
}

pub struct RenderPair {
    pub path: PathBuf,
    pub renderer: TemplateRenderer,
}

impl RenderPair {
    pub fn new<C, T>(concrete_path: C, template_path: T) -> Result<Self>
    where
        C: Into<PathBuf>,
        T: AsRef<Path>,
    {
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_file(
            "hook",
            template_path.as_ref(),
        )?;
        Ok(RenderPair {
            path: concrete_path.into(),
            renderer: renderer,
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

    fn stdout(&self) -> Option<BufReader<File>> {
        match File::open(&self.stdout_log_file) {
            Ok(f) => Some(BufReader::new(f)),
            Err(_) => None,
        }
    }

    #[allow(dead_code)]
    fn stderr(&self) -> Option<BufReader<File>> {
        match File::open(&self.stderr_log_file) {
            Ok(f) => Some(BufReader::new(f)),
            Err(_) => None,
        }
    }

    fn stream_output<H: Hook>(&mut self, service_group: &ServiceGroup, process: &mut Child) {
        let mut stdout_log =
            File::create(&self.stdout_log_file).expect("couldn't create log output file");
        let mut stderr_log =
            File::create(&self.stderr_log_file).expect("couldn't create log output file");

        let preamble_str = self.stream_preamble::<H>(service_group);
        if let Some(ref mut stdout) = process.stdout {
            for line in BufReader::new(stdout).lines() {
                if let Some(ref l) = line.ok() {
                    outputln!(preamble preamble_str, l);
                    stdout_log.write_fmt(format_args!("{}\n", l)).expect(
                        "couldn't write line",
                    );
                }
            }
        }
        if let Some(ref mut stderr) = process.stderr {
            for line in BufReader::new(stderr).lines() {
                if let Some(ref l) = line.ok() {
                    outputln!(preamble preamble_str, l);
                    stderr_log.write_fmt(format_args!("{}\n", l)).expect(
                        "couldn't write line",
                    );
                }
            }
        }
    }

    fn stream_preamble<H: Hook>(&self, service_group: &ServiceGroup) -> String {
        format!("{} hook[{}]:", service_group, H::file_name())
    }
}

#[cfg(test)]
#[cfg(not(windows))]
mod tests {
    use super::*;
    use std::fs::{self, DirBuilder};
    use tempdir::TempDir;
    use std::process::{Command, Stdio};

    use std::iter;
    use std::string::ToString;
    use manager::sys::Sys;
    use config::GossipListenAddr;
    use http_gateway::ListenAddr;
    use hcore::package::{PackageIdent, PackageInstall};
    use manager::service::{Pkg, Cfg};
    use manager::service::spec::ServiceBind;
    use census::CensusRing;

    use hcore::service::ServiceGroup;
    use butterfly::member::MemberList;
    use butterfly::rumor::service::Service as ServiceRumor;
    use butterfly::rumor::service_config::ServiceConfig as ServiceConfigRumor;
    use butterfly::rumor::service_file::ServiceFile as ServiceFileRumor;
    use butterfly::rumor::election::Election as ElectionRumor;
    use butterfly::rumor::election::ElectionUpdate as ElectionUpdateRumor;
    use butterfly::rumor::service::SysInfo;
    use butterfly::rumor::RumorStore;

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

    as_ref_path_impl!(FileUpdatedHook
                      HealthCheckHook
                      InitHook
                      PostRunHook
                      ReconfigureHook
                      ReloadHook
                      RunHook
                      SmokeTestHook
                      SuitabilityHook);

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
        TempDir::new("habitat_hooks_test").expect("create temp dir")
    }

    fn service_group() -> ServiceGroup {
        ServiceGroup::new(None, "test_service", "test_group", None)
            .expect("couldn't create ServiceGroup")
    }

    fn file_content<P>(path: P) -> String
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).expect("Could not open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(
            "Unable to read file!",
        );
        contents
    }

    fn create_with_content<P, C>(path: P, content: C)
    where
        P: AsRef<Path>,
        C: ToString,
    {
        let mut file = File::create(path).expect("Cannot create file");
        file.write_all(content.to_string().as_bytes()).expect(
            "Cannot write to file",
        );
    }

    ////////////////////////////////////////////////////////////////////////

    #[test]
    fn hashing_a_hook_that_already_exists_returns_a_hash_of_the_file() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InitHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing init hook");

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
        let hook = InitHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing init hook");

        assert_eq!(hash_content(hook.path()).unwrap(), "");
    }

    #[test]
    fn updating_a_hook_with_the_same_content_is_a_noop() {
        let service_group = service_group();
        let concrete_path = rendered_hooks_path();
        let template_path = hook_templates_path();

        let hook = InitHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing init hook");

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

        let hook = InitHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing init hook");

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

        let hook = InitHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing init hook");

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

        let hook = InitHook::load(&service_group, &concrete_path, &template_path)
            .expect("Could not create testing init hook");

        ////////////////////////////////////////////////////////////////////////
        // BEGIN RENDER CONTEXT SETUP
        // (See comment above)

        let sys = Sys::new(true, GossipListenAddr::default(), ListenAddr::default());

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
        let pkg = Pkg::from_install(pkg_install).expect("Could not create package!");

        // This is gross, but it actually works
        let cfg_path = concrete_path.as_ref().join("default.toml");
        create_with_content(cfg_path, &String::from("message = \"Hello\""));

        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_ref().to_path_buf()))
            .expect("Could not create config");

        // SysInfo is basic Swim infrastructure information
        let mut sys_info = SysInfo::new();
        sys_info.set_ip("1.2.3.4".to_string());
        sys_info.set_hostname("hostname".to_string());
        sys_info.set_gossip_ip("0.0.0.0".to_string());
        sys_info.set_gossip_port(7777);
        sys_info.set_http_gateway_ip("0.0.0.0".to_string());
        sys_info.set_http_gateway_port(9631);

        let sg_one = service_group.clone(); // ServiceGroup::new("shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one = ServiceRumor::new("member-a", &pg_id, &sg_one, &sys_info, None);
        service_store.insert(service_one);

        let election_store: RumorStore<ElectionRumor> = RumorStore::default();
        let mut election = ElectionRumor::new("member-a", sg_one.clone(), 10);
        election.finish();
        election_store.insert(election);

        let election_update_store: RumorStore<ElectionUpdateRumor> = RumorStore::default();

        let member_list = MemberList::new();

        let service_config_store: RumorStore<ServiceConfigRumor> = RumorStore::default();
        let service_file_store: RumorStore<ServiceFileRumor> = RumorStore::default();

        let mut ring = CensusRing::new("member-a");
        ring.update_from_rumors(
            &service_store,
            &election_store,
            &election_update_store,
            &member_list,
            &service_config_store,
            &service_file_store,
        );

        let bindings = iter::empty::<&ServiceBind>();

        let ctx = RenderContext::new(&service_group, &sys, &pkg, &cfg, &ring, bindings);

        // END RENDER CONTEXT SETUP
        ////////////////////////////////////////////////////////////////////////

        assert_eq!(hook.compile(&ctx).unwrap(), true);

        let post_change_content = file_content(&hook);
        let expected = r#"#!/bin/bash

echo "The message is Hello"
"#;
        assert_eq!(post_change_content, expected);

        // Compiling again should result in no changes
        assert_eq!(hook.compile(&ctx).unwrap(), false);
        let post_second_change_content = file_content(&hook);
        assert_eq!(post_second_change_content, post_change_content);
    }

    #[test]
    fn compile_hook_table() {

        let tmp_root = rendered_hooks_path();
        std::env::set_var("FS_ROOT", tmp_root.path());

        let template_root = tmp_root
            .path()
            .join("hab")
            .join("svc")
            .join("test_service")
            .join("hooks");
        DirBuilder::new()
            .recursive(true)
            .create(template_root.clone())
            .unwrap();

        let service_group = service_group();

        let concrete_path = template_root.clone(); //rendered_hooks_path();
        let template_path = hook_templates_path();

        ////////////////////////////////////////////////////////////////////////
        // BEGIN RENDER CONTEXT SETUP
        // (See comment above)

        let sys = Sys::new(true, GossipListenAddr::default(), ListenAddr::default());

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
        let pkg = Pkg::from_install(pkg_install).expect("Could not create package!");

        // This is gross, but it actually works
        let cfg_path = &concrete_path.as_path().join("default.toml");
        create_with_content(cfg_path, &String::from("message = \"Hello\""));

        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_path().to_path_buf()))
            .expect("Could not create config");

        // SysInfo is basic Swim infrastructure information
        let mut sys_info = SysInfo::new();
        sys_info.set_ip("1.2.3.4".to_string());
        sys_info.set_hostname("hostname".to_string());
        sys_info.set_gossip_ip("0.0.0.0".to_string());
        sys_info.set_gossip_port(7777);
        sys_info.set_http_gateway_ip("0.0.0.0".to_string());
        sys_info.set_http_gateway_port(9631);

        let sg_one = service_group.clone(); // ServiceGroup::new("shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one = ServiceRumor::new("member-a", &pg_id, &sg_one, &sys_info, None);
        service_store.insert(service_one);

        let election_store: RumorStore<ElectionRumor> = RumorStore::default();
        let mut election = ElectionRumor::new("member-a", sg_one.clone(), 10);
        election.finish();
        election_store.insert(election);

        let election_update_store: RumorStore<ElectionUpdateRumor> = RumorStore::default();

        let member_list = MemberList::new();

        let service_config_store: RumorStore<ServiceConfigRumor> = RumorStore::default();
        let service_file_store: RumorStore<ServiceFileRumor> = RumorStore::default();

        let mut ring = CensusRing::new("member-a");
        ring.update_from_rumors(
            &service_store,
            &election_store,
            &election_update_store,
            &member_list,
            &service_config_store,
            &service_file_store,
        );

        let bindings = iter::empty::<&ServiceBind>();

        let ctx = RenderContext::new(&service_group, &sys, &pkg, &cfg, &ring, bindings);

        // END RENDER CONTEXT SETUP
        ////////////////////////////////////////////////////////////////////////

        let hook_table = HookTable::load(&service_group, &template_path);
        assert_eq!(hook_table.compile(&service_group, &ctx), true);

        // Verify init hook
        let init_hook_content = file_content(&hook_table.init.as_ref().expect("no init hook??"));
        assert_eq!(
            init_hook_content,
            "#!/bin/bash\n\necho \"The message is Hello\"\n"
        );
        // Verify run hook
        let run_hook_content = file_content(&hook_table.run.as_ref().expect("no run hook??"));
        assert_eq!(
            run_hook_content,
            "#!/bin/bash\n\necho \"Running a program\"\n"
        );

        // Recompiling again results in no changes
        assert_eq!(hook_table.compile(&service_group, &ctx), false);

        // Re-Verify init hook
        let init_hook_content = file_content(&hook_table.init.as_ref().expect("no init hook??"));
        assert_eq!(
            init_hook_content,
            "#!/bin/bash\n\necho \"The message is Hello\"\n"
        );
        // Re-Verify run hook
        let run_hook_content = file_content(&hook_table.run.as_ref().expect("no run hook??"));
        assert_eq!(
            run_hook_content,
            "#!/bin/bash\n\necho \"Running a program\"\n"
        );
    }

    ////////////////////////////////////////////////////////////////////////

    #[test]
    fn hook_output() {
        let tmp_dir = TempDir::new("habitat_hooks_test").expect("create temp dir");
        let logs_dir = tmp_dir.path().join("logs");
        DirBuilder::new().recursive(true).create(logs_dir).expect(
            "couldn't create logs dir",
        );
        let mut cmd = Command::new(hook_fixtures_path().join(InitHook::file_name()));
        cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(
            Stdio::piped(),
        );
        let mut child = cmd.spawn().expect("couldn't run hook");
        let stdout_log = tmp_dir.path().join("logs").join(format!(
            "{}.stdout.log",
            InitHook::file_name()
        ));
        let stderr_log = tmp_dir.path().join("logs").join(format!(
            "{}.stderr.log",
            InitHook::file_name()
        ));
        let mut hook_output = HookOutput::new(&stdout_log, &stderr_log);
        let service_group = ServiceGroup::new(None, "dummy", "service", None).expect(
            "couldn't create ServiceGroup",
        );

        hook_output.stream_output::<InitHook>(&service_group, &mut child);

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

        fs::remove_dir_all(tmp_dir).expect("remove temp dir");
    }
}
