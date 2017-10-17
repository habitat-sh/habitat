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

extern crate protobuf;
use self::protobuf::RepeatedField;
use jobsrv::data_store::DataStore;
use protocol::jobsrv;

#[test]
fn migration() {
    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
}

#[test]
fn create_job() {
    let mut job = jobsrv::Job::new();
    job.mut_project().set_vcs_type(String::from("git"));
    job.mut_project().set_vcs_data(String::from(
        "http://github.com/habitat-sh/habitat",
    ));
    job.mut_project().set_name("core/habitat".to_string());
    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
    ds.create_job(&mut job).expect("Failed to create a job");
}

fn test_job() -> jobsrv::Job {
    let mut job = jobsrv::Job::new();
    job.set_id(0);
    job.mut_project().set_vcs_type(String::from("git"));
    job.mut_project().set_vcs_data(String::from(
        "http://github.com/habitat-sh/habitat",
    ));
    job.mut_project().set_name("core/habitat".to_string());
    job
}

#[test]
fn get_job() {
    let mut job = test_job();
    let mut job2 = test_job();
    job2.mut_project().set_vcs_installation_id(1);
    let mut job3 = test_job();

    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
    let rjob1 = ds.create_job(&mut job).expect("Failed to create a job");
    let rjob2 = ds.create_job(&mut job2).expect("Failed to create a job");
    let rjob3 = ds.create_job(&mut job3).expect("Failed to create a job");
    let mut get_job = jobsrv::JobGet::new();
    get_job.set_id(rjob1.get_id());
    let j1 = ds.get_job(&get_job).expect("Failed to get job 0").expect(
        "Job should exist",
    );

    get_job.set_id(rjob2.get_id());
    let j2 = ds.get_job(&get_job).expect("Failed to get job 2").expect(
        "Job should exist",
    );

    get_job.set_id(rjob3.get_id());
    let j3 = ds.get_job(&get_job).expect("Failed to get job 3").expect(
        "Job should exist",
    );
    assert!(j1.get_id() != 0);
    assert!(!j1.get_project().has_vcs_installation_id());
    assert!(j2.get_id() != 0);
    assert_eq!(j2.get_project().get_vcs_installation_id(), 1);
    assert!(j3.get_id() != 0);
    assert!(!j3.get_project().has_vcs_installation_id());
}

#[test]
fn get_job_does_not_exist() {
    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
    let mut get_job = jobsrv::JobGet::new();
    get_job.set_id(0);
    let result = ds.get_job(&get_job).expect("Failed to get job");
    assert!(result.is_none());
}

#[test]
fn pending_jobs() {
    let mut job1 = test_job();
    let mut job2 = test_job();
    let mut job3 = test_job();
    let mut job4 = test_job();
    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
    let rjob1 = ds.create_job(&mut job1).expect("Failed to create job");
    let _rjob2 = ds.create_job(&mut job2).expect("Failed to create job");
    let _rjob3 = ds.create_job(&mut job3).expect("Failed to create job");
    let _rjob4 = ds.create_job(&mut job4).expect("Failed to create job");

    // Get one job, it should be FIFO, and it should have its status set to Dispatched
    let pending_jobs = ds.pending_jobs(1).expect("Failed to get pendings job");
    assert_eq!(pending_jobs.len(), 1, "Failed to find a pending job");
    assert_eq!(
        pending_jobs[0].get_id(),
        rjob1.get_id(),
        "First in is not first out"
    );

    let mut get_job = jobsrv::JobGet::new();
    get_job.set_id(rjob1.get_id());

    let job1_dispatched = ds.get_job(&get_job)
        .expect("Failed to get job entry")
        .expect("Failed to find the job entry");
    assert_eq!(job1_dispatched.get_state(), jobsrv::JobState::Dispatched);

    // Get the remaining jobs; a larger number results in the total set
    let remaining_jobs = ds.pending_jobs(5).expect(
        "Failed to get remaining pending jobs",
    );
    assert_eq!(
        remaining_jobs.len(),
        3,
        "Failed to get all the remaining jobs"
    );

    // No jobs returns an empty array
    let no_jobs = ds.pending_jobs(100).expect(
        "Failed to get empty pending jobs",
    );
    assert_eq!(no_jobs.len(), 0);
}

#[test]
fn update_job() {
    let mut job1 = test_job();
    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
    let mut rjob1 = ds.create_job(&mut job1).expect("Failed to create job");

    let mut get_job = jobsrv::JobGet::new();
    get_job.set_id(rjob1.get_id());
    let pending_job = ds.get_job(&get_job)
        .expect("Failed to get job from database")
        .expect("No job found");
    assert_eq!(pending_job.get_state(), jobsrv::JobState::Pending);

    rjob1.set_state(jobsrv::JobState::Failed);
    ds.update_job(&rjob1).expect("Failed to update job state");
    let failed_job = ds.get_job(&get_job)
        .expect("Failed to get job from database")
        .expect("No job found");
    assert_eq!(failed_job.get_state(), jobsrv::JobState::Failed);
}

#[test]
fn create_job_group() {
    let project_names = vec![(String::from("Foo/Bar"), String::from("Foo/Bar/0/Baz"))];
    let mut msg = jobsrv::JobGroupSpec::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);
    ds.create_job_group(&msg, project_names).expect(
        "Failed to create a group",
    );
}

