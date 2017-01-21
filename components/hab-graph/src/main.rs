
#[macro_use]
extern crate log;
extern crate petgraph;
pub mod graph;

use petgraph::Graph;
use petgraph::dot::{Dot, Config};

fn main() {
    println!("Hello, graph!\n");

    let mut deps = Graph::<usize, usize>::new();
    let a = deps.add_node(10);
    let b = deps.add_node(11);
    let c = deps.add_node(12);
    let d = deps.add_node(13);
    let e = deps.add_node(14);
    let f = deps.add_node(15);
    let g = deps.add_node(16);
    let h = deps.add_node(17);

    deps.extend_with_edges(&[(a, c), (b, c), (c, f), (c, e), (d, e), (e, f), (g, h)]);

    println!("Input:");
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

    println!("\nReverse dependencies:");

    match graph::rdeps(&deps, a) {
        Ok(ref v) => {
            for n in v {
                print!("{} ", n);
            }
            println!("");
        }
        Err(_) => println!("Error"),
    }
}
