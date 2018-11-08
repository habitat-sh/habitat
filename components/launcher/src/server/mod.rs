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

mod handlers;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(unix)]
use std::process::ExitStatus;

use core;
use core::fs::{launcher_root_path, FS_ROOT_PATH};
use core::os::process::{self, Pid, Signal};
use core::os::signals::{self, SignalEvent};
use core::package::{PackageIdent, PackageInstall};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
#[cfg(unix)]
use libc;
use protobuf;
use protocol::{self, ERR_NO_RETRY_EXCODE, OK_NO_RETRY_EXCODE};
use semver::{Version, VersionReq};

use self::handlers::Handler;
use error::{Error, Result};
use service::Service;
use {SUP_CMD, SUP_PACKAGE_IDENT};

const IPC_CONNECT_TIMEOUT_SECS: &'static str = "HAB_LAUNCH_SUP_CONNECT_TIMEOUT_SECS";
const DEFAULT_IPC_CONNECT_TIMEOUT_SECS: u64 = 5;
const SUP_CMD_ENVVAR: &'static str = "HAB_SUP_BINARY";
static LOGKEY: &'static str = "SV";

const SUP_VERSION_CHECK_DISABLE: &'static str = "HAB_LAUNCH_NO_SUP_VERSION_CHECK";
// Version 0.56 is somewhat arbitrary. This functionality is for when we make
// changes to the launcher that depend on supervisor behavior that hasn't
// always existed such as https://github.com/habitat-sh/habitat/issues/5380
const SUP_VERSION_REQ: &'static str = ">= 0.56";

type Receiver = IpcReceiver<Vec<u8>>;
type Sender = IpcSender<Vec<u8>>;

enum TickState {
    Continue,
    Exit(i32),
}

pub struct Server {
    pid_file_path: PathBuf,
    services: ServiceTable,
    tx: Sender,
    rx: Receiver,
    pipe: String,
    supervisor: Child,
    args: Vec<String>,
}

impl Drop for Server {
    fn drop(&mut self) {
        fs::remove_file(&self.pid_file_path).ok();
        self.remove_pipe();
    }
}

impl Server {
    pub fn new(args: Vec<String>) -> Result<Self> {
        let launcher_root = launcher_root_path(Some(&*core::fs::FS_ROOT_PATH));
        fs::create_dir_all(&launcher_root)?;
        let pid_file_path = launcher_root.join("PID");
        let mut pid_file = fs::File::create(&pid_file_path)?;
        write!(&mut pid_file, "{}", process::current_pid())?;

        let ((rx, tx), supervisor, pipe) = Self::init(&args, false)?;
        Ok(Server {
            pid_file_path: pid_file_path,
            services: ServiceTable::default(),
            tx: tx,
            rx: rx,
            pipe: pipe,
            supervisor: supervisor,
            args: args,
        })
    }

    /// Spawn a Supervisor and setup a bi-directional IPC connection to it.
    ///
    /// Passing a value of true to the `clean` argument will force the Supervisor to clean the
    /// Launcher's process LOCK before starting. This is useful when restarting a Supervisor
    /// that terminated gracefully.
    fn init(args: &[String], clean: bool) -> Result<((Receiver, Sender), Child, String)> {
        let (server, pipe) = IpcOneShotServer::new().map_err(Error::OpenPipe)?;
        let supervisor = spawn_supervisor(&pipe, args, clean)?;
        let channel = setup_connection(server)?;
        Ok((channel, supervisor, pipe))
    }

    fn remove_pipe(&self) {
        if fs::remove_file(&self.pipe).is_err() {
            error!("Could not remove old pipe to supervisor {}", self.pipe);
        } else {
            debug!("Removed old pipe to supervisor {}", self.pipe);
        }
    }

    #[allow(unused_must_use)]
    fn reload(&mut self) -> Result<()> {
        self.supervisor.kill();
        self.supervisor.wait();
        let ((rx, tx), supervisor, pipe) = Self::init(&self.args, true)?;
        self.tx = tx;
        self.rx = rx;
        self.supervisor = supervisor;
        // We're connecting to a new supervisor instance, so we need to remove
        // the socket files for the old pipe to avoid https://github.com/habitat-sh/habitat/issues/4673
        self.remove_pipe();
        self.pipe = pipe;
        Ok(())
    }

