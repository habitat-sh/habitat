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
use hcore::package::ident::PackageIdent;
use hcore::package::PackageInstall;
use petgraph;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageDeps {
    ident: PackageIdent,
    deps: Vec<PackageIdent>,
}

impl From<PackageInstall> for PackageDeps {
    fn from(install: PackageInstall) -> Self {
        PackageDeps {
            ident: install.ident().clone(),
            deps: install.deps().unwrap(),
        }
    }
}

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

    // Given a list of packages and their dependencies, build
    // a directed package graph
    pub fn build<T>(&mut self, packages: T) -> Result<(usize, usize)>
    where
        T: Iterator<Item = PackageDeps>,
    {
        for p in packages {
            self.extend(&p.ident, &p.deps);
        }

        Ok((self.graph.node_count(), self.graph.edge_count()))
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

    /// Remove a package from a graph
    ///
    /// This will not remove the package if it is a dependency of any package
    ///
    /// Returns None if package is not in graph
    ///
    /// Returns Some(true) if package is removed
    /// Return  Some(false) if package is a dependendency
    pub fn remove(&mut self, package: &PackageIdent) -> Option<bool> {
        match self.count_rdeps(package) {
            Some(0) => self.do_remove(package),
            Some(_) => Some(false),
            None => None
        }
    }

    fn do_remove(&mut self, package: &PackageIdent) -> Option<bool> {
        match self.nodes.remove(&package) {
            Some(idx) => {
                // And remove from graph
                match self.graph.remove_node(idx) {
                    Some(ident) => assert_eq!(&ident, package),
                    None => panic!("removed node from map but it wasn't in the graph: {}", package)
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
    pub fn count_packages(&self) -> usize {
        self.graph.node_count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

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
        let mut graph = PackageGraph::new();
        let packages = Vec::new();

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 0);
        assert_eq!(ecount, 0);
    }

    #[test]
    fn no_deps_graph() {
        let mut graph = PackageGraph::new();

        let packages = vec![
            empty_package_deps(PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap()),
            empty_package_deps(PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap()),
        ];

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 2);
        assert_eq!(ecount, 0);
    }

    #[test]
    fn simplest_graph() {
        let mut graph = PackageGraph::new();

        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
        ];

        assert_eq!(false, graph.has_package(&a));
        assert_eq!(false, graph.has_package(&b));
        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert!(graph.has_package(&a));
        assert!(graph.has_package(&b));

        assert_eq!(ncount, 2);
        assert_eq!(ecount, 1);
    }

    #[test]
    fn count_deps_non_existent_package() {
        let mut graph = PackageGraph::new();

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

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 4);
        assert_eq!(ecount, 4);

        let does_not_exist = PackageIdent::from_str("core/baz").unwrap();
        assert!(graph.count_deps(&does_not_exist).is_none());
        assert!(graph.count_rdeps(&does_not_exist).is_none());
    }

    #[test]
    fn count_deps() {
        let mut graph = PackageGraph::new();

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

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 4);
        assert_eq!(ecount, 4);

        assert_eq!(graph.count_deps(&a).unwrap(), 0);
        assert_eq!(graph.count_deps(&b).unwrap(), 1);
        assert_eq!(graph.count_deps(&c).unwrap(), 1);
        assert_eq!(graph.count_deps(&d).unwrap(), 2);
    }

    #[test]
    fn count_rdeps() {
        let mut graph = PackageGraph::new();

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

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 4);
        assert_eq!(ecount, 4);

        assert_eq!(graph.count_rdeps(&a).unwrap(), 2);
        assert_eq!(graph.count_rdeps(&b).unwrap(), 1);
        assert_eq!(graph.count_rdeps(&c).unwrap(), 1);
        assert_eq!(graph.count_rdeps(&d).unwrap(), 0);
    }

    #[test]
    fn remove_package_no_rdeps() {
        let mut graph = PackageGraph::new();

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

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 4);
        assert_eq!(ecount, 4);

        assert_eq!(graph.count_rdeps(&d).unwrap(), 0);

        assert!(graph.remove(&d).unwrap());
        // package count decremented on remove
        assert_eq!(graph.has_package(&d), false);
        assert_eq!(graph.count_packages(), 3);

        // rdeps of dependencies should have decreased too
        assert_eq!(graph.count_rdeps(&b).unwrap(), 0);
        assert_eq!(graph.count_rdeps(&c).unwrap(), 0);
    }

    #[test]
    fn cant_remove_package_with_rdeps() {
        let mut graph = PackageGraph::new();

        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let b = PackageIdent::from_str("core/foo/1.0/20180704142702").unwrap();
        let packages = vec![
            empty_package_deps(a.clone()),
            package_deps(b.clone(), &vec![a.clone()]),
        ];

        let (ncount, ecount) = graph.build(packages.into_iter()).unwrap();
        assert_eq!(ncount, 2);
        assert_eq!(ecount, 1);

        assert_eq!(graph.count_rdeps(&a).unwrap(), 1);

        assert_eq!(graph.remove(&a).unwrap(), false);
        assert_eq!(graph.count_packages(), 2);
    }
}
