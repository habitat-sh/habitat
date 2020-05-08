pub mod pkg;

#[cfg(test)]
pub mod test {
    use habitat_butterfly::{member::Member as ButterflyMember,
                            server::{Server as ButterflyServer,
                                     Suitability as ButterflySuitability}};
    use std::{net::{IpAddr,
                    Ipv4Addr,
                    SocketAddr},
              sync::Mutex};

    lazy_static! {
        static ref SWIM_PORT: Mutex<u16> = Mutex::new(6666);
        static ref GOSSIP_PORT: Mutex<u16> = Mutex::new(7777);
    }

    #[derive(Debug)]
    struct ZeroSuitability;
    impl ButterflySuitability for ZeroSuitability {
        fn suitability_for_msr(&self, _service_group: &str) -> u64 { 0 }
    }

    pub fn start_butterfly_server() -> ButterflyServer {
        let swim_port;
        {
            let mut swim_port_guard = SWIM_PORT.lock().expect("SWIM_PORT poisoned");
            swim_port = *swim_port_guard;
            *swim_port_guard += 1;
        }
        let swim_listen = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), swim_port);
        let gossip_port;
        {
            let mut gossip_port_guard = GOSSIP_PORT.lock().expect("GOSSIP_PORT poisoned");
            gossip_port = *gossip_port_guard;
            *gossip_port_guard += 1;
        }
        let gossip_listen = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), gossip_port);
        let mut member = ButterflyMember::default();
        member.swim_port = swim_port;
        member.gossip_port = gossip_port;
        ButterflyServer::new(swim_listen,
                             gossip_listen,
                             member,
                             None,
                             None,
                             None,
                             std::sync::Arc::new(ZeroSuitability)).unwrap()
    }
}