    fn forward_signal(&self, signal: Signal) {
        if let Err(err) = core::os::process::signal(self.supervisor.id() as Pid, signal) {
            error!(
                "Unable to signal Supervisor, {}, {}",
                self.supervisor.id(),
                err
            );
        }
    }

    fn handle_message(&mut self) -> Result<TickState> {
        match self.rx.try_recv() {
            Ok(bytes) => {
                dispatch(&self.tx, &bytes, &mut self.services);
                Ok(TickState::Continue)
            }
            Err(_) => {
                match self.supervisor.try_wait() {
                    Ok(None) => Ok(TickState::Continue),
                    Ok(Some(status)) => {
                        // Supervisor exited
                        self.handle_supervisor_exit(status.code())
                    }
                    Err(err) => {
                        warn!("Unable to wait for Supervisor, {}", err);
                        Err(Error::SupShutdown)
                    }
                }
            }
        }
    }

    /// Given that a Supervisor process has exited with a specific
    /// exit code, figure out whether we need to restart it or not.
    // TODO (CM): Consider pulling the status checks into this as
    // well, accepting an ExitStatus instead of Option<i32>
    fn handle_supervisor_exit(&mut self, code: Option<i32>) -> Result<TickState> {
        debug!("launcher::server::handle_supervisor_exit(code: {:?})", code);
        match code {
            Some(ERR_NO_RETRY_EXCODE) => {
                self.services.kill_all();
                Ok(TickState::Exit(ERR_NO_RETRY_EXCODE))
            }
            Some(OK_NO_RETRY_EXCODE) => {
                self.services.kill_all();
                Ok(TickState::Exit(0))
            }
            Some(_) => Err(Error::SupShutdown),
            None => {
                // TODO (CM): kill services?
                Err(Error::SupShutdown)
            }
        }
    }

    fn reap_services(&mut self) {
        self.services.reap_services()
    }

    fn shutdown(&mut self) {
        debug!("Shutting down...");
        if send(&self.tx, &protocol::Shutdown::new()).is_err() {
            warn!("Forcefully stopping Supervisor: {}", self.supervisor.id());
            if let Err(err) = self.supervisor.kill() {
                warn!(
                    "Unable to kill Supervisor, {}, {}",
                    self.supervisor.id(),
                    err
                );
            }
        }
        self.supervisor.wait().ok();
        self.services.kill_all();
        outputln!("Hasta la vista, services.");
    }

    fn tick(&mut self) -> Result<TickState> {
        // TODO (CM): Yes, we have `reap_services` as well as
        // `reap_zombie_orphans`... perhaps they need different
        // names. However, this is a distinction that might be nice to
        // collapse in the future.
        //
        // `reap_services` is a cross-platform method to reap (and keep
        // track of) processes that are Habitat
        // services. `reap_zombie_orphans` is basically a Unix-only
        // method to take care of any orphan processes that get
        // re-parented to the Launcher, when it is running as PID 1,
        // when their parents end before they do.
        //
        // There is some natural overlap between the two on Unix
        // platforms that would be nice to collapse, but it needs to
        // be done in a way that the basic functionality of process
        // tracking still works on Windows.
        self.reap_services();
        match signals::check_for_signal() {
            Some(SignalEvent::Shutdown) => {
                self.shutdown();
                return Ok(TickState::Exit(0));
            }
            Some(SignalEvent::WaitForChild) => {
                // We only return Some if we ended up reaping our
                // Supervisor; otherwise, we don't need to do anything
                // special. If the supervisor exits but reap_zombie_orphans()
                // doesn't catch the signal (such as on Windows), we will still
                // handle that properly in handle_message().
                if let Some(result) = self.reap_zombie_orphans() {
                    return result;
                }
            }
            Some(SignalEvent::Passthrough(signal)) => {
                self.forward_signal(signal);
            }
            None => (),
        }
        self.handle_message()
    }

