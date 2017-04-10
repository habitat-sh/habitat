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

use scheduler::data_store::DataStore;
use protocol::scheduler;
use protocol::jobsrv;

#[test]
fn migration() {
    let ds = datastore_test!(DataStore);
    ds.setup().expect("Failed to migrate data");
}

#[test]
fn create_group() {
    let project_names = vec![String::from("Foo/Bar")];
    let mut msg = scheduler::GroupCreate::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);
    ds.create_group(&msg, project_names)
        .expect("Failed to create a group");
}

#[test]
fn get_group() {
    let project_names = vec![String::from("Foo/Bar")];
    let mut msg = scheduler::GroupCreate::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);
    let group1 = ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");
    let group2 = ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");
    let group3 = ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");

    let mut get_msg1 = scheduler::GroupGet::new();
    get_msg1.set_group_id(group1.get_id());
    let mut get_msg2 = scheduler::GroupGet::new();
    get_msg2.set_group_id(group2.get_id());
    let mut get_msg3 = scheduler::GroupGet::new();
    get_msg3.set_group_id(group3.get_id());

    let g1 = ds.get_group(&get_msg1)
        .expect("Failed to get group 1")
        .expect("Group should exist");
    let g2 = ds.get_group(&get_msg2)
        .expect("Failed to get group 2")
        .expect("Group should exist");
    let g3 = ds.get_group(&get_msg3)
        .expect("Failed to get group 3")
        .expect("Group should exist");

    assert!(g1.get_id() != 0);
    assert!(g2.get_id() != 0);
    assert!(g3.get_id() != 0);
}

#[test]
fn get_group_does_not_exist() {
    let ds = datastore_test!(DataStore);
    let mut get_msg = scheduler::GroupGet::new();
    get_msg.set_group_id(0);

    let result = ds.get_group(&get_msg).expect("Failed to get group");
    assert!(result.is_none());
}

#[test]
fn pending_groups() {
    let project_names = vec![String::from("Foo/Bar")];
    let mut msg = scheduler::GroupCreate::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);

    let group1 = ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");
    ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");
    ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");

    // Get one group, it should be FIFO, and it should have its state set to Dispatching
    let pending_groups = ds.pending_groups(1)
        .expect("Failed to get pendings group");
    assert_eq!(pending_groups.len(), 1, "Failed to find a pending group");
    assert_eq!(pending_groups[0].get_id(),
               group1.get_id(),
               "First in is not first out");

    let mut get_msg1 = scheduler::GroupGet::new();
    get_msg1.set_group_id(group1.get_id());

    let group1_dispatched = ds.get_group(&get_msg1)
        .expect("Failed to get group entry")
        .expect("Failed to find the group entry");
    assert_eq!(group1_dispatched.get_state(),
               scheduler::GroupState::Dispatching);

    // Get the remaining groups; a larger number results in the total set
    let remaining_groups = ds.pending_groups(5)
        .expect("Failed to get remaining pending groups");
    assert_eq!(remaining_groups.len(),
               2,
               "Failed to get all the remaining groups");

    // No groups returns an empty array
    let no_groups = ds.pending_groups(100)
        .expect("Failed to get empty pending groups");
    assert_eq!(no_groups.len(), 0);
}

#[test]
fn set_group_state() {
    let project_names = vec![String::from("Foo/Bar")];
    let mut msg = scheduler::GroupCreate::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);

    let group = ds.create_group(&msg, project_names.clone())
        .expect("Failed to create a group");

    let mut get_msg = scheduler::GroupGet::new();
    get_msg.set_group_id(group.get_id());

    let pending_group = ds.get_group(&get_msg)
        .expect("Failed to get group from database")
        .expect("No group found");
    assert_eq!(pending_group.get_state(), scheduler::GroupState::Pending);

    ds.set_group_state(group.get_id(), scheduler::GroupState::Complete)
        .expect("Failed to update group state");

    let completed_group = ds.get_group(&get_msg)
        .expect("Failed to get group from database")
        .expect("No group found");
    assert_eq!(completed_group.get_state(), scheduler::GroupState::Complete);
}

#[test]
fn set_group_job_state() {
    let project_names = vec![String::from("Foo/Bar")];
    let mut msg = scheduler::GroupCreate::new();
    msg.set_origin(String::from("Foo"));
    msg.set_package(String::from("Bar"));

    let ds = datastore_test!(DataStore);

    let group = ds.create_group(&msg, project_names)
        .expect("Failed to create a group");

    let mut job = jobsrv::Job::new();
    job.set_id(100);
    job.set_owner_id(group.get_id());
    job.set_state(jobsrv::JobState::Complete);
    job.mut_project().set_name(String::from("Foo/Bar"));

    ds.set_group_job_state(&job)
        .expect("Failed to set group job state");

    let mut get_msg = scheduler::GroupGet::new();
    get_msg.set_group_id(group.get_id());

    let group = ds.get_group(&get_msg)
        .expect("Failed to get group from database")
        .expect("No group found");

    assert_eq!(group.get_projects().len(), 1);
    assert_eq!(group.get_projects().last().unwrap().get_state(),
               scheduler::ProjectState::Success);
}
