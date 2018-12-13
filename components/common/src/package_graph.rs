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

use bimap::BiMap;
use error::Result;
use hcore::fs as hfs;
use hcore::package;
use hcore::package::ident::PackageIdent;
use hcore::package::PackageInstall;
use petgraph;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::{Bfs, Reversed, Walker};

use std::path::Path;

pub struct PackageGraph {
    nodes: BiMap<PackageIdent, NodeIndex>,
    graph: StableGraph<PackageIdent, usize, petgraph::Directed>,
}

impl PackageGraph {
    fn empty() -> Self {
        PackageGraph {
            nodes: BiMap::new(),
            graph: StableGraph::new(),
        }
    }

    /// Construct a `PackageGraph` from all the packages stored in in the habitat `pkgs`
    /// directory
    pub fn from_root_path(fs_root_path: &Path) -> Result<Self> {
        let mut pg = Self::empty();
        pg.load(fs_root_path)?;
        Ok(pg)
    }

    /// Load a set of packages that are stored in a package_path under a habitat
    /// root directory
    fn load(&mut self, fs_root_path: &Path) -> Result<()> {
        let package_path = hfs::pkg_root_path(Some(&fs_root_path));
        let idents = package::all_packages(&package_path)?;

        for ident in idents {
            let p = PackageInstall::load(&ident, Some(fs_root_path))?;
            let deps = p.deps()?;
            self.extend(&ident, &deps);
        }

        Ok(())
    }

    /// Return (and possibly create) a NodeIndex for a given PackageIdent.
    /// Upon returning, the node will be guaranteed to be in the graph
    ///
    fn node_idx(&mut self, package: &PackageIdent) -> NodeIndex {
        match self.nodes.get_by_left(package) {
            Some(&idx) => idx,
            None => {
                let idx = self.graph.add_node(package.clone());

                // BiMap only allows a value to appear one time in the left
                // and right hand sides of the map.  Let's assert that we're not going to
                // move the `idx` from one package to another.
                assert!(!self.nodes.contains_right(&idx));

                self.nodes.insert(package.clone(), idx);

                idx
            }
        }
    }

    /// Extend a graph by adding in dependencies for a package
    fn extend(&mut self, package: &PackageIdent, deps: &[PackageIdent]) -> (usize, usize) {
        let idx = self.node_idx(package);

        for dep in deps {
            let dep_idx = self.node_idx(dep);
            self.graph.extend_with_edges(&[(idx, dep_idx)]);
        }

        assert_eq!(self.graph.node_count(), self.nodes.len());

        (self.graph.node_count(), self.graph.edge_count())
    }

    /// Return the dependencies of a given Package Identifier
    ///
    /// Returns `None` if the package identifier is not in the graph
    pub fn ordered_deps(&self, package: &PackageIdent) -> Vec<&PackageIdent> {
        self.nodes
            .get_by_left(package)
            .map(|&idx| {
                // BFS returns the original node as the first node
                // `skip` it here so it's not in the result Vec
                let bfs = Bfs::new(&self.graph, idx).iter(&self.graph).skip(1);

                bfs.map(|child| self.graph.node_weight(child).unwrap())
                    .collect()
            }).unwrap_or_else(Vec::new)
    }

    /// Return the dependencies of a given Package Identifier as `PackageIdent`s. This
    /// allows you to modify the underlying graph (via `PackageGraph::remove`) while traversing the
    /// dependencies
    ///
    /// Returns `None` if the package identifier is not in the graph

    pub fn owned_ordered_deps(&self, package: &PackageIdent) -> Vec<PackageIdent> {
        self.ordered_deps(&package)
            .iter()
            .map(|&p| p.clone())
            .collect()
    }
    /// Return the reverse dependencies of a given Package Identifier
    ///
    /// Returns `None` if the package identifier is not in the graph
    pub fn ordered_reverse_deps(&self, package: &PackageIdent) -> Vec<&PackageIdent> {
        self.nodes
            .get_by_left(package)
            .map(|&idx| {
                // BFS returns the original node as the first node
                // `skip` it here so it's not in the result Vec
                let bfs = Bfs::new(&self.graph, idx)
                    .iter(Reversed(&self.graph))
                    .skip(1);

                bfs.map(|child| self.graph.node_weight(child).unwrap())
                    .collect()
            }).unwrap_or_else(Vec::new)
    }

    /// Remove a package from a graph
    ///
    /// This will not remove the package if it is a dependency of any package
    ///
    /// Returns true if package is removed
    /// Return  false if package was not removed
    pub fn remove(&mut self, package: &PackageIdent) -> bool {
        if let Some(0) = self.count_rdeps(package) {
            self.do_remove(package)
        } else {
            false
        }
    }

    /// Cleanly remove the node from both the node list and the graph
    ///
    /// Returns true if package is removed
    /// Returns false if package was not in graph
    fn do_remove(&mut self, package: &PackageIdent) -> bool {
        self.nodes
            .remove_by_left(package)
            .map(|(_, idx)| {
                match self.graph.remove_node(idx) {
                    Some(ident) => assert_eq!(&ident, package),
                    None => panic!(
                        "removed node from map but it wasn't in the graph: {}",
                        package
                    ),
                }
                true
            }).unwrap_or(false)
    }

