// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use error::Result;
use hcore::fs as hfs;
use hcore::package;
use hcore::package::ident::PackageIdent;
use hcore::package::PackageInstall;
use petgraph;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Bfs;
use std::collections::HashMap;
use std::path::Path;

pub struct PackageGraph {
    nodes: HashMap<PackageIdent, NodeIndex>,
    graph: StableGraph<PackageIdent, usize, petgraph::Directed>,
}

impl PackageGraph {
    pub fn new() -> Self {
        PackageGraph {
            nodes: HashMap::<PackageIdent, NodeIndex>::new(),
            graph: StableGraph::<PackageIdent, usize>::new(),
        }
    }

    // Load a set of packages that are stored in a package_path under a habitat
    // root directory
    pub fn load(&mut self, fs_root_path: &Path) -> Result<(usize, usize)> {
        let package_path = hfs::pkg_root_path(Some(&fs_root_path));
        let idents = package::all_packages(&package_path)?;

        for ident in &idents {
            let p = PackageInstall::load(&ident, Some(fs_root_path))?;
            let deps = p.deps()?;
            self.extend(&ident, &deps);
        }

        Ok((self.node_count(), self.edge_count()))
    }

    /// Return (and possibly create) a NodeIndex for a given PackageIdent.
    /// Upon returning, the node will be guaranteed to be in the graph
    fn node_idx(&mut self, package: &PackageIdent) -> NodeIndex {
        match self.nodes.get(package) {
            Some(&idx) => idx,
            None => {
                let idx = self.graph.add_node(package.clone());
                self.nodes.insert(package.clone(), idx);
                idx
            }
        }
    }

    /// Extend a graph by adding in dependencies for a package
    fn extend(&mut self, package: &PackageIdent, deps: &Vec<PackageIdent>) -> (usize, usize) {
        let idx = self.node_idx(package);

        for dep in deps {
            let dep_idx = self.node_idx(dep);
            self.graph.extend_with_edges(&[(idx, dep_idx)]);
        }

        assert_eq!(self.graph.node_count(), self.nodes.len());

        (self.graph.node_count(), self.graph.edge_count())
    }

    /// Return the dependencies in a topological order (ie. packages will appear before their dependencies)
    pub fn ordered_deps(&self, package: &PackageIdent) -> Vec<PackageIdent> {
        let mut result = Vec::<PackageIdent>::new();
        match self.nodes.get(package) {
            Some(&idx) => {
                let mut bfs = Bfs::new(&self.graph, idx);

                // BFS returns the original node on first call to next()
                // consume it here so it's not in the result Vec
                match bfs.next(&self.graph) {
                    Some(n) => assert_eq!(&n, &idx),
                    None => unreachable!("package is always in BFS from itself"),
                }

                while let Some(child) = bfs.next(&self.graph) {
                    if let Some(child_pkg) = self.graph.node_weight(child) {
                        result.push(child_pkg.clone());
                    }
                }
            }
            None => (),
        }
        result
    }

    /// Remove a package from a graph
    ///
    /// This will not remove the package if it is a dependency of any package
    ///
    /// Returns None if package was not in graph
    /// Returns Some(true) if package is removed
    /// Return  Some(false) if package is a dependency
    pub fn remove(&mut self, package: &PackageIdent) -> Option<bool> {
        match self.count_rdeps(package) {
            Some(0) => self.do_remove(package),
            Some(_) => Some(false),
            None => None,
        }
    }

    /// Cleanly remove the node from both the node list and the graph
    ///
    /// Returns None if package was not in graph
    /// Returns Some(true) if package is removed
    fn do_remove(&mut self, package: &PackageIdent) -> Option<bool> {
        match self.nodes.remove(&package) {
            Some(idx) => {
                // And remove from graph
                match self.graph.remove_node(idx) {
                    Some(ident) => assert_eq!(&ident, package),
                    None => panic!(
                        "removed node from map but it wasn't in the graph: {}",
                        package
                    ),
                }
                Some(true)
            }
            None => None,
        }
    }

    /// does a specific PackageIdent appear in the graph
    ///
    pub fn has_package(&self, package: &PackageIdent) -> bool {
        self.nodes.contains_key(package)
    }

    fn count_edges(&self, package: &PackageIdent, direction: petgraph::Direction) -> Option<usize> {
        match self.nodes.get(package) {
            Some(&idx) => {
                let deps: Vec<NodeIndex> = self.graph.neighbors_directed(idx, direction).collect();
                Some(deps.len())
            }
            None => None,
        }
    }

    /// Returns the number of dependancies for a package
    ///
    /// Returns None if the package is not in the graph
    pub fn count_deps(&self, ident: &PackageIdent) -> Option<usize> {
        self.count_edges(ident, petgraph::Outgoing)
    }

    /// Returns the number of package which have this package as a dependency
    ///
    /// Returns None if the package is not in the graph
    pub fn count_rdeps(&self, ident: &PackageIdent) -> Option<usize> {
        self.count_edges(ident, petgraph::Incoming)
    }

