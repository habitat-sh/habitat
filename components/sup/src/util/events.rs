// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use wonder::actor::{self, ActorResult, ActorSender, HandleResult, InitResult, StopReason};

use cadence::prelude::*;
use cadence::{StatsdClient, UdpMetricSink, DEFAULT_PORT};

use error::{Error, Result, SupError};

static LOGKEY: &'static str = "EV";

#[derive(Debug)]
pub enum EventMessage {
    Event(String),
    Stop,
    Ok,
}

pub type EventSinkActor = actor::Actor<EventMessage>;

pub struct EventSink;

impl EventSink {
    pub fn stop(actor: &actor::Actor<EventMessage>) -> Result<()> {
        match actor.call(EventMessage::Stop) {
            Ok(_) => Ok(()),
            Err(err) => Err(SupError::from(err)),
        }
    }

    /*
    pub fn send_event(actor: &EventSinkActor, event: String) -> ActorResult<()> {
        actor.cast(EventMessage::Event(event))
    }
    */

    /*
    fn init_client(&mut self) -> Result<()> {
        let host = ("172.17.0.3", DEFAULT_PORT);
        let client = StatsdClient::<UdpMetricSink>::from_udp_host(
            "my.metrics", host).unwrap();
    }
    */
}


impl actor::GenServer for EventSink {
    type T = EventMessage;
    type S = ();
    type E = SupError;

    fn init(&self, _tx: &ActorSender<Self::T>, _: &mut Self::S) -> InitResult<Self::E> {
        Ok(None)
    }

    fn handle_call(&self,
                   message: Self::T,
                   _: &ActorSender<Self::T>,
                   _: &ActorSender<Self::T>,
                   _: &mut Self::S)
        -> HandleResult<Self::T> {
            match message {
                EventMessage::Stop => HandleResult::Stop(StopReason::Normal, Some(EventMessage::Ok)),
                msg => {
                    HandleResult::Stop(StopReason::Fatal(format!("unexpected call message: {:?}", msg)),
                    Some(EventMessage::Ok))
                }
            }
        }

    fn handle_cast(&self,
                   message: Self::T,
                   _: &ActorSender<Self::T>,
                   _: &ActorSender<Self::T>,
                   _: &mut Self::S) ->
        HandleResult<Self::T> {
            match message {
                EventMessage::Event(value) => {
                    let host = ("172.17.0.3", DEFAULT_PORT);
                    let client = StatsdClient::<UdpMetricSink>::from_udp_host(
                                "my.metrics", host).unwrap();
                    client.incr(&value);
                    println!("Got an event update: {}", &value);
                    HandleResult::NoReply(None)
                },
                _ => HandleResult::NoReply(None)
            }
        }

}


