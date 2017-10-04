// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};

use hab_core::env;
use hab_core::fs as hfs;
use hab_core::os::process::{self, Signal};

use error::Result;
use runner::log_pipe::LogPipe;
use runner::{NONINTERACTIVE_ENVVAR, RUNNER_DEBUG_ENVVAR};
use runner::workspace::Workspace;

lazy_static! {
    /// Absolute path to the Docker exporter program
    static ref DOCKER_EXPORTER_PROGRAM: PathBuf = hfs::resolve_cmd_in_pkg(
        "hab-pkg-export-docker",
        include_str!(concat!(env!("OUT_DIR"), "/DOCKER_EXPORTER_PKG_IDENT")),
    );

    /// Absolute path to the Dockerd program
    static ref DOCKERD_PROGRAM: PathBuf = hfs::resolve_cmd_in_pkg(
        "dockerd",
        include_str!(concat!(env!("OUT_DIR"), "/DOCKER_PKG_IDENT")),
    );
}

const DOCKER_HOST_ENVVAR: &'static str = "DOCKER_HOST";

pub struct DockerExporterSpec {
    pub username: String,
    pub password: String,
    pub docker_hub_repo_name: String,
    pub latest_tag: bool,
    pub version_tag: bool,
    pub version_release_tag: bool,
    pub custom_tag: Option<String>,
}

pub struct DockerExporter<'a> {
    spec: DockerExporterSpec,
    workspace: &'a Workspace,
    bldr_url: &'a str,
}

impl<'a> DockerExporter<'a> {
    /// Creates a new Docker exporter for a given `Workspace` and Builder URL.
    pub fn new(spec: DockerExporterSpec, workspace: &'a Workspace, bldr_url: &'a str) -> Self {
        DockerExporter {
            spec,
            workspace,
            bldr_url,
        }
    }

    /// Spawns a Docker export command, pipes output streams to the given `LogPipe` and returns the
    /// process' `ExitStatus`.
    ///
    /// # Errors
    ///
    /// * If the child process can't be spawned
    /// * If the calling thread can't wait on the child process
    /// * If the `LogPipe` fails to pipe output
    pub fn export(&self, log_pipe: &mut LogPipe) -> Result<ExitStatus> {
        let dockerd = self.spawn_dockerd()?;
        let exit_status = self.run_export(log_pipe);
        self.teardown_dockerd(dockerd).err().map(|e| {
            error!("failed to teardown dockerd instance, err={:?}", e)
        });
        exit_status
    }

    fn run_export(&self, log_pipe: &mut LogPipe) -> Result<ExitStatus> {
        let sock = self.dockerd_sock();

        let mut cmd = Command::new(&*DOCKER_EXPORTER_PROGRAM);
        cmd.current_dir(self.workspace.root());
        cmd.arg("--image-name");
        cmd.arg(&self.spec.docker_hub_repo_name);
        cmd.arg("--base-pkgs-url");
        cmd.arg(&self.bldr_url);
        cmd.arg("--url");
        cmd.arg(&self.bldr_url);
        if self.spec.latest_tag {
            cmd.arg("--tag-latest");
        }
        if self.spec.version_tag {
            cmd.arg("--tag-version");
        }
        if self.spec.version_release_tag {
            cmd.arg("--tag-version-release");
        }
        if let Some(ref custom_tag) = self.spec.custom_tag {
            cmd.arg("--tag-custom");
            cmd.arg(custom_tag);
        }
        cmd.arg("--push-image");
        cmd.arg("--username");
        cmd.arg(&self.spec.username);
        cmd.arg("--password");
        cmd.arg(&self.spec.password);
        cmd.arg("--rm-image");
        cmd.arg(self.workspace.last_built()?.path); // Locally built artifact
        debug!("building docker export command, cmd={:?}", &cmd);

        cmd.env_clear();
        if let Some(_) = env::var_os(RUNNER_DEBUG_ENVVAR) {
            cmd.env("RUST_LOG", "debug");
        }
        cmd.env(NONINTERACTIVE_ENVVAR, "true"); // Disables progress bars
        cmd.env("TERM", "xterm-256color"); // Emits ANSI color codes
        debug!(
            "setting docker export command env, {}={}",
            DOCKER_HOST_ENVVAR,
            sock
        );
        cmd.env(DOCKER_HOST_ENVVAR, sock); // Use the job-specific `dockerd`
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        debug!("spawning docker export command");
        let mut child = cmd.spawn()?;
        log_pipe.pipe(&mut child)?;
        let exit_status = child.wait()?;
        debug!("completed docker export command, status={:?}", exit_status);
        Ok(exit_status)
    }

    fn spawn_dockerd(&self) -> Result<Child> {
        let root = self.dockerd_path();
        let env_path = &*DOCKERD_PROGRAM.parent().expect(
            "Dockerd parent directory exists",
        );
        let daemon_json = root.join("etc/daemon.json");
        fs::create_dir_all(daemon_json.parent().expect(
            "Daemon JSON parent directory exists",
        ))?;
        {
            let mut f = File::create(&daemon_json)?;
            f.write_all(b"{}")?;
        }

        let mut cmd = Command::new(&*DOCKERD_PROGRAM);
        if let Some(_) = env::var_os(RUNNER_DEBUG_ENVVAR) {
            cmd.arg("-D");
        }
        cmd.arg("-H");
        cmd.arg(self.dockerd_sock());
        cmd.arg("--pidfile");
        cmd.arg(root.join("var/run/docker.pid"));
        cmd.arg("--data-root");
        cmd.arg(root.join("var/lib/docker"));
        cmd.arg("--exec-root");
        cmd.arg(root.join("var/run/docker"));
        cmd.arg("--config-file");
        cmd.arg(daemon_json);
        // TODO fn: Hard-coding this feels wrong. I'd like the {{pkg.svc_run_group}} for this
        // service ideally. Probably plumb more config through for this.
        cmd.arg("--group");
        cmd.arg("hab");
        cmd.arg("--iptables=false");
        cmd.arg("--ip-masq=false");
        cmd.arg("--ipv6=false");
        cmd.arg("--raw-logs");
        debug!("building dockerd command, cmd={:?}", &cmd);
        cmd.env_clear();
        debug!(
            "setting docker export command env, PATH={}",
            env_path.display()
        );
        cmd.env("PATH", env_path); // Sadly, `dockerd` needs its collaborator programs on `PATH`
        cmd.stdout(Stdio::from(File::create(
            self.workspace.root().join("dockerd.stdout.log"),
        )?));
        cmd.stderr(Stdio::from(File::create(
            self.workspace.root().join("dockerd.stderr.log"),
        )?));

        debug!("spawning dockerd export command");
        Ok(cmd.spawn()?)
    }

    fn teardown_dockerd(&self, mut dockerd: Child) -> Result<()> {
        debug!(
            "signaling dockerd to shutdown pid={}, sig={:?}",
            dockerd.id(),
            Signal::TERM
        );
        process::signal(dockerd.id(), Signal::TERM)?;
        dockerd.wait()?;
        debug!("terminated dockerd");
        // TODO fn: clean up `self.dockerd_root()` directory
        Ok(())
    }

    fn dockerd_path(&self) -> PathBuf {
        self.workspace.root().join("dockerd")
    }

    fn dockerd_sock(&self) -> String {
        format!(
            "unix://{}",
            self.dockerd_path().join("var/run/docker.sock").display()
        )
    }
}
