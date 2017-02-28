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


// TBD - This is a WIP for now, persistence to be added later

use std::collections::HashMap;

use config::Config;
use error::Result;
use protocol::jobsrv::Job;
use protocol::scheduler::{Group, GroupState};

#[derive(Debug, Clone)]
pub struct JobGroup {
    group: Group,
    jobs: HashMap<u64, Job>,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    curr_id: u64,
    job_groups: HashMap<u64, JobGroup>,
}

impl DataStore {
    pub fn new(_: &Config) -> Result<DataStore> {
        let groups = HashMap::new();
        Ok(DataStore {
            curr_id: 1,
            job_groups: groups,
        })
    }

    pub fn setup(&self) -> Result<()> {
        Ok(())
    }

    pub fn create_group(&mut self, group: &mut Group) -> Result<()> {
        group.set_group_id(self.curr_id);
        group.set_state(GroupState::Pending);

        let job_group = JobGroup {
            group: group.clone(),
            jobs: HashMap::new(),
        };
        self.job_groups.insert(self.curr_id, job_group);
        self.curr_id = self.curr_id + 1;

        Ok(())
    }

    pub fn set_group_state(&mut self, group: &Group) -> Result<()> {
        if let Some(job_group) = self.job_groups.get_mut(&group.get_group_id()) {
            (*job_group).group = group.clone();
        };
        Ok(())
    }

    pub fn add_group_job(&mut self, group: &Group, job: &Job) -> Result<()> {
        if let Some(job_group) = self.job_groups.get_mut(&group.get_group_id()) {
            (*job_group).jobs.insert(job.get_id(), job.clone());
        };
        Ok(())
    }
}
