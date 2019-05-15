mod listen_ctl_addr;

pub use self::listen_ctl_addr::ListenCtlAddr;

/// Bundles up information about the user and group that a supervised
/// service should be run as. If the Supervisor itself is running with
/// root-like permissions, then these will be for `SVC_USER` and
/// `SVC_GROUP` for a service. If not, it will be for the user the
/// Supervisor itself is running as.
///
/// On Windows, all but `username` will be `None`. On Linux,
/// `username` and `groupname` may legitimately be `None`, but `uid`
/// and `gid` should always be `Some`.
#[derive(Debug, Default)]
pub struct UserInfo {
    /// Windows required, Linux optional
    pub username: Option<String>,
    /// Linux preferred
    pub uid: Option<u32>,
    /// Linux optional
    pub groupname: Option<String>,
    /// Linux preferred
    pub gid: Option<u32>,
}
