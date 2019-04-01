// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

//! All the individual event types that can be sent out by the
//! Supervisor.
//!
//! A key aspect of the current design is that all events are
//! collections of *references*, rather than owned types. This means
//! less unnecessary allocations and copying in order to create
//! events.

use super::EventCore;
use crate::manager::service::{HealthCheck as DomainHealthCheck,
                              Service,
                              UpdateStrategy as DomainUpdateStrategy};
use prost::{Enumeration,
            Message};

include!(concat!(env!("OUT_DIR"), "/chef.habitat.supervisor.event.rs"));

// Note: `UpdateStrategy` here is the protobuf-generated type for the
// event we're sending out; `DomainUpdateStrategy` is the one we use
// elsewhere in the Supervisor.
impl Into<UpdateStrategy> for DomainUpdateStrategy {
    fn into(self) -> UpdateStrategy {
        match self {
            DomainUpdateStrategy::None => UpdateStrategy::None,
            DomainUpdateStrategy::AtOnce => UpdateStrategy::AtOnce,
            DomainUpdateStrategy::Rolling => UpdateStrategy::Rolling,
        }
    }
}

// Note: `HealthCheck` here is the protobuf-generated type for the
// event we're sending out; `DomainHealthCheck` is the one we use
// elsewhere in the Supervisor.
impl Into<HealthCheck> for DomainHealthCheck {
    fn into(self) -> HealthCheck {
        match self {
            DomainHealthCheck::Ok => HealthCheck::Ok,
            DomainHealthCheck::Warning => HealthCheck::Warning,
            DomainHealthCheck::Critical => HealthCheck::Critical,
            DomainHealthCheck::Unknown => HealthCheck::Unknown,
        }
    }
}

impl Service {
    /// Create a protobuf metadata struct for Service-related event messages.
    pub(super) fn to_service_metadata(&self) -> ServiceMetadata {
        ServiceMetadata { package_ident:   self.pkg.ident.to_string(),
                          spec_ident:      self.spec_ident.to_string(),
                          service_group:   self.service_group.to_string(),
                          update_channel:  self.channel.to_string(),
                          update_strategy: self.update_strategy.into(), }
    }
}

impl EventCore {
    /// Create a protobuf metadata struct for all event messages.
    pub(super) fn to_event_metadata(&self) -> EventMetadata {
        EventMetadata { supervisor_id: self.supervisor_id.clone(),
                        ip_address:    self.ip_address.to_string(),
                        timestamp:     None, }
    }
}

pub trait EventMessage: Message + Sized {
    /// All messages will have some top-level metadata about the
    /// Supervisor they come from. This function allows us to set it
    /// generically when we send the message out.
    fn event_metadata(&mut self, event_metadata: EventMetadata);

    /// Convert a message to bytes for sending to NATS.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = bytes::BytesMut::with_capacity(self.encoded_len());
        // The only way this can fail is if the buffer doesn't have
        // enough room. We just set that, though, so something would
        // have to be seriously wrong in Prost for this to fail.
        self.encode(&mut buf)
            .expect("UNEXPECTED PROST ERROR: encoded_len() was not long enough!");
        buf.to_vec()
    }
}

// TODO (CM): these are repetitive, and will only get more so as we
// add events. Consider implementing via macro instead.

impl EventMessage for ServiceStartedEvent {
    fn event_metadata(&mut self, event_metadata: EventMetadata) {
        self.event_metadata = Some(event_metadata);
    }
}

impl EventMessage for ServiceStoppedEvent {
    fn event_metadata(&mut self, event_metadata: EventMetadata) {
        self.event_metadata = Some(event_metadata);
    }
}

impl EventMessage for HealthCheckEvent {
    fn event_metadata(&mut self, event_metadata: EventMetadata) {
        self.event_metadata = Some(event_metadata);
    }
}
