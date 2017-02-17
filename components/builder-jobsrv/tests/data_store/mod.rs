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

use jobsrv::data_store::DataStore;
use protocol::jobsrv;
use protocol::vault;

#[test]
fn migration() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
    });
}

#[test]
fn create_job() {
    let mut job = jobsrv::Job::new();
    let mut git = vault::VCSGit::new();
    git.set_url(String::from("http://github.com/habitat-sh/habitat"));
    job.mut_project().set_git(git);

    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        ds.create_job(&mut job).expect("Failed to create a job");
    });
}

fn test_job() -> jobsrv::Job {
    let mut job = jobsrv::Job::new();
    job.set_id(0);
    let mut git = vault::VCSGit::new();
    git.set_url(String::from("http://github.com/habitat-sh/habitat"));
    job.mut_project().set_git(git);
    job
}

#[test]
fn get_job() {
    let mut job = test_job();
    let mut job2 = test_job();
    let mut job3 = test_job();

    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        ds.create_job(&mut job).expect("Failed to create a job");
        ds.create_job(&mut job2).expect("Failed to create a job");
        ds.create_job(&mut job3).expect("Failed to create a job");
        let j1 = ds.get_job(job.get_id()).expect("Failed to get job 0").expect("Job should exist");
        let j2 = ds.get_job(job2.get_id()).expect("Failed to get job 2").expect("Job should exist");
        let j3 = ds.get_job(job3.get_id()).expect("Failed to get job 3").expect("Job should exist");
        assert!(j1.get_id() != 0);
        assert!(j2.get_id() != 0);
        assert!(j3.get_id() != 0);
    });
}

#[test]
fn get_job_does_not_exist() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        let result = ds.get_job(0).expect("Failed to get job");
        assert!(result.is_none());
    });
}

#[test]
fn pending_jobs() {
    let mut job1 = test_job();
    let mut job2 = test_job();
    let mut job3 = test_job();
    let mut job4 = test_job();
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        ds.create_job(&mut job1).expect("Failed to create job");
        ds.create_job(&mut job2).expect("Failed to create job");
        ds.create_job(&mut job3).expect("Failed to create job");
        ds.create_job(&mut job4).expect("Failed to create job");

        // Get one job, it should be FIFO, and it should have its status set to Dispatched
        let pending_jobs = ds.pending_jobs(1).expect("Failed to get pendings job");
        assert_eq!(pending_jobs.len(), 1, "Failed to find a pending job");
        assert_eq!(pending_jobs[0].get_id(),
                   job1.get_id(),
                   "First in is not first out");
        let job1_dispatched = ds.get_job(job1.get_id())
            .expect("Failed to get job entry")
            .expect("Failed to find the job entry");
        assert_eq!(job1_dispatched.get_state(), jobsrv::JobState::Dispatched);

        // Get the remaining jobs; a larger number results in the total set
        let remaining_jobs = ds.pending_jobs(5).expect("Failed to get remaining pending jobs");
        assert_eq!(remaining_jobs.len(),
                   3,
                   "Failed to get all the remaining jobs");

        // No jobs returns an empty array
        let no_jobs = ds.pending_jobs(100).expect("Failed to get empty pending jobs");
        assert_eq!(no_jobs.len(), 0);
    });
}

#[test]
fn set_job_state() {
    let mut job1 = test_job();
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        ds.create_job(&mut job1).expect("Failed to create job");
        let pending_job = ds.get_job(job1.get_id())
            .expect("Failed to get job from database")
            .expect("No job found");
        assert_eq!(pending_job.get_state(), jobsrv::JobState::Pending);

        job1.set_state(jobsrv::JobState::Failed);
        ds.set_job_state(&job1).expect("Failed to update job state");
        let failed_job = ds.get_job(job1.get_id())
            .expect("Failed to get job from database")
            .expect("No job found");
        assert_eq!(failed_job.get_state(), jobsrv::JobState::Failed);
    });
}
