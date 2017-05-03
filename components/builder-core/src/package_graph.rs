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
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::{is_cyclic_directed, connected_components};
use hab_core::package::PackageIdent;

use rdeps::rdeps;

#[derive(Debug)]
pub struct Stats {
    pub node_count: usize,
    pub edge_count: usize,
    pub connected_comp: usize,
    pub is_cyclic: bool,
    pub plan_count: usize,
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
        let id = if self.package_map.contains_key(name) {
            let val = *self.package_map.get(name).unwrap();
            val
        } else {
            self.package_names.push(String::from(name.clone()));
            assert_eq!(self.package_names[self.package_max], name);

            let node_index = self.graph.add_node(self.package_max);
            self.package_map
                .insert(String::from(name), (self.package_max, node_index));
            self.package_max = self.package_max + 1;

            let parts: Vec<&str> = name.split("/").collect();
            assert!(parts.len() >= 2);
            let short_name = format!("{}/{}", parts[0], parts[1]);

            let pkg_ident = PackageIdent::from_str(name).unwrap();

            let entry = self.latest_map
                .entry(short_name)
                .or_insert(pkg_ident.clone());
            if pkg_ident > *entry {
                *entry = pkg_ident;
            };

            (self.package_max - 1, node_index)
        };

        id
    }

    pub fn build<T>(&mut self, packages: T) -> (usize, usize)
        where T: Iterator<Item = scheduler::Package>
    {
        assert!(self.package_max == 0);

        for p in packages {
            let name = format!("{}", p.get_ident());
            let (pkg_id, pkg_node) = self.generate_id(&name);

            assert_eq!(pkg_id, pkg_node.index());

            let deps = p.get_deps();
            for dep in deps {
                let depname = format!("{}", dep);
                let (_, dep_node) = self.generate_id(&depname);
                self.graph.extend_with_edges(&[(dep_node, pkg_node)]);
            }
        }

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn extend(&mut self, package: &scheduler::Package) -> (usize, usize) {
        let name = format!("{}", package.get_ident());
        let (pkg_id, pkg_node) = self.generate_id(&name);

        assert_eq!(pkg_id, pkg_node.index());

        let deps = package.get_deps();
        for dep in deps {
            let depname = format!("{}", dep);
            let (_, dep_node) = self.generate_id(&depname);
            self.graph.extend_with_edges(&[(dep_node, pkg_node)]);
        }

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn rdeps(&self, name: &str) -> Option<Vec<(String, String)>> {
        let mut v: Vec<(String, String)> = Vec::new();
        let mut map: HashMap<String, bool> = HashMap::new();

        match self.package_map.get(name) {
            Some(&(_, pkg_node)) => {
                match rdeps(&self.graph, pkg_node) {
                    Ok(deps) => {
                        for n in deps {
                            let parts: Vec<&str> = self.package_names[n].split("/").collect();
                            assert!(parts.len() >= 2);
                            let name = format!("{}/{}", parts[0], parts[1]);

                            if !map.contains_key(&name) {
                                let s = format!("{}", self.latest_map.get(&name).unwrap());
                                map.insert(name.clone(), true);
                                v.push((name, s));
                            }
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
        let v: Vec<&str> = name.split('/').collect();
        if v.len() == 2 {
            let phrase = format!("{}/", name);

            let v: Vec<String> = self.package_names
                .iter()
                .cloned()
                .filter(|s| s.starts_with(&phrase))
                .collect();

            // We can safely unwrap below since we checked the format
            let mut pkgs: Vec<PackageIdent> = v.iter()
                .map(|x| PackageIdent::from_str(x).unwrap())
                .collect();

            // TODO: The PackageIdent compare is extremely slow, causing even small lists
            // to take significant time to sort. Look at speeding this up if it becomes a
            // bottleneck.
            pkgs.sort();

            return match pkgs.pop() {
                       Some(p) => Some(format!("{}", p)),
                       None => None,
                   };
        }
        None
    }

    pub fn stats(&self) -> Stats {
        Stats {
            node_count: self.graph.node_count(),
            edge_count: self.graph.edge_count(),
            connected_comp: connected_components(&self.graph),
            is_cyclic: is_cyclic_directed(&self.graph),
            plan_count: self.latest_map.len(),
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
