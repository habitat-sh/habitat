/// Networked progress bar for displaying a remote request's operation status over time.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetProgress {
    /// Number of total units until bar is complete.
    #[prost(uint64, required, tag="1")]
    pub total: u64,
    /// Number of total units processed thus far.
    #[prost(uint64, required, tag="2")]
    pub position: u64,
}
/// Client to server request for authenticating a client connection. This is the first message a
/// SrvProtocol client will make to a SrvProtocol server.
///
/// If the `secret_key` provided matches with what the server has then the client may continue
/// sending requests. Connections will be aborted by the server if there is no match.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Handshake {
    /// A shared secret between the destination server and the calling client.
    #[prost(string, optional, tag="1")]
    pub secret_key: ::std::option::Option<std::string::String>,
}
/// Wrapper type for a list of ServiceBinds.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServiceBindList {
    #[prost(message, repeated, tag="1")]
    pub binds: ::std::vec::Vec<super::types::ServiceBind>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SupDepart {
    #[prost(string, optional, tag="1")]
    pub member_id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcFilePut {
    #[prost(message, optional, tag="1")]
    pub service_group: ::std::option::Option<super::types::ServiceGroup>,
    /// TODO: Make this a string
    #[prost(bytes, optional, tag="2")]
    pub content: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="3")]
    pub filename: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="4")]
    pub version: ::std::option::Option<u64>,
    #[prost(bool, optional, tag="5", default="false")]
    pub is_encrypted: ::std::option::Option<bool>,
}
/// Request for retrieving the default configuration for a given service.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcGetDefaultCfg {
    /// Package identifier to target running service.
    #[prost(message, optional, tag="1")]
    pub ident: ::std::option::Option<super::types::PackageIdent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcValidateCfg {
    /// Service group of a running service to validate a configuration change against.
    #[prost(message, optional, tag="1")]
    pub service_group: ::std::option::Option<super::types::ServiceGroup>,
    /// Structured and self-describing string format contained in the configuration string.
    #[prost(enumeration="super::types::service_cfg::Format", optional, tag="2", default="Toml")]
    pub format: ::std::option::Option<i32>,
    /// Unencrypted configuration to validate.
    #[prost(bytes, optional, tag="3")]
    pub cfg: ::std::option::Option<std::vec::Vec<u8>>,
}
/// Request to set a running service's configuration to the given values.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcSetCfg {
    /// Service group of a running service to set a new configuration for.
    #[prost(message, optional, tag="1")]
    pub service_group: ::std::option::Option<super::types::ServiceGroup>,
    /// Encrypted configuration to set.
    ///
    /// TODO: Make this a string
    #[prost(bytes, optional, tag="2")]
    pub cfg: ::std::option::Option<std::vec::Vec<u8>>,
    /// Incarnation of this configuration.
    #[prost(uint64, optional, tag="3")]
    pub version: ::std::option::Option<u64>,
    /// If the payload in `cfg` is encrypted with the remote Supervisor's Ring Key.
    #[prost(bool, optional, tag="4", default="false")]
    pub is_encrypted: ::std::option::Option<bool>,
}
/// Request to load a new service.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcLoad {
    /// Package identifier for the service to load. Using a more qualified identifier will load a
    /// more specific package.
    #[prost(message, optional, tag="1")]
    pub ident: ::std::option::Option<super::types::PackageIdent>,
    /// An application environment to place the service in.
    #[prost(message, optional, tag="2")]
    pub application_environment: ::std::option::Option<super::types::ApplicationEnvironment>,
    /// List of service binds to use when configuring the service.
    #[prost(message, optional, tag="3")]
    pub binds: ::std::option::Option<ServiceBindList>,
    /// Set to true if any binds were set by the caller. This field is needed because a blank list
    /// and map will be present in a case when no binds were set *and* when binds are removed.
    #[prost(bool, optional, tag="5")]
    pub specified_binds: ::std::option::Option<bool>,
    /// Indicate how bind availability affects service start-up
    #[prost(enumeration="super::types::BindingMode", optional, tag="14")]
    pub binding_mode: ::std::option::Option<i32>,
    /// Remote http URL for the Builder service to receive package updates from.
    #[prost(string, optional, tag="6")]
    pub bldr_url: ::std::option::Option<std::string::String>,
    /// Remote channel on the Builder service to receive package updates from.
    #[prost(string, optional, tag="7")]
    pub bldr_channel: ::std::option::Option<std::string::String>,
    /// A filepath on disk which can be specified to override the package's configuration and hooks.
    /// This is useful when testing services on a local Supervisor before packaging them.
    #[prost(string, optional, tag="8")]
    pub config_from: ::std::option::Option<std::string::String>,
    /// If set to true, any loaded service matching this request's package ident will be unloaded
    /// and this request's will replace it.
    #[prost(bool, optional, tag="9", default="false")]
    pub force: ::std::option::Option<bool>,
    /// Service group name for the service.
    #[prost(string, optional, tag="10", default="default")]
    pub group: ::std::option::Option<std::string::String>,
    /// Encrypted password for a Windows service.
    #[prost(string, optional, tag="11")]
    pub svc_encrypted_password: ::std::option::Option<std::string::String>,
    /// Topology which the service will run in.
    #[prost(enumeration="super::types::Topology", optional, tag="12")]
    pub topology: ::std::option::Option<i32>,
    /// Update strategy for the service.
    #[prost(enumeration="super::types::UpdateStrategy", optional, tag="13")]
    pub update_strategy: ::std::option::Option<i32>,
    /// Health Check interval for the service
    #[prost(message, optional, tag="15")]
    pub health_check_interval: ::std::option::Option<super::types::HealthCheckInterval>,
}
/// Request to unload a loaded service.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcUnload {
    #[prost(message, optional, tag="1")]
    pub ident: ::std::option::Option<super::types::PackageIdent>,
    /// Name of the signal to send the service to shut it down (e.g.,
    /// "TERM" and not "SIGTERM"). Only applies to Unix platforms.
    #[prost(string, optional, tag="2")]
    pub signal: ::std::option::Option<std::string::String>,
    /// Timeout in before killing the service
    #[prost(uint32, optional, tag="3")]
    pub timeout_in_seconds: ::std::option::Option<u32>,
}
/// Request to start a loaded and stopped service.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcStart {
    #[prost(message, optional, tag="1")]
    pub ident: ::std::option::Option<super::types::PackageIdent>,
}
/// Request to stop a loaded and started service.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcStop {
    #[prost(message, optional, tag="1")]
    pub ident: ::std::option::Option<super::types::PackageIdent>,
    /// Name of the signal to send the service to shut it down (e.g.,
    /// "TERM" and not "SIGTERM"). Only applies to Unix platforms.
    #[prost(string, optional, tag="2")]
    pub signal: ::std::option::Option<std::string::String>,
    /// Timeout in before killing the service
    #[prost(uint32, optional, tag="3")]
    pub timeout_in_seconds: ::std::option::Option<u32>,
}
/// Request to retrieve the service status of one or all services.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SvcStatus {
    /// If specified, the reply will contain only the service status for the requested service. If
    /// left blank then all services will report their status.
    #[prost(message, optional, tag="1")]
    pub ident: ::std::option::Option<super::types::PackageIdent>,
}
/// A reply to various requests which contains a pre-formatted console line.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConsoleLine {
    #[prost(string, required, tag="1")]
    pub line: std::string::String,
    #[prost(string, optional, tag="2")]
    pub color: ::std::option::Option<std::string::String>,
    #[prost(bool, required, tag="3")]
    pub bold: bool,
}