    /// When the supervisor runs as the init process (e.g. in a
    /// container), it will become the parent of any processes whose
    /// parents terminate before they do (as is standard on Linux). We
    /// need to call `waitpid` on these children to prevent a zombie
    /// horde from ultimately bringing down the system.
    ///
    /// Note that we are not (yet?) doing anything with
    /// `prctl(PR_SET_CHILD_SUBREAPER, ...)` to make the Launcher a
    /// subreaper; this behavior currently handles the case when the
    /// Launcher is running as PID 1.
    ///
    /// (See http://man7.org/linux/man-pages/man2/prctl.2.html for
    /// further information.)
    #[cfg(unix)]
    fn reap_zombie_orphans(&mut self) -> Option<Result<TickState>> {
        // Record the disposition of the Supervisor if it is a child
        // process being reaped; our ultimate response is dependent on
        // this.
        let mut reaped_sup_status: Option<ExitStatus> = None;
        let mut waitpid_status = 0 as libc::c_int;

        // We reap as many child processes as need reaping.
        loop {
            // We're not calling waitpid with WUNTRACED or WCONTINUED,
            // so we shouldn't be getting SIGCHLD from STOP or CONT
            // signals sent to a Supervisor; only when the Supervisor
            // process ends somehow.
            let res = unsafe { libc::waitpid(-1, &mut waitpid_status, libc::WNOHANG) };
            if res > 0 {
                // Some child process ended; let's see if it was the Supervisor
                if res == self.supervisor.id() as libc::pid_t {
                    debug!("Reaped supervisor process, PID {}", res);
                    // Note: from_raw is a Unix-only call
                    reaped_sup_status = Some(ExitStatus::from_raw(waitpid_status));
                } else {
                    debug!("Reaped a non-supervisor child process, PID {}", res);
                }
            } else if res == 0 {
                // There are no more children waiting
                break;
            } else {
                warn!("Error waiting for child process: {}", res);
                break;
            }
        }

        // If we reaped our supervisor, then we return a TickState so
        // we can figure out whether or restart or not.
        //
        // If we just reaped non-supervisor processes, though, we
        // return `None` to indicate there's nothing special that
        // needs to happen.
        if let Some(status) = reaped_sup_status {
            // A Supervisor process ended; it either ended normally
            // with an exit code, or it was terminated by a signal
            // that it couldn't otherwise handle.
            //
            // In the latter case, we treat it as though the
            // Supervisor shut down, but we will not restart.
            if let Some(exit_code) = status.code() {
                debug!("Supervisor exit status: {}", exit_code);
                Some(self.handle_supervisor_exit(Some(exit_code)))
            } else if let Some(signal) = status.signal() {
                // If you TERM or INT the Supervisor (currently), the
                // Supervisor does not otherwise catch the signal. The
                // previous Launcher implementation ultimately shut
                // down in this scenario, but by accident, and through
                // at least one restart/stop cycle of the
                // Supervisor. This just makes the behavior explicit;
                // it can be revisited later.
                outputln!(
                    "Supervisor process killed by signal {}; shutting everything down now",
                    signal
                );
                Some(self.handle_supervisor_exit(Some(ERR_NO_RETRY_EXCODE)))
            } else {
                // We should never get here; a Linux process either
                // exits with a status code, or it was killed with a
                // signal.
                warn!("UNEXPECTED RESULT: Supervisor process ended, but neither exit status nor terminating signal are available");
                Some(self.handle_supervisor_exit(None))
            }
        } else {
            // A supervisor didn't end; carry on your merry way
            None
        }
    }

    /// Windows doesn't have the same orphan-reaping behavior as Linux;
    /// returning `None` means that there's nothing special that needs
    /// to be done.
    #[cfg(windows)]
    fn reap_zombie_orphans(&mut self) -> Option<Result<TickState>> {
        None
    }
}

#[derive(Debug, Default)]
pub struct ServiceTable(HashMap<u32, Service>);

impl ServiceTable {
    pub fn get(&self, pid: u32) -> Option<&Service> {
        self.0.get(&pid)
    }

    pub fn get_mut(&mut self, pid: u32) -> Option<&mut Service> {
        self.0.get_mut(&pid)
    }

    pub fn insert(&mut self, service: Service) {
        self.0.insert(service.id(), service);
    }

    pub fn remove(&mut self, pid: u32) -> Option<Service> {
        self.0.remove(&pid)
    }

