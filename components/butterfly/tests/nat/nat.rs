use std::collections::HashMap;

use nat::{TestNetworkSwitchBoard, ZoneID};

use habitat_butterfly::member::Health;

// Convenient macro for forcing the coercion of arrays into
// slices. Useful when creating a slice of AsRef<T>, from a slice of
// slices. Having a slice of arrays would work only when all the array
// have the same length.
macro_rules! slc {
    ($($x:expr),*) => {
        &([$($x,)*])[..]
    };
    // This is to allow a trailing comma.
    ($($x:expr),*,) => {
        slc![$($x),*]
    }
}

// Convenient macro for inline creation of hashmaps.
macro_rules! hm(
    {$($key:expr => $value:expr),+} => {
        {
            [
                $(
                    ($key, $value),
                )+
            ].iter().cloned().collect::<HashMap<_, _>>()
        }
    };
    // This form of the macro is to allow the leading comma.
    { $($key:expr => $value:expr),+, } => {
        hm!{ $($key => $value),+ }
    };
);

#[test]
fn servers_establish_the_same_zone_few() {
    let switch_board = TestNetworkSwitchBoard::new();
    let zone = ZoneID::new(1);
    let server0 = switch_board.start_server_in_zone(zone);
    let server1 = switch_board.start_server_in_zone(zone);

    server0.talk_to(&[&server1]);
    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(switch_board.wait_for_same_settled_zone(&[&server0, &server1]));
}

#[test]
fn servers_establish_the_same_zone_many() {
    let switch_board = TestNetworkSwitchBoard::new();
    let zone = ZoneID::new(1);
    let server0 = switch_board.start_server_in_zone(zone);
    let server1 = switch_board.start_server_in_zone(zone);
    let server2 = switch_board.start_server_in_zone(zone);
    let server3 = switch_board.start_server_in_zone(zone);
    let server4 = switch_board.start_server_in_zone(zone);
    let server5 = switch_board.start_server_in_zone(zone);

    server0.talk_to(&[&server1, &server2, &server3]);
    server2.talk_to(&[&server3, &server4, &server5]);
    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(switch_board.wait_for_same_settled_zone(&[
        &server0, &server1, &server2, &server3, &server4, &server5,
    ]));
}

#[test]
fn sibling_zones_get_merged() {
    let switch_board = TestNetworkSwitchBoard::new();
    let zone = ZoneID::new(1);
    let server0a = switch_board.start_server_in_zone(zone);
    let server0b = switch_board.start_server_in_zone(zone);
    let server1a = switch_board.start_server_in_zone(zone);
    let server1b = switch_board.start_server_in_zone(zone);

    server0a.talk_to(&[&server0b]);
    server1a.talk_to(&[&server1b]);
    assert!(switch_board.wait_for_health_of_those(Health::Alive, &[&server0a, &server0b]));
    assert!(switch_board.wait_for_health_of_those(Health::Alive, &[&server1a, &server1b]));
    assert!(
        switch_board
            .wait_for_splitted_same_zone(&[&[&server0a, &server0b], &[&server1a, &server1b],])
    );

    let server2 = switch_board.start_server_in_zone(zone);

    server2.talk_to(&[&server0a, &server1a]);
    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(
        switch_board
            .wait_for_same_settled_zone(&[&server0a, &server0b, &server1a, &server1b, &server2])
    );
}

#[test]
fn different_zones_get_different_ids_few() {
    let switch_board = TestNetworkSwitchBoard::new();
    let parent_zone = ZoneID::new(1);
    let child_zone = ZoneID::new(2);
    let mut nat = switch_board.setup_nat(
        parent_zone,
        child_zone,
        //None,
    );
    let hole0 = nat.punch_hole();
    let parent_server0 = switch_board.start_server_in_zone(parent_zone);
    let child_server0 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole0.into_additional_address(),
        },
    );

    nat.make_route(hole0, child_server0.addr);
    parent_server0.talk_to(&[&hole0]);
    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(switch_board.wait_for_disjoint_settled_zones(&[&[&parent_server0], &[&child_server0]]));
}

#[test]
fn different_zones_get_different_ids_many() {
    let switch_board = TestNetworkSwitchBoard::new();
    let parent_zone = ZoneID::new(1);
    let child_zone = ZoneID::new(2);
    let mut nat = switch_board.setup_nat(
        parent_zone,
        child_zone,
        //None,
    );
    let hole0 = nat.punch_hole();
    let hole1 = nat.punch_hole();
    let hole2 = nat.punch_hole();
    let parent_server0 = switch_board.start_server_in_zone(parent_zone);
    let parent_server1 = switch_board.start_server_in_zone(parent_zone);
    let parent_server2 = switch_board.start_server_in_zone(parent_zone);
    let child_server0 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole0.into_additional_address(),
        },
    );
    let child_server1 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole1.into_additional_address(),
        },
    );
    let child_server2 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole2.into_additional_address(),
        },
    );

    nat.make_route(hole0, child_server0.addr);
    nat.make_route(hole1, child_server1.addr);
    nat.make_route(hole2, child_server2.addr);
    parent_server0.talk_to(&[&parent_server1, &parent_server2]);
    child_server0.talk_to(&[&child_server1, &child_server2]);
    parent_server1.talk_to(&[&hole0, &hole1]);
    parent_server2.talk_to(&[&hole2]);
    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(switch_board.wait_for_disjoint_settled_zones(&[
        &[&parent_server0, &parent_server1, &parent_server2],
        &[&child_server0, &child_server1, &child_server2],
    ]));
}

