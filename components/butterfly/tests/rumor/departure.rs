use crate::btest;
use habitat_butterfly::{client::Client,
                        member::Health};

#[test]
fn two_members_share_departures() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_departure(0);
    net.wait_for_gossip_rounds(1);
    assert!(net[1].departure_store
                  .contains_rumor("departure", net[0].member_id()));
}

#[test]
#[ignore]
fn departure_via_client() {
    let mut net = btest::SwimNet::new(3);
    net.mesh();

    net.wait_for_gossip_rounds(1);
    let mut client =
        Client::new(&net[0].gossip_addr().to_string(), None).expect("Cannot create Butterfly \
                                                                     Client");
    client.send_departure(&net[1].member_id())
          .expect("Cannot send the departure");
    net.wait_for_gossip_rounds(1);
    assert!(net[2].departure_store
                  .contains_rumor("departure", net[1].member_id()));
    assert_wait_for_health_of!(net, 1, Health::Departed);
}
