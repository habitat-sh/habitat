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
use std::io;
use std::path::Path;

use core::os::process::Pid;
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use protobuf;
use protocol;

use error::{Error, Result};

type Env = HashMap<String, String>;
type IpcServer = IpcOneShotServer<Vec<u8>>;

pub struct LauncherCli {
    tx: IpcSender<Vec<u8>>,
    rx: IpcReceiver<Vec<u8>>,
}

impl LauncherCli {
    pub fn connect(pipe: String) -> Result<Self> {
        let tx = IpcSender::connect(pipe).map_err(Error::Connect)?;
        let (ipc_srv, pipe) = IpcServer::new().map_err(Error::BadPipe)?;
        let mut cmd = protocol::Register::new();
        cmd.set_pipe(pipe);
        Self::send(&tx, &cmd)?;
        let (rx, raw) = ipc_srv.accept().map_err(|_| Error::AcceptConn)?;
        Self::read::<protocol::NetOk>(&raw)?;
        Ok(LauncherCli { tx: tx, rx: rx })
    }

    /// Read a launcher protocol message from a byte array
    fn read<T>(bytes: &[u8]) -> Result<T>
    where
        T: protobuf::MessageStatic,
    {
        let txn = protocol::NetTxn::from_bytes(bytes).map_err(
            Error::Deserialize,
        )?;
        if txn.message_id() == "NetErr" {
            let err = txn.decode::<protocol::NetErr>().map_err(Error::Deserialize)?;
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
            Ok(Some(_)) |
            Err(Error::IPCIO(_)) => true,
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
    pub fn spawn<I, B, U, G, P>(
        &self,
        id: I,
        bin: B,
        user: U,
        group: G,
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
        msg.set_svc_user(user.to_string());
        msg.set_svc_group(group.to_string());
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
