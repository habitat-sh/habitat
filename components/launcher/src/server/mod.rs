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
use std::env;
use std::path::PathBuf;
use std::process::{self, Child, Command, Stdio};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use core;
use core::package::{PackageIdent, PackageInstall};
use core::os::process::Signal;
use core::os::signals::{self, SignalEvent};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use protobuf;
use protocol;

use self::handlers::Handler;
use {SUP_CMD, SUP_PACKAGE_IDENT};
use error::{Error, Result};
use service::Service;

const SUP_CMD_ENVVAR: &'static str = "HAB_SUP_BINARY";
static LOGKEY: &'static str = "SV";

type Receiver = IpcReceiver<Vec<u8>>;
type Sender = IpcSender<Vec<u8>>;

pub struct Server {
    services: ServiceTable,
    tx: Sender,
    rx: Receiver,
    supervisor: Child,
}

impl Server {
    fn init() -> Result<Self> {
        let (server, pipe) = IpcOneShotServer::new().map_err(Error::OpenPipe)?;
        let supervisor = spawn_supervisor(&pipe)?;
        let (rx, tx) = setup_connection(server)?;
        Ok(Server {
            services: ServiceTable::default(),
            tx: tx,
            rx: rx,
            supervisor: supervisor,
        })
    }

    #[allow(unused_must_use)]
    pub fn reload(&mut self) -> Result<()> {
        self.supervisor.kill();
        self.supervisor.wait();
        let server = Self::init()?;
        self.tx = server.tx;
        self.rx = server.rx;
        self.supervisor = server.supervisor;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        self.reap_zombies();
        match signals::check_for_signal() {
            Some(SignalEvent::Shutdown) => self.shutdown(),
            Some(SignalEvent::Passthrough(signal)) => self.forward_signal(signal),
            None => (),
        }
        self.handle_message()
    }

    fn forward_signal(&self, signal: Signal) {
        if let Err(err) = core::os::process::signal(self.supervisor.id(), signal) {
            error!(
                "Unable to signal Supervisor, {}, {}",
                self.supervisor.id(),
                err
            );
        }
    }

    fn handle_message(&mut self) -> Result<()> {
        match self.rx.try_recv() {
            Ok(bytes) => {
                dispatch(&self.tx, &bytes, &mut self.services);
                Ok(())
            }
            Err(_) => {
                match self.supervisor.try_wait() {
                    Ok(None) => Ok(()),
                    Ok(Some(status)) => {
                        debug!("Supervisor exited: {}", status);
                        Err(Error::SupShutdown)
                    }
                    Err(err) => {
                        warn!("Unable to wait for Supervisor, {}", err);
                        Err(Error::SupShutdown)
                    }
                }
            }
        }
    }

    fn reap_zombies(&mut self) {
        self.services.reap_zombies()
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
        process::exit(0);
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

    fn reap_zombies(&mut self) {
        let mut dead: Vec<u32> = vec![];
        for service in self.0.values_mut() {
            match service.try_wait() {
                Ok(None) => (),
                Ok(Some(code)) => {
                    debug!("Child exited, {}, {}", service.id(), code);
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
    let bytes = txn.build_reply(msg)
        .map_err(Error::Serialize)?
        .to_bytes()
        .map_err(Error::Serialize)?;
    tx.send(bytes).map_err(Error::Send)?;
    Ok(())
}

pub fn run() -> Result<()> {
    let mut server = Server::init()?;
    signals::init();
    loop {
        if server.tick().is_ok() {
            thread::sleep(Duration::from_millis(1_000));
        } else {
            while server.reload().is_err() {
                thread::sleep(Duration::from_millis(5_000));
            }
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
    let (rx, raw) = server.accept().map_err(|_| Error::AcceptConn)?;
    let txn = protocol::NetTxn::from_bytes(&raw).map_err(
        Error::Deserialize,
    )?;
    let mut msg = txn.decode::<protocol::Register>().map_err(
        Error::Deserialize,
    )?;
    let tx = IpcSender::connect(msg.take_pipe()).map_err(Error::Connect)?;
    send(&tx, &protocol::NetOk::new())?;
    Ok((rx, tx))
}

fn spawn_supervisor(pipe: &str) -> Result<Child> {
    let binary = supervisor_cmd()?;
    let mut command = Command::new(&binary);
    let args: Vec<String> = env::args().skip(1).collect();
    debug!("Starting Supervisor...");
    let child = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env(protocol::LAUNCHER_PIPE_ENV, pipe)
        .args(args)
        .spawn()
        .map_err(Error::SupSpawn)?;
    Ok(child)
}

fn supervisor_cmd() -> Result<PathBuf> {
    if let Ok(command) = core::env::var(SUP_CMD_ENVVAR) {
        return Ok(PathBuf::from(command));
    }
    let ident = PackageIdent::from_str(SUP_PACKAGE_IDENT).unwrap();
    match PackageInstall::load_at_least(&ident, None) {
        Ok(install) => {
            match core::fs::find_command_in_pkg(SUP_CMD, &install, "/") {
                Ok(Some(cmd)) => Ok(cmd),
                _ => Err(Error::SupBinaryNotFound),
            }
        }
        Err(_) => Err(Error::SupPackageNotFound),
    }
}
