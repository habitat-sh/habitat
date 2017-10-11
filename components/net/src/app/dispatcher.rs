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
use std::error;
use std::marker::PhantomData;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use protobuf;
use protocol::{Message, Protocol};

use super::AppState;
use super::config::AppCfg;
use conn::{ConnErr, ConnEvent, RouteConn};

/// Dispatchers connect to Message Queue Servers
pub trait Dispatcher: Sized + Send + 'static {
    const APP_NAME: &'static str;
    const PROTOCOL: Protocol;

    type Config: AsRef<AppCfg> + Send;
    type Error: error::Error;
    type State: AppState;

    /// An initialization callback called before application startup. You should set up any state
    /// and start any additional side-car threads here.
    fn app_init(
        Self::Config,
        Arc<String>,
    ) -> Result<<Self::State as AppState>::InitState, Self::Error>;

    /// Returns a function dispatch table mapping which maps which protocol message is handled
    /// by which `Handler`.
    fn dispatch_table() -> &'static DispatchTable<Self>;
}

/// Message handler for incoming protocol messages.
pub trait Handler<T: Dispatcher>: Send + Sync + 'static {
    fn handle(&self, &mut Message, &mut RouteConn, &mut T::State) -> Result<(), T::Error>;
}

impl<T, F> Handler<T> for F
where
    T: Dispatcher,
    F: Sync
        + Send
        + 'static
        + Fn(&mut Message, &mut RouteConn, &mut T::State) -> Result<(), T::Error>,
{
    fn handle(
        &self,
        message: &mut Message,
        conn: &mut RouteConn,
        state: &mut T::State,
    ) -> Result<(), T::Error> {
        (*self)(message, conn, state)
    }
}

impl<T> Handler<T> for Box<Handler<T>>
where
    T: Dispatcher,
{
    fn handle(
        &self,
        message: &mut Message,
        conn: &mut RouteConn,
        state: &mut T::State,
    ) -> Result<(), T::Error> {
        (**self).handle(message, conn, state)
    }
}

pub struct DispatcherPool<T>
where
    T: Dispatcher,
{
    reply_queue: Arc<String>,
    request_queue: Arc<String>,
    workers: Vec<mpsc::Receiver<()>>,
    marker: PhantomData<T>,
}

impl<T> DispatcherPool<T>
where
    T: Dispatcher,
{
    pub fn new(reply_queue: Arc<String>, request_queue: Arc<String>, config: &T::Config) -> Self {
        DispatcherPool {
            reply_queue: reply_queue,
            request_queue: request_queue,
            workers: Vec::with_capacity(config.as_ref().worker_count),
            marker: PhantomData,
        }
    }

    /// Start a pool of message dispatchers.
    pub fn run(mut self, state: <T::State as AppState>::InitState) {
        let worker_count = self.workers.capacity();
        for worker_id in 0..worker_count {
            self.spawn_dispatcher(state.clone(), worker_id);
        }
        thread::spawn(move || loop {
            for i in 0..worker_count {
                // Refactor this if/when the standard library ever stabilizes select for mpsc
                // https://doc.rust-lang.org/std/sync/mpsc/struct.Select.html
                match self.workers[i].try_recv() {
                    Err(mpsc::TryRecvError::Disconnected) => {
                        info!("Worker[{}] restarting...", i);
                        self.spawn_dispatcher(state.clone(), i);
                    }
                    Ok(msg) => warn!("Worker[{}] sent unexpected msg: {:?}", i, msg),
                    Err(mpsc::TryRecvError::Empty) => continue,
                }
            }
            thread::sleep(Duration::from_millis(500));
        });
    }

    fn spawn_dispatcher(&mut self, state: <T::State as AppState>::InitState, worker_id: usize) {
        let (tx, rx) = mpsc::sync_channel(1);
        let state = match T::State::build(state) {
            Ok(state) => state,
            Err(err) => panic!("Dispatcher failed to initialize state, {}", err),
        };
        let reply_queue = self.reply_queue.clone();
        let request_queue = self.request_queue.clone();
        thread::spawn(move || {
            worker_run::<T>(tx, worker_id, reply_queue, request_queue, state)
        });
        if rx.recv().is_ok() {
            debug!("worker[{}] ready", worker_id);
            self.workers.insert(worker_id, rx);
        } else {
            error!("worker[{}] failed to start", worker_id);
            self.workers.remove(worker_id);
        }
    }
}

pub struct DispatchTable<T>(HashMap<&'static str, Box<Handler<T>>>);

impl<T> DispatchTable<T>
where
    T: Dispatcher,
{
    pub fn new() -> Self {
        DispatchTable(HashMap::new())
    }

    /// Returns a `Handler` for the given message-id.
    pub fn get(&self, message_id: &str) -> Option<&Box<Handler<T>>> {
        self.0.get(message_id)
    }

    /// Registers a protobuf message to a given `Handler`.
    pub fn register<H>(&mut self, msg: &'static protobuf::reflect::MessageDescriptor, handler: H)
    where
        H: Handler<T>,
    {
        if self.0.insert(msg.name(), Box::new(handler)).is_some() {
            panic!(
                "Attempted to register a second handler for message, '{}'",
                msg.name()
            );
        }
    }
}

/// Helper function for dispatching incoming protocol messages within a running worker.
fn dispatch<T>(message: &mut Message, conn: &mut RouteConn, state: &mut T::State)
where
    T: Dispatcher,
{
    trace!("dispatch, {}", message);
    match T::dispatch_table().get(message.message_id()) {
        Some(handler) => {
            if let Err(err) = (**handler).handle(message, conn, state) {
                error!("{}", err);
            }
        }
        None => warn!("dispatch, recv unknown message, {}", message.message_id()),
    }
}

/// Main thread for a running dispatch worker.
fn worker_run<T>(
    rz: mpsc::SyncSender<()>,
    id: usize,
    reply_queue: Arc<String>,
    request_queue: Arc<String>,
    mut state: T::State,
) where
    T: Dispatcher,
{
    let mut message = Message::default();
    let mut conn = RouteConn::new(request_queue.clone()).unwrap();
    conn.connect(&*reply_queue).unwrap();
    rz.send(()).unwrap();
    loop {
        message.reset();
        trace!("worker[{}] waiting for message", id);
        match conn.wait_recv(&mut message, -1) {
            Ok(ConnEvent::OnMessage) => (),
            Ok(ConnEvent::OnConnect) => warn!("dispatcher unexpectedly received OnConnect event"),
            Err(ConnErr::Shutdown(_)) => break,
            Err(err) => {
                warn!("worker[{}], {}", id, err);
                continue;
            }
        }
        dispatch::<T>(&mut message, &mut conn, &mut state);
    }
}
