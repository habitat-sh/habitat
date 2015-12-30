// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The gossip infrastructure.
//!
//! Start with the server module, then read the rumor, detector, and member modules.

pub mod rumor;
pub mod lamport_clock;
pub mod member;
pub mod server;
pub mod client;
pub mod detector;
