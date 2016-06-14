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

use setup;
use util::supervisor::Supervisor;

// Start two supervisors, see that they block waiting for quorum. Then start a third and see the
// quorum reached.
#[ignore]
#[test]
fn minimum_quorum() {
    setup::origin_setup();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new_with_topology("leader");
    let sup_b = Supervisor::with_peer_topology(&sup_a, "leader");

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    assert_eq!(sup_a.minimum_quorum(), false);
    assert_eq!(sup_b.minimum_quorum(), false);

    let sup_c = Supervisor::with_peer_topology(&sup_a, "leader");
    assert!(sup_c.wait_for_alive(&sup_a));
    assert!(sup_c.wait_for_alive(&sup_b));

    assert_eq!(sup_a.minimum_quorum(), true);
    assert_eq!(sup_b.minimum_quorum(), true);
    assert_eq!(sup_c.minimum_quorum(), true);
}

// Start three supervisors. See that they elect one, and only one, leader.
#[ignore]
#[test]
fn elects_a_leader() {
    setup::origin_setup();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new_with_topology("leader");
    let sup_b = Supervisor::with_peer_topology(&sup_a, "leader");
    let sup_c = Supervisor::with_peer_topology(&sup_a, "leader");

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_a.wait_for_alive(&sup_c));
    assert!(sup_b.wait_for_alive(&sup_a));
    assert!(sup_b.wait_for_alive(&sup_c));
    assert!(sup_c.wait_for_alive(&sup_a));
    assert!(sup_c.wait_for_alive(&sup_b));

    // Make sure we all have leaders
    assert!(sup_a.wait_for_leader());
    assert!(sup_b.wait_for_leader());
    assert!(sup_c.wait_for_leader());

    // Make sure we have the same leader
    assert_eq!(sup_a.leader(), sup_b.leader());
    assert_eq!(sup_a.leader(), sup_c.leader());
}

// Start three supervisors; once they have a leader, kill it. The remaining two should elect a new
// leader from amongst themselves.
//
// This test is ignored because it is consistently inconsistent.
#[test]
#[ignore]
fn elects_on_failure() {
    setup::origin_setup();
    setup::simple_service_gossip();

    let mut sup_a = Supervisor::new_with_topology("leader");
    let mut sup_b = Supervisor::with_peer_topology(&sup_a, "leader");
    let mut sup_c = Supervisor::with_peer_topology(&sup_a, "leader");

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_a.wait_for_alive(&sup_c));
    assert!(sup_b.wait_for_alive(&sup_a));
    assert!(sup_b.wait_for_alive(&sup_c));
    assert!(sup_c.wait_for_alive(&sup_a));
    assert!(sup_c.wait_for_alive(&sup_b));

    // Make sure we all have a leader
    assert!(sup_a.wait_for_leader());
    assert!(sup_b.wait_for_leader());
    assert!(sup_c.wait_for_leader());

    // Get the term of the current election
    let term = sup_a.term();

    let og_leader_id = {
        sup_a.stop_if_leader()
            .or(sup_b.stop_if_leader())
            .or(sup_c.stop_if_leader())
            .expect("We had a leader, but then.. we couldn't find it in our list")
    };

    if sup_a.id != og_leader_id {
        assert!(sup_a.wait_for_term_newer_than(term));
        assert!(sup_a.wait_for_leader());
        assert!(sup_a.leader() != og_leader_id);
    }

    if sup_b.id != og_leader_id {
        assert!(sup_b.wait_for_term_newer_than(term));
        assert!(sup_b.wait_for_leader());
        assert!(sup_b.leader() != og_leader_id);
    }

    if sup_c.id != og_leader_id {
        assert!(sup_c.wait_for_term_newer_than(term));
        assert!(sup_c.wait_for_leader());
        assert!(sup_c.leader() != og_leader_id);
    }
}

