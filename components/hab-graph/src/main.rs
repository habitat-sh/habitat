
#[macro_use]
extern crate log;
extern crate petgraph;
extern crate walkdir;
extern crate habitat_core as hab_core;
extern crate habitat_builder_protocol as protocol;

pub mod graph;
pub mod scanner;

use scanner::Scanner;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};


fn main() {
    println!("Hello, graph!\n");

    let mut deps = Graph::<usize, usize>::new();
    let a = deps.add_node(10);
    let b = deps.add_node(11);
    deps.extend_with_edges(&[(a, b), (b, a)]);

    let mut scanner = Scanner::new("/Users/salam/Workspace/habitat/components/hab-graph/pkgs/0");
    scanner.scan();
}
