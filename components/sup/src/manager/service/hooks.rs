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
use std::process::{Child, ExitStatus};
use std::result;

use ansi_term::Colour;
use hcore;
use hcore::service::ServiceGroup;
use serde::{Serialize, Serializer};

use super::health;
use error::Result;
use fs;
use manager::service::ServiceConfig;
use supervisor::RuntimeConfig;
use templating::Template;
use util;

pub const HOOK_PERMISSIONS: u32 = 0o755;
static LOGKEY: &'static str = "HK";

pub fn stdout_log_path<T>(service_group: &ServiceGroup) -> PathBuf
    where T: Hook
{
    fs::svc_logs_path(service_group.service()).join(format!("{}.stdout.log", T::file_name()))
}

pub fn stderr_log_path<T>(service_group: &ServiceGroup) -> PathBuf
    where T: Hook
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
        where C: AsRef<Path>,
              T: AsRef<Path>
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
                debug!("{} not found at {}, not loading",
                       Self::file_name(),
                       template.display());
                None
            }
        }
    }

    fn new(service_group: &ServiceGroup, render_pair: RenderPair) -> Self;

    /// Compile a hook into it's destination service directory.
    fn compile(&self, cfg: &ServiceConfig) -> Result<()> {
        let toml = try!(cfg.to_toml());
        let svc_data = util::convert::toml_to_json(toml);
        let data = try!(self.template().render("hook", &svc_data));
        let mut file = try!(File::create(self.path()));
        try!(file.write_all(data.as_bytes()));
        try!(hcore::util::perm::set_owner(self.path(), &cfg.pkg.svc_user, &cfg.pkg.svc_group));
        try!(hcore::util::perm::set_permissions(self.path(), HOOK_PERMISSIONS));
        debug!("{} compiled to {}",
               Self::file_name(),
               self.path().display());
        Ok(())
    }

    /// Run a compiled hook.
    fn run(&self, service_group: &ServiceGroup, cfg: &RuntimeConfig) -> Self::ExitValue {
        let mut cmd = match util::create_command(self.path(), cfg) {
            Ok(c) => c,
            Err(err) => {
                outputln!(preamble service_group,
                    "Hook command failed to be created, {}, {}", Self::file_name(), err);
                return Self::ExitValue::default();
            }
        };
        let mut child = match cmd.spawn() {
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

    fn handle_exit<'a>(&self,
                       group: &ServiceGroup,
                       output: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue;

    fn path(&self) -> &Path;

    fn template(&self) -> &Template;

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

    fn handle_exit<'a>(&self,
                       _: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
        status.success()
    }

    fn path(&self) -> &Path {
        &self.render_pair.path
    }

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn run(&self, _: &ServiceGroup, _: &RuntimeConfig) -> Self::ExitValue {
        panic!("The run hook is a an exception to the lifetime of a service. It should only be \
                run by the supervisor module!");
    }

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       _: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
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

    fn handle_exit<'a>(&self,
                       service_group: &ServiceGroup,
                       hook_output: &'a HookOutput,
                       status: &ExitStatus)
                       -> Self::ExitValue {
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

    fn template(&self) -> &Template {
        &self.render_pair.template
    }

    fn stdout_log_path(&self) -> &Path {
        &self.stdout_log_path
    }

    fn stderr_log_path(&self) -> &Path {
        &self.stderr_log_path
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
    cfg_incarnation: u64,
}

impl HookTable {
    /// Compile all loaded hooks from the table into their destination service directory.
    pub fn compile(&mut self, service_group: &ServiceGroup, config: &ServiceConfig) {
        if self.cfg_incarnation != 0 && config.incarnation <= self.cfg_incarnation {
            debug!("{}, Hooks already compiled with the latest configuration incarnation, \
                    skipping",
                   service_group);
            return;
        }
        self.cfg_incarnation = config.incarnation;
        if let Some(ref hook) = self.file_updated {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.health_check {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.init {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.reload {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.reconfigure {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.suitability {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.run {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.post_run {
            self.compile_one(hook, service_group, config);
        }
        if let Some(ref hook) = self.smoke_test {
            self.compile_one(hook, service_group, config);
        }
        debug!("{}, Hooks compiled", service_group);
    }

    /// Read all available hook templates from the table's package directory into the table.
    pub fn load_hooks<T, U>(mut self, service_group: &ServiceGroup, hooks: T, templates: U) -> Self
        where T: AsRef<Path>,
              U: AsRef<Path>
    {
        if let Some(meta) = std::fs::metadata(templates.as_ref()).ok() {
            if meta.is_dir() {
                self.file_updated = FileUpdatedHook::load(service_group, &hooks, &templates);
                self.health_check = HealthCheckHook::load(service_group, &hooks, &templates);
                self.suitability = SuitabilityHook::load(service_group, &hooks, &templates);
                self.init = InitHook::load(service_group, &hooks, &templates);
                self.reload = ReloadHook::load(service_group, &hooks, &templates);
                self.reconfigure = ReconfigureHook::load(service_group, &hooks, &templates);
                self.run = RunHook::load(service_group, &hooks, &templates);
                self.post_run = PostRunHook::load(service_group, &hooks, &templates);
                self.smoke_test = SmokeTestHook::load(service_group, &hooks, &templates);
            }
        }
        debug!("{}, Hooks loaded, destination={}, templates={}",
               service_group,
               hooks.as_ref().display(),
               templates.as_ref().display());
        self
    }

    fn compile_one<H>(&self, hook: &H, service_group: &ServiceGroup, config: &ServiceConfig)
        where H: Hook
    {
        hook.compile(config)
            .unwrap_or_else(|e| {
                                outputln!(preamble service_group,
                "Failed to compile {} hook: {}", H::file_name(), e);
                            });
    }
}

pub struct RenderPair {
    pub path: PathBuf,
    pub template: Template,
}

impl RenderPair {
    pub fn new<C, T>(concrete_path: C, template_path: T) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>
    {
        let mut template = Template::new();
        template
            .register_template_file("hook", template_path.as_ref())?;
        Ok(RenderPair {
               path: concrete_path.into(),
               template: template,
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
        where S: Serializer
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
                    stdout_log
                        .write_fmt(format_args!("{}\n", l))
                        .expect("couldn't write line");
                }
            }
        }
        if let Some(ref mut stderr) = process.stderr {
            for line in BufReader::new(stderr).lines() {
                if let Some(ref l) = line.ok() {
                    outputln!(preamble preamble_str, l);
                    stderr_log
                        .write_fmt(format_args!("{}\n", l))
                        .expect("couldn't write line");
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


    fn hook_fixtures_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("hooks")
    }

    #[test]
    fn hook_output() {
        let tmp_dir = TempDir::new("habitat_hooks_test").expect("create temp dir");
        let logs_dir = tmp_dir.path().join("logs");
        DirBuilder::new()
            .recursive(true)
            .create(logs_dir)
            .expect("couldn't create logs dir");
        let mut cmd = Command::new(hook_fixtures_path().join(InitHook::file_name()));
        cmd.stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd.spawn().expect("couldn't run hook");
        let stdout_log = tmp_dir
            .path()
            .join("logs")
            .join(format!("{}.stdout.log", InitHook::file_name()));
        let stderr_log = tmp_dir
            .path()
            .join("logs")
            .join(format!("{}.stderr.log", InitHook::file_name()));
        let mut hook_output = HookOutput::new(&stdout_log, &stderr_log);
        let service_group =
            ServiceGroup::new("dummy", "service", None).expect("couldn't create ServiceGroup");

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
