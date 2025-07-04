#[cfg(windows)]
use super::pipe_hook_client::PipeHookClient;
use habitat_common::{error::Result,
                     outputln,
                     templating::{hooks::{self,
                                          ExitCode,
                                          Hook,
                                          HookOutput,
                                          RenderPair},
                                  package::Pkg,
                                  TemplateRenderer},
                     FeatureFlag};
#[cfg(windows)]
use habitat_core::os::process::windows_child::ExitStatus;
use log::debug;
use serde::Serialize;
#[cfg(not(windows))]
use std::process::ExitStatus;
use std::{self,
          io::BufRead,
          path::{Path,
                 PathBuf},
          sync::Arc};

static LOGKEY: &str = "HK";

#[derive(Debug, Default)]
pub struct StandardStreams {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

#[derive(Debug)]
pub struct ProcessOutput {
    standard_streams: StandardStreams,
    exit_status:      ExitStatus,
}

impl ProcessOutput {
    fn new(hook_output: &HookOutput, exit_status: ExitStatus) -> Self {
        Self { standard_streams: StandardStreams { stdout: hook_output.stdout_str().ok(),
                                                   stderr: hook_output.stderr_str().ok(), },
               exit_status }
    }

    pub fn from_raw(standard_streams: StandardStreams, exit_status: ExitStatus) -> Self {
        Self { standard_streams,
               exit_status }
    }

    pub fn exit_status(&self) -> ExitStatus { self.exit_status }

    pub fn standard_streams(self) -> StandardStreams { self.standard_streams }
}

#[derive(Debug, Serialize)]
pub struct FileUpdatedHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for FileUpdatedHook {
    type ExitValue = bool;

    const FILE_NAME: &'static str = "file-updated";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        FileUpdatedHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self, _: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        status.success()
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct HealthCheckHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
    #[cfg(windows)]
    #[serde(skip_serializing)]
    pipe_client:     Option<PipeHookClient>,
}

#[cfg(windows)]
impl HealthCheckHook {
    fn pipe_client(path: PathBuf,
                   feature_flags: FeatureFlag,
                   out_path: PathBuf,
                   err_path: PathBuf)
                   -> Option<PipeHookClient> {
        if feature_flags.contains(FeatureFlag::NO_NAMED_PIPE_HEALTH_CHECK) {
            None
        } else {
            Some(PipeHookClient::new(Self::FILE_NAME.to_string(), path, out_path, err_path))
        }
    }
}

impl Hook for HealthCheckHook {
    type ExitValue = ProcessOutput;

    const FILE_NAME: &'static str = "health-check";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        #[cfg(windows)]
        let feature_flags = _feature_flags;
        let out_path = hooks::stdout_log_path::<Self>(package_name);
        let err_path = hooks::stderr_log_path::<Self>(package_name);
        #[cfg(windows)]
        let path = pair.path.clone();
        HealthCheckHook { render_pair:                 pair,
                          #[cfg(windows)]
                          pipe_client:                 Self::pipe_client(path,
                                                                         feature_flags,
                                                                         out_path.clone(),
                                                                         err_path.clone()),
                          stdout_log_path:             out_path,
                          stderr_log_path:             err_path, }
    }

    #[cfg(windows)]
    fn run<T>(&self,
              service_group: &str,
              pkg: &Pkg,
              svc_encrypted_password: Option<T>)
              -> Result<Self::ExitValue>
        where T: ToString
    {
        if let Some(client) = &self.pipe_client {
            match client.exec_hook(service_group, pkg, svc_encrypted_password) {
                Ok(exit) => {
                    let hook_output =
                        HookOutput::new(self.stdout_log_path(), self.stderr_log_path());
                    Ok(self.handle_exit(pkg, &hook_output, ExitStatus::from(exit)))
                }
                Err(err) => {
                    outputln!(preamble service_group,
                        "Hook failed to run, {}, {}", Self::FILE_NAME, err);
                    Err(err)
                }
            }
        } else {
            self.run_impl(service_group, pkg, svc_encrypted_password)
        }
    }

