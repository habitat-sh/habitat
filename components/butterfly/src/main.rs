use env_logger;

use std::{env,
          net::SocketAddr,
          thread,
          time::Duration};

use habitat_butterfly::{member,
                        server::{self,
                                 Suitability},
                        trace};

#[derive(Debug)]
struct ZeroSuitability;
impl Suitability for ZeroSuitability {
    fn get(&self, _service_group: &str) -> u64 { 0 }
}

fn main() {
    env_logger::init();
    let mut args = env::args();
    let _ = args.next();

    let bind_to = args.next().unwrap();
    println!("Binding to {}", bind_to);
    println!("Starting test butterfly");

    let bind_to_addr = bind_to.parse::<SocketAddr>().unwrap();
    let bind_port = bind_to_addr.port();
    let mut gossip_bind_addr = bind_to_addr;
    let gport = bind_port + 1;
    gossip_bind_addr.set_port(gport);

    let mut member = member::Member::default();
    member.swim_port = bind_port;
    member.gossip_port = gport;

    let mut server = server::Server::new(bind_to_addr,
                                         gossip_bind_addr,
                                         member,
                                         trace::Trace::default(),
                                         None,
                                         None,
                                         None,
                                         Box::new(ZeroSuitability)).unwrap();
    println!("Server ID: {}", server.member_id());

    let targets: Vec<String> = args.collect();
    for target in &targets {
        let addr: SocketAddr = target.parse().unwrap();
        let mut member = member::Member::default();
        member.address = format!("{}", addr.ip());
        member.swim_port = addr.port();
        member.gossip_port = addr.port();
        server.member_list.add_initial_member_imlw(member);
    }

    server.start_mlr(server::timing::Timing::default())
          .expect("Cannot start server");
    loop {
        println!("{:#?}", server.member_list);
        thread::sleep(Duration::from_millis(1000));
    }
}
