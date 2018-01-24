// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Modules which can be serialized and deserialized to and from Google Protobufs.
//!
//! The types in the contained modules are largely generated or are new-type wrappers around
//! generated code. All generated code is placed in [`generated`] which comes from Protobuf
//! definition files in the `protocols` directory at the root of the Supervisor crate.
//!
//! # Defining New Protocols
//!
//! A new generated module is created at `protocols::generated::{T}` where `T` is the name of your
//! Protobuf file placed in the `protocols` directory. For example, given the file
//! `protocols/net.proto`, a new Rust module will be placed at `src/protocols/generated/net.rs`.
//!
//! Each time a new Protobuf file is added, you will need to update the `generated` module with
//! an entry of the newly generated module. Given the previous example, you will need to add
//! `pub mod net` to the generated module.
//!
//! WARNING: Do not attempt to create a protocol named after a reserved Rust type, such as `mod`,
//!          as this will lead to undefined behaviour.
//!
//! # Generating Protocols
//!
//! Protocol files are generated when the `protocols` feature is passed to Cargo. You can do this
//! by running `cargo build --features protocols`. This command should be run each time a new
//! protocol file is added or anytime one is changed. Generated files are to be committed to
//! version control. Files are generated on your workstation and committed to version control and
//! *not* by a build server intentionally. This is to ensure we have the source available for
//! all protocol files.

pub mod ctl;
pub mod net;
pub mod types;
mod generated;