#[test]
fn different_zones_get_different_ids_with_unexposed_servers() {
    let switch_board = TestNetworkSwitchBoard::new();
    let parent_zone = ZoneID::new(1);
    let child_zone = ZoneID::new(2);
    let mut nat = switch_board.setup_nat(
        parent_zone,
        child_zone,
        //None,
    );
    let hole0 = nat.punch_hole();
    let hole1 = nat.punch_hole();
    let hole2 = nat.punch_hole();
    let parent_server0 = switch_board.start_server_in_zone(parent_zone);
    let parent_server1 = switch_board.start_server_in_zone(parent_zone);
    let parent_server2 = switch_board.start_server_in_zone(parent_zone);
    let child_server0 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole0.into_additional_address(),
        },
    );
    let child_server1 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole1.into_additional_address(),
        },
    );
    let child_server2 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole2.into_additional_address(),
        },
    );
    let child_server3 = switch_board.start_server_in_zone(child_zone);
    let child_server4 = switch_board.start_server_in_zone(child_zone);
    let child_server5 = switch_board.start_server_in_zone(child_zone);

    nat.make_route(hole0, child_server0.addr);
    nat.make_route(hole1, child_server1.addr);
    nat.make_route(hole2, child_server2.addr);

    parent_server0.talk_to(&[&parent_server1, &parent_server2]);
    child_server0.talk_to(&[&child_server1, &child_server2, &child_server3]);
    child_server3.talk_to(&[&child_server4, &child_server5]);
    parent_server1.talk_to(&[&hole0, &hole1]);
    parent_server2.talk_to(&[&hole2]);
    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(switch_board.wait_for_disjoint_settled_zones(&[
        slc![&parent_server0, &parent_server1, &parent_server2],
        slc![
            &child_server0,
            &child_server1,
            &child_server2,
            &child_server3,
            &child_server4,
            &child_server5,
        ],
    ]));
}

// TODO(krnowak): This needs to wait for sibling handling.
#[allow(dead_code)]
fn different_zones_get_different_ids_outcast() {
    let switch_board = TestNetworkSwitchBoard::new();
    let parent_zone = ZoneID::new(1);
    let child_zone = ZoneID::new(2);
    let mut nat = switch_board.setup_nat(
        parent_zone,
        child_zone,
        //None,
    );
    let hole0 = nat.punch_hole();
    let hole1 = nat.punch_hole();
    let hole2 = nat.punch_hole();
    let hole3 = nat.punch_hole();
    let parent_server0 = switch_board.start_server_in_zone(parent_zone);
    let parent_server1 = switch_board.start_server_in_zone(parent_zone);
    let parent_server2 = switch_board.start_server_in_zone(parent_zone);
    let child_server0 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole0.into_additional_address(),
        },
    );
    let child_server1 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole1.into_additional_address(),
        },
    );
    let child_server2 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole2.into_additional_address(),
        },
    );
    let child_server3 = switch_board.start_server_in_zone_with_additional_addresses(
        child_zone,
        hm!{
            "-".to_string() => hole3.into_additional_address(),
        },
    );

    nat.make_route(hole0, child_server0.addr);
    nat.make_route(hole1, child_server1.addr);
    nat.make_route(hole2, child_server2.addr);
    nat.make_route(hole3, child_server3.addr);
    parent_server0.talk_to(&[&parent_server1, &parent_server2]);
    child_server0.talk_to(&[&child_server1, &child_server2]);
    parent_server1.talk_to(&[&hole0, &hole1]);
    parent_server2.talk_to(&[&hole2]);
    assert!(switch_board.wait_for_health_of_those(
        Health::Alive,
        &[
            &parent_server0,
            &parent_server1,
            &parent_server2,
            &child_server0,
            &child_server1,
            &child_server2
        ],
    ));
    assert!(switch_board.wait_for_disjoint_settled_zones(&[
        &[&parent_server0, &parent_server1, &parent_server2],
        &[&child_server0, &child_server1, &child_server2],
    ]));

    child_server3.talk_to(&[&parent_server0, &parent_server1, &parent_server2]);

    assert!(switch_board.wait_for_health_of_all(Health::Alive));
    assert!(switch_board.wait_for_disjoint_settled_zones(&[
        slc![&parent_server0, &parent_server1, &parent_server2],
        slc![
            &child_server0,
            &child_server1,
            &child_server2,
            &child_server3
        ],
    ]));
}
