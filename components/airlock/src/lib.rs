// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

extern crate errno;
extern crate libc;
#[macro_use]
extern crate log;
extern crate rand;
extern crate unshare;
extern crate users;

pub mod command;
mod error;
mod fs_root;
mod filesystem;
mod mount;
mod pty;

pub use error::{Error, Result};
pub use fs_root::{FsRoot, FsRootPolicy};