    fn handle_exit(&self,
                   pkg: &Pkg,
                   hook_output: &HookOutput,
                   status: ExitStatus)
                   -> Self::ExitValue {
        if status.code().is_none() {
            Self::output_termination_message(&pkg.name, status);
        }
        ProcessOutput::new(hook_output, status)
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct InitHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for InitHook {
    type ExitValue = bool;

    const FILE_NAME: &'static str = "init";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        InitHook { render_pair:     pair,
                   stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                   stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self, pkg: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => true,
            Some(code) => {
                outputln!(preamble pkg_name, "Initialization failed! '{}' exited with \
                    status code {}", Self::FILE_NAME, code);
                false
            }
            None => {
                outputln!(preamble pkg_name, "Initialization failed! '{}' exited without a \
                    status code", Self::FILE_NAME);
                false
            }
        }
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct RunHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for RunHook {
    type ExitValue = ExitCode;

    const FILE_NAME: &'static str = "run";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        RunHook { render_pair:     pair,
                  stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                  stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn run<T>(&self, _: &str, _: &Pkg, _: Option<T>) -> Result<Self::ExitValue>
        where T: ToString
    {
        panic!("The run hook is a an exception to the lifetime of a service. It should only be \
                run by the Supervisor module!");
    }

    fn handle_exit(&self, pkg: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        match status.code() {
            Some(code) => ExitCode(code),
            None => {
                Self::output_termination_message(&pkg.name, status);
                ExitCode::default()
            }
        }
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct PostRunHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for PostRunHook {
    type ExitValue = ExitCode;

    const FILE_NAME: &'static str = "post-run";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        PostRunHook { render_pair:     pair,
                      stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                      stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self, pkg: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        match status.code() {
            Some(code) => ExitCode(code),
            None => {
                Self::output_termination_message(&pkg.name, status);
                ExitCode::default()
            }
        }
    }

    fn should_retry(exit_value: &Self::ExitValue) -> bool {
        const SHOULD_NOT_RETRY: ExitCode = ExitCode(0);
        exit_value != &SHOULD_NOT_RETRY
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

/// This hook is deprecated and will be removed in a future release.
#[derive(Debug, Serialize)]
pub struct ReloadHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for ReloadHook {
    type ExitValue = ExitCode;

    const FILE_NAME: &'static str = "reload";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        ReloadHook { render_pair:     pair,
                     stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                     stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self, pkg: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => ExitCode(0),
            Some(code) => {
                outputln!(preamble pkg_name, "Reload failed! '{}' exited with \
                    status code {}", Self::FILE_NAME, code);
                ExitCode(code)
            }
            None => {
                Self::output_termination_message(pkg_name, status);
                ExitCode::default()
            }
        }
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct ReconfigureHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for ReconfigureHook {
    type ExitValue = ExitCode;

    const FILE_NAME: &'static str = "reconfigure";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        ReconfigureHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self, pkg: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        match status.code() {
            Some(code) => ExitCode(code),
            None => {
                Self::output_termination_message(&pkg.name, status);
                ExitCode::default()
            }
        }
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct SuitabilityHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl SuitabilityHook {
    fn parse_suitability(reader: impl BufRead, pkg_name: &str) -> Option<u64> {
        if let Some(line_reader) = reader.lines().last() {
            match line_reader {
                Ok(line) => {
                    match line.trim().parse::<u64>() {
                        Ok(suitability) => {
                            outputln!(preamble pkg_name,
                                      "Reporting suitability of: {}", suitability);
                            return Some(suitability);
                        }
                        Err(err) => {
                            outputln!(preamble pkg_name, "Parsing suitability failed: {}", err);
                        }
                    };
                }
                Err(err) => {
                    outputln!(preamble pkg_name, "Failed to read last line of stdout: {}", err);
                }
            };
        } else {
            outputln!(preamble pkg_name, "{} did not print anything to stdout", Self::FILE_NAME);
        }
        None
    }
}

impl Hook for SuitabilityHook {
    type ExitValue = Option<u64>;

    const FILE_NAME: &'static str = "suitability";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        SuitabilityHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self,
                   pkg: &Pkg,
                   hook_output: &HookOutput,
                   status: ExitStatus)
                   -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => {
                match hook_output.stdout() {
                    Ok(reader) => {
                        return Self::parse_suitability(reader, pkg_name);
                    }
                    Err(e) => {
                        outputln!(preamble pkg_name,
                                  "Failed to open stdout file: {}", e);
                    }
                }
            }
            Some(code) => {
                outputln!(preamble pkg_name,
                          "{} exited with status code {}", Self::FILE_NAME, code);
            }
            None => {
                Self::output_termination_message(pkg_name, status);
            }
        }
        None
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

#[derive(Debug, Serialize)]
pub struct PostStopHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for PostStopHook {
    type ExitValue = bool;

    const FILE_NAME: &'static str = "post-stop";

    fn new(package_name: &str, pair: RenderPair, _feature_flags: FeatureFlag) -> Self {
        PostStopHook { render_pair:     pair,
                       stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                       stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit(&self, pkg: &Pkg, _: &HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => true,
            Some(code) => {
                outputln!(preamble pkg_name, "Post stop failed! '{}' exited with \
                    status code {}", Self::FILE_NAME, code);
                false
            }
            None => {
                Self::output_termination_message(pkg_name, status);
                false
            }
        }
    }

    fn path(&self) -> &Path { &self.render_pair.path }

    fn renderer(&self) -> &TemplateRenderer { &self.render_pair.renderer }

    fn stdout_log_path(&self) -> &Path { &self.stdout_log_path }

    fn stderr_log_path(&self) -> &Path { &self.stderr_log_path }
}

/// A lookup of hooks that have changed after compilation.
#[derive(Default)]
pub struct HookCompileTable {
    health_check: bool,
    init:         bool,
    file_updated: bool,
    reload:       bool,
    reconfigure:  bool,
    suitability:  bool,
    run:          bool,
    post_run:     bool,
    post_stop:    bool,
}

impl HookCompileTable {
    pub fn new() -> Self { Self::default() }

    pub fn reload_changed(&self) -> bool { self.reload }

    pub fn reconfigure_changed(&self) -> bool { self.reconfigure }

    pub fn init_changed(&self) -> bool { self.init }

    pub fn run_changed(&self) -> bool { self.run }

    pub fn post_run_changed(&self) -> bool { self.post_run }

    pub fn changed(&self) -> bool {
        let Self { health_check,
                   init,
                   file_updated,
                   reload,
                   reconfigure,
                   suitability,
                   run,
                   post_run,
                   post_stop, } = self;
        *health_check
        || *init
        || *file_updated
        || *reload
        || *reconfigure
        || *suitability
        || *run
        || *post_run
        || *post_stop
    }
}

// Queryable representation of a hook
#[derive(Debug, Clone, Serialize)]
pub struct HookQueryModel {
    pub render_pair:     PathBuf,
    pub stdout_log_path: PathBuf,
    pub stderr_log_path: PathBuf,
}

// Queryable representation of all hooks of a service
#[derive(Debug, Clone, Serialize)]
pub struct HookTableQueryModel {
    pub health_check: Option<HookQueryModel>,
    pub init:         Option<HookQueryModel>,
    pub file_updated: Option<HookQueryModel>,
    pub reload:       Option<HookQueryModel>,
    pub reconfigure:  Option<HookQueryModel>,
    pub suitability:  Option<HookQueryModel>,
    pub run:          Option<HookQueryModel>,
    pub post_run:     Option<HookQueryModel>,
    pub post_stop:    Option<HookQueryModel>,
}

impl HookTableQueryModel {
    pub fn new(hook_table: &HookTable) -> HookTableQueryModel {
        HookTableQueryModel {
            health_check: hook_table.health_check.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            init: hook_table.init.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            file_updated: hook_table.file_updated.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            reload: hook_table.reload.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            reconfigure: hook_table.reconfigure.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            suitability: hook_table.suitability.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            run: hook_table.run.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            post_run: hook_table.post_run.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() }),
            post_stop: hook_table.post_stop.as_ref().map(|hook| HookQueryModel { render_pair: hook.render_pair.path.clone(), stdout_log_path: hook.stdout_log_path.clone(), stderr_log_path: hook.stderr_log_path.clone() })
        }
    }
}

// Hooks wrapped in Arcs represent a possibly-temporary state while we
// refactor hooks to be able to run asynchronously.
#[derive(Debug, Default, Serialize)]
pub struct HookTable {
    pub health_check: Option<Arc<HealthCheckHook>>,
    pub init:         Option<Arc<InitHook>>,
    pub file_updated: Option<FileUpdatedHook>,
    pub reload:       Option<ReloadHook>,
    pub reconfigure:  Option<ReconfigureHook>,
    pub suitability:  Option<SuitabilityHook>,
    pub run:          Option<RunHook>,
    pub post_run:     Option<Arc<PostRunHook>>,
    pub post_stop:    Option<Arc<PostStopHook>>,
}

impl HookTable {
    /// Read all available hook templates from the table's package directory into the table.
    pub fn load<P, T>(package_name: &str,
                      templates: T,
                      hooks_path: P,
                      feature_flags: FeatureFlag)
                      -> Self
        where P: AsRef<Path>,
              T: AsRef<Path>
    {
        let mut table = HookTable::default();
        if let Ok(meta) = std::fs::metadata(templates.as_ref()) {
            if meta.is_dir() {
                table.file_updated =
                    FileUpdatedHook::load(package_name, &hooks_path, &templates, feature_flags);
                table.health_check = HealthCheckHook::load(package_name,
                                                           &hooks_path,
                                                           &templates,
                                                           feature_flags).map(Arc::new);
                table.suitability =
                    SuitabilityHook::load(package_name, &hooks_path, &templates, feature_flags);
                table.init = InitHook::load(package_name, &hooks_path, &templates, feature_flags).map(Arc::new);
                table.reload =
                    ReloadHook::load(package_name, &hooks_path, &templates, feature_flags);
                table.reconfigure =
                    ReconfigureHook::load(package_name, &hooks_path, &templates, feature_flags);
                table.run = RunHook::load(package_name, &hooks_path, &templates, feature_flags);
                table.post_run = PostRunHook::load(package_name,
                                                   &hooks_path,
                                                   &templates,
                                                   feature_flags).map(Arc::new);
                table.post_stop = PostStopHook::load(package_name,
                                                     &hooks_path,
                                                     &templates,
                                                     feature_flags).map(Arc::new);
            }
        }
        debug!("{}, Hooks loaded, destination={}, templates={}",
               package_name,
               hooks_path.as_ref().display(),
               templates.as_ref().display());
        table
    }

    /// Compile all loaded hooks from the table into their destination service directory.
    pub fn compile<T>(&self, service_group: &str, ctx: &T) -> HookCompileTable
        where T: Serialize
    {
        debug!("{:?}", self);
        let mut changed = HookCompileTable::new();
        if let Some(ref hook) = self.file_updated {
            changed.file_updated = self.compile_one(hook, service_group, ctx);
        }
        if let Some(ref hook) = self.health_check {
            changed.health_check = self.compile_one(hook.as_ref(), service_group, ctx);
        }
        if let Some(ref hook) = self.init {
            changed.init = self.compile_one(hook.as_ref(), service_group, ctx);
        }
        if let Some(ref hook) = self.reload {
            changed.reload |= self.compile_one(hook, service_group, ctx);
        }
        if let Some(ref hook) = self.reconfigure {
            changed.reconfigure = self.compile_one(hook, service_group, ctx);
        }
        if let Some(ref hook) = self.suitability {
            changed.suitability = self.compile_one(hook, service_group, ctx);
        }
        if let Some(ref hook) = self.run {
            changed.run = self.compile_one(hook, service_group, ctx);
        }
        if let Some(ref hook) = self.post_run {
            changed.post_run = self.compile_one(hook.as_ref(), service_group, ctx);
        }
        if let Some(ref hook) = self.post_stop {
            changed.post_stop = self.compile_one(hook.as_ref(), service_group, ctx);
        }
        changed
    }

    fn compile_one<H, T>(&self, hook: &H, service_group: &str, ctx: &T) -> bool
        where H: Hook,
              T: Serialize
    {
        match hook.compile(service_group, ctx) {
            Ok(status) => status,
            Err(e) => {
                outputln!(preamble service_group,
                          "Failed to compile {} hook: {}", H::FILE_NAME, e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::RenderContext,
                *};
    use crate::{census::CensusRing,
                manager::sys::Sys};
    use habitat_butterfly::{member::MemberList,
                            rumor::{election::{self,
                                               Election as ElectionRumor,
                                               ElectionUpdate as ElectionUpdateRumor},
                                    service::{Service as ServiceRumor,
                                              SysInfo},
                                    service_config::ServiceConfig as ServiceConfigRumor,
                                    service_file::ServiceFile as ServiceFileRumor,
                                    RumorStore}};
    use habitat_common::{templating::{config::Cfg,
                                      package::Pkg,
                                      test_helpers::*},
                         types::{GossipListenAddr,
                                 HttpListenAddr,
                                 ListenCtlAddr}};
    #[cfg(not(any(all(target_os = "linux",
                          any(target_arch = "x86_64", target_arch = "aarch64")),
                      all(target_os = "windows", target_arch = "x86_64"),)))]
    use habitat_core::package::metadata::MetaFile;
    use habitat_core::{crypto::keys::KeyCache,
                       fs::CACHE_KEY_PATH,
                       locked_env_var,
                       package::{PackageIdent,
                                 PackageInstall},
                       service::{ServiceBind,
                                 ServiceGroup}};
    use std::{convert,
              fs,
              io::BufReader,
              iter,
              net::{IpAddr,
                    Ipv4Addr}};
    use tempfile::TempDir;

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
                      ReloadHook
                      ReconfigureHook
                      RunHook
                      SuitabilityHook
                      PostStopHook);

