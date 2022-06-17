use crate::error::{ConnectError,
                   IPCCommandError,
                   IPCReadError,
                   ReceiveError,
                   SendError,
                   TryIPCCommandError,
                   TryReceiveError};
use habitat_common::types::UserInfo;
use habitat_core::os::process::Pid;
use habitat_launcher_protocol as protocol;
use ipc_channel::ipc::{IpcError,
                       IpcOneShotServer,
                       IpcReceiver,
                       IpcSender,
                       TryRecvError};
use log::{debug,
          error,
          trace,
          warn};
use std::{collections::BTreeMap,
          path::Path,
          thread,
          time::{Duration,
                 Instant}};

type Env = BTreeMap<String, String>;
type IpcServer = IpcOneShotServer<Vec<u8>>;

// Defines how long to wait to receive a reply from the Launcher.
//
// Initially used for calls to get the PID from a service as a way to
// deal with older Launchers that don't know about that protocol
// message, and don't reply back on unknown messages.
//
// Don't override this value unless you know what you're doing.
habitat_core::env_config_duration!(LauncherInteractionTimeout,
                                   HAB_LAUNCHER_INTERACTION_TIMEOUT_MS => from_millis,
                                   Duration::from_millis(1000));

pub enum LauncherStatus {
    Running,
    GracefullyShutdown,
    Shutdown,
    Unknown,
}

pub struct LauncherCli {
    tx:      IpcSender<Vec<u8>>,
    rx:      IpcReceiver<Vec<u8>>,
    /// Maximum wait time for interactions that can timeout.
    timeout: Duration,
}

impl LauncherCli {
    pub fn connect(pipe_to_launcher: String) -> Result<Self, ConnectError> {
        // Estabish a connection to the launcher's IPC server
        debug!("LauncherCli::connect({})", pipe_to_launcher);
        let tx = IpcSender::connect(pipe_to_launcher).map_err(ConnectError::LauncherUnreachable)?;
        // Start a IPC server to listen for responses from the launcher
        let (ipc_srv, pipe_to_sup) = IpcServer::new().map_err(ConnectError::IPCServerStartup)?;
        debug!("IpcServer::new() returned pipe_to_sup: {}", pipe_to_sup);
        // Register the supervisor with the launcher by sending a register command
        let cmd = protocol::Register { pipe: pipe_to_sup };
        Self::send(&tx, &cmd).map_err(ConnectError::LauncherRegisterSend)?;
        // Accpet the incoming connection from the launcher and read the response
        let (rx, raw) = ipc_srv.accept()
                               .map_err(ConnectError::IPCIncomingConnection)?;
        Self::read::<protocol::NetOk>(&raw).map_err(ConnectError::LauncherRegisterReceive)?;

        let timeout = LauncherInteractionTimeout::configured_value().into();

        Ok(LauncherCli { tx, rx, timeout })
    }

    /// Read a launcher protocol message from a byte array
    fn read<T>(bytes: &[u8]) -> Result<T, IPCReadError>
        where T: protocol::LauncherMessage
    {
        let txn = protocol::NetTxn::from_bytes(bytes).map_err(IPCReadError::ProtocolDeserialize)?;
        if txn.message_id() == "NetErr" {
            let err = txn.decode::<protocol::NetErr>()
                         .map_err(IPCReadError::PayloadDeserialize)?;
            return Err(IPCReadError::LauncherCommand(err));
        }
        let msg = txn.decode::<T>()
                     .map_err(IPCReadError::PayloadDeserialize)?;
        Ok(msg)
    }

    /// Receive and read protocol message from an IpcReceiver
    fn recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<T, ReceiveError>
        where T: protocol::LauncherMessage
    {
        match rx.recv() {
            Ok(bytes) => Ok(Self::read(&bytes)?),
            Err(err) => Err(ReceiveError::IPCReceive(err)),
        }
    }

    /// EXPERIMENTAL! BEWARE! EXPERIMENTAL!
    ///
    /// When this was added, we needed a way for a Supervisor to be
    /// able to recover from sending a message to an older version of
    /// the Launcher that didn't know how to process that message. At
    /// the time, the Launcher would just ignore the message, but we
    /// always use a `recv` to wait for the response, so the
    /// Supervisor would hang.
    ///
    /// This function is an attempt to get around this situation, but
    /// ONLY for the specific scenarios where it is appropriate. I am
    /// hesitant to introduce a global timeout at this point, because
    /// it's difficult to know how that will affect the overall
    /// interactions between the Supervisor and the Launcher (it
    /// *should* be fine, but I can't guarantee that right now).
    ///
    /// As such, use this with caution and intention.
    fn recv_timeout<T>(rx: &IpcReceiver<Vec<u8>>, timeout: Duration) -> Result<T, TryReceiveError>
        where T: protocol::LauncherMessage
    {
        // If ipc_channel implemented this directly, we wouldn't have
        // to do this :(
        let start_time = Instant::now();
        loop {
            match rx.try_recv() {
                Ok(bytes) => {
                    let msg = Self::read(&bytes).map_err(TryReceiveError::IPCRead)?;
                    return Ok(msg);
                }
                Err(TryRecvError::Empty) => {
                    trace!("try_recv would block; waiting 5ms");
                    thread::sleep(Duration::from_millis(5));
                }
                Err(TryRecvError::IpcError(err)) => {
                    return Err(TryReceiveError::IPCReceive(err));
                }
            }
            if start_time.elapsed() > timeout {
                return Err(TryReceiveError::Timeout);
            }
        }
    }

