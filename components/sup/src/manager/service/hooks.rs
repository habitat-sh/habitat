use super::health;
use habitat_common::{outputln,
                     templating::{hooks::{self,
                                          ExitCode,
                                          Hook,
                                          HookOutput,
                                          RenderPair},
                                  package::Pkg,
                                  TemplateRenderer}};
#[cfg(windows)]
use habitat_core::os::process::windows_child::ExitStatus;
use serde::Serialize;
#[cfg(not(windows))]
use std::process::ExitStatus;
use std::{self,
          io::prelude::*,
          path::{Path,
                 PathBuf},
          sync::Arc};

static LOGKEY: &'static str = "HK";

#[derive(Debug, Serialize)]
pub struct FileUpdatedHook {
    render_pair:     RenderPair,
    stdout_log_path: PathBuf,
    stderr_log_path: PathBuf,
}

impl Hook for FileUpdatedHook {
    type ExitValue = bool;

    fn file_name() -> &'static str { "file-updated" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        FileUpdatedHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, _: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
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
}

impl Hook for HealthCheckHook {
    type ExitValue = health::HealthCheckResult;

    fn file_name() -> &'static str { "health-check" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        HealthCheckHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => health::HealthCheckResult::Ok,
            Some(1) => health::HealthCheckResult::Warning,
            Some(2) => health::HealthCheckResult::Critical,
            Some(3) => health::HealthCheckResult::Unknown,
            Some(code) => {
                outputln!(preamble pkg_name,
                    "Health check exited with an unknown status code, {}", code);
                health::HealthCheckResult::default()
            }
            None => {
                Self::output_termination_message(pkg_name, status);
                health::HealthCheckResult::default()
            }
        }
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

    fn file_name() -> &'static str { "init" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        InitHook { render_pair:     pair,
                   stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                   stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => true,
            Some(code) => {
                outputln!(preamble pkg_name, "Initialization failed! '{}' exited with \
                    status code {}", Self::file_name(), code);
                false
            }
            None => {
                outputln!(preamble pkg_name, "Initialization failed! '{}' exited without a \
                    status code", Self::file_name());
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

    fn file_name() -> &'static str { "run" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        RunHook { render_pair:     pair,
                  stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                  stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn run<T>(&self, _: &str, _: &Pkg, _: Option<T>) -> Self::ExitValue
        where T: ToString
    {
        panic!("The run hook is a an exception to the lifetime of a service. It should only be \
                run by the Supervisor module!");
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
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

    fn file_name() -> &'static str { "post-run" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        PostRunHook { render_pair:     pair,
                      stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                      stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
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

    fn file_name() -> &'static str { "reload" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        ReloadHook { render_pair:     pair,
                     stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                     stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => ExitCode(0),
            Some(code) => {
                outputln!(preamble pkg_name, "Reload failed! '{}' exited with \
                    status code {}", Self::file_name(), code);
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

    fn file_name() -> &'static str { "reconfigure" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        ReconfigureHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
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

impl Hook for SuitabilityHook {
    type ExitValue = Option<u64>;

    fn file_name() -> &'static str { "suitability" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        SuitabilityHook { render_pair:     pair,
                          stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                          stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self,
                       pkg: &Pkg,
                       hook_output: &'a HookOutput,
                       status: ExitStatus)
                       -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => {
                if let Some(reader) = hook_output.stdout() {
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
                                        outputln!(preamble pkg_name,
                                            "Parsing suitability failed: {}", err);
                                    }
                                };
                            }
                            Err(err) => {
                                outputln!(preamble pkg_name,
                                    "Failed to read last line of stdout: {}", err);
                            }
                        };
                    } else {
                        outputln!(preamble pkg_name,
                                  "{} did not print anything to stdout", Self::file_name());
                    }
                }
            }
            Some(code) => {
                outputln!(preamble pkg_name,
                    "{} exited with status code {}", Self::file_name(), code);
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

    fn file_name() -> &'static str { "post-stop" }

    fn new(package_name: &str, pair: RenderPair) -> Self {
        PostStopHook { render_pair:     pair,
                       stdout_log_path: hooks::stdout_log_path::<Self>(package_name),
                       stderr_log_path: hooks::stderr_log_path::<Self>(package_name), }
    }

    fn handle_exit<'a>(&self, pkg: &Pkg, _: &'a HookOutput, status: ExitStatus) -> Self::ExitValue {
        let pkg_name = &pkg.name;
        match status.code() {
            Some(0) => true,
            Some(code) => {
                outputln!(preamble pkg_name, "Post stop failed! '{}' exited with \
                    status code {}", Self::file_name(), code);
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

    pub fn run_changed(&self) -> bool { self.run }

    pub fn post_run_changed(&self) -> bool { self.post_run }

    pub fn changed(&self) -> bool {
        match self {
            Self { health_check,
                   init,
                   file_updated,
                   reload,
                   reconfigure,
                   suitability,
                   run,
                   post_run,
                   post_stop, } => {
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
    }
}

// Hooks wrapped in Arcs represent a possibly-temporary state while we
// refactor hooks to be able to run asynchronously.
#[derive(Debug, Default, Serialize)]
pub struct HookTable {
    pub health_check: Option<Arc<HealthCheckHook>>,
    pub init:         Option<InitHook>,
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
    pub fn load<P, T>(package_name: &str, templates: T, hooks_path: P) -> Self
        where P: AsRef<Path>,
              T: AsRef<Path>
    {
        let mut table = HookTable::default();
        if let Ok(meta) = std::fs::metadata(templates.as_ref()) {
            if meta.is_dir() {
                table.file_updated = FileUpdatedHook::load(package_name, &hooks_path, &templates);
                table.health_check =
                    HealthCheckHook::load(package_name, &hooks_path, &templates).map(Arc::new);
                table.suitability = SuitabilityHook::load(package_name, &hooks_path, &templates);
                table.init = InitHook::load(package_name, &hooks_path, &templates);
                table.reload = ReloadHook::load(package_name, &hooks_path, &templates);
                table.reconfigure = ReconfigureHook::load(package_name, &hooks_path, &templates);
                table.run = RunHook::load(package_name, &hooks_path, &templates);
                table.post_run =
                    PostRunHook::load(package_name, &hooks_path, &templates).map(Arc::new);
                table.post_stop =
                    PostStopHook::load(package_name, &hooks_path, &templates).map(Arc::new);
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
            changed.init = self.compile_one(hook, service_group, ctx);
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
                          "Failed to compile {} hook: {}", H::file_name(), e);
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
    use habitat_common::{cli::FS_ROOT,
                         templating::{config::Cfg,
                                      package::Pkg,
                                      test_helpers::*},
                         types::{GossipListenAddr,
                                 HttpListenAddr,
                                 ListenCtlAddr}};
    use habitat_core::{fs::cache_key_path,
                       package::{PackageIdent,
                                 PackageInstall},
                       service::{ServiceBind,
                                 ServiceGroup}};
    use std::{fs,
              iter};
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
        ServiceGroup::new(None, "test_service", "test_group", None).expect("couldn't create \
                                                                            ServiceGroup")
    }

    ////////////////////////////////////////////////////////////////////////

    #[test]
    fn compile_hook_table() {
        let tmp_root = rendered_hooks_path();
        let hooks_path = tmp_root.path().join("hooks");
        fs::create_dir_all(&hooks_path).unwrap();

        let service_group = service_group();

        let concrete_path = hooks_path.clone(); // rendered_hooks_path();
        let template_path = hook_templates_path();

        ////////////////////////////////////////////////////////////////////////
        // BEGIN RENDER CONTEXT SETUP
        // (See comment above)

        let sys = Sys::new(true,
                           GossipListenAddr::default(),
                           ListenCtlAddr::default(),
                           HttpListenAddr::default());

        let pg_id = PackageIdent::new("testing",
                                      &service_group.service(),
                                      Some("1.0.0"),
                                      Some("20170712000000"));

        let pkg_install = PackageInstall::new_from_parts(pg_id.clone(),
                                                         PathBuf::from("/tmp"),
                                                         PathBuf::from("/tmp"),
                                                         PathBuf::from("/tmp"));
        let pkg = Pkg::from_install(&pkg_install).expect("Could not create package!");

        // This is gross, but it actually works
        let cfg_path = &concrete_path.as_path().join("default.toml");
        create_with_content(cfg_path, "message = \"Hello\"");

        let cfg = Cfg::new(&pkg, Some(&concrete_path.as_path().to_path_buf()))
            .expect("Could not create config");

        // SysInfo is basic Swim infrastructure information
        let mut sys_info = SysInfo::default();
        sys_info.ip = "1.2.3.4".to_string();
        sys_info.hostname = "hostname".to_string();
        sys_info.gossip_ip = "0.0.0.0".to_string();
        sys_info.gossip_port = 7777;
        sys_info.http_gateway_ip = "0.0.0.0".to_string();
        sys_info.http_gateway_port = 9631;

        let sg_one = service_group.clone(); // ServiceGroup::new("shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one = ServiceRumor::new("member-a", &pg_id, sg_one.clone(), sys_info, None);
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

        let mut ring = CensusRing::new("member-a");
        ring.update_from_rumors_rsr_mlr(&cache_key_path(Some(&*FS_ROOT)),
                                        &service_store,
                                        &election_store,
                                        &election_update_store,
                                        &member_list,
                                        &service_config_store,
                                        &service_file_store);

        let bindings = iter::empty::<&ServiceBind>();

        let ctx = RenderContext::new(&service_group, &sys, &pkg, &cfg, &ring, bindings);

        // END RENDER CONTEXT SETUP
        ////////////////////////////////////////////////////////////////////////

        let hook_table = HookTable::load(&service_group, &template_path, &hooks_path);
        assert!(hook_table.compile(&service_group, &ctx).changed());

        // Verify init hook
        let init_hook_content = file_content(&hook_table.init.as_ref().expect("no init hook??"));
        let expected_init_hook = "#!/bin/bash\n\necho \"The message is Hello\"\n";
        let expected_run_hook = "#!/bin/bash\n\necho \"Running a program\"\n";
        assert_eq!(init_hook_content, expected_init_hook);

        // Verify run hook
        let run_hook_content = file_content(&hook_table.run.as_ref().expect("no run hook??"));
        assert_eq!(run_hook_content, expected_run_hook);

        // Recompiling again results in no changes
        assert!(!hook_table.compile(&service_group, &ctx).changed());

        // Re-Verify init hook
        let init_hook_content = file_content(&hook_table.init.as_ref().expect("no init hook??"));
        assert_eq!(init_hook_content, expected_init_hook);

        // Re-Verify run hook
        let run_hook_content = file_content(&hook_table.run.as_ref().expect("no run hook??"));
        assert_eq!(run_hook_content, expected_run_hook);
    }
}
