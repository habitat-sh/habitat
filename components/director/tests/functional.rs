// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_director as director;
extern crate habitat_core as hcore;
extern crate tempdir;
extern crate time;

use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::str::FromStr;

use tempdir::TempDir;

use director::*;
use director::task::{Task, ExecContext, ExecParams};

// call a closure in a loop until it returns true
// or timeout after 30 seconds and return false
pub fn wait_until_true<F>(dc: &mut Task, some_fn: F) -> bool
    where F: Fn(&mut Task) -> bool
{
    let wait_duration = time::Duration::seconds(30);
    let current_time = time::now_utc().to_timespec();
    let stop_time = current_time + wait_duration;
    while time::now_utc().to_timespec() < stop_time {
        if some_fn(dc) {
            return true;
        }
    }
    false
}

// Create a Task for testing
fn get_test_dc(name: &str) -> (PathBuf, Task) {
    let tmp_service_path = TempDir::new(name).unwrap();
    let mut sd = ServiceDef::from_str("core.functional_test.somegroup.someorg").unwrap();
    sd.cli_args = Some("-v".to_string());
    let mut exec_ctx = ExecContext::default();
    exec_ctx.sup_path = PathBuf::from("/bin/bash");
    let tsp = tmp_service_path.into_path();
    let tsp2 = tsp.clone();
    exec_ctx.service_root = PathBuf::from(tsp);

    let exec_params = ExecParams::new(SocketAddrV4::from_str("127.0.0.1:9000").unwrap(),
                                      SocketAddrV4::from_str("127.0.0.1:8000").unwrap(),
                                      None);

    (tsp2, Task::new(exec_ctx, exec_params, sd))
}

/// This test starts a Task process using `bash -c sleep 1`
/// as it's executable. This allows me to check and see if the states
/// are correct before starting, when stopped, and when restarted.
/// The ExecContext allows us to change the name of the hab-sup
/// binary and the path to the service directory.
#[test]
fn task_state_test() {

    let (_tmp, mut dc) = get_test_dc("first");
    dc.service_def.cli_args = Some("-c sleep 1".to_string());

    // check unstarted state
    assert_eq!(None, dc.pid);
    assert_eq!(0, dc.starts);
    assert!(dc.is_down());

    // check started state
    dc.start().unwrap();
    assert_eq!(1, dc.starts);
    assert!(dc.pid.is_some());
    assert!(dc.is_up());
    assert!(dc.pid_file().is_file());
    // does the contents of the pidfile match dc.pid?
    assert_eq!(dc.pid.unwrap(), dc.read_pidfile().unwrap().unwrap());

    assert!(wait_until_true(&mut dc, |d| {
        d.check_process().unwrap();
        d.pid.is_none() && d.is_down()
    }));

    // pidfile shouldn't exist anymore
    assert!(!dc.pid_file().is_file());

    // make sure we can start it again
    dc.start().unwrap();
    assert_eq!(2, dc.starts);
    assert!(dc.pid.is_some());
    assert!(dc.is_up());
    assert!(dc.pid_file().is_file());
    // does the contents of the pidfile match dc.pid?
    assert_eq!(dc.pid.unwrap(), dc.read_pidfile().unwrap().unwrap());

    assert!(wait_until_true(&mut dc, |d| {
        d.check_process().unwrap();
        d.pid.is_none() && d.is_down()
    }));

    // pidfile shouldn't exist anymore
    assert!(!dc.pid_file().is_file());
}