// Start five supervisors. Partition the leader and one follower. This results in the leader not
// having quorum, stopping the service, and the three supervisors with quorum electing a new leader
// amongst themselves.
//
// We are ignoring this test for now, as it is super unreliable.
#[test]
#[ignore]
fn leader_without_quorum_stops_service_remainder_elects_new_leader() {
    setup::origin_setup();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new_with_permanent_topology("leader");
    let sup_b = Supervisor::with_peer_permanent_topology(&sup_a, "leader");
    let sup_c = Supervisor::with_peer_permanent_topology(&sup_a, "leader");
    let sup_d = Supervisor::with_peer_permanent_topology(&sup_a, "leader");
    let sup_e = Supervisor::with_peer_permanent_topology(&sup_a, "leader");

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_a.wait_for_alive(&sup_c));
    assert!(sup_a.wait_for_alive(&sup_d));
    assert!(sup_a.wait_for_alive(&sup_e));

    assert!(sup_b.wait_for_alive(&sup_a));
    assert!(sup_b.wait_for_alive(&sup_c));
    assert!(sup_b.wait_for_alive(&sup_d));
    assert!(sup_b.wait_for_alive(&sup_e));

    assert!(sup_c.wait_for_alive(&sup_a));
    assert!(sup_c.wait_for_alive(&sup_b));
    assert!(sup_c.wait_for_alive(&sup_d));
    assert!(sup_c.wait_for_alive(&sup_e));

    assert!(sup_d.wait_for_alive(&sup_a));
    assert!(sup_d.wait_for_alive(&sup_b));
    assert!(sup_d.wait_for_alive(&sup_c));
    assert!(sup_d.wait_for_alive(&sup_e));

    assert!(sup_e.wait_for_alive(&sup_a));
    assert!(sup_e.wait_for_alive(&sup_b));
    assert!(sup_e.wait_for_alive(&sup_c));
    assert!(sup_e.wait_for_alive(&sup_d));

    // Make sure we all have a leader
    assert!(sup_a.wait_for_leader());
    assert!(sup_b.wait_for_leader());
    assert!(sup_c.wait_for_leader());
    assert!(sup_d.wait_for_leader());
    assert!(sup_e.wait_for_leader());

    // Get the current leader id
    let og_leader_id = sup_a.leader();

    // The current leader supervisor
    let og_leader = {
        if sup_a.id == og_leader_id {
            &sup_a
        } else if sup_b.id == og_leader_id {
            &sup_b
        } else if sup_c.id == og_leader_id {
            &sup_c
        } else if sup_d.id == og_leader_id {
            &sup_d
        } else if sup_e.id == og_leader_id {
            &sup_e
        } else {
            panic!("No supervisor for leader id {}", og_leader_id);
        }
    };

    // The peer we are going to partition off
    let og_leader_peer = {
        if sup_a.id != og_leader_id {
            &sup_a
        } else {
            &sup_b
        }
    };

    fn in_leader_group(sup: &Supervisor, ogl: &Supervisor, ogp: &Supervisor) -> bool {
        sup.id == ogl.id || sup.id == ogp.id
    }

    // Get the term of the current election
    let term = sup_a.term();

    // Partition everyone from the leader and its peer
    if !in_leader_group(&sup_a, og_leader, og_leader_peer) {
        sup_a.netsplit(&og_leader);
        sup_a.netsplit(&og_leader_peer);
    };

    if !in_leader_group(&sup_b, og_leader, og_leader_peer) {
        sup_b.netsplit(&og_leader);
        sup_b.netsplit(&og_leader_peer);
    };

    if !in_leader_group(&sup_c, og_leader, og_leader_peer) {
        sup_c.netsplit(&og_leader);
        sup_c.netsplit(&og_leader_peer);
    };

    if !in_leader_group(&sup_d, og_leader, og_leader_peer) {
        sup_d.netsplit(&og_leader);
        sup_d.netsplit(&og_leader_peer);
    };

    if !in_leader_group(&sup_e, og_leader, og_leader_peer) {
        sup_e.netsplit(&og_leader);
        sup_e.netsplit(&og_leader_peer);
    };

    // Check for confirmed down
    if !in_leader_group(&sup_a, og_leader, og_leader_peer) {
        assert!(sup_a.wait_for_confirmed(&og_leader));
        assert!(sup_a.wait_for_confirmed(&og_leader_peer));
    };

    if !in_leader_group(&sup_b, og_leader, og_leader_peer) {
        assert!(sup_b.wait_for_confirmed(&og_leader));
        assert!(sup_b.wait_for_confirmed(&og_leader_peer));
    };

    if !in_leader_group(&sup_c, og_leader, og_leader_peer) {
        assert!(sup_c.wait_for_confirmed(&og_leader));
        assert!(sup_c.wait_for_confirmed(&og_leader_peer));
    };

    if !in_leader_group(&sup_d, og_leader, og_leader_peer) {
        assert!(sup_d.wait_for_confirmed(&og_leader));
        assert!(sup_d.wait_for_confirmed(&og_leader_peer));
    };

    if !in_leader_group(&sup_e, og_leader, og_leader_peer) {
        assert!(sup_e.wait_for_confirmed(&og_leader));
        assert!(sup_e.wait_for_confirmed(&og_leader_peer));
    };

    // Check that the right behavior happened on both sides; if you are on the leader or its peer,
    // you should be running an election that never ends. If you are not hte leader or its peer,
    // you should have run an election that elects a new leader.
    if in_leader_group(&sup_a, &og_leader, &og_leader_peer) {
        if sup_a.id == og_leader.id {
            assert!(sup_a.wait_for_status_down());
        } else {
        }
    } else {
        assert!(sup_a.wait_for_term_newer_than(term));
        assert!(sup_a.wait_for_leader());
        assert!(sup_a.leader() != og_leader.id);
    }

    if in_leader_group(&sup_b, &og_leader, &og_leader_peer) {
        if sup_b.id == og_leader.id {
            assert!(sup_b.wait_for_status_down());
        } else {
        }
    } else {
        assert!(sup_b.wait_for_term_newer_than(term));
        assert!(sup_b.wait_for_leader());
        assert!(sup_b.leader() != og_leader.id);
    }

    if in_leader_group(&sup_c, &og_leader, &og_leader_peer) {
        if sup_c.id == og_leader.id {
            assert!(sup_c.wait_for_status_down());
        } else {
        }
    } else {
        assert!(sup_c.wait_for_term_newer_than(term));
        assert!(sup_c.wait_for_leader());
        assert!(sup_c.leader() != og_leader.id);
    }

    if in_leader_group(&sup_d, &og_leader, &og_leader_peer) {
        if sup_d.id == og_leader.id {
            assert!(sup_d.wait_for_status_down());
        } else {
        }
    } else {
        assert!(sup_d.wait_for_term_newer_than(term));
        assert!(sup_d.wait_for_leader());
        assert!(sup_d.leader() != og_leader.id);
    }

    if in_leader_group(&sup_e, &og_leader, &og_leader_peer) {
        if sup_e.id == og_leader.id {
            assert!(sup_e.wait_for_status_down());
        } else {
        }
    } else {
        assert!(sup_e.wait_for_term_newer_than(term));
        assert!(sup_e.wait_for_leader());
        assert!(sup_e.leader() != og_leader.id);
    }

    // Rejoin
    if !in_leader_group(&sup_a, og_leader, og_leader_peer) {
        sup_a.netjoin(&og_leader);
        sup_a.netjoin(&og_leader_peer);
        assert!(sup_a.wait_for_alive(&og_leader));
        assert!(sup_a.wait_for_alive(&og_leader_peer));
    };

    if !in_leader_group(&sup_b, og_leader, og_leader_peer) {
        sup_b.netjoin(&og_leader);
        sup_b.netjoin(&og_leader_peer);
        assert!(sup_b.wait_for_alive(&og_leader));
        assert!(sup_b.wait_for_alive(&og_leader_peer));
    };

    if !in_leader_group(&sup_c, og_leader, og_leader_peer) {
        sup_c.netjoin(&og_leader);
        sup_c.netjoin(&og_leader_peer);
        assert!(sup_c.wait_for_alive(&og_leader));
        assert!(sup_c.wait_for_alive(&og_leader_peer));
    };

    if !in_leader_group(&sup_d, og_leader, og_leader_peer) {
        sup_d.netjoin(&og_leader);
        sup_d.netjoin(&og_leader_peer);
        assert!(sup_d.wait_for_alive(&og_leader));
        assert!(sup_d.wait_for_alive(&og_leader_peer));
    };

    if !in_leader_group(&sup_e, og_leader, og_leader_peer) {
        sup_e.netjoin(&og_leader);
        sup_e.netjoin(&og_leader_peer);
        assert!(sup_e.wait_for_alive(&og_leader));
        assert!(sup_e.wait_for_alive(&og_leader_peer));
    };

    // assert!(og_leader.wait_for_leader());
    // assert!(og_leader.wait_for_leader());
}