    /// Send a command to a Launcher
    fn send<T>(tx: &IpcSender<Vec<u8>>, message: &T) -> Result<(), SendError>
        where T: protocol::LauncherMessage
    {
        let txn = protocol::NetTxn::build(message).map_err(SendError::PayloadSerialize)?;
        let bytes = txn.to_bytes().map_err(SendError::ProtocolSerialize)?;
        tx.send(bytes).map_err(SendError::IPCSend)?;
        Ok(())
    }

    /// Receive and read protocol message from an IpcReceiver
    fn try_recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<Option<T>, ReceiveError>
        where T: protocol::LauncherMessage
    {
        match rx.try_recv() {
            Ok(bytes) => {
                let msg = Self::read::<T>(&bytes)?;
                Ok(Some(msg))
            }
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::IpcError(err)) => Err(ReceiveError::IPCReceive(err)),
        }
    }

    pub fn launcher_status(&self) -> LauncherStatus {
        match Self::try_recv::<protocol::Shutdown>(&self.rx) {
            // We haven't received any command to shutdown
            Ok(None) => LauncherStatus::Running,
            // Received a shutdown command
            Ok(Some(_)) => LauncherStatus::GracefullyShutdown,
            // Launcher IPC channel was disconnected
            Err(ReceiveError::IPCReceive(IpcError::Disconnected)) => LauncherStatus::Shutdown,
            // Received a bad message, or encountered an IO error while communicating via IPC
            Err(err) => {
                error!("Unexpected IPC communication error while checking for a shutdown \
                        request: {}",
                       err);
                LauncherStatus::Unknown
            }
        }
    }

    /// Restart a running process with the same arguments
    pub fn restart(&self, pid: Pid) -> Result<Pid, IPCCommandError> {
        let msg = protocol::Restart { pid: pid.into() };
        Self::send(&self.tx, &msg).map_err(|err| IPCCommandError::Send("restart", err))?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx).map_err(|err| {
                                                                 IPCCommandError::Receive("restart",
                                                                                          err)
                                                             })?;
        Ok(reply.pid as Pid)
    }

    /// Send a process spawn command to the connected Launcher
    ///
    /// `username` and `groupname` are string names, while `uid` and
    /// `gid` are numeric IDs. Newer versions of the Launcher can
    /// accept either, but prefer numeric IDs.
    pub fn spawn(&self,
                 id: &str,
                 bin: &Path,
                 UserInfo { username,
                            uid,
                            groupname,
                            gid, }: UserInfo,
                 password: Option<&str>,
                 env: Env)
                 -> Result<Pid, IPCCommandError> {
        // On Windows, we only expect user to be Some.
        //
        // On Linux, we expect uid and gid to be Some, while
        // user and groupname may be either Some or None. Only the IDs are
        // used; names are only for backward compatibility with older
        // Launchers.
        let msg = protocol::Spawn { binary: bin.to_string_lossy().into_owned(),
                                    svc_user: username,
                                    svc_group: groupname,
                                    svc_user_id: uid,
                                    svc_group_id: gid,
                                    svc_password: password.map(str::to_string),
                                    env,
                                    id: id.to_string() };

        Self::send(&self.tx, &msg).map_err(|err| IPCCommandError::Send("spawn", err))?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx).map_err(|err| {
                                                                 IPCCommandError::Receive("spawn",
                                                                                          err)
                                                             })?;
        if reply.pid == 0 {
            warn!(target: "pidfile_tracing", "Spawn operation for {} resulted in a spawned PID of 0, which \
                   should be impossible! (proceeding anyway)",
                  id);
        }
        Ok(reply.pid as Pid)
    }

    /// Query the launcher for the PID of the named service. If the
    /// Launcher is aware of it, you'll get `Ok(Some(Pid))`
    pub fn pid_of(&self, service_name: &str) -> Result<Option<Pid>, TryIPCCommandError> {
        let msg = protocol::PidOf { service_name: service_name.to_string(), };
        Self::send(&self.tx, &msg).map_err(|err| TryIPCCommandError::Send("pid_of", err))?;
        // This should be a recv_timeout until pidfile-less
        // supervisors are the norm. We only expect to not receive a
        // response when dealing with older Launchers that didn't know
        // how to return PIDs.
        let reply = Self::recv_timeout::<protocol::PidIs>(&self.rx, self.timeout).map_err(|err| TryIPCCommandError::TryReceive("pid_of", err))?;
        // TODO (CM): really, we need to have all our protocol types
        // that use pids actually use a Pid type that's nonzero, with
        // lots of descriptive errors for failures.
        match reply.pid {
            Some(pid) => Ok(Some(pid as Pid)),
            None => Ok(None),
        }
    }

    /// Query the launcher for its version. If the
    /// Launcher is aware of it, you'll get `Ok(u32)`
    pub fn version(&self) -> Result<u32, TryIPCCommandError> {
        let msg = protocol::Version {};
        Self::send(&self.tx, &msg).map_err(|err| TryIPCCommandError::Send("version", err))?;

        // We only expect to not receive a response when dealing with
        // older Launchers that didn't know how to return its version.
        let reply = Self::recv_timeout::<protocol::VersionNumber>(&self.rx, self.timeout).map_err(|err| TryIPCCommandError::TryReceive("version", err))?;
        Ok(reply.version)
    }

    pub fn terminate(&self, pid: Pid) -> Result<i32, IPCCommandError> {
        let msg = protocol::Terminate { pid: pid.into() };
        Self::send(&self.tx, &msg).map_err(|err| IPCCommandError::Send("terminate", err))?;
        let reply =
            Self::recv::<protocol::TerminateOk>(&self.rx).map_err(|err| {
                                                             IPCCommandError::Receive("terminate",
                                                                                      err)
                                                         })?;
        Ok(reply.exit_code)
    }
}
