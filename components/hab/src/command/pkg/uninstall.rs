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

use std::fs;
use std::path::Path;
use std::str::FromStr;

use super::{ExecutionStrategy, Scope};
use common::package_graph::PackageGraph;
use common::ui::{Status, UIWriter, UI};
use error::{Error, Result};
use hcore::error as herror;
use hcore::fs as hfs;
use hcore::package::{all_packages, Identifiable, PackageIdent, PackageInstall};

use hcore::package::list::temp_package_directory;

///
/// Delete a package and all dependencies which are not used by other packages.
/// We do an ordered traverse of the dependencies, updating the graph as we delete a
/// package. This lets us use simple logic where we continually check if the package
/// we're trying to delete currently has no packages depending on it.
///
/// The full logic is:
/// 1. We find the fully qualified package ident and all its dependencies
/// 2. We find all packages on the filesystem and convert them into a graph
/// 3. We do a BFS on the graph to get the dependencies in order
/// 4. Update our excludes list with any running services
/// 4. We check if the specified package has any reverse deps
///     4a. If there are, we throw an error
///     4b. If not, we delete the package
/// 5. For each dependency we check if there are any packages which depend on it
///     5a. If there are, we skip it
///     5b. If there are not, we delete it from disk and the graph
///
pub fn start(
    ui: &mut UI,
    ident: &PackageIdent,
    fs_root_path: &Path,
    execution_strategy: ExecutionStrategy,
    scope: Scope,
    cli_excludes: Vec<PackageIdent>,
) -> Result<()> {
    // 1.
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    let ident = pkg_install.ident();
    ui.begin(format!("Uninstalling {}", &ident))?;

    // 2.
    let packages = load_all_packages(&fs_root_path)?;
    let mut graph = PackageGraph::new();
    graph.build(&packages)?;

    // 3.
    let deps = graph.ordered_deps(&ident);

    // 4. Update excludes if a supervisor is running
    let excludes = if launcher_is_running(&fs_root_path) {
        ui.status(
            Status::Determining,
            "list of running services in supervisor",
        )?;
        with_supervisor_excludes(&fs_root_path, cli_excludes)?
    } else {
        cli_excludes
    };

    // 5.
    match graph.count_rdeps(&ident) {
        None => {
            // package not in graph - this shouldn't happen but could be a race condition in Step 2 with another hab uninstall. We can
            // continue as what we wanted (package to be removed) has already happened. We're going to continue and try and delete down through the
            // dependency tree anyway
            ui.warn(format!("Tried to find dependant packages of {} but it wasn't in graph.  Maybe another uninstall command was run at the same time?", &ident))?;
        }
        Some(0) => {
            maybe_delete(
                ui,
                &fs_root_path,
                &pkg_install,
                &execution_strategy,
                &excludes,
            )?;
            graph.remove(&ident);
        }
        Some(c) => {
            return Err(Error::CannotRemovePackage(ident.clone(), c));
        }
    }

    // 6.
    let mut count = 0;
    match scope {
        Scope::Package => {
            ui.status(Status::Skipping, "dependencies (--no-deps specified)")?;
        }
        Scope::PackageAndDependencies => {
            for p in &deps {
                match graph.count_rdeps(&p) {
                    None => {
                        // package not in graph - this shouldn't happen but could be a race condition in Step 2 with another hab uninstall. We can
                        // continue as what we wanted (package to be removed) has already happened. We're going to continue and try and delete down through the
                        // dependency tree anyway
                        ui.warn(format!("Tried to find dependant packages of {} but it wasn't in graph.  Maybe another uninstall command was run at the same time?", &p))?;
                    }
                    Some(0) => {
                        let install = packages.iter().find(|&i| i.ident() == p).unwrap();
                        maybe_delete(ui, &fs_root_path, &install, &execution_strategy, &excludes)?;

                        graph.remove(&p);
                        count = count + 1;
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
    excludes: &Vec<PackageIdent>,
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

    // The excludes list could be looser than the fully qualified idents.  E.g. if core/redis is on the
    // exclude list then we should exclude core/redis/1.1.0/20180608091936.  We use the `Identifiable`
    // trait which supplies this logic for PackageIdents
    let should_exclude = excludes.iter().any(|i| i.satisfies(ident));
    if should_exclude {
        ui.status(
            Status::Skipping,
            format!("{}. It is on the exclusion list", &ident),
        )?;
        Ok(false)
    } else {
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
                        match contents.collect::<Vec<_>>().len() {
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

fn load_all_packages(fs_root_path: &Path) -> Result<Vec<PackageInstall>> {
    let package_path = hfs::pkg_root_path(Some(&fs_root_path));
    let idents = all_packages(&package_path)?;

    let mut result = Vec::with_capacity(idents.len());
    for i in idents {
        let pkg_install = PackageInstall::load(&i, Some(fs_root_path))?;
        result.push(pkg_install);
    }
    Ok(result)
}

/// Check if we have a launcher/supervisor running out of this habitat root.
/// If the launcher PID file exists then the supervisor is up and running
fn launcher_is_running(fs_root_path: &Path) -> bool {
    let launcher_root = hfs::launcher_root_path(Some(fs_root_path));

    let pid_file_path = launcher_root.join("PID");

    pid_file_path.is_file()
}

fn with_supervisor_excludes(
    _fs_root_path: &Path,
    excludes: Vec<PackageIdent>,
) -> Result<Vec<PackageIdent>> {
    Ok(excludes)
}
