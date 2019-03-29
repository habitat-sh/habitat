

use habitat_butterfly::member::Health;
use habitat_core::crypto::keys::sym_key::SymKey;

use crate::btest;

#[test]
fn symmetric_encryption_of_wire_payloads() {
    let ring_key = SymKey::generate_pair_for_ring("wolverine").expect("Failed to generate an in \
                                                                       memory symkey");
    let mut net = btest::SwimNet::new_ring_encryption(2, &ring_key);
    net.connect(0, 1);
    assert_wait_for_health_of!(net, [0..2, 0..2], Health::Alive);
    net.add_service(0, "core/beast/1.2.3/20161208121212");
    net.wait_for_gossip_rounds(2);
    assert!(net[1].service_store
                  .contains_rumor("beast.prod", net[0].member_id()));
}
