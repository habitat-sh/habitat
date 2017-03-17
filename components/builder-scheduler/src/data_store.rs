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
use rand::{Rng, thread_rng};

use config::Config;
use error::Result;
use protocol::jobsrv::{self, Job, JobState};
use protocol::scheduler as proto;
use protocol::scheduler::Group;
use protobuf::RepeatedField;

#[derive(Debug, Clone)]
pub struct ProjectData {
    name: String,
    state: proto::ProjectState,
}

#[derive(Debug, Clone)]
pub struct GroupData {
    id: u64,
    state: proto::GroupState,
    projects: Vec<ProjectData>,
    jobs: Vec<Job>,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    groups: Vec<GroupData>,
    job_map: HashMap<u64, usize>,
    group_map: HashMap<u64, usize>,
}

impl DataStore {
    pub fn new(_: &Config) -> Result<DataStore> {
        Ok(DataStore {
               groups: Vec::new(),
               job_map: HashMap::new(),
               group_map: HashMap::new(),
           })
    }

    pub fn setup(&self) -> Result<()> {
        Ok(())
    }

    pub fn create_group(&mut self, project_names: Vec<String>) -> Result<Group> {
        let mut projects = Vec::new();
        for name in project_names {
            let project = ProjectData {
                name: name,
                state: proto::ProjectState::NotStarted,
            };
            projects.push(project);
        }

        let mut rng = thread_rng();
        let id = rng.gen::<u64>();

        let group_data = GroupData {
            id: id,
            state: proto::GroupState::Pending,
            projects: projects,
            jobs: Vec::new(),
        };

        self.groups.push(group_data.clone());

        assert!(!self.group_map.contains_key(&id));
        self.group_map.insert(id, self.groups.len() - 1);

        println!("Group created: {:?}", group_data);

        Ok(DataStore::group_data_to_group(&group_data))
    }

    pub fn get_group(&self, group_id: u64) -> Option<Group> {
        if self.group_map.contains_key(&group_id) {
            let index = *self.group_map.get(&group_id).unwrap();
            assert!(self.groups[index].id == group_id);

            Some(DataStore::group_data_to_group(&self.groups[index]))
        } else {
            warn!("Group id {} not found", group_id);
            None
        }
    }

    fn group_data_to_group(group_data: &GroupData) -> Group {
        let mut group = proto::Group::new();

        group.set_id(group_data.id);
        group.set_state(group_data.state);

        let mut projects = RepeatedField::new();
        for project_data in group_data.projects.iter() {
            let mut project = proto::Project::new();
            project.set_name(project_data.name.clone());
            project.set_state(project_data.state);
            projects.push(project);
        }
        group.set_projects(projects);

        let mut jobs = RepeatedField::new();
        for group_job in group_data.jobs.iter() {
            jobs.push(group_job.clone());
        }
        group.set_jobs(jobs);

        group
    }

    pub fn set_group_state(&mut self, group_id: u64, group_state: proto::GroupState) -> Result<()> {
        if self.group_map.contains_key(&group_id) {
            println!("Updating group state, id: {}, state: {:?}",
                     group_id,
                     group_state);

            let index = *self.group_map.get(&group_id).unwrap();
            assert!(self.groups[index].id == group_id);
            self.groups[index].state = group_state;
        } else {
            warn!("Group id {} not found", group_id);
        }
        Ok(())
    }

    pub fn add_group_job(&mut self, group_id: u64, job: &Job) -> Result<()> {
        if self.group_map.contains_key(&group_id) {
            let job_id = job.get_id();
            println!("Adding job id {} to group {}", job_id, group_id);

            let index = *self.group_map.get(&group_id).unwrap();
            assert!(self.groups[index].id == group_id);

            assert!(!self.job_map.contains_key(&job_id));
            self.groups[index].jobs.push(job.clone());
            self.job_map.insert(job_id, index);
            self.set_group_job_state(job_id, job.get_state()).unwrap();
        } else {
            warn!("Group id {} not found", group_id);
        }

        Ok(())
    }

    pub fn find_group_id_for_job(&self, job_id: u64) -> Option<u64> {
        if self.job_map.contains_key(&job_id) {
            let index = *self.job_map.get(&job_id).unwrap();

            Some(self.groups[index].id)
        } else {
            warn!("Job id {} not found", job_id);
            None
        }
    }

    pub fn set_group_job_state(&mut self, job_id: u64, job_state: JobState) -> Result<()> {
        if self.job_map.contains_key(&job_id) {
            let index = *self.job_map.get(&job_id).unwrap();

            println!("Updating job status, job id: {}, state: {:?}",
                     job_id,
                     job_state);

            let mut project_name = String::new();
            for job_elem in self.groups[index].jobs.iter_mut() {
                if job_elem.get_id() == job_id {
                    job_elem.set_state(job_state);
                    project_name = String::from(job_elem.get_project().get_name());
                    break;
                }
            }

            for project_elem in self.groups[index].projects.iter_mut() {
                if project_elem.name == project_name {
                    let project_state = match job_state {
                        jobsrv::JobState::Complete => proto::ProjectState::Success,
                        jobsrv::JobState::Failed |
                        jobsrv::JobState::Rejected => proto::ProjectState::Failure,
                        _ => proto::ProjectState::InProgress,
                    };

                    project_elem.state = project_state;
                    break;
                }
            }
        } else {
            warn!("Job id {} not found", job_id);
        }

        Ok(())
    }

    pub fn pending_groups(&mut self, count: i32) -> Result<Vec<Group>> {
        let mut groups = Vec::new();
        let mut curr = 0;

        for group_data in self.groups.iter_mut() {
            if group_data.state == proto::GroupState::Pending {
                group_data.state = proto::GroupState::Dispatching;
                groups.push(DataStore::group_data_to_group(&group_data));

                curr = curr + 1;
                if curr >= count {
                    break;
                };
            }
        }

        Ok(groups)
    }
}
