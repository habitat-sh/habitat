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

use std::ffi::OsString;

use common::ui::UI;

use command::studio;
use error::Result;

pub fn start(
    ui: &mut UI,
    plan_context: &str,
    root: Option<&str>,
    src: Option<&str>,
    keys: Option<&str>,
    reuse: bool,
    windows: bool,
    docker: bool,
) -> Result<()> {
    let mut args: Vec<OsString> = Vec::new();
    if let Some(root) = root {
        args.push("-r".into());
        args.push(root.into());
    }
    if let Some(src) = src {
        args.push("-s".into());
        args.push(src.into());
    }
    if let Some(keys) = keys {
        args.push("-k".into());
        args.push(keys.into());
    }
    args.push("build".into());
    if studio::native_studio_support() && reuse {
        args.push("-R".into());
    }
    args.push(plan_context.into());
    if cfg!(target_os = "windows") && windows {
        args.push("-w".into());
    }
    if studio::native_studio_support() && docker {
        args.push("-D".into());
    }
    studio::enter::start(ui, args)
}