    fn hook_templates_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                 .join("fixtures")
                                                 .join("hooks")
                                                 .join("hook_templates")
    }

    fn rendered_hooks_path() -> TempDir { TempDir::new().expect("create temp dir") }

    fn service_group() -> ServiceGroup {
        ServiceGroup::new("test_service", "test_group", None).expect("couldn't create ServiceGroup")
    }

    async fn pkg(service_group: &ServiceGroup) -> Pkg {
        let pg_id = PackageIdent::new("testing",
                                      service_group.service(),
                                      Some("1.0.0"),
                                      Some("20170712000000"));
        let pkg_install = PackageInstall::new_from_parts(pg_id,
                                                         PathBuf::from("/tmp"),
                                                         PathBuf::from("/tmp"),
                                                         PathBuf::from("/tmp"));
        // Platforms without standard package support require all packages to be native packages
        #[cfg(not(any(all(target_os = "linux",
                          any(target_arch = "x86_64", target_arch = "aarch64")),
                      all(target_os = "windows", target_arch = "x86_64"))))]
        {
            tokio::fs::create_dir_all(pkg_install.installed_path()).await
                                                                   .unwrap();
            create_with_content(pkg_install.installed_path()
                                           .join(MetaFile::PackageType.to_string()),
                                "native");
        }
        Pkg::from_install(&pkg_install).await.unwrap()
    }

    fn ctx<'a>(service_group: &'a ServiceGroup,
               pkg: &'a Pkg,
               sys: &'a Sys,
               cfg: &'a Cfg,
               ring: &'a mut CensusRing)
               -> RenderContext<'a> {
        // SysInfo is basic Swim infrastructure information
        let sys_info = SysInfo { ip: "1.2.3.4".to_string(),
                                 hostname: "hostname".to_string(),
                                 gossip_ip: "0.0.0.0".to_string(),
                                 gossip_port: 7777,
                                 http_gateway_ip: "0.0.0.0".to_string(),
                                 http_gateway_port: 9631,
                                 ..Default::default() };

        let sg_one = service_group.clone(); // ServiceGroup::new("shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one = ServiceRumor::new("member-a", &pkg.ident, sg_one.clone(), sys_info, None);
        service_store.insert_rsw(service_one);

        let election_store: RumorStore<ElectionRumor> = RumorStore::default();
        let mut election = ElectionRumor::new("member-a",
                                              &sg_one,
                                              election::Term::default(),
                                              10,
                                              true /* has_quorum */);
        election.finish();
        election_store.insert_rsw(election);

        let election_update_store: RumorStore<ElectionUpdateRumor> = RumorStore::default();

        let member_list = MemberList::new();

        let service_config_store: RumorStore<ServiceConfigRumor> = RumorStore::default();
        let service_file_store: RumorStore<ServiceFileRumor> = RumorStore::default();

        ring.update_from_rumors_rsr_mlr(&KeyCache::new(&*CACHE_KEY_PATH),
                                        &service_store,
                                        &election_store,
                                        &election_update_store,
                                        &member_list,
                                        &service_config_store,
                                        &service_file_store);

        let bindings = iter::empty::<&ServiceBind>();

        RenderContext::new(service_group, sys, pkg, cfg, ring, bindings)
    }

    ////////////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn compile_hook_table() {
        let tmp_root = rendered_hooks_path();
        let hooks_path = tmp_root.path().join("hooks");
        fs::create_dir_all(&hooks_path).unwrap();
        let service_group = service_group();
        let concrete_path = hooks_path.clone(); // rendered_hooks_path();
        let template_path = hook_templates_path();

        // This is gross, but it actually works
        let cfg_path = &concrete_path.as_path().join("default.toml");
        create_with_content(cfg_path, "message = \"Hello\"");

        let pkg = pkg(&service_group).await;
        let sys = Sys::new(true,
                           GossipListenAddr::default(),
                           ListenCtlAddr::default(),
                           HttpListenAddr::default(),
                           IpAddr::V4(Ipv4Addr::LOCALHOST));
        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_path().to_path_buf()))
            .expect("Could not create config");
        let mut ring = CensusRing::new("member-a");
        let ctx = ctx(&service_group, &pkg, &sys, &cfg, &mut ring);

        let hook_table = HookTable::load(&service_group,
                                         &template_path,
                                         &hooks_path,
                                         FeatureFlag::empty());
        assert!(hook_table.compile(&service_group, &ctx).changed());

        // Verify init hook
        let init_hook_content = file_content(hook_table.init
                                                       .as_ref()
                                                       .map(convert::AsRef::as_ref)
                                                       .expect("no init hook??"));
        let init_hook_content_normalized = init_hook_content.replace('\r', "");
        let expected_init_hook = "#!/bin/bash\n\necho \"The message is Hello\"\n";
        let expected_run_hook = "#!/bin/bash\n\necho \"Running a program\"\n";
        assert_eq!(init_hook_content_normalized, expected_init_hook);

        // Verify run hook
        let run_hook_content = file_content(hook_table.run.as_ref().expect("no run hook??"));
        let run_hook_content_normalized = run_hook_content.replace('\r', "");
        assert_eq!(run_hook_content_normalized, expected_run_hook);

        // Recompiling again results in no changes
        assert!(!hook_table.compile(&service_group, &ctx).changed());

        // Re-Verify init hook
        let init_hook_content = file_content(hook_table.init
                                                       .as_ref()
                                                       .map(convert::AsRef::as_ref)
                                                       .expect("no init hook??"));
        let init_hook_content_normalized = init_hook_content.replace('\r', "");
        assert_eq!(init_hook_content_normalized, expected_init_hook);

        // Re-Verify run hook
        let run_hook_content = file_content(hook_table.run.as_ref().expect("no run hook??"));
        let run_hook_content_normalized = run_hook_content.replace('\r', "");
        assert_eq!(run_hook_content_normalized, expected_run_hook);
    }

    #[test]
    fn parse_suitability() {
        #[allow(clippy::string_lit_as_bytes)]
        let reader = BufReader::new("".as_bytes());
        let result = SuitabilityHook::parse_suitability(reader, "test_pkg_name");
        assert!(result.is_none());
        #[allow(clippy::string_lit_as_bytes)]
        let reader = BufReader::new("test\nanother\ninvalid".as_bytes());
        let result = SuitabilityHook::parse_suitability(reader, "test_pkg_name");
        assert!(result.is_none());
        #[allow(clippy::string_lit_as_bytes)]
        #[allow(clippy::string_lit_as_bytes)]
        let reader = BufReader::new("3".as_bytes());
        let result = SuitabilityHook::parse_suitability(reader, "test_pkg_name");
        assert_eq!(result.unwrap(), 3);
        #[allow(clippy::string_lit_as_bytes)]
        let reader = BufReader::new("test\nanother\n124".as_bytes());
        let result = SuitabilityHook::parse_suitability(reader, "test_pkg_name");
        assert_eq!(result.unwrap(), 124);
    }

    locked_env_var!(HAB_HOOK_PIPE_SCRIPT, pipe_service_path);

    #[cfg(windows)]
    #[tokio::test]
    async fn run_named_pipe_health_check_hook() {
        use habitat_core::fs::svc_logs_path;

        let var = pipe_service_path();
        let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
                                                              .join("named_pipe_service.ps1");
        var.set(&script);

        let service_group = service_group();
        let tmp_root = rendered_hooks_path().keep();
        let hooks_path = tmp_root.clone().join("hooks");
        fs::create_dir_all(&hooks_path).unwrap();
        fs::create_dir_all(svc_logs_path(service_group.service())).unwrap();
        let concrete_path = hooks_path.clone();
        let template_path = hook_templates_path();

        let hook = HealthCheckHook::load(service_group.service(),
                                         &concrete_path,
                                         &template_path,
                                         FeatureFlag::empty()).expect("Could not create testing \
                                                                       healch-check hook");

        let pkg = pkg(&service_group).await;
        let sys = Sys::new(true,
                           GossipListenAddr::default(),
                           ListenCtlAddr::default(),
                           HttpListenAddr::default(),
                           IpAddr::V4(Ipv4Addr::LOCALHOST));
        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_path().to_path_buf()))
            .expect("Could not create config");
        let mut ring = CensusRing::new("member-a");
        let ctx = ctx(&service_group, &pkg, &sys, &cfg, &mut ring);

        hook.compile(&service_group, &ctx).unwrap();

        let result = hook.run(&service_group, &pkg, None::<&str>).unwrap();

        assert_eq!(Some(1), result.exit_status().code());
        assert!(result.standard_streams()
                      .stdout
                      .unwrap()
                      .contains("Named pipe created"));
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn do_not_run_named_pipe_health_check_hook_under_feature_flag() {
        use habitat_core::fs::svc_logs_path;

        let var = pipe_service_path();
        let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
                                                              .join("named_pipe_service.ps1");
        var.set(&script);

        let service_group = service_group();
        let tmp_root = rendered_hooks_path().keep();
        let hooks_path = tmp_root.clone().join("hooks");
        fs::create_dir_all(&hooks_path).unwrap();
        fs::create_dir_all(svc_logs_path(service_group.service())).unwrap();
        let concrete_path = hooks_path.clone();
        let template_path = hook_templates_path();
        let mut flags = FeatureFlag::empty();
        flags.insert(FeatureFlag::NO_NAMED_PIPE_HEALTH_CHECK);

        let hook = HealthCheckHook::load(service_group.service(),
                                         &concrete_path,
                                         &template_path,
                                         flags).expect("Could not create testing healch-check \
                                                        hook");

        let pkg = pkg(&service_group).await;
        let sys = Sys::new(true,
                           GossipListenAddr::default(),
                           ListenCtlAddr::default(),
                           HttpListenAddr::default(),
                           IpAddr::V4(Ipv4Addr::LOCALHOST));
        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_path().to_path_buf()))
            .expect("Could not create config");
        let mut ring = CensusRing::new("member-a");
        let ctx = ctx(&service_group, &pkg, &sys, &cfg, &mut ring);

        hook.compile(&service_group, &ctx).unwrap();

        let result = hook.run(&service_group, &pkg, None::<&str>).unwrap();

        assert_eq!(Some(1), result.exit_status().code());
        assert!(!result.standard_streams()
                       .stdout
                       .unwrap()
                       .contains("Named pipe created"));
    }
}
