use crate::error::{Error,
                   Result};
use habitat_common::types::UserInfo;
use habitat_core::os::process::Pid;
use habitat_launcher_protocol::{self as protocol,
                                Error as ProtocolError};
use ipc_channel::ipc::{IpcOneShotServer,
                       IpcReceiver,
                       IpcSender};
use std::{collections::BTreeMap,
          io,
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
pub struct LauncherCli {
    tx: IpcSender<Vec<u8>>,
    rx: IpcReceiver<Vec<u8>>,
    // We persist the pipe identifier so we can delete the file on drop.
    // This is not necessary on Windows because named pipes are removed
    // upon releasing the last handle to the pipe. The ipc-channel crate
    // wraps the pipe in a WinHandle whose drop impl calls CloseHandle.
    #[cfg(not(windows))]
    pipe: String,

    /// Maximum wait time for interactions that can timeout.
    timeout: Duration,
}

#[cfg(not(windows))]
impl Drop for LauncherCli {
    fn drop(&mut self) {
        if std::fs::remove_file(&self.pipe).is_err() {
            error!("Could not remove old pipe to launcher {}", self.pipe);
        } else {
            debug!("Removed old pipe to launcher {}", self.pipe);
        }
    }
}

impl LauncherCli {
    pub fn connect(pipe_to_launcher: String) -> Result<Self> {
        debug!("LauncherCli::connect({})", pipe_to_launcher);
        let tx = IpcSender::connect(pipe_to_launcher).map_err(Error::Connect)?;
        let (ipc_srv, pipe_to_sup) = IpcServer::new().map_err(Error::BadPipe)?;
        debug!("IpcServer::new() returned pipe_to_sup: {}", pipe_to_sup);
        let cmd = protocol::Register { pipe: pipe_to_sup.clone(), };
        Self::send(&tx, &cmd)?;
        let (rx, raw) = ipc_srv.accept().map_err(|_| Error::AcceptConn)?;
        Self::read::<protocol::NetOk>(&raw)?;

        let timeout = LauncherInteractionTimeout::configured_value().into();

        Ok(LauncherCli { tx,
                         rx,
                         #[cfg(not(windows))]
                         pipe: pipe_to_sup,
                         timeout })
    }

    /// Read a launcher protocol message from a byte array
    fn read<T>(bytes: &[u8]) -> Result<T>
        where T: protocol::LauncherMessage
    {
        let txn = protocol::NetTxn::from_bytes(bytes)?;
        if txn.message_id() == "NetErr" {
            let err = txn.decode::<protocol::NetErr>()?;
            return Err(Error::Protocol(ProtocolError::NetErr(err)));
        }
        let msg = txn.decode::<T>()?;
        Ok(msg)
    }

    /// Receive and read protocol message from an IpcReceiver
    fn recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<T>
        where T: protocol::LauncherMessage
    {
        match rx.recv() {
            Ok(bytes) => Self::read(&bytes),
            Err(err) => Err(Error::from(*err)),
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
    fn recv_timeout<T>(rx: &IpcReceiver<Vec<u8>>, timeout: Duration) -> Result<T>
        where T: protocol::LauncherMessage
    {
        // If ipc_channel implemented this directly, we wouldn't have
        // to do this :(
        let timeout = Instant::now() + timeout;
        loop {
            match rx.try_recv().map_err(|e| Error::from(*e)) {
                Ok(bytes) => return Self::read(&bytes),
                Err(Error::IPCIO(io::ErrorKind::WouldBlock)) => {
                    trace!("try_recv would block; waiting 5ms");
                    thread::sleep(Duration::from_millis(5));
                }
                Err(err) => {
                    return Err(err);
                }
            }
            if Instant::now() > timeout {
                return Err(Error::Timeout);
            }
        }
    }

    /// Send a command to a Launcher
    fn send<T>(tx: &IpcSender<Vec<u8>>, message: &T) -> Result<()>
        where T: protocol::LauncherMessage
    {
        let txn = protocol::NetTxn::build(message)?;
        let bytes = txn.to_bytes()?;
        tx.send(bytes).map_err(Error::Send)?;
        Ok(())
    }

    /// Receive and read protocol message from an IpcReceiver
    fn try_recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<Option<T>>
        where T: protocol::LauncherMessage
    {
        match rx.try_recv().map_err(|err| Error::from(*err)) {
            Ok(bytes) => {
                let msg = Self::read::<T>(&bytes)?;
                Ok(Some(msg))
            }
            Err(Error::IPCIO(io::ErrorKind::WouldBlock)) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub fn is_stopping(&self) -> bool {
        match Self::try_recv::<protocol::Shutdown>(&self.rx) {
            Ok(Some(_)) | Err(Error::IPCIO(_)) => true,
            Ok(None) => false,
            Err(err) => panic!("Unexpected error checking for shutdown request, {}", err),
        }
    }

    /// Restart a running process with the same arguments
    pub fn restart(&self, pid: Pid) -> Result<Pid> {
        let msg = protocol::Restart { pid: pid.into() };
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx)?;
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
                 -> Result<Pid> {
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

        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx)?;
        if reply.pid == 0 {
            warn!(target: "pidfile_tracing", "Spawn operation for {} resulted in a spawned PID of 0, which \
                   should be impossible! (proceeding anyway)",
                  id);
        }
        Ok(reply.pid as Pid)
    }

    /// Query the launcher for the PID of the named service. If the
    /// Launcher is aware of it, you'll get `Ok(Some(Pid))`
    pub fn pid_of(&self, service_name: &str) -> Result<Option<Pid>> {
        let msg = protocol::PidOf { service_name: service_name.to_string(), };
        Self::send(&self.tx, &msg)?;
        // This should be a recv_timeout until pidfile-less
        // supervisors are the norm. We only expect to not receive a
        // response when dealing with older Launchers that didn't know
        // how to return PIDs.
        let reply = Self::recv_timeout::<protocol::PidIs>(&self.rx, self.timeout)?;
        // TODO (CM): really, we need to have all our protocol types
        // that use pids actually use a Pid type that's nonzero, with
        // lots of descriptive errors for failures.
        match reply.pid {
            Some(pid) => Ok(Some(pid as Pid)),
            None => Ok(None),
        }
    }

    pub fn terminate(&self, pid: Pid) -> Result<i32> {
        let msg = protocol::Terminate { pid: pid.into() };
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::TerminateOk>(&self.rx)?;
        Ok(reply.exit_code)
    }
}
