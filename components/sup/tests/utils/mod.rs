//! Utility functions for testing a Supervisor
pub mod fixture_root;
pub mod fs;
pub mod hab_root;
pub mod test_butterfly;
pub mod test_sup;

// Re-export the key structs of this package for ergonomics.
pub use self::{fixture_root::FixtureRoot,
               fs::{copy_dir,
                    setup_package_files,
                    FileSnapshot,
                    FileSystemSnapshot},
               hab_root::HabRoot,
               test_sup::TestSup};