#[test]
fn get_job_group() {
    let project_names = vec![(String::from("Foo/Bar"), String::from("Foo/Bar/0/Baz"))];
    let mut msg = jobsrv::JobGroupSpec::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);
    let group1 = ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );
    let group2 = ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );
    let group3 = ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );

    let mut get_msg1 = jobsrv::JobGroupGet::new();
    get_msg1.set_group_id(group1.get_id());
    let mut get_msg2 = jobsrv::JobGroupGet::new();
    get_msg2.set_group_id(group2.get_id());
    let mut get_msg3 = jobsrv::JobGroupGet::new();
    get_msg3.set_group_id(group3.get_id());

    let g1 = ds.get_job_group(&get_msg1)
        .expect("Failed to get group 1")
        .expect("Group should exist");
    let g2 = ds.get_job_group(&get_msg2)
        .expect("Failed to get group 2")
        .expect("Group should exist");
    let g3 = ds.get_job_group(&get_msg3)
        .expect("Failed to get group 3")
        .expect("Group should exist");

    assert!(g1.get_id() != 0);
    assert!(g2.get_id() != 0);
    assert!(g3.get_id() != 0);
}

#[test]
fn get_group_does_not_exist() {
    let ds = datastore_test!(DataStore);
    let mut get_msg = jobsrv::JobGroupGet::new();
    get_msg.set_group_id(0);

    let result = ds.get_job_group(&get_msg).expect("Failed to get group");
    assert!(result.is_none());
}

#[test]
fn pending_groups() {
    let project_names = vec![(String::from("Foo/Bar"), String::from("Foo/Bar/0/Baz"))];
    let mut msg = jobsrv::JobGroupSpec::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);

    let group1 = ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );
    ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );
    ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );

    // Get one group, it should be FIFO, and it should have its state set to Dispatching
    let pending_groups = ds.pending_job_groups(1).expect(
        "Failed to get pendings group",
    );
    assert_eq!(pending_groups.len(), 1, "Failed to find a pending group");
    assert_eq!(
        pending_groups[0].get_id(),
        group1.get_id(),
        "First in is not first out"
    );

    let mut get_msg1 = jobsrv::JobGroupGet::new();
    get_msg1.set_group_id(group1.get_id());

    let group1_dispatched = ds.get_job_group(&get_msg1)
        .expect("Failed to get group entry")
        .expect("Failed to find the group entry");
    assert_eq!(
        group1_dispatched.get_state(),
        jobsrv::JobGroupState::GroupDispatching
    );

    // Get the remaining groups; a larger number results in the total set
    let remaining_groups = ds.pending_job_groups(5).expect(
        "Failed to get remaining pending groups",
    );
    assert_eq!(
        remaining_groups.len(),
        2,
        "Failed to get all the remaining groups"
    );

    // No groups returns an empty array
    let no_groups = ds.pending_job_groups(100).expect(
        "Failed to get empty pending groups",
    );
    assert_eq!(no_groups.len(), 0);
}

#[test]
fn set_job_group_state() {
    let project_names = vec![(String::from("Foo/Bar"), String::from("Foo/Bar/0/Baz"))];
    let mut msg = jobsrv::JobGroupSpec::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);

    let group = ds.create_job_group(&msg, project_names.clone()).expect(
        "Failed to create a group",
    );

    let mut get_msg = jobsrv::JobGroupGet::new();
    get_msg.set_group_id(group.get_id());

    let pending_group = ds.get_job_group(&get_msg)
        .expect("Failed to get group from database")
        .expect("No group found");
    assert_eq!(
        pending_group.get_state(),
        jobsrv::JobGroupState::GroupPending
    );

    ds.set_job_group_state(group.get_id(), jobsrv::JobGroupState::GroupComplete)
        .expect("Failed to update group state");

    let completed_group = ds.get_job_group(&get_msg)
        .expect("Failed to get group from database")
        .expect("No group found");
    assert_eq!(
        completed_group.get_state(),
        jobsrv::JobGroupState::GroupComplete
    );
}

#[test]
fn create_graph_package() {
    let mut msg = jobsrv::JobGraphPackageCreate::new();
    msg.set_ident(String::from("Foo/Bar/123/456"));
    msg.set_target(String::from("quantum"));

    let mut deps = RepeatedField::new();
    deps.push(String::from("Foo/Baz/321/654"));
    msg.set_deps(deps);

    let ds = datastore_test!(DataStore);
    let package = ds.create_job_graph_package(&msg).expect(
        "Failed to create a graph package",
    );

    assert_eq!(package.get_ident(), "Foo/Bar/123/456");
    assert_eq!(package.get_target(), "quantum");

    let packages = ds.get_job_graph_packages().expect(
        "Failed to get graph packages",
    );
    assert_eq!(packages.len(), 1);
    assert_eq!(packages.last().unwrap().get_ident(), "Foo/Bar/123/456");
}

#[test]
fn get_graph_stats() {
    let ds = datastore_test!(DataStore);

    let mut stats_msg = jobsrv::JobGraphPackageStatsGet::new();
    stats_msg.set_origin(String::from("Foo"));

    let stats = ds.get_job_graph_package_stats(&stats_msg).expect(
        "Failed to get graph stats",
    );

    assert_eq!(stats.get_plans(), 0);
    assert_eq!(stats.get_builds(), 0);
    assert_eq!(stats.get_unique_packages(), 0);
}