    /// does a specific PackageIdent appear in the graph
    ///
    pub fn has_package(&self, package: &PackageIdent) -> bool {
        self.nodes.contains_left(package)
    }

    fn count_edges(&self, package: &PackageIdent, direction: petgraph::Direction) -> Option<usize> {
        self.nodes
            .get_by_left(package)
            .map(|&idx| self.graph.neighbors_directed(idx, direction).count())
    }

    /// Returns the number of package which have this package as a dependency
    ///
    /// Returns `None` if the package is not in the graph
    pub fn count_rdeps(&self, ident: &PackageIdent) -> Option<usize> {
        self.count_edges(ident, petgraph::Incoming)
    }

    fn neighbours(
        &self,
        package: &PackageIdent,
        direction: petgraph::Direction,
    ) -> Vec<&PackageIdent> {
        self.nodes
            .get_by_left(package)
            .map(|&idx| {
                self.graph
                    .neighbors_directed(idx, direction)
                    .map(|n| self.nodes.get_by_right(&n).unwrap()) //  unwrap here is ok as we have consistency between `self.graph` and `self.nodes`
                    .collect()
            }).unwrap_or_else(Vec::new)
    }

    /// Returns the direct dependencies for a package.
    ///
    /// Returns `None` if the package is not in the graph
    pub fn deps(&self, package: &PackageIdent) -> Vec<&PackageIdent> {
        self.neighbours(package, petgraph::Outgoing)
    }

    /// Returns the direct reverse dependencies for a package.
    ///
    /// Returns `None` if the package is not in the graph
    pub fn rdeps(&self, package: &PackageIdent) -> Vec<&PackageIdent> {
        self.neighbours(package, petgraph::Incoming)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    impl PackageGraph {
        /// Returns the number of dependencies for a package
        ///
        /// Returns `None` if the package is not in the graph
        pub fn count_deps(&self, ident: &PackageIdent) -> Option<usize> {
            self.count_edges(ident, petgraph::Outgoing)
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

    struct PackageDeps {
        ident: PackageIdent,
        deps: Vec<PackageIdent>,
    }

    fn build(packages: Vec<PackageDeps>) -> PackageGraph {
        let mut graph = PackageGraph::empty();
        for p in &packages {
            graph.extend(&p.ident, &p.deps);
        }
        graph
    }

    fn empty_package_deps(ident: PackageIdent) -> PackageDeps {
        PackageDeps {
            ident,
            deps: vec![],
        }
    }

    fn package_deps(ident: PackageIdent, deps: &Vec<PackageIdent>) -> PackageDeps {
        PackageDeps {
            ident,
            deps: deps.to_vec(),
        }
    }

    fn empty_vec() -> Vec<&'static PackageIdent> {
        vec![]
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
    fn different_origins_graph() {
        // We have non-standard implementations of `Ord`, `PartialOrd` for `PackageIdent`.  Make sure this
        // doesn't mess with the requirements of `BiMap`
        let packages = vec![
            empty_package_deps(PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap()),
            empty_package_deps(PackageIdent::from_str("mine/redis/2.1.0/20180704142101").unwrap()),
        ];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 0);
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
    fn deps() {
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

        assert_eq!(graph.deps(&a), empty_vec());
        assert_eq!(graph.deps(&b), vec![&a]);
        assert_eq!(graph.deps(&c), vec![&a]);
        let result = graph.deps(&d);
        assert!(result.contains(&b.as_ref()));
        assert!(result.contains(&c.as_ref()));
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
    fn rdeps() {
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

        let rdeps = graph.rdeps(&a);
        assert!(rdeps.contains(&b.as_ref()));
        assert!(rdeps.contains(&c.as_ref()));
        assert_eq!(graph.rdeps(&b), vec![&d]);
        assert_eq!(graph.rdeps(&c), vec![&d]);
        assert_eq!(graph.rdeps(&d), empty_vec());
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

        assert!(graph.remove(&d));
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

        assert_eq!(graph.remove(&a), false);
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
        assert_eq!(odeps, empty_vec());
    }

    #[test]
    fn ordered_deps_non_existent_package() {
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
        assert_eq!(graph.ordered_deps(&does_not_exist), empty_vec());
        assert_eq!(graph.ordered_reverse_deps(&does_not_exist), empty_vec());
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
        let expected = vec![&c, &b, &a];
        assert_eq!(expected, odeps);
    }

    #[test]
    fn owned_ordered_deps_are_in_order() {
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

        let odeps = graph.owned_ordered_deps(&d);
        let expected = vec![c, b, a];
        assert_eq!(expected, odeps);
    }
    #[test]
    fn ordered_reverse_deps_of_empty_deps() {
        let a = PackageIdent::from_str("core/redis/2.1.0/20180704142101").unwrap();
        let packages = vec![empty_package_deps(a.clone())];

        let graph = build(packages);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);

        let odeps = graph.ordered_reverse_deps(&a);
        assert_eq!(odeps, empty_vec());
    }

    #[test]
    fn ordered_reverse_deps_are_in_order() {
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

        let odeps = graph.ordered_reverse_deps(&a);
        let expected = vec![&b, &c, &d];
        assert_eq!(expected, odeps);
    }
}
