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

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::str::FromStr;
use protocol::scheduler;
use petgraph::{Graph, Direction};
use petgraph::graph::{NodeIndex, EdgeIndex};
use petgraph::visit::EdgeRef;
use petgraph::algo::{is_cyclic_directed, connected_components};
use hab_core::package::PackageIdent;

use rdeps::rdeps;

#[derive(Debug)]
pub struct Stats {
    pub node_count: usize,
    pub edge_count: usize,
    pub connected_comp: usize,
    pub is_cyclic: bool,
}

#[derive(Eq)]
struct HeapEntry {
    pkg_index: usize,
    rdep_count: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &HeapEntry) -> Ordering {
        self.rdep_count.cmp(&other.rdep_count)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &HeapEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &HeapEntry) -> bool {
        self.pkg_index == other.pkg_index
    }
}

fn short_name(name: &str) -> String {
    let parts: Vec<&str> = name.split("/").collect();
    assert!(parts.len() >= 2);
    format!("{}/{}", parts[0], parts[1])
}

pub struct PackageGraph {
    package_max: usize,
    package_map: HashMap<String, (usize, NodeIndex)>,
    latest_map: HashMap<String, PackageIdent>,
    package_names: Vec<String>,
    graph: Graph<usize, usize>,
}

impl PackageGraph {
    pub fn new() -> Self {
        PackageGraph {
            package_max: 0,
            package_map: HashMap::new(),
            latest_map: HashMap::new(),
            package_names: Vec::new(),
            graph: Graph::<usize, usize>::new(),
        }
    }

    fn generate_id(&mut self, name: &str) -> (usize, NodeIndex) {
        let short_name = short_name(name);

        let id = if self.package_map.contains_key(&short_name) {
            let val = *self.package_map.get(&short_name).unwrap();
            val
        } else {
            self.package_names.push(short_name.clone());
            assert_eq!(self.package_names[self.package_max], short_name);

            let node_index = self.graph.add_node(self.package_max);
            self.package_map
                .insert(short_name.clone(), (self.package_max, node_index));
            self.package_max = self.package_max + 1;

            (self.package_max - 1, node_index)
        };

        id
    }

    pub fn build<T>(&mut self, packages: T) -> (usize, usize)
        where T: Iterator<Item = scheduler::Package>
    {
        assert!(self.package_max == 0);

        for p in packages {
            self.extend(&p);
        }

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn extend(&mut self, package: &scheduler::Package) -> (usize, usize) {
        let name = format!("{}", package.get_ident());
        let (pkg_id, pkg_node) = self.generate_id(&name);

        assert_eq!(pkg_id, pkg_node.index());

        let pkg_ident = PackageIdent::from_str(&name).unwrap();
        let short_name = short_name(&name);

        let add_deps = if self.latest_map.contains_key(&short_name) {
            let latest = self.latest_map.get(&short_name).unwrap();

            if pkg_ident < *latest {
                false
            } else {
                assert!(pkg_ident > *latest);

                let edge_ids: Vec<EdgeIndex> = self.graph
                    .edges_directed(pkg_node, Direction::Incoming)
                    .map(|e| e.id())
                    .collect();
                for edge_id in edge_ids {
                    self.graph.remove_edge(edge_id).unwrap();
                }
                true
            }
        } else {
            self.latest_map.insert(short_name, pkg_ident.clone());
            true
        };

        if add_deps {
            let deps = package.get_deps();
            for dep in deps {
                let depname = format!("{}", dep);
                let (_, dep_node) = self.generate_id(&depname);
                self.graph.extend_with_edges(&[(dep_node, pkg_node)]);
            }
        }

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn rdeps(&self, name: &str) -> Option<Vec<(String, String)>> {
        let mut v: Vec<(String, String)> = Vec::new();

        match self.package_map.get(name) {
            Some(&(_, pkg_node)) => {
                match rdeps(&self.graph, pkg_node) {
                    Ok(deps) => {
                        for n in deps {
                            let name = self.package_names[n].clone();
                            let ident = format!("{}", self.latest_map.get(&name).unwrap());
                            v.push((name, ident));
                        }
                    }
                    Err(e) => panic!("Error: {:?}", e),
                }
            }
            None => return None,
        }

        Some(v)
    }

    // Mostly for debugging
    pub fn rdeps_dump(&self) {
        debug!("Reverse dependencies:");

        for (pkg_name, pkg_id) in &self.package_map {
            let (_, node) = *pkg_id;
            debug!("{}", pkg_name);

            match rdeps(&self.graph, node) {
                Ok(v) => {
                    for n in v {
                        debug!("|_ {}", self.package_names[n]);
                    }
                }
                Err(e) => panic!("Error: {:?}", e),
            }
        }
    }

    pub fn search(&self, phrase: &str) -> Vec<String> {
        let v: Vec<String> = self.package_names
            .iter()
            .cloned()
            .filter(|s| s.contains(phrase))
            .collect();

        v
    }

    // Given an identifier in 'origin/name' format, returns the
    // most recent version (fully-qualified package ident string)
    pub fn resolve(&self, name: &str) -> Option<String> {
        match self.latest_map.get(name) {
            Some(ident) => Some(format!("{}", ident)),
            None => None,
        }
    }

    pub fn stats(&self) -> Stats {
        Stats {
            node_count: self.graph.node_count(),
            edge_count: self.graph.edge_count(),
            connected_comp: connected_components(&self.graph),
            is_cyclic: is_cyclic_directed(&self.graph),
        }
    }

    pub fn top(&self, max: usize) -> Vec<(String, usize)> {
        let mut v = Vec::new();
        let mut heap = BinaryHeap::new();

        for (_, pkg_id) in &self.package_map {
            let (index, node) = *pkg_id;

            match rdeps(&self.graph, node) {
                Ok(v) => {
                    let he = HeapEntry {
                        pkg_index: index,
                        rdep_count: v.len(),
                    };
                    heap.push(he);
                }
                Err(e) => panic!("Error: {:?}", e),
            }
        }

        let mut i = 0;
        while (i < max) && !heap.is_empty() {
            let he = heap.pop().unwrap();
            v.push((self.package_names[he.pkg_index].clone(), he.rdep_count));
            i = i + 1;
        }

        v
    }
}
