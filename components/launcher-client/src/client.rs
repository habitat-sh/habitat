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

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use crate::core::os::process::Pid;
use crate::protocol;
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use protobuf;

use crate::error::{Error, Result};

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
        let mut cmd = protocol::Register::new();
        cmd.set_pipe(pipe_to_sup);
        Self::send(&tx, &cmd)?;
        let (rx, raw) = ipc_srv.accept().map_err(|_| Error::AcceptConn)?;
        Self::read::<protocol::NetOk>(&raw)?;
        Ok(LauncherCli {
            tx: tx,
            rx: rx,
            pipe: cmd.take_pipe(),
        })
    }

    /// Read a launcher protocol message from a byte array
    fn read<T>(bytes: &[u8]) -> Result<T>
    where
        T: protobuf::MessageStatic,
    {
        let txn = protocol::NetTxn::from_bytes(bytes).map_err(Error::Deserialize)?;
        if txn.message_id() == "NetErr" {
            let err = txn
                .decode::<protocol::NetErr>()
                .map_err(Error::Deserialize)?;
            return Err(Error::Protocol(err));
        }
        let msg = txn.decode::<T>().map_err(Error::Deserialize)?;
        Ok(msg)
    }

    /// Receive and read protocol message from an IpcReceiver
    fn recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<T>
    where
        T: protobuf::MessageStatic,
    {
        match rx.recv() {
            Ok(bytes) => Self::read(&bytes),
            Err(err) => Err(Error::from(*err)),
        }
    }

    /// Send a command to a Launcher
    fn send<T>(tx: &IpcSender<Vec<u8>>, message: &T) -> Result<()>
    where
        T: protobuf::MessageStatic,
    {
        let txn = protocol::NetTxn::build(message).map_err(Error::Serialize)?;
        let bytes = txn.to_bytes().map_err(Error::Serialize)?;
        tx.send(bytes).map_err(Error::Send)?;
        Ok(())
    }

    /// Receive and read protocol message from an IpcReceiver
    fn try_recv<T>(rx: &IpcReceiver<Vec<u8>>) -> Result<Option<T>>
    where
        T: protobuf::MessageStatic,
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
        let mut msg = protocol::Restart::new();
        msg.set_pid(pid.into());
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx)?;
        Ok(reply.get_pid() as Pid)
    }

    /// Send a process spawn command to the connected Launcher
    ///
    /// `user` and `group` are string names, while `user_id` and
    /// `group_id` are numeric IDs. Newer versions of the Launcher can
    /// accept either, but prefer numeric IDs.
    pub fn spawn<I, B, U, G, P>(
        &self,
        id: I,
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
        let mut msg = protocol::Spawn::new();
        msg.set_binary(bin.as_ref().to_path_buf().to_string_lossy().into_owned());

        // On Windows, we only expect user to be Some.
        //
        // On Linux, we expect user_id and group_id to be Some, while
        // user and group may be either Some or None. Only the IDs are
        // used; names are only for backward compatibility with older
        // Launchers.
        if let Some(name) = user {
            msg.set_svc_user(name.to_string());
        }
        if let Some(name) = group {
            msg.set_svc_group(name.to_string());
        }
        if let Some(uid) = user_id {
            msg.set_svc_user_id(uid);
        }
        if let Some(gid) = group_id {
            msg.set_svc_group_id(gid);
        }

        // This is only for Windows
        if let Some(password) = password {
            msg.set_svc_password(password.to_string());
        }
        msg.set_env(env);
        msg.set_id(id.to_string());
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::SpawnOk>(&self.rx)?;
        Ok(reply.get_pid() as Pid)
    }

    pub fn terminate(&self, pid: Pid) -> Result<i32> {
        let mut msg = protocol::Terminate::new();
        msg.set_pid(pid.into());
        Self::send(&self.tx, &msg)?;
        let reply = Self::recv::<protocol::TerminateOk>(&self.rx)?;
        Ok(reply.get_exit_code())
    }
}
