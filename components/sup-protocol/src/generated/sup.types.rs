#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationEnvironment {
    #[prost(string, required, tag="1")]
    pub application: std::string::String,
    #[prost(string, required, tag="2")]
    pub environment: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PackageIdent {
    #[prost(string, required, tag="1")]
    pub origin: std::string::String,
    #[prost(string, required, tag="2")]
    pub name: std::string::String,
    #[prost(string, optional, tag="3")]
    pub version: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub release: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProcessStatus {
    #[prost(int64, optional, tag="1")]
    pub elapsed: ::std::option::Option<i64>,
    #[prost(uint32, optional, tag="2")]
    pub pid: ::std::option::Option<u32>,
    #[prost(enumeration="ProcessState", required, tag="3")]
    pub state: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceBind {
    #[prost(string, required, tag="1")]
    pub name: std::string::String,
    #[prost(message, required, tag="2")]
    pub service_group: ServiceGroup,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceCfg {
    /// The self describing string format used in each configuration field. This
    /// is present if we ever change from using TOML to represent service configurations
    /// to another self describing type.
    #[prost(enumeration="service_cfg::Format", optional, tag="1", default="Toml")]
    pub format: ::std::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub default: ::std::option::Option<std::string::String>,
}
pub mod service_cfg {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum Format {
        Toml = 0,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceGroup {
    #[prost(string, required, tag="1")]
    pub service: std::string::String,
    #[prost(string, required, tag="2")]
    pub group: std::string::String,
    #[prost(message, optional, tag="3")]
    pub application_environment: ::std::option::Option<ApplicationEnvironment>,
    #[prost(string, optional, tag="4")]
    pub organization: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceStatus {
    #[prost(message, required, tag="1")]
    pub ident: PackageIdent,
    #[prost(message, optional, tag="2")]
    pub process: ::std::option::Option<ProcessStatus>,
    #[prost(message, required, tag="3")]
    pub service_group: ServiceGroup,
    #[prost(enumeration="DesiredState", optional, tag="5")]
    pub desired_state: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HealthCheckInterval {
    #[prost(uint64, required, tag="1")]
    pub seconds: u64,
}
/// Encapsulate all possible sources we can install packages from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstallSource {
    /// Install from a remote hosting the package
    Ident = 0,
    /// Install from a local archive file
    Archive = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessState {
    Down = 0,
    Up = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DesiredState {
    /// The DesiredNone variant allows backwards compatibility of current hab binaries
    /// with older (< 0.61) Supervisors.
    DesiredNone = -1,
    DesiredDown = 0,
    DesiredUp = 1,
}
/// The relationship of a service with peers in the same service group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Topology {
    Standalone = 0,
    Leader = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateStrategy {
    None = 0,
    AtOnce = 1,
    Rolling = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BindingMode {
    /// Services may start whether binds are available or not
    Relaxed = 0,
    /// Service start-up is blocked until all binds are available
    Strict = 1,
}
