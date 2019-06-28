#[allow(unused_variables)]
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::{assert_pkg_user_and_group,
                        can_run_services_as_svc_user,
                        get_current_groupname,
                        get_current_username,
                        get_effective_uid,
                        get_gid_by_name,
                        get_home_for_user,
                        get_uid_by_name,
                        root_level_account};

#[cfg(unix)]
pub mod linux;

#[cfg(unix)]
pub use self::linux::{assert_pkg_user_and_group,
                      can_run_services_as_svc_user,
                      get_current_groupname,
                      get_current_username,
                      get_effective_gid,
                      get_effective_groupname,
                      get_effective_uid,
                      get_effective_username,
                      get_gid_by_name,
                      get_home_for_user,
                      get_uid_by_name,
                      root_level_account};
