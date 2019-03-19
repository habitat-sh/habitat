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

// TODO: It is NOT AT ALL clear that every individual event should be
// its own struct, though that obviously works. This is just a
// convenient and flexible (if perhaps a bit verbose) way to start.

use super::Event;
use habitat_core::{package::PackageIdent,
                   service::ServiceGroup};

#[derive(Debug)]
pub struct ServiceStarted<'a> {
    /// The fully-qualified identifier of the service that has just started
    // TODO (CM): I *really* want a FullyQualifiedIdentifier type now
    pub ident: &'a PackageIdent,
    /// The identifier the service was loaded as. This could be (and
    /// often is) an abbreviated identifier, like "core/redis".
    pub spec_ident: &'a PackageIdent,
    /// The service group of the running service
    pub service_group: &'a ServiceGroup,
}
impl<'a> Event for ServiceStarted<'a> {}

#[derive(Debug)]
pub struct ServiceStopped<'a> {
    /// The fully-qualified identifier of the service that has just started
    pub ident: &'a PackageIdent,
    //    pub spec_ident: &'a PackageIdent,
    /// The service group of the running service
    pub service_group: &'a ServiceGroup,
}
impl<'a> Event for ServiceStopped<'a> {}
