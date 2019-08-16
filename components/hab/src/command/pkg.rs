pub mod binlink;
pub mod build;
pub mod channels;
pub mod delete;
pub mod demote;
pub mod dependencies;
pub mod download;
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
