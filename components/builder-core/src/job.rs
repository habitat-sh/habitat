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

use std::fmt;
use std::ops::{Deref, DerefMut};
use protocol::jobsrv;
use protocol::originsrv;
use github_api_client::GitHubCfg;
use vcs;

pub struct Job(jobsrv::Job);

impl Job {
    pub fn new(job: jobsrv::Job) -> Self {
        Job(job)
    }

    pub fn vcs(&self, config: GitHubCfg) -> vcs::VCS {
        match self.0.get_project().get_vcs_type() {
            "git" => {
                let installation_id: Option<u32> = {
                    if self.0.get_project().has_vcs_installation_id() {
                        Some(self.0.get_project().get_vcs_installation_id())
                    } else {
                        None
                    }
                };
                vcs::VCS::new(
                    String::from(self.0.get_project().get_vcs_type()),
                    String::from(self.0.get_project().get_vcs_data()),
                    config,
                    installation_id,
                )
            }
            _ => panic!("unknown vcs associated with jobs project"),
        }
    }

    pub fn origin(&self) -> &str {
        let items = self.0
            .get_project()
            .get_name()
            .split("/")
            .collect::<Vec<&str>>();
        assert!(
            items.len() == 2,
            format!(
                "Invalid project identifier - {}",
                self.0.get_project().get_id()
            )
        );
        items[0]
    }
}

impl Deref for Job {
    type Target = jobsrv::Job;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Job {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let integrations: Vec<originsrv::OriginIntegration> = self.0
            .get_integrations()
            .into_iter()
            .map(|i| {
                let mut r = originsrv::OriginIntegration::new();
                r.set_origin(i.get_origin().to_string());
                r.set_integration(i.get_integration().to_string());
                r.set_name(i.get_name().to_string());
                r.set_body("[secure]".to_string());
                r
            })
            .collect();

        f.debug_struct("Job")
            .field("id", &self.0.get_id())
            .field("owner_id", &self.0.get_owner_id())
            .field("state", &self.0.get_owner_id())
            .field("project", &self.0.get_project())
            .field("created_at", &self.0.get_created_at())
            .field("channel", &self.0.get_channel())
            .field("build_started_at", &self.0.get_build_started_at())
            .field("build_finished_at", &self.0.get_build_finished_at())
            .field("package_ident", &self.0.get_package_ident())
            .field("integrations", &integrations)
            .field("project_integrations", &self.0.get_project_integrations())
            .finish()
    }
}
