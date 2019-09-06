#[macro_use]
mod common;
mod encryption;
mod rumor;

use common as btest;
use habitat_butterfly::{self,
                        member::Health};

#[test]
fn two_members_meshed_confirm_one_member() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    assert_wait_for_health_of_mlr!(net, 0, 1, Health::Alive);
    assert_wait_for_health_of_mlr!(net, 1, 0, Health::Alive);
    net[0].pause();
    assert_wait_for_health_of_mlr!(net, 1, 0, Health::Suspect);
    assert_wait_for_health_of_mlr!(net, 1, 0, Health::Confirmed);
}

#[test]
fn six_members_meshed_confirm_one_member() {
    let mut net = btest::SwimNet::new(6);
    net.mesh();
    net[0].pause();
    assert_wait_for_health_of_mlr!(net, 0, Health::Confirmed);
}

#[test]
fn six_members_meshed_partition_one_node_from_another_node_remains_alive() {
    let mut net = btest::SwimNet::new(6);
    net.mesh();
    net.block(0, 1);
    net.wait_for_rounds(2);
    assert_wait_for_health_of_mlr!(net, 1, Health::Alive);
}

#[test]
fn six_members_meshed_partition_half_of_nodes_from_each_other_both_sides_confirmed() {
    let mut net = btest::SwimNet::new(6);
    net.mesh();
    assert_wait_for_health_of_mlr!(net, 0, Health::Alive);
    net.partition(0..3, 3..6);
    assert_wait_for_health_of_mlr!(net, [0..3, 3..6], Health::Confirmed);
}

#[test]
fn six_members_unmeshed_become_fully_meshed_via_gossip() {
    let mut net = btest::SwimNet::new(6);
    net.connect(0, 1);
    net.connect(1, 2);
    net.connect(2, 3);
    net.connect(3, 4);
    net.connect(4, 5);
    assert_wait_for_health_of_mlr!(net, [0..6, 0..6], Health::Alive);
}

#[test]
fn six_members_unmeshed_confirm_one_member() {
    let mut net = btest::SwimNet::new(6);
    net.connect(0, 1);
    net.connect(1, 2);
    net.connect(2, 3);
    net.connect(3, 4);
    net.connect(4, 5);
    assert_wait_for_health_of_mlr!(net, [0..6, 0..6], Health::Alive);
    net[0].pause();
    assert_wait_for_health_of_mlr!(net, 0, Health::Confirmed);
}

#[test]
fn six_members_unmeshed_partition_and_rejoin_no_persistent_peers() {
    let mut net = btest::SwimNet::new(6);
    net.connect(0, 1);
    net.connect(1, 2);
    net.connect(2, 3);
    net.connect(3, 4);
    net.connect(4, 5);
    assert_wait_for_health_of_mlr!(net, [0..6, 0..6], Health::Alive);
    net.partition(0..3, 3..6);
    assert_wait_for_health_of_mlr!(net, [0..3, 3..6], Health::Confirmed);
    net.unpartition(0..3, 3..6);
    assert_wait_for_health_of_mlr!(net, [0..3, 3..6], Health::Confirmed);
}

#[test]
fn six_members_unmeshed_partition_and_rejoin_persistent_peers() {
    let mut net = btest::SwimNet::new(6);
    net[0].member
          .write()
          .expect("Member lock is poisoned")
          .set_persistent();
    net[4].member
          .write()
          .expect("Member lock is poisoned")
          .set_persistent();
    net.connect(0, 1);
    net.connect(1, 2);
    net.connect(2, 3);
    net.connect(3, 4);
    net.connect(4, 5);
    assert_wait_for_health_of_mlr!(net, [0..6, 0..6], Health::Alive);
    net.partition(0..3, 3..6);
    assert_wait_for_health_of_mlr!(net, [0..3, 3..6], Health::Confirmed);
    net.unpartition(0..3, 3..6);
    assert_wait_for_health_of_mlr!(net, [0..3, 3..6], Health::Alive);
}

#[test]
fn six_members_unmeshed_allows_graceful_departure() {
    let mut net = btest::SwimNet::new(6);
    net.connect(0, 1);
    net.connect(1, 2);
    net.connect(2, 3);
    net.connect(3, 4);
    net.connect(4, 5);
    assert_wait_for_health_of_mlr!(net, [0..6, 0..6], Health::Alive);
    net[0].set_departed_mlw();
    net[0].pause();
    assert_wait_for_health_of_mlr!(net, 0, Health::Departed);
}

#[test]
fn ten_members_meshed_confirm_one_member() {
    let mut net = btest::SwimNet::new(10);
    net.mesh();
    net[0].pause();
    assert_wait_for_health_of_mlr!(net, 0, Health::Confirmed);
}
