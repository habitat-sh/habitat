#[allow(unused_variables)]
#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as implementation;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as implementation;

// Common functions across platforms
pub use implementation::{assert_pkg_user_and_group,
                         get_current_groupname,
                         get_current_username,
                         get_effective_uid,
                         get_gid_by_name,
                         get_home_for_user,
                         get_uid_by_name};

// Unix-specific functions
#[cfg(unix)]
pub use unix::{get_effective_gid,
               get_effective_groupname,
               get_effective_username,
               get_members_by_groupname};
