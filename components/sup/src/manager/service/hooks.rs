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

use hcore;
use hcore::service::ServiceGroup;
use serde::{Serialize, Serializer};

use super::health;
use error::Result;
use manager::service::ServiceConfig;
use supervisor::RuntimeConfig;
use templating::Template;
use util;

use ansi_term::Colour;

pub const HOOK_PERMISSIONS: u32 = 0o755;
static LOGKEY: &'static str = "HK";

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

    fn load<C, T, L>(service_group: &ServiceGroup,
                     concrete_path: C,
                     template_path: T,
                     logs_path: L)
                     -> Option<Self>
        where C: AsRef<Path>,
              T: AsRef<Path>,
              L: AsRef<Path>
    {
        let concrete = concrete_path.as_ref().join(Self::file_name());
        let template = template_path.as_ref().join(Self::file_name());
        let logs_prefix = logs_path.as_ref().join(Self::file_name());
        match std::fs::metadata(&template) {
            Ok(_) => {
                match Self::new(concrete, template, logs_prefix) {
                    Ok(hook) => Some(hook),
                    Err(err) => {
                        outputln!(preamble service_group, "Failed to load hook: {}", err);
                        None
                    }
                }
            }
            Err(_) => {
                debug!("{} not found at {}, not loading",
                       Self::file_name(),
                       template.display());
                None
            }
        }
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>;

    /// Compile a hook into it's destination service directory.
    fn compile(&self, cfg: &ServiceConfig) -> Result<()> {
        let toml = try!(cfg.to_toml());
        let svc_data = util::convert::toml_to_json(toml);
        let data = try!(self.template().render("hook", &svc_data));
        let mut file = try!(std::fs::File::create(self.path()));
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
        let mut child = match util::create_command(self.path(), &cfg.svc_user, &cfg.svc_group)
                  .spawn() {
            Ok(child) => child,
            Err(err) => {
                outputln!(preamble service_group,
                    "Hook failed to run, {}, {}", Self::file_name(), err);
                return Self::ExitValue::default();
            }
        };
        let mut hook_output = HookOutput::new(self.logs_prefix());
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

    fn handle_exit(&self,
                   group: &ServiceGroup,
                   output: &HookOutput,
                   status: &ExitStatus)
                   -> Self::ExitValue;

    fn path(&self) -> &Path;

    fn template(&self) -> &Template;

    fn logs_prefix(&self) -> &Path;
}

#[derive(Debug, Serialize)]
pub struct FileUpdatedHook(RenderPair, LogsPrefix);

impl Hook for FileUpdatedHook {
    type ExitValue = bool;

    fn file_name() -> &'static str {
        "file_updated"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(FileUpdatedHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   _: &ServiceGroup,
                   _: &HookOutput,
                   status: &ExitStatus)
                   -> Self::ExitValue {
        status.success()
    }

    fn path(&self) -> &Path {
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct HealthCheckHook(RenderPair, LogsPrefix);

impl Hook for HealthCheckHook {
    type ExitValue = health::HealthCheck;

    fn file_name() -> &'static str {
        "health_check"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(HealthCheckHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   _: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct InitHook(RenderPair, LogsPrefix);

impl Hook for InitHook {
    type ExitValue = bool;

    fn file_name() -> &'static str {
        "init"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(InitHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   _: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct RunHook(RenderPair, LogsPrefix);

impl Hook for RunHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "run"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(RunHook(pair, logs_prefix.into()))
    }

    fn run(&self, _: &ServiceGroup, _: &RuntimeConfig) -> Self::ExitValue {
        panic!("The run hook is a an exception to the lifetime of a service. It should only be \
                run by the supervisor module!");
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   _: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct ReloadHook(RenderPair, LogsPrefix);

impl Hook for ReloadHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "reload"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(ReloadHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   _: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct ReconfigureHook(RenderPair, LogsPrefix);

impl Hook for ReconfigureHook {
    type ExitValue = ExitCode;

    fn file_name() -> &'static str {
        "reconfigure"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(ReconfigureHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   _: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct SmokeTestHook(RenderPair, LogsPrefix);

impl Hook for SmokeTestHook {
    type ExitValue = health::SmokeCheck;

    fn file_name() -> &'static str {
        "smoke_test"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(SmokeTestHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   _: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
    }
}

#[derive(Debug, Serialize)]
pub struct SuitabilityHook(RenderPair, LogsPrefix);

impl Hook for SuitabilityHook {
    type ExitValue = Option<u64>;

    fn file_name() -> &'static str {
        "suitability"
    }

    fn new<C, T, L>(concrete_path: C, template_path: T, logs_prefix: L) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>,
              L: Into<PathBuf>
    {
        let pair = RenderPair::new(concrete_path, template_path)?;
        Ok(SuitabilityHook(pair, logs_prefix.into()))
    }

    fn handle_exit(&self,
                   service_group: &ServiceGroup,
                   hook_output: &HookOutput,
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
        &self.0.path
    }

    fn template(&self) -> &Template {
        &self.0.template
    }

    fn logs_prefix(&self) -> &Path {
        &self.1
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
        if let Some(ref hook) = self.smoke_test {
            self.compile_one(hook, service_group, config);
        }
        debug!("{}, Hooks compiled", service_group);
    }

    /// Read all available hook templates from the table's package directory into the table.
    pub fn load_hooks<T, U, L>(mut self,
                               service_group: &ServiceGroup,
                               hooks: T,
                               templates: U,
                               logs_dir: L)
                               -> Self
        where T: AsRef<Path>,
              U: AsRef<Path>,
              L: AsRef<Path>
    {
        if let Some(meta) = std::fs::metadata(templates.as_ref()).ok() {
            if meta.is_dir() {
                self.file_updated =
                    FileUpdatedHook::load(service_group, &hooks, &templates, &logs_dir);
                self.health_check =
                    HealthCheckHook::load(service_group, &hooks, &templates, &logs_dir);
                self.suitability =
                    SuitabilityHook::load(service_group, &hooks, &templates, &logs_dir);
                self.init = InitHook::load(service_group, &hooks, &templates, &logs_dir);
                self.reload = ReloadHook::load(service_group, &hooks, &templates, &logs_dir);
                self.reconfigure =
                    ReconfigureHook::load(service_group, &hooks, &templates, &logs_dir);
                self.run = RunHook::load(service_group, &hooks, &templates, &logs_dir);
                self.smoke_test = SmokeTestHook::load(service_group, &hooks, &templates, &logs_dir);
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
        hook.compile(config).unwrap_or_else(|e| {
                                                outputln!(preamble service_group,
                "Failed to compile {} hook: {}", H::file_name(), e);
                                            });
    }
}

type LogsPrefix = PathBuf;

struct RenderPair {
    path: PathBuf,
    template: Template,
}

impl RenderPair {
    pub fn new<C, T>(concrete_path: C, template_path: T) -> Result<Self>
        where C: Into<PathBuf>,
              T: AsRef<Path>
    {
        let mut template = Template::new();
        template.register_template_file("hook", template_path.as_ref())?;
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
        serializer.serialize_str(&self.path
                                      .as_os_str()
                                      .to_string_lossy()
                                      .into_owned())
    }
}

pub struct HookOutput {
    stdout_log_file: PathBuf,
    stderr_log_file: PathBuf,
}

impl HookOutput {
    fn new<P>(log_prefix: P) -> Self
        where P: Into<PathBuf>
    {
        let mut stdout_log_file = log_prefix.into();
        let mut stderr_log_file = stdout_log_file.clone();

        stdout_log_file.set_extension("stdout.log");
        stderr_log_file.set_extension("stderr.log");

        HookOutput {
            stdout_log_file: stdout_log_file,
            stderr_log_file: stderr_log_file,
        }
    }

    fn stdout(&self) -> Option<BufReader<File>> {
        match File::open(&self.stdout_log_file) {
            Ok(f) => Some(BufReader::new(f)),
            Err(_) => None,
        }
    }

    fn stderr(&self) -> Option<BufReader<File>> {
        match File::open(&self.stderr_log_file.clone()) {
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
                    stdout_log.write_fmt(format_args!("{}\n", l)).expect("couldn't write line");
                }
            }
        }
        if let Some(ref mut stderr) = process.stderr {
            for line in BufReader::new(stderr).lines() {
                if let Some(ref l) = line.ok() {
                    outputln!(preamble preamble_str, l);
                    stderr_log.write_fmt(format_args!("{}\n", l)).expect("couldn't write line");
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
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("hooks")
    }

    #[test]
    fn hook_output() {
        let tmp_dir = TempDir::new("habitat_hooks_test").expect("create temp dir");
        let logs_dir = tmp_dir.path().join("logs");
        DirBuilder::new().recursive(true).create(logs_dir).expect("couldn't create logs dir");
        let mut cmd = Command::new(hook_fixtures_path().join(InitHook::file_name()));
        cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = cmd.spawn().expect("couldn't run hook");

        let log_path = tmp_dir.path().join("logs").join(InitHook::file_name());
        let mut hook_output = HookOutput::new(log_path);
        let service_group =
            ServiceGroup::new("dummy", "service", None).expect("couldn't create ServiceGroup");

        hook_output.stream_output::<InitHook>(&service_group, &mut child);

        let mut stdout = String::new();
        hook_output.stdout()
            .unwrap()
            .read_to_string(&mut stdout)
            .expect("couldn't read stdout");
        assert_eq!(stdout, "This is stdout\n");

        let mut stderr = String::new();
        hook_output.stderr()
            .unwrap()
            .read_to_string(&mut stderr)
            .expect("couldn't read stderr");
        assert_eq!(stderr, "This is stderr\n");

        fs::remove_dir_all(tmp_dir).expect("remove temp dir");
    }
}