    fn kill_all(&mut self) {
        for service in self.0.values_mut() {
            outputln!(preamble service.name(), "Stopping...");
            let shutdown_method = service.kill();
            outputln!(preamble service.name(), "Shutdown OK: {}", shutdown_method);
        }
    }

    fn reap_services(&mut self) {
        let mut dead: Vec<u32> = vec![];
        for service in self.0.values_mut() {
            match service.try_wait() {
                Ok(None) => (),
                Ok(Some(code)) => {
                    outputln!(
                        "Child for service '{}' with PID {} exited with code {}",
                        service.name(),
                        service.id(),
                        code
                    );
                    dead.push(service.id());
                }
                Err(err) => {
                    warn!("Error waiting for child, {}, {}", service.id(), err);
                    dead.push(service.id());
                }
            }
        }
        for pid in dead {
            self.0.remove(&pid);
        }
    }
}

////////////////////////
// Public Func
//

pub fn reply<T>(tx: &Sender, txn: &protocol::NetTxn, msg: &T) -> Result<()>
where
    T: protobuf::MessageStatic,
{
    let bytes = txn
        .build_reply(msg)
        .map_err(Error::Serialize)?
        .to_bytes()
        .map_err(Error::Serialize)?;
    tx.send(bytes).map_err(Error::Send)?;
    Ok(())
}

pub fn run(args: Vec<String>) -> Result<i32> {
    let mut server = Server::new(args)?;
    signals::init();
    loop {
        match server.tick() {
            Ok(TickState::Continue) => thread::sleep(Duration::from_millis(100)),
            Ok(TickState::Exit(code)) => {
                return Ok(code);
            }
            Err(_) => while server.reload().is_err() {
                thread::sleep(Duration::from_millis(1_000));
            },
        }
    }
}

pub fn send<T>(tx: &Sender, msg: &T) -> Result<()>
where
    T: protobuf::MessageStatic,
{
    let bytes = protocol::NetTxn::build(msg)
        .map_err(Error::Serialize)?
        .to_bytes()
        .map_err(Error::Serialize)?;
    tx.send(bytes).map_err(Error::Send)?;
    Ok(())
}

////////////////////////
// Private Func
//

fn dispatch(tx: &Sender, bytes: &[u8], services: &mut ServiceTable) {
    let msg = match protocol::NetTxn::from_bytes(bytes) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Unable to decode NetTxn from Supervisor, {}", err);
            return;
        }
    };
    let func = match msg.message_id() {
        "Restart" => handlers::RestartHandler::run,
        "Spawn" => handlers::SpawnHandler::run,
        "Terminate" => handlers::TerminateHandler::run,
        unknown => {
            warn!("Received unknown message from Supervisor, {}", unknown);
            return;
        }
    };
    func(tx, msg, services);
}

fn setup_connection(server: IpcOneShotServer<Vec<u8>>) -> Result<(Receiver, Sender)> {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Set up the connection in a separate thread because ipc-channel doesn't support timeouts
    let handle = thread::spawn(move || {
        {
            let (ref lock, _) = *pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            debug!("connect thread started");
        }
        let (rx, raw) = server.accept().map_err(|_| Error::AcceptConn)?;
        let txn = protocol::NetTxn::from_bytes(&raw).map_err(Error::Deserialize)?;
        let mut msg = txn
            .decode::<protocol::Register>()
            .map_err(Error::Deserialize)?;
        let tx = IpcSender::connect(msg.take_pipe()).map_err(Error::Connect)?;
        send(&tx, &protocol::NetOk::new())?;
        {
            let (_, ref cvar) = *pair2;
            debug!("Connect thread finished; notifying waiting thread");
            cvar.notify_one();
        }
        Ok((rx, tx))
    });

    let (ref lock, ref cvar) = *pair;
    let timeout_secs = core::env::var(IPC_CONNECT_TIMEOUT_SECS)
        .unwrap_or("".to_string())
        .parse()
        .unwrap_or(DEFAULT_IPC_CONNECT_TIMEOUT_SECS);

    debug!("Waiting on connect thread for {} secs", timeout_secs);
    let (started, wait_result) = cvar
        .wait_timeout(
            lock.lock().expect("IPC connection startup lock poisoned"),
            Duration::from_secs(timeout_secs),
        ).expect("IPC connection startup lock poisoned");

    if *started && !wait_result.timed_out() {
        handle.join().unwrap()
    } else {
        debug!(
            "Timeout exceeded waiting for IPC connection (started: {})",
            *started
        );
        Err(Error::AcceptConn)
    }
}

