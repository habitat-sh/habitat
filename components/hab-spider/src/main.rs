
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate petgraph;
extern crate walkdir;
extern crate habitat_core as hab_core;
extern crate habitat_builder_protocol as protocol;

pub mod rdeps;
pub mod spider;

use spider::Spider;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::io;

fn main() {
    env_logger::init().unwrap();

    println!("Hello, graph!");
    println!("\nHit ENTER to start");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut deps = Graph::<usize, usize>::new();
    let a = deps.add_node(10);
    let b = deps.add_node(11);
    deps.extend_with_edges(&[(a, b), (b, a)]);

    let mut spider = Spider::new("/Users/salam/Workspace/habitat/components/hab-graph/pkgs/0");
    spider.crawl();

    println!("\nHit ENTER to exit");
    io::stdin().read_line(&mut s).unwrap();
}
