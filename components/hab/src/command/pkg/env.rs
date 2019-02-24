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

use std::{collections::HashMap,
          path::Path};

use crate::hcore::package::{PackageIdent,
                            PackageInstall};

use crate::error::Result;

pub fn start(ident: &PackageIdent, fs_root_path: &Path) -> Result<()> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    let env = pkg_install.environment_for_command()?;
    render_environment(env);
    Ok(())
}

#[cfg(unix)]
fn render_environment(env: HashMap<String, String>) {
    for (key, value) in env.into_iter() {
        println!("export {}=\"{}\"", key, value);
    }
}

#[cfg(windows)]
fn render_environment(env: HashMap<String, String>) {
    for (key, value) in env.into_iter() {
        println!("$env:{}=\"{}\"", key, value);
    }
}
