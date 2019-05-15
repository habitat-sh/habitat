use std::path::Path;

use super::{DependencyRelation,
            Scope};
use crate::{common::package_graph::PackageGraph,
            error::Result,
            hcore::package::{PackageIdent,
                             PackageInstall}};

/// Show the dependencies for an installed package.
///
/// We can either show the dependencies of the package or show the packages that are dependent on
/// the provided identifier
pub fn start(ident: &PackageIdent,
             scope: Scope,
             direction: DependencyRelation,
             fs_root_path: &Path)
             -> Result<()> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;

    let graph = PackageGraph::from_root_path(fs_root_path)?;

    let deps = match &direction {
        DependencyRelation::Requires => {
            match &scope {
                Scope::Package => graph.deps(&pkg_install.ident()),
                Scope::PackageAndDependencies => graph.ordered_deps(&pkg_install.ident()),
            }
        }
        DependencyRelation::Supports => {
            match &scope {
                Scope::Package => graph.rdeps(&pkg_install.ident()),
                Scope::PackageAndDependencies => graph.ordered_reverse_deps(&pkg_install.ident()),
            }
        }
    };

    for dep in &deps {
        println!("{}", dep);
    }

    Ok(())
}
