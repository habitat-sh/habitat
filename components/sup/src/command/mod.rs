// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! The CLI commands.
//!
//! The Supervisor's command line actions are defined here; one module per command. Their names map
//! 1:1 to the actual command line arguments, with one exception - `_` is translated to `-` on the
//! CLI.

pub mod start;
pub mod configure;
pub mod shell;
