use crate::error::{Error,
                   Result};
use habitat_common::types::UserInfo;
use habitat_core::os::process::Pid;
use habitat_launcher_protocol::{self as protocol,
                                Error as ProtocolError};
use ipc_channel::ipc::{IpcOneShotServer,
                       IpcReceiver,
                       IpcSender};
use std::{collections::HashMap,
          io,
          path::Path};

type Env = HashMap<String, String>;
type IpcServer = IpcOneShotServer<Vec<u8>>;

pub struct LauncherCli {
    tx: IpcSender<Vec<u8>>,
    rx: IpcReceiver<Vec<u8>>,
    // We persist the pipe identifier so we can delete the file on drop.
    // This is not necessary on Windows because named pipes are removed
    // upon releasing the last handle to the pipe. The ipc-channel crate
    // wraps the pipe in a WinHandle whose drop impl calls CloseHandle.
    #[cfg(not(windows))]
    pipe: String,
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
        Ok(LauncherCli { tx,
                         rx,
                         #[cfg(not(windows))]
                         pipe: pipe_to_sup })
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

    pub fn terminate(&self, pid: Pid) -> Result<i32> {
        let msg = protocol::Terminate { pid: pid.into() };
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::TerminateOk>(&self.rx)?;
        Ok(reply.exit_code)
    }
}
