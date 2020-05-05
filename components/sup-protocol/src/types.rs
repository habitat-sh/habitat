//! Generic types shared between the Supervisor and other components in the Habitat software
//! ecosystem.
//!
//! Note: See `protocols/types.proto` for type level documentation for generated types.
//! JW TODO: These types should be moved to the _core crate_ and where they will replace their
//!          vanilla Rust type counterparts that we define there.

use crate::{core::{self,
                   package::{self,
                             Identifiable}},
            message,
            net::{self,
                  ErrCode,
                  NetErr}};
use std::{fmt,
          str::FromStr};

include!(concat!(env!("OUT_DIR"), "/sup.types.rs"));

impl message::MessageStatic for PackageIdent {
    const MESSAGE_ID: &'static str = "PackageIdent";
}
impl message::MessageStatic for ProcessStatus {
    const MESSAGE_ID: &'static str = "ProcessStatus";
}
impl message::MessageStatic for ServiceBind {
    const MESSAGE_ID: &'static str = "ServiceBind";
}
impl message::MessageStatic for ServiceCfg {
    const MESSAGE_ID: &'static str = "ServiceCfg";
}
impl message::MessageStatic for ServiceGroup {
    const MESSAGE_ID: &'static str = "ServiceGroup";
}
impl message::MessageStatic for ServiceStatus {
    const MESSAGE_ID: &'static str = "ServiceStatus";
}
impl message::MessageStatic for HealthCheckInterval {
    const MESSAGE_ID: &'static str = "HealthCheckInterval";
}

impl ServiceGroup {
    pub fn validate(value: &str) -> core::Result<()> {
        core::service::ServiceGroup::validate(value)?;
        Ok(())
    }
}

impl fmt::Display for BindingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match *self {
            BindingMode::Relaxed => "relaxed",
            BindingMode::Strict => "strict",
        };
        write!(f, "{}", value)
    }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.version.as_ref(), self.release.as_ref()) {
            (Some(ref version), Some(ref release)) => {
                write!(f, "{}/{}/{}/{}", self.origin, self.name, version, release,)
            }
            (Some(ref version), None) => write!(f, "{}/{}/{}", self.origin, self.name, version,),
            (None, Some(_)) | (None, None) => write!(f, "{}/{}", self.origin, self.name),
        }
    }
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match *self {
            ProcessState::Down => "down",
            ProcessState::Up => "up",
        };
        write!(f, "{}", state)
    }
}

impl fmt::Display for DesiredState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match *self {
            DesiredState::DesiredDown => "down",
            DesiredState::DesiredUp => "up",
            DesiredState::DesiredNone => "<none>",
        };
        write!(f, "{}", state)
    }
}

impl FromStr for BindingMode {
    type Err = NetErr;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "relaxed" => Ok(BindingMode::Relaxed),
            "strict" => Ok(BindingMode::Strict),
            _ => {
                Err(net::err(ErrCode::InvalidPayload,
                             format!("Invalid binding mode \"{}\", must be \
                                      `relaxed` or `strict`.",
                                     value)))
            }
        }
    }
}

impl FromStr for ProcessState {
    type Err = NetErr;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "0" => Ok(ProcessState::Down),
            "1" => Ok(ProcessState::Up),
            _ => {
                Err(net::err(ErrCode::InvalidPayload,
                             format!("Invalid process state \"{:?}\", must \
                                      be `up` or `down`.",
                                     value)))
            }
        }
    }
}

impl FromStr for DesiredState {
    type Err = NetErr;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "0" => Ok(DesiredState::DesiredDown),
            "1" => Ok(DesiredState::DesiredUp),
            // The DesiredNone variant allows for backwards compatibility with < 0.61 Supervisors,
            // prior to when DesiredState was introduced.
            "<none>" => Ok(DesiredState::DesiredNone),
            _ => {
                Err(net::err(ErrCode::InvalidPayload,
                             format!("Invalid desired state \"{:?}\", must \
                                      be `up`, `down` or `<none>`.",
                                     value)))
            }
        }
    }
}

impl FromStr for ServiceBind {
    type Err = NetErr;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let sb = core::service::ServiceBind::from_str(value).map_err(|e| {
                                                                net::err(ErrCode::InvalidPayload, e)
                                                            })?;
        Ok(sb.into())
    }
}

impl From<core::service::ServiceBind> for ServiceBind {
    fn from(bind: core::service::ServiceBind) -> Self {
        Self { name:          bind.name().to_string(),
               service_group: ServiceGroup::from(bind.service_group().clone()), }
    }
}

impl FromStr for ServiceGroup {
    type Err = NetErr;

    fn from_str(value: &str) -> Result<ServiceGroup, Self::Err> {
        let sg = core::service::ServiceGroup::from_str(value).map_err(|e| {
                                                                 net::err(ErrCode::InvalidPayload,
                                                                          e)
                                                             })?;
        Ok(sg.into())
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value = format!("{}.{}", self.service, self.group);
        if let Some(ref organization) = self.organization {
            value.push_str(&format!("@{}", organization));
        }
        write!(f, "{}", value)
    }
}

// JW TODO: These trait implementations to provide translations between protocol messages and
// concrete Rust types defined in the core crate will go away eventually. We need to put the
// core crate back into the Supervisor's repository and untangle our dependency hell before
// that can happen.

impl From<core::service::HealthCheckInterval> for HealthCheckInterval {
    fn from(h: core::service::HealthCheckInterval) -> Self { Self { seconds: h.into() } }
}

impl From<package::PackageIdent> for PackageIdent {
    fn from(ident: package::PackageIdent) -> Self {
        Self { origin:  ident.origin,
               name:    ident.name,
               version: ident.version,
               release: ident.release, }
    }
}

