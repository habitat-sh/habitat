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

use std::ffi::OsStr;

use hcore::os::process::windows_child::Child;

use crate::error::Result;
use crate::manager::service::Pkg;

pub fn run<T, S>(path: S, pkg: &Pkg, svc_encrypted_password: Option<T>) -> Result<Child>
where
    T: ToString,
    S: AsRef<OsStr>,
{
    let ps_cmd = format!("iex $(gc {} | out-string)", path.as_ref().to_string_lossy());
    let args = vec!["-NonInteractive", "-command", ps_cmd.as_str()];
    Ok(Child::spawn(
        "pwsh.exe",
        args,
        &pkg.env,
        &pkg.svc_user,
        svc_encrypted_password,
    )?)
}
