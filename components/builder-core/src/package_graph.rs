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
            self.package_map.insert(short_name.clone(), (
                self.package_max,
                node_index,
            ));
            self.package_max = self.package_max + 1;

            (self.package_max - 1, node_index)
        };

        id
    }

    pub fn build<T>(&mut self, packages: T) -> (usize, usize)
    where
        T: Iterator<Item = scheduler::Package>,
    {
        assert!(self.package_max == 0);

        for p in packages {
            self.extend(&p);
        }

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn check_extend(&mut self, package: &scheduler::Package) -> bool {
        let name = format!("{}", package.get_ident());
        let pkg_short_name = short_name(&name);

        // If package is brand new, we can't have a circular dependency
        if !self.package_map.contains_key(&pkg_short_name) {
            debug!("check_extend: no package found - OK");
            return true;
        }

        let (_, pkg_node) = *self.package_map.get(&pkg_short_name).unwrap();

        // Temporarily remove edges
        let mut saved_nodes = Vec::new();
        let neighbors: Vec<NodeIndex> = self.graph
            .neighbors_directed(pkg_node, Direction::Incoming)
            .collect();
        for n in neighbors {
            let e = self.graph.find_edge(n, pkg_node).unwrap();
            saved_nodes.push(n);
            self.graph.remove_edge(e).unwrap();
        }

        // Check to see if extension would create a circular dependency
        let mut circular_dep = false;
        let mut dep_nodes = Vec::new();
        let deps = package.get_deps();
        for dep in deps {
            let dep_name = format!("{}", dep);
            let dep_short_name = short_name(&dep_name);

            // No-op if dep hasn't been added yet
            if self.package_map.contains_key(&dep_short_name) {
                let (_, dep_node) = *self.package_map.get(&dep_short_name).unwrap();
                dep_nodes.push(dep_node);

                self.graph.extend_with_edges(&[(dep_node, pkg_node)]);

                // Check for circular dependency
                if is_cyclic_directed(&self.graph) {
                    debug!(
                        "graph is cyclic after adding {} -> {} - failing check_extend",
                        dep_name,
                        name
                    );
                    circular_dep = true;
                    break;
                }
            }
        }

        // Undo the edge changes
        for dep_node in dep_nodes {
            let e = self.graph.find_edge(dep_node, pkg_node).unwrap();
            self.graph.remove_edge(e).unwrap();
        }

        for saved_node in saved_nodes {
            self.graph.extend_with_edges(&[(saved_node, pkg_node)]);
        }

        !circular_dep
    }

    pub fn extend(&mut self, package: &scheduler::Package) -> (usize, usize) {
        let name = format!("{}", package.get_ident());
        let (pkg_id, pkg_node) = self.generate_id(&name);

        assert_eq!(pkg_id, pkg_node.index());

        let pkg_ident = PackageIdent::from_str(&name).unwrap();
        let short_name = short_name(&name);

        let add_deps = if self.latest_map.contains_key(&short_name) {
            let skip_update = {
                let latest = self.latest_map.get(&short_name).unwrap();
                pkg_ident < *latest
            };

            if skip_update {
                false
            } else {
                let neighbors: Vec<NodeIndex> = self.graph
                    .neighbors_directed(pkg_node, Direction::Incoming)
                    .collect();
                for n in neighbors {
                    let e = self.graph.find_edge(n, pkg_node).unwrap();
                    self.graph.remove_edge(e).unwrap();
                }
                self.latest_map.insert(short_name, pkg_ident.clone());
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

                // sanity check
                if is_cyclic_directed(&self.graph) {
                    warn!(
                        "graph is cyclic after adding {} -> {} - rolling back",
                        depname,
                        name
                    );
                    let e = self.graph.find_edge(dep_node, pkg_node).unwrap();
                    self.graph.remove_edge(e).unwrap();
                }
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

#[cfg(test)]
mod test {
    use protobuf::RepeatedField;
    use super::*;

    #[test]
    fn empty_graph() {
        let mut graph = PackageGraph::new();
        let packages = Vec::new();

        let (ncount, ecount) = graph.build(packages.into_iter());
        assert_eq!(ncount, 0);
        assert_eq!(ecount, 0);
    }

    #[test]
    fn disallow_circular_dependency() {
        let mut graph = PackageGraph::new();
        let mut packages = Vec::new();

        let mut package1 = scheduler::Package::new();
        package1.set_ident("foo/bar/1/2".to_string());
        let mut package1_deps = RepeatedField::new();
        package1_deps.push("foo/baz/1/2".to_string());
        package1.set_deps(package1_deps);
        packages.push(package1);

        let mut package2 = scheduler::Package::new();
        package2.set_ident("foo/baz/1/2".to_string());
        let mut package2_deps = RepeatedField::new();
        package2_deps.push("foo/bar/1/2".to_string());
        package2.set_deps(package2_deps);
        packages.push(package2.clone());

        let (ncount, ecount) = graph.build(packages.into_iter());

        assert_eq!(ncount, 2);
        assert_eq!(ecount, 1); // only the first edge added

        let stats = graph.stats();
        assert_eq!(stats.is_cyclic, false);

        let pre_check = graph.check_extend(&package2);
        assert_eq!(pre_check, false);
    }

    #[test]
    fn pre_check_with_dep_not_present() {
        let mut graph = PackageGraph::new();

        let mut package1 = scheduler::Package::new();
        package1.set_ident("foo/bar/1/2".to_string());
        let mut package1_deps = RepeatedField::new();
        package1_deps.push("foo/baz/1/2".to_string());
        package1.set_deps(package1_deps);

        let mut package2 = scheduler::Package::new();
        package2.set_ident("foo/baz/1/2".to_string());
        let mut package2_deps = RepeatedField::new();
        package2_deps.push("foo/xyz/1/2".to_string());
        package2.set_deps(package2_deps);

        let pre_check1 = graph.check_extend(&package1);
        assert_eq!(pre_check1, true);

        let (_, _) = graph.extend(&package1);

        let pre_check2 = graph.check_extend(&package2);
        assert_eq!(pre_check2, true);

        let (_, _) = graph.extend(&package2);
    }
}
