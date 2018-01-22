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

use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::process::Command;

use error::{Error, Result};

#[derive(Debug)]
pub struct NetworkNamespace(PathBuf);

impl NetworkNamespace {
    pub fn new(path: PathBuf) -> Self {
        NetworkNamespace(path)
    }

    pub fn create(&self, interface: &str, gateway: &IpAddr, user: &str) -> Result<()> {
        let mut cmd = Command::new("airlock");
        cmd.arg("netns");
        cmd.arg("create");
        cmd.arg("--interface");
        cmd.arg(interface);
        cmd.arg("--gateway");
        cmd.arg(gateway.to_string());
        cmd.arg("--ns-dir");
        cmd.arg(&self.0);
        cmd.arg("--user");
        cmd.arg(user);
        debug!("building airlock networking setup command, cmd={:?}", &cmd);

        debug!("spawning airlock networking setup command");
        let mut child = cmd.spawn().map_err(|e| {
            Error::AirlockNetworking(self.0.to_path_buf(), e)
        })?;
        let exit_status = child.wait().map_err(|e| {
            Error::AirlockNetworking(self.0.to_path_buf(), e)
        })?;
        info!(
            "completed airlock networking setup command, status={:?}",
            exit_status
        );

        if exit_status.success() {
            Ok(())
        } else {
            Err(Error::AirlockFailure(exit_status))
        }
    }

    pub fn destroy(&self) -> Result<()> {
        let mut cmd = Command::new("airlock");
        cmd.arg("netns");
        cmd.arg("destroy");
        cmd.arg("--ns-dir");
        cmd.arg(&self.0);
        debug!(
            "building airlock networking destroy command, cmd={:?}",
            &cmd
        );

        debug!("spawning airlock networking destroy command");
        let mut child = cmd.spawn().map_err(|e| {
            Error::AirlockNetworking(self.0.to_path_buf(), e)
        })?;
        let exit_status = child.wait().map_err(|e| {
            Error::AirlockNetworking(self.0.to_path_buf(), e)
        })?;
        info!(
            "completed airlock networking destroy command, status={:?}",
            exit_status
        );

        if exit_status.success() {
            Ok(())
        } else {
            Err(Error::AirlockFailure(exit_status))
        }
    }

    pub fn exists(&self) -> bool {
        self.0.exists() && self.0.is_dir()
    }

    pub fn ns_dir(&self) -> &Path {
        self.0.as_ref()
    }

    pub fn userns(&self) -> PathBuf {
        self.0.join("userns")
    }

    pub fn netns(&self) -> PathBuf {
        self.0.join("netns")
    }
}
