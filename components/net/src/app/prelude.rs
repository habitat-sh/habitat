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

//! The App Prelude
//!
//! The purpose of this module is to alleviate imports of the common app traits by adding a glob
//! import to the top of app heavy modules.
//!
//! ```rust,no_run
//! # #![allow(unused_imports)]
//! use habitat_net::app::prelude::*;
//! ```

pub use std::sync::Arc;

pub use protobuf::MessageStatic;
pub use protocol::{Message, Protocol};
pub use super::start as app_start;
pub use super::AppState;
pub use super::config::AppCfg;
pub use super::dispatcher::{Dispatcher, DispatchTable};
pub use super::error::{AppError, AppResult};
pub use conn::RouteConn;
pub use error::{ErrCode, NetError, NetOk};
