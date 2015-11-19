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

//! The CLI commands.
//!
//! Bldr's command line actions are defined here; one module per command. Their names map 1:1 to
//! the actual command line arguments, with one exception - `_` is translated to `-` on the CLI.

pub mod install;
pub mod start;
pub mod key;
pub mod key_upload;
pub mod upload;
pub mod repo;
pub mod configure;
