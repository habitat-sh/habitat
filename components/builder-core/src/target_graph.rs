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

use std::str::FromStr;
use std::iter::Iterator;
use std::collections::HashMap;
use hab_core::package::PackageTarget;
use protocol::scheduler;
use package_graph::PackageGraph;

pub struct TargetGraphStats {
    pub target: PackageTarget,
    pub node_count: usize,
    pub edge_count: usize,
}

pub struct TargetGraph {
    graphs: HashMap<PackageTarget, PackageGraph>,
}

impl TargetGraph {
    pub fn new() -> Self {
        let mut graphs = HashMap::new();

        // We only support the following targets currently
        for target_str in &["x86_64-linux", "x86_64-windows"] {
            graphs.insert(
                PackageTarget::from_str(target_str).unwrap(),
                PackageGraph::new(),
            );
        }

        TargetGraph { graphs: graphs }
    }

    pub fn graph(&self, target_str: &str) -> Option<&PackageGraph> {
        match PackageTarget::from_str(target_str) {
            Ok(target) => self.graphs.get(&target),
            Err(err) => {
                error!(
                    "Invalid target specified for TargetGraph: {}! Err: {}",
                    target_str,
                    err
                );
                None
            }
        }
    }

    pub fn graph_mut(&mut self, target_str: &str) -> Option<&mut PackageGraph> {
        match PackageTarget::from_str(target_str) {
            Ok(target) => self.graphs.get_mut(&target),
            Err(err) => {
                error!(
                    "Invalid target specified for TargetGraph: {}! Err: {}",
                    target_str,
                    err
                );
                None
            }
        }
    }

    pub fn build<T>(&mut self, packages: T) -> Vec<TargetGraphStats>
    where
        T: Iterator<Item = scheduler::Package>,
    {
        for p in packages {
            match self.graph_mut(p.get_target()) {
                Some(ref mut graph) => {
                    graph.extend(&p);
                }
                None => (),
            }
        }

        let mut target_stats = Vec::new();
        for (target, graph) in self.graphs.iter() {
            let stats = graph.stats();
            let ts = TargetGraphStats {
                target: target.clone(),
                node_count: stats.node_count,
                edge_count: stats.edge_count,
            };
            target_stats.push(ts);
        }

        target_stats
    }
}
