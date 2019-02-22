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

use std::{fs,
          path::Path,
          str::FromStr};

use super::{ExecutionStrategy,
            Scope};
use crate::{common::{package_graph::PackageGraph,
                     ui::{Status,
                          UIWriter,
                          UI}},
            error::{Error,
                    Result},
            hcore::{error as herror,
                    fs as hfs,
                    package::{Identifiable,
                              PackageIdent,
                              PackageInstall}}};

use crate::hcore::package::list::temp_package_directory;

/// Delete a package and all dependencies which are not used by other packages.
/// We do an ordered traverse of the dependencies, updating the graph as we delete a
/// package. This lets us use simple logic where we continually check if the package
/// we're trying to delete currently has no packages depending on it.
///
/// The full logic is:
/// 1. We find the fully qualified package ident and all its dependencies
/// 2. We find all packages on the filesystem and convert them into a graph
/// 3. We do a BFS on the graph to get the dependencies in order
/// 4. We check if the specified package has any reverse deps
///     4a. If there are, we throw an error
///     4b. If not, we delete the package
/// 5. For each dependency we check if there are any packages which depend on it
///     5a. If there are, we skip it
///     5b. If there are not, we delete it from disk and the graph
///
/// `excludes` is a list of user-supplied `PackageIdent`s.
/// `services` is a list of fully-qualified `PackageIdent`s which are currently
///    running in a supervisor out of the `fs_root_path`.
pub fn start(
    ui: &mut UI,
    ident: &PackageIdent,
    fs_root_path: &Path,
    execution_strategy: ExecutionStrategy,
    scope: Scope,
    excludes: Vec<PackageIdent>,
    services: Vec<PackageIdent>,
) -> Result<()> {
    // 1.
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    let ident = pkg_install.ident();
    ui.begin(format!("Uninstalling {}", &ident))?;

    if !services.is_empty() {
        ui.status(
            Status::Determining,
            "list of running services in supervisor",
        )?;
        for s in &services {
            ui.status(Status::Found, format!("running service {}", s))?;
        }
    }

    // 2.
    let mut graph = PackageGraph::from_root_path(fs_root_path)?;

    // 3.
    let deps = graph.owned_ordered_deps(&ident);

    // 4.
    match graph.count_rdeps(&ident) {
        None => {
            // package not in graph - this shouldn't happen but could be a race condition in Step 2
            // with another hab uninstall. We can continue as what we wanted (package to
            // be removed) has already happened. We're going to continue and try and delete down
            // through the dependency tree anyway
            ui.warn(format!(
                "Tried to find dependant packages of {} but it wasn't in graph.  Maybe another \
                 uninstall command was run at the same time?",
                &ident
            ))?;
        }
        Some(0) => {
            maybe_delete(
                ui,
                &fs_root_path,
                &pkg_install,
                &execution_strategy,
                &excludes,
                &services,
            )?;
            graph.remove(&ident);
        }
        Some(c) => {
            return Err(Error::CannotRemovePackage(ident.clone(), c));
        }
    }

    // 5.
    let mut count = 0;
    match scope {
        Scope::Package => {
            ui.status(Status::Skipping, "dependencies (--no-deps specified)")?;
        }
        Scope::PackageAndDependencies => {
            for p in &deps {
                match graph.count_rdeps(&p) {
                    None => {
                        // package not in graph - this shouldn't happen but could be a race
                        // condition in Step 2 with another hab uninstall. We can
                        // continue as what we wanted (package to be removed) has already happened.
                        // We're going to continue and try and delete down through the
                        // dependency tree anyway
                        ui.warn(format!(
                            "Tried to find dependant packages of {} but it wasn't in graph.  \
                             Maybe another uninstall command was run at the same time?",
                            &p
                        ))?;
                    }
                    Some(0) => {
                        let install = PackageInstall::load(&p, Some(fs_root_path))?;
                        maybe_delete(
                            ui,
                            &fs_root_path,
                            &install,
                            &execution_strategy,
                            &excludes,
                            &services,
                        )?;

                        graph.remove(&p);
                        count += 1;
                    }
                    Some(c) => {
                        ui.status(
                            Status::Skipping,
                            format!("{}. It is a dependency of {} packages", &p, c),
                        )?;
                    }
                }
            }
        }
    }

    match execution_strategy {
        ExecutionStrategy::DryRun => {
            ui.end(format!(
                "Would uninstall {} and {} dependencies (Dry run)",
                &ident, count
            ))?;
        }
        ExecutionStrategy::Run => {
            ui.end(format!(
                "Uninstall of {} and {} dependencies complete",
                &ident, count
            ))?;
        }
    };
    Ok(())
}

/// Delete a package from disk, depending upon the ExecutionStrategy supplied
///
/// Returns:
///   Ok(true) - package is deleted
///   Ok(false) - package would be deleted but it's a dry run
///   Err(_) -  IO problem deleting package from filesystem
fn maybe_delete(
    ui: &mut UI,
    fs_root_path: &Path,
    install: &PackageInstall,
    strategy: &ExecutionStrategy,
    excludes: &[PackageIdent],
    services: &[PackageIdent],
) -> Result<bool> {
    let ident = install.ident();
    let pkg_root_path = hfs::pkg_root_path(Some(fs_root_path));

    let hab = PackageIdent::from_str("core/hab")?;
    if ident.satisfies(&hab) {
        ui.status(
            Status::Skipping,
            format!("{}. You can't uninstall core/hab", &ident),
        )?;
        return Ok(false);
    }

    let is_running = services.iter().any(|i| i.satisfies(ident));
    if is_running {
        ui.status(
            Status::Skipping,
            format!("{}. It is currently running in the supervisor", &ident),
        )?;
        return Ok(false);
    }

    // The excludes list could be looser than the fully qualified idents.  E.g. if core/redis is on
    // the exclude list then we should exclude core/redis/1.1.0/20180608091936.  We use the
    // `Identifiable` trait which supplies this logic for PackageIdents
    let should_exclude = excludes.iter().any(|i| i.satisfies(ident));
    if should_exclude {
        ui.status(
            Status::Skipping,
            format!("{}. It is on the exclusion list", &ident),
        )?;
        return Ok(false);
    }

    match strategy {
        ExecutionStrategy::DryRun => {
            ui.status(Status::DryRunDeleting, &ident)?;
            Ok(false)
        }
        ExecutionStrategy::Run => {
            ui.status(Status::Deleting, &ident)?;
            let pkg_dir = install.installed_path();
            do_clean_delete(&pkg_root_path, &pkg_dir)
        }
    }
}

/// Delete empty parent directories from a given path. don't traverse above
/// the `pkg_root_path`
fn do_clean_delete(pkg_root_path: &Path, real_install_path: &Path) -> Result<bool> {
    // This match will always return Ok(Path) as the install path is always 4 levels
    // below the pkg_root_path
    match real_install_path.parent() {
        Some(real_install_base) => {
            let temp_install_path = temp_package_directory(real_install_path)?;
            fs::rename(&real_install_path, &temp_install_path)?;
            fs::remove_dir_all(&temp_install_path)?;

            for p in real_install_base.ancestors() {
                // Let's be safe and not rm below the package directories
                if p == pkg_root_path {
                    break;
                }
                match p.read_dir() {
                    Ok(contents) => {
                        // This will calculate the amount of items in the directory
                        match contents.count() {
                            0 => fs::remove_dir(&p)?,
                            _ => break,
                        }
                    }
                    Err(e) => return Err(Error::HabitatCore(herror::Error::IO(e))),
                }
            }
            Ok(true)
        }
        None => unreachable!("Install path doesn't have a parent"),
    }
}
