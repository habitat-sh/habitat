// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use setup;
use util::supervisor::Supervisor;

// Start two supervisors, and make sure they see each other
#[test]
fn two_supervisors_link() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_a.has_member(&sup_b));
}

// Start two supervisors, stop one, make sure they see the failure
#[test]
fn two_supervisors_detect_failure() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_a.has_member(&sup_b));

    sup_b.docker.stop();

    // You need time for the connection to time out, then the pingreq fails
    assert!(sup_a.wait_for_suspect(&sup_b));

    // Then the grace window closes, and we call you dead
    assert!(sup_a.wait_for_confirmed(&sup_b));
}

// Start three supervisors, a, b, and c. A is linked to b, and b is linked to c. a, b, and c all
// see the other two members, even though they were not provided initially.
#[test]
fn members_are_gossiped() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);
    let sup_c = Supervisor::with_peer(&sup_b);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_b.has_member(&sup_c));
    assert!(sup_a.has_member(&sup_b));
    assert!(sup_a.has_member(&sup_c));
    assert!(sup_c.has_member(&sup_a));
    assert!(sup_c.has_member(&sup_b));
}

// Start three supervisors, a, b, c. Split a and c; confirm that neither a nor c is marked
// as anything but alive, confirming they are routing their gossip through b.
#[test]
fn routes_around_failure() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);
    let sup_c = Supervisor::with_peer(&sup_b);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_b.has_member(&sup_c));
    assert!(sup_a.has_member(&sup_b));
    assert!(sup_a.has_member(&sup_c));
    assert!(sup_c.has_member(&sup_a));
    assert!(sup_c.has_member(&sup_b));

    sup_a.netsplit(&sup_c);
    assert!(sup_a.keeps_member_alive(&sup_c));
    assert!(sup_c.keeps_member_alive(&sup_a));
}

// Incarnation update on suspicion. Start A, B, and C. A and C split from from B; B is marked
// suspect. A and C then rejoin B, and upon receipt of the suspect rumor about itself, B increments
// its incarnation and shares its Alive rumor. A and C then see B as alive again.
#[test]
fn incarnation_updates_on_suspicion() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);
    let sup_c = Supervisor::with_peer(&sup_b);

    // Make sure we are all alive
    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));
    assert!(sup_c.wait_for_alive(&sup_b));

    // Split a and c from b
    sup_a.netsplit(&sup_b);
    sup_c.netsplit(&sup_b);

    // B should be suspected
    assert!(sup_a.wait_for_suspect(&sup_b));
    assert!(sup_c.wait_for_suspect(&sup_b));

    // Rejoin and confirm b is alive
    sup_a.netjoin(&sup_b);
    sup_c.netjoin(&sup_b);

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_c.wait_for_alive(&sup_b));

    // Validate that the incarnation has increased
    assert_eq!(sup_b.incarnation(), 1);
}

// Ressurection and partition tolerance. Start a, and b; a and b are both permanent members. a
// splits from b; a is marked confirmed dead. The split is then joined, and both A and B update
// their incarnation and are then marked alive.
#[test]
fn ressurection_of_permanent_members() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new_with_permanent();
    let sup_b = Supervisor::with_peer_permanent(&sup_a);

    // Make sure we are both alive
    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    // Split them apart
    sup_a.netsplit(&sup_b);

    // Confirm they are dead
    assert!(sup_a.wait_for_confirmed(&sup_b));
    assert!(sup_b.wait_for_confirmed(&sup_a));

    // Rejoin and confirm they are alive
    sup_a.netjoin(&sup_b);

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    // Validate that the incarnation has increased
    assert_eq!(sup_a.incarnation(), 1);
    assert_eq!(sup_b.incarnation(), 1);
}

// If you ever find yourself completely isolated - you are alive, but every other peer is dead,
// pretend that every peer is permanent, until you are no longer isolated. This protects from the
// case where you are on your way to being partitioned, but you had requests in flight for a member
// that never lands - and you wind up isolated.
#[test]
fn isolated_members_find_a_way_to_rejoin() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);

    // Make sure we are both alive
    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    // Split them apart
    sup_a.netsplit(&sup_b);

    // Confirm they are dead
    assert!(sup_a.wait_for_confirmed(&sup_b));
    assert!(sup_b.wait_for_confirmed(&sup_a));

    // Rejoin and confirm they are alive
    sup_a.netjoin(&sup_b);

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    // Validate that the incarnation has increased
    assert_eq!(sup_a.incarnation(), 1);
    assert_eq!(sup_b.incarnation(), 1);
}
