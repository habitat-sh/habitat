//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use wonder::actor::{self, GenServer, HandleResult, InitResult, StopReason, ActorSender};
use discovery::etcd;

use error::{BldrError, BldrResult};

const TIMEOUT_MS: u64 = 200;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    ConfigToml(Option<String>),
    Config,
    Ok,
    Stop
}

#[derive(Debug)]
pub struct UserActor;

impl UserActor {
    pub fn config_string(actor: &actor::Actor<Message>) -> BldrResult<Option<String>> {
        match try!(actor.call(Message::Config)) {
            Message::ConfigToml(config_string) => Ok(config_string),
            _ => unreachable!(),
        }
    }
}

pub struct UserActorState {
    ctx: Option<Sender<bool>>,
    crx: Option<Receiver<Option<String>>>,
    config_string: Option<String>,
    watch_key: String,
}

impl UserActorState {
    pub fn new(watch_key: String) -> UserActorState {
        UserActorState {
            ctx: None,
            crx: None,
            config_string: None,
            watch_key: watch_key
        }
    }
}

impl GenServer for UserActor {
    type T = Message;
    type S = UserActorState;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        let (ctx, wrx) = channel();
        let (wtx, crx) = channel();
        etcd::watch(&state.watch_key, 1, true, true, wtx, wrx);
        state.ctx = Some(ctx);
        state.crx = Some(crx);
        Ok(Some(0))
    }

    fn handle_timeout(&self, _tx: &ActorSender<Self::T>, _me: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
        if let Some(ref crx) = state.crx {
            match crx.try_recv() {
                Ok(Some(toml_string)) => {
                    state.config_string = Some(toml_string);
                },
                Ok(None) => {
                    state.config_string = None;
                },
                Err(TryRecvError::Empty) => { },
                Err(e) => return HandleResult::Stop(
                    StopReason::Fatal(
                        format!("User Actor caught unexpected error: {:?}", e)),
                        None,
                        ),
            }
        }
        HandleResult::NoReply(Some(TIMEOUT_MS))
    }

    fn handle_call(&self, message: Self::T, _caller: &ActorSender<Self::T>, _me: &ActorSender<Self::T>, state: &mut Self::S) -> HandleResult<Self::T> {
        if let Some(ref crx) = state.crx {
            match crx.try_recv() {
                Ok(Some(toml_string)) => {
                    state.config_string = Some(toml_string);
                },
                Ok(None) => {
                    state.config_string = None;
                },
                Err(TryRecvError::Empty) => { },
                Err(e) => return HandleResult::Stop(
                    StopReason::Fatal(
                        format!("User Actor caught unexpected error: {:?}", e)),
                        Some(Message::Ok),
                        ),
            }
        }

        match message {
           Message::Stop => {
               HandleResult::Stop(StopReason::Normal, Some(Message::Ok))
           },
           Message::Config => {
               match state.config_string {
                   Some(ref toml_string) => {
                       HandleResult::Reply(Message::ConfigToml(Some(toml_string.clone())), Some(TIMEOUT_MS))
                   },
                   None => {
                       HandleResult::Reply(Message::ConfigToml(None), Some(TIMEOUT_MS))
                   }
               }
           },
           Message::Ok => HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! I send YOU Ok!")), Some(Message::Ok)),
           Message::ConfigToml(_) => HandleResult::Stop(StopReason::Fatal(format!("You don't send me CensusToml(_)! I send YOU CensusToml(_)!")), Some(Message::Ok)),
        }
    }
}

