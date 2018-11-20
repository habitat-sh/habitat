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

use std::path::Path;

use super::{DependencyDirection, Scope};
use common::package_graph::PackageGraph;
use error::Result;
use hcore::package::{PackageIdent, PackageInstall};

/// Show the dependencies for an installed package.
///
/// We can either show the dependencies of the package or show the packages that are dependent on the provided
/// identifier
///
pub fn start(
    ident: &PackageIdent,
    scope: &Scope,
    direction: &DependencyDirection,
    fs_root_path: &Path,
) -> Result<()> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;

    let mut graph = PackageGraph::new();
    graph.load(fs_root_path)?;

    let deps = match &direction {
        DependencyDirection::Down => match &scope {
            Scope::Package => graph.deps(&pkg_install.ident()).unwrap_or(vec![]),
            Scope::PackageAndDependencies => graph.ordered_deps(&pkg_install.ident()),
        },
        DependencyDirection::Up => match &scope {
            Scope::Package => graph.rdeps(&pkg_install.ident()).unwrap_or(vec![]),
            Scope::PackageAndDependencies => graph.ordered_reverse_deps(&pkg_install.ident()),
        },
    };

    for dep in &deps {
        println!("{}", dep);
    }

    Ok(())
}