    /// Returns the number of packages in the package graph
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the number of edges (dependencies) in the package graph
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    struct PackageDeps {
        ident: PackageIdent,
        deps: Vec<PackageIdent>,
    }

    fn build(packages: Vec<PackageDeps>) -> PackageGraph {
        let mut graph = PackageGraph::new();
        for p in &packages {
            graph.extend(&p.ident, &p.deps);
        }
        graph
    }

    fn empty_package_deps(ident: PackageIdent) -> PackageDeps {
        PackageDeps {
            ident: ident,
            deps: vec![],
        }
    }

    fn package_deps(ident: PackageIdent, deps: &Vec<PackageIdent>) -> PackageDeps {
        PackageDeps {
            ident: ident,
            deps: deps.to_vec(),
        }
    }

    #[test]
    fn empty_graph() {
        let packages = Vec::new();

        let graph = build(packages);
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn no_deps_graph() {
        let packages = vec![
            empty_package_deps(PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap()),
            empty_package_deps(PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap()),
        ];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn simplest_graph() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
        ];

        let graph = build(packages);
        assert!(graph.has_package(&a));
        assert!(graph.has_package(&b));

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn count_deps_non_existent_package() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let c = PackageIdent::from_str("core/foo/1.0/20180704142805").unwrap();
        let d = PackageIdent::from_str("core/bar/1.0/20180704142805").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
            package_deps(c.clone(), &vec![a.clone()]),
            package_deps(d.clone(), &vec![b.clone(), c.clone()]),
        ];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);

        let does_not_exist = PackageIdent::from_str("core/baz").unwrap();
        assert!(graph.count_deps(&does_not_exist).is_none());
        assert!(graph.count_rdeps(&does_not_exist).is_none());
    }

    #[test]
    fn count_deps() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let c = PackageIdent::from_str("core/foo/1.0/20180704142805").unwrap();
        let d = PackageIdent::from_str("core/bar/1.0/20180704142805").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
            package_deps(c.clone(), &vec![a.clone()]),
            package_deps(d.clone(), &vec![b.clone(), c.clone()]),
        ];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);

        assert_eq!(graph.count_deps(&a).unwrap(), 0);
        assert_eq!(graph.count_deps(&b).unwrap(), 1);
        assert_eq!(graph.count_deps(&c).unwrap(), 1);
        assert_eq!(graph.count_deps(&d).unwrap(), 2);
    }

    #[test]
    fn count_rdeps() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let c = PackageIdent::from_str("core/foo/1.0/20180704142805").unwrap();
        let d = PackageIdent::from_str("core/bar/1.0/20180704142805").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
            package_deps(c.clone(), &vec![a.clone()]),
            package_deps(d.clone(), &vec![b.clone(), c.clone()]),
        ];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);

        assert_eq!(graph.count_rdeps(&a).unwrap(), 2);
        assert_eq!(graph.count_rdeps(&b).unwrap(), 1);
        assert_eq!(graph.count_rdeps(&c).unwrap(), 1);
        assert_eq!(graph.count_rdeps(&d).unwrap(), 0);
    }

    #[test]
    fn remove_package_no_rdeps() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let c = PackageIdent::from_str("core/foo/1.0/20180704142805").unwrap();
        let d = PackageIdent::from_str("core/bar/1.0/20180704142805").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
            package_deps(c.clone(), &vec![a.clone()]),
            package_deps(d.clone(), &vec![b.clone(), c.clone()]),
        ];

        let mut graph = build(packages);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);

        assert_eq!(graph.count_rdeps(&d).unwrap(), 0);

        assert!(graph.remove(&d).unwrap());
        // package count decremented on remove
        assert_eq!(graph.has_package(&d), false);
        assert_eq!(graph.node_count(), 3);

        // rdeps of dependencies should have decreased too
        assert_eq!(graph.count_rdeps(&b).unwrap(), 0);
        assert_eq!(graph.count_rdeps(&c).unwrap(), 0);
    }

    #[test]
    fn cant_remove_package_with_rdeps() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
        ];

        let mut graph = build(packages);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        assert_eq!(graph.count_rdeps(&a).unwrap(), 1);

        assert_eq!(graph.remove(&a).unwrap(), false);
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn ordered_deps_of_empty_deps() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let packages = vec![empty_package_deps(a.clone())];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);

        let odeps = graph.ordered_deps(&a);
        assert_eq!(&Vec::<PackageIdent>::new(), &odeps);
    }

    #[test]
    fn ordered_deps_are_in_order() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let c = PackageIdent::from_str("core/bar/1.0/20180704142805").unwrap();
        let d = PackageIdent::from_str("core/baz/1.0/20180704142805").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
            package_deps(c.clone(), &vec![b.clone()]),
            package_deps(d.clone(), &vec![c.clone()]),
        ];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);

        let odeps = graph.ordered_deps(&d);
        let expected = vec![c, b, a];
        assert_eq!(expected, odeps);
    }
}