/// Return whether the given version string matches SUP_VERSION_REQ parsed as
/// a semver::VersionReq.
///
/// Example inputs (that is `hab-sup --version` outputs):
/// hab-sup 0.59.0/20180712161546
/// hab-sup 0.62.0-dev
fn is_supported_supervisor_version(version_output: String) -> bool {
    if let Some(version_str) = version_output
        .split(' ') // ["hab-sup", <version-number>]
        .last() // drop "hab-sup", keep <version-number>
        .unwrap() // split() always returns an 1+ element iterator
        .split(|c| c == '/' || c == '-') // strip "-dev" or "/build"
        .next()
    {
        debug!(
            "Checking Supervisor version '{}' against requirement '{}'",
            version_str, SUP_VERSION_REQ
        );
        match Version::parse(version_str) {
            Ok(version) => VersionReq::parse(SUP_VERSION_REQ)
                .expect("invalid semantic version requirement")
                .matches(&version),
            Err(e) => {
                error!("{}: {}", e, version_str);
                false
            }
        }
    } else {
        error!(
            "Expected 'hab-sup <semantic-version>', found '{}'",
            version_output
        );
        false
    }
}

/// Start a Supervisor as a child process.
///
/// Passing a value of true to the `clean` argument will force the Supervisor to clean the
/// Launcher's process LOCK before starting. This is useful when restarting a Supervisor
/// that terminated gracefully.
fn spawn_supervisor(pipe: &str, args: &[String], clean: bool) -> Result<Child> {
    let binary = supervisor_cmd()?;

    if core::env::var(SUP_VERSION_CHECK_DISABLE).is_ok() {
        warn!("Launching Supervisor {:?} without version checking", binary);
    } else {
        debug!("Checking Supervisor {:?} version", binary);
        let version_check = Command::new(&binary).arg("--version").output()?;
        let sup_version = String::from_utf8_lossy(&version_check.stdout);
        if !is_supported_supervisor_version(sup_version.trim().to_string()) {
            error!("This Launcher requires Habitat version {}", SUP_VERSION_REQ);
            error!("This check can be disabled by setting the {} environment variable to a non-empty string when starting the supervisor", SUP_VERSION_CHECK_DISABLE);
            error!("Disabling this check may result in undefined behavior; please update to a newer Habitat version");
            error!("For more information see https://github.com/habitat-sh/habitat/pull/5484");
            return Err(Error::SupBinaryVersion);
        }
    }

    let mut command = Command::new(&binary);
    if clean {
        command.env(protocol::LAUNCHER_LOCK_CLEAN_ENV, clean.to_string());
    }
    debug!(
        "Starting Supervisor {:?} with args {:?}, {}={}...",
        binary,
        args,
        protocol::LAUNCHER_PIPE_ENV,
        pipe
    );
    let child = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env(protocol::LAUNCHER_PIPE_ENV, pipe)
        .env(
            protocol::LAUNCHER_PID_ENV,
            process::current_pid().to_string(),
        ).args(args)
        .spawn()
        .map_err(Error::SupSpawn)?;
    Ok(child)
}

/// Determines the most viable Supervisor binary to run and returns a `PathBuf` to it.
///
/// Setting a filepath value to the `HAB_SUP_BINARY` env variable will force that binary to be used
/// instead.
fn supervisor_cmd() -> Result<PathBuf> {
    if let Ok(command) = core::env::var(SUP_CMD_ENVVAR) {
        return Ok(PathBuf::from(command));
    }
    let ident = PackageIdent::from_str(SUP_PACKAGE_IDENT).unwrap();
    let fs_root_path = FS_ROOT_PATH.as_path();
    match PackageInstall::load_at_least(&ident, Some(fs_root_path)) {
        Ok(install) => match core::fs::find_command_in_pkg(SUP_CMD, &install, fs_root_path) {
            Ok(Some(cmd)) => Ok(cmd),
            _ => Err(Error::SupBinaryNotFound),
        },
        Err(_) => Err(Error::SupPackageNotFound),
    }
}