impl Into<package::PackageIdent> for PackageIdent {
    fn into(self) -> package::PackageIdent {
        package::PackageIdent::new(self.origin, self.name, self.version, self.release)
    }
}

impl fmt::Display for service_cfg::Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match *self {
            service_cfg::Format::Toml => "TOML",
        };
        write!(f, "{}", state)
    }
}

impl From<core::service::BindingMode> for BindingMode {
    fn from(mode: core::service::BindingMode) -> Self {
        match mode {
            core::service::BindingMode::Strict => BindingMode::Strict,
            core::service::BindingMode::Relaxed => BindingMode::Relaxed,
        }
    }
}

impl Into<core::service::BindingMode> for BindingMode {
    fn into(self) -> core::service::BindingMode {
        match self {
            BindingMode::Strict => core::service::BindingMode::Strict,
            BindingMode::Relaxed => core::service::BindingMode::Relaxed,
        }
    }
}

impl From<core::service::ServiceGroup> for ServiceGroup {
    fn from(service_group: core::service::ServiceGroup) -> Self {
        let mut proto = ServiceGroup::default();
        proto.group = service_group.group().to_string();
        proto.service = service_group.service().to_string();
        if let Some(organization) = service_group.org() {
            proto.organization = Some(organization.to_string());
        }
        proto
    }
}

impl Into<core::service::ServiceGroup> for ServiceGroup {
    fn into(self) -> core::service::ServiceGroup {
        core::service::ServiceGroup::new(self.service,
                                         self.group,
                                         self.organization.as_deref()).unwrap()
    }
}

impl Identifiable for PackageIdent {
    fn origin(&self) -> &str { &self.origin }

    fn name(&self) -> &str { &self.name }

    fn version(&self) -> Option<&str> { self.version.as_deref() }

    fn release(&self) -> Option<&str> { self.release.as_deref() }
}

impl Topology {
    fn as_str(&self) -> &str {
        match *self {
            Topology::Leader => "leader",
            Topology::Standalone => "standalone",
        }
    }
}

impl FromStr for Topology {
    type Err = NetErr;

    fn from_str(topology: &str) -> Result<Self, Self::Err> {
        match topology {
            "leader" => Ok(Topology::Leader),
            "standalone" => Ok(Topology::Standalone),
            _ => Err(net::err(ErrCode::InvalidPayload, "Invalid topology.")),
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

impl UpdateStrategy {
    fn as_str(&self) -> &str {
        match *self {
            UpdateStrategy::None => "none",
            UpdateStrategy::AtOnce => "at-once",
            UpdateStrategy::Rolling => "rolling",
        }
    }
}

impl FromStr for UpdateStrategy {
    type Err = NetErr;

    fn from_str(strategy: &str) -> Result<Self, Self::Err> {
        match strategy {
            "none" => Ok(UpdateStrategy::None),
            "at-once" => Ok(UpdateStrategy::AtOnce),
            "rolling" => Ok(UpdateStrategy::Rolling),
            _ => Err(net::err(ErrCode::InvalidPayload, "Invalid update strategy.")),
        }
    }
}

impl fmt::Display for UpdateStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

impl UpdateCondition {
    pub const VARIANTS: &'static [&'static str] = &["latest", "track-channel"];

    pub fn as_str(&self) -> &str {
        match *self {
            UpdateCondition::Latest => "latest",
            UpdateCondition::TrackChannel => "track-channel",
        }
    }
}

impl FromStr for UpdateCondition {
    type Err = NetErr;

    fn from_str(strategy: &str) -> Result<Self, Self::Err> {
        match strategy {
            "latest" => Ok(UpdateCondition::Latest),
            "track-channel" => Ok(UpdateCondition::TrackChannel),
            _ => Err(net::err(ErrCode::InvalidPayload, "Invalid update condition.")),
        }
    }
}

impl fmt::Display for UpdateCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.as_str()) }
}

#[cfg(test)]
mod test {
    use toml;

    use std::str::FromStr;

    use super::*;

    #[test]
    fn topology_default() {
        // This should always be the default topology, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(Topology::default(), Topology::Standalone);
    }

    #[test]
    fn topology_from_str() {
        assert_eq!(Topology::from_str("leader").unwrap(), Topology::Leader);
        assert_eq!(Topology::from_str("standalone").unwrap(),
                   Topology::Standalone);
    }

    #[test]
    fn topology_from_str_invalid() {
        let topology_str = "dope";

        assert!(Topology::from_str(topology_str).is_err());
    }

    #[test]
    fn topology_to_string() {
        assert_eq!("standalone", Topology::Standalone.to_string());
        assert_eq!("leader", Topology::Leader.to_string());
    }

    #[test]
    fn topology_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: Topology,
        }
        let toml = r#"
            key = "leader"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, Topology::Leader);
    }

    #[test]
    fn topology_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: Topology,
        }
        let data = Data { key: Topology::Leader, };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "leader""#))
    }

    #[test]
    fn update_strategy_default() {
        // This should always be the default update strategy, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(UpdateStrategy::default(), UpdateStrategy::None);
    }

    #[test]
    fn update_strategy_from_str() {
        let strategy_str = "at-once";
        let strategy = UpdateStrategy::from_str(strategy_str).unwrap();

        assert_eq!(strategy, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_from_str_invalid() {
        let strategy_str = "dope";

        assert!(UpdateStrategy::from_str(strategy_str).is_err());
    }

    #[test]
    fn update_strategy_to_string() {
        let strategy = UpdateStrategy::AtOnce;

        assert_eq!("at-once", strategy.to_string())
    }

    #[test]
    fn update_strategy_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let toml = r#"
            key = "at-once"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let data = Data { key: UpdateStrategy::AtOnce, };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "at-once""#));
    }
}
