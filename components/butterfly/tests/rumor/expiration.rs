use crate::btest;
use habitat_butterfly::member::Health;
use std::{env,
          thread,
          time::Duration};

#[test]
fn member_rumors_expire() {
    env_logger::try_init().ok();
    env::set_var("HAB_CONFIRMED_TIMEOUT_MS", "3000");
    env::set_var("HAB_DEPARTURE_TIMEOUT_MS", "3000");
    env::set_var("HAB_RUMOR_EXPIRE_THREAD_SLEEP_MS", "1000");
    env::set_var("HAB_RUMOR_EXPIRATION_SECS", "10");

    let mut net = btest::SwimNet::new(3);
    net.mesh();
    println!("Node 0 = {:?}", net[0]);
    println!("Node 1 = {:?}", net[1]);
    println!("Node 2 = {:?}", net[2]);
    assert_wait_for_health_of_mlr!(net, 0, Health::Alive);
    assert_wait_for_health_of_mlr!(net, 1, Health::Alive);
    assert_wait_for_health_of_mlr!(net, 2, Health::Alive);
    // assert_wait_for_health_of_mlr!(net, [0..3, 0..3], Health::Alive);
    // println!("Node 0 memberlist = {:?}", &net[0].member_list);
    // println!("Node 1 memberlist = {:?}", &net[1].member_list);
    // println!("Node 2 memberlist = {:?}", &net[2].member_list);
    assert_eq!(net[0].member_list.len_mlr(), 3);
    assert_eq!(net[1].member_list.len_mlr(), 3);
    assert_eq!(net[2].member_list.len_mlr(), 3);
    net[0].pause();
    assert_wait_for_health_of_mlr!(net, 0, Health::Suspect);
    assert_wait_for_health_of_mlr!(net, 0, Health::Confirmed);
    assert_wait_for_health_of_mlr!(net, 0, Health::Departed);
    thread::sleep(Duration::from_secs(12));
    assert_eq!(net[1].member_list.len_mlr(),
               1,
               "Node 1 should've purged Node 0 by now but its member list contains {} members \
                instead of {}",
               net[1].member_list.len_mlr(),
               2);
}

#[test]
fn service_rumors_expire() {
    assert!(true);
}

#[test]
fn service_config_rumors_expire() {
    assert!(true);
}

#[test]
fn service_file_rumors_expire() {
    assert!(true);
}

#[test]
fn election_rumors_expire() {
    assert!(true);
}

#[test]
fn election_update_rumors_expire() {
    assert!(true);
}
