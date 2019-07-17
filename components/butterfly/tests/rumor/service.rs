use crate::btest;

#[test]
fn two_members_share_services() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.wait_for_rounds(2);
    assert!(net[1].service_store
                  .lock_rsr()
                  .service_group("witcher.prod")
                  .contains_id(net[0].member_id()));
}
