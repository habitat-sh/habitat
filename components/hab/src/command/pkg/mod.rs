// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

pub mod binlink;
pub mod build;
pub mod channels;
pub mod demote;
pub mod dependencies;
pub mod env;
pub mod exec;
pub mod export;
pub mod hash;
pub mod header;
pub mod info;
pub mod list;
pub mod path;
pub mod promote;
pub mod provides;
pub mod search;
pub mod sign;
pub mod uninstall;
pub mod upload;
pub mod verify;

/// Used in commands like uninstall which provide a --dry-run option
#[derive(Clone, Copy)]
pub enum ExecutionStrategy {
    /// Don't actually run commands that mutate the state of the system,
    /// simply print their output
    DryRun,
    /// Run commands which mutate state
    Run,
}

/// Used in `hab pkg` commands to choose where to apply the command to just a package
/// or the package and its dependencies
#[derive(Clone, Copy)]
pub enum Scope {
    Package,
    PackageAndDependencies,
}

/// Express the relationship between two packages
/// `Requires`: a dependency from a package to one it depends on
/// `Supports`: a dependency from a package to one that depends on it
#[derive(Clone, Copy)]
pub enum DependencyRelation {
    Requires,
    Supports,
}
