use crate::btest;
use habitat_butterfly::member::Health;
use habitat_core::crypto::keys::sym_key::SymKey;

#[test]
fn symmetric_encryption_of_wire_payloads() {
    let ring_key = SymKey::generate_pair_for_ring("wolverine").expect("Failed to generate an in \
                                                                       memory symkey");
    let mut net = btest::SwimNet::new_ring_encryption(2, &ring_key);
    net.connect_smr(0, 1);
    assert_wait_for_health_of_mlr!(net, [0..2, 0..2], Health::Alive);
    net.add_service(0, "core/beast/1.2.3/20161208121212");
    net.wait_for_gossip_rounds(2);
    assert!(net[1].service_store
                  .lock_rsr()
                  .service_group("beast.prod")
                  .contains_id(net[0].member_id()));
}
