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

use std::ffi::OsString;

use crate::common::ui::UI;

use crate::error::Result;

const EXPORT_CMD: &str = "hab-pkg-export-helm";
const EXPORT_CMD_ENVVAR: &str = "HAB_PKG_EXPORT_HELM_BINARY";
const EXPORT_PKG_IDENT: &str = "core/hab-pkg-export-helm";
const EXPORT_PKG_IDENT_ENVVAR: &str = "HAB_PKG_EXPORT_HELM_PKG_IDENT";

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
    crate::command::pkg::export::export_common::start(ui,
                                                      args,
                                                      EXPORT_CMD,
                                                      EXPORT_CMD_ENVVAR,
                                                      EXPORT_PKG_IDENT,
                                                      EXPORT_PKG_IDENT_ENVVAR)
}
