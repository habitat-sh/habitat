// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashMap, HashSet};

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::{toposort, is_cyclic_directed};
use petgraph::visit::{Bfs, Walker};

#[derive(Debug, PartialEq)]
pub enum GraphErr {
    GraphCyclic,
}

pub type GType = usize;

pub fn rdeps(g: &Graph<GType, GType>, n: NodeIndex) -> Result<Vec<GType>, GraphErr> {
    if is_cyclic_directed(&g) {
        error!("Input graph should not be cyclic!");
        return Err(GraphErr::GraphCyclic);
    }

    // unwrap should never panic as we pre-check for cycle
    let t: Vec<GType> = toposort(&g, None)
        .unwrap()
        .iter()
        .map(|k| k.index())
        .collect();

    let bfs: Vec<GType> = Bfs::new(&g, n).iter(&g).map(|k| k.index()).collect();

    let mut topo_map: HashMap<usize, usize> = HashMap::new(); // Node -> Topo index

    for (i, j) in t.iter().enumerate() {
        topo_map.insert(*j, i);
    }

    // Collect BFS nodes in topological order
    let start: usize = n.index();
    let mut curr: usize = *topo_map.get(&start).unwrap(); // Where to start in topo array

    let mut bfs_set: HashSet<&usize> = bfs.iter().collect();
    let mut v: Vec<GType> = Vec::new();

    bfs_set.remove(&start);
    while !bfs_set.is_empty() {
        if bfs_set.contains(&t[curr]) {
            v.push(t[curr]);
            bfs_set.remove(&t[curr]);
        }
        curr = curr + 1;
    }

    Ok(v)
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use rdeps::*;

    #[test]
    fn fails_with_cyclic_graph() {
        let mut deps = Graph::<usize, usize>::new();
        let a = deps.add_node(10);
        let b = deps.add_node(11);
        let c = deps.add_node(12);

        deps.extend_with_edges(&[(a, b), (b, c), (c, a)]);

        match rdeps(&deps, a) {
            Ok(_) => panic!("Cyclic graph should fail!"),
            Err(e) => assert_eq!(e, GraphErr::GraphCyclic),
        }
    }

    #[test]
    fn basic_graph_works() {
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

        match rdeps(&deps, a) {
            Ok(v) => {
                static EXPECTED: &'static [usize] = &[2, 4, 5];
                assert_eq!(v.as_slice(), EXPECTED);
            }
            Err(e) => {
                panic!("Failed with error: {:?}", e);
            }
        }

        match rdeps(&deps, b) {
            Ok(v) => {
                static EXPECTED: &'static [usize] = &[2, 4, 5];
                assert_eq!(v.as_slice(), EXPECTED);
            }
            Err(e) => {
                panic!("Failed with error: {:?}", e);
            }
        }
    }
}
