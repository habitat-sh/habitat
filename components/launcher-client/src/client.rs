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

use crate::error::{Error, Result};
use habitat_core::os::process::Pid;
use habitat_launcher_protocol::{self as protocol, Error as ProtocolError};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use std::{collections::HashMap, fs, io, path::Path};

type Env = HashMap<String, String>;
type IpcServer = IpcOneShotServer<Vec<u8>>;

pub struct LauncherCli {
    tx: IpcSender<Vec<u8>>,
    rx: IpcReceiver<Vec<u8>>,
    pipe: String,
}

impl Drop for LauncherCli {
    fn drop(&mut self) {
        if fs::remove_file(&self.pipe).is_err() {
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
        let cmd = protocol::Register {
            pipe: pipe_to_sup.clone(),
        };
        Self::send(&tx, &cmd)?;
        let (rx, raw) = ipc_srv.accept().map_err(|_| Error::AcceptConn)?;
        Self::read::<protocol::NetOk>(&raw)?;
        Ok(LauncherCli {
            tx: tx,
            rx: rx,
            pipe: pipe_to_sup,
        })
    }

    /// Read a launcher protocol message from a byte array
    fn read<T>(bytes: &[u8]) -> Result<T>
    where
        T: protocol::LauncherMessage,
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
    where
        T: protocol::LauncherMessage,
    {
        match rx.recv() {
            Ok(bytes) => Self::read(&bytes),
            Err(err) => Err(Error::from(*err)),
        }
    }

    /// Send a command to a Launcher
    fn send<T>(tx: &IpcSender<Vec<u8>>, message: &T) -> Result<()>
    where
        T: protocol::LauncherMessage,
    {
        let txn = protocol::NetTxn::build(message)?;
        let bytes = txn.to_bytes()?;
        tx.send(bytes).map_err(Error::Send)?;
        Ok(())
    }

    /// Receive and read protocol message from an IpcReceiver
    fn try_recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<Option<T>>
    where
        T: protocol::LauncherMessage,
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
    /// `user` and `group` are string names, while `user_id` and
    /// `group_id` are numeric IDs. Newer versions of the Launcher can
    /// accept either, but prefer numeric IDs.
    pub fn spawn<I, B, U, G, P>(
        &self,
        id: &I,
        bin: B,
        user: Option<U>,
        group: Option<G>,
        user_id: Option<u32>,
        group_id: Option<u32>,
        password: Option<P>,
        env: Env,
    ) -> Result<Pid>
    where
        I: ToString,
        B: AsRef<Path>,
        U: ToString,
        G: ToString,
        P: ToString,
    {
        // On Windows, we only expect user to be Some.
        //
        // On Linux, we expect user_id and group_id to be Some, while
        // user and group may be either Some or None. Only the IDs are
        // used; names are only for backward compatibility with older
        // Launchers.
        let msg = protocol::Spawn {
            binary: bin.as_ref().to_path_buf().to_string_lossy().into_owned(),
            svc_user: user.map(|u| u.to_string()),
            svc_group: group.map(|g| g.to_string()),
            svc_user_id: user_id,
            svc_group_id: group_id,
            svc_password: password.map(|p| p.to_string()),
            env: env,
            id: id.to_string(),
            ..Default::default()
        };

        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx)?;
        Ok(reply.pid as Pid)
    }

    pub fn terminate(&self, pid: Pid) -> Result<i32> {
        let msg = protocol::Terminate { pid: pid.into() };
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::TerminateOk>(&self.rx)?;
        Ok(reply.exit_code)
    }
}
