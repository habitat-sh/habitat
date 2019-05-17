use crate::error::{Error,
                   Result};
use regex::Regex;
use serde_derive::{Deserialize,
                   Serialize};
use std::{fmt,
          num::ParseIntError,
          ops::{Deref,
                DerefMut},
          result,
          str::FromStr,
          time::Duration};

lazy_static::lazy_static! {
    static ref SG_FROM_STR_RE: Regex =
        Regex::new(r"\A((?P<application_environment>[^#@]+)#)?(?P<service>[^#@]+)\.(?P<group>[^#@.]+)(@(?P<organization>[^#@.]+))?\z").unwrap();

    static ref AE_FROM_STR_RE: Regex =
        Regex::new(r"\A(?P<application>[^#.@]+)\.(?P<environment>[^#.@]+)\z").unwrap();
}

/// Determines how the presence of bound service groups affects the
/// starting of a service.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum BindingMode {
    /// Binds may be satisfied at runtime, and are not required to be
    /// satisfied before a service starts. Modern distributed services
    /// should be constructed in this way.
    Relaxed,
    /// Binds *must* be satisfied before a service can start. Legacy
    /// applications that cannot cope with the absence of a service
    /// dependency at startup should bind with this mode.
    Strict,
}

impl Default for BindingMode {
    /// Strict is the default _for now_, since that's the de facto
    /// behavior that has been in place for until this point.
    ///
    /// Once this feature has been available for a while (and before
    /// Habitat hits 1.0), Relaxed will become the default, because a
    /// well-behaved service in a distributed system should be able to
    /// gracefully degrade when one of its service dependencies is not
    /// available, including at start-up.
    fn default() -> BindingMode { BindingMode::Strict }
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

impl FromStr for BindingMode {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "relaxed" => Ok(BindingMode::Relaxed),
            "strict" => Ok(BindingMode::Strict),
            _ => Err(Error::BadBindingMode(value.to_string())),
        }
    }
}

/// A binding from a service name to a service group that provides that service
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ServiceBind {
    name:          String,
    service_group: ServiceGroup,
}

impl ServiceBind {
    pub fn new(name: &str, service_group: ServiceGroup) -> Self {
        Self { name: name.to_string(),
               service_group }
    }

    pub fn name(&self) -> &str { &self.name }

    pub fn service_group(&self) -> &ServiceGroup { &self.service_group }
}

impl FromStr for ServiceBind {
    type Err = Error;

    fn from_str(bind_str: &str) -> result::Result<Self, Self::Err> {
        let parts: Vec<_> = bind_str.split(':').collect();
        match parts.as_slice() {
            [name, sg_str] => ServiceGroup::from_str(sg_str).map(|sg| ServiceBind::new(name, sg)),
            _ => Err(Error::InvalidBinding(bind_str.to_string())),
        }
    }
}

impl fmt::Display for ServiceBind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.service_group)
    }
}

impl<'de> serde::Deserialize<'de> for ServiceBind {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct ServiceBindVisitor;

        impl<'de> serde::de::Visitor<'de> for ServiceBindVisitor {
            type Value = ServiceBind;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter,
                       "a service bind in name:service_group format (example cache:redis.cache)")
            }

            fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
                where E: serde::de::Error
            {
                ServiceBind::from_str(s).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Str(s), &self)
                })
            }
        }

        deserializer.deserialize_str(ServiceBindVisitor)
    }
}

impl serde::Serialize for ServiceBind {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ServiceGroup(String);

impl ServiceGroup {
    pub fn new<S1, S2>(app_env: Option<&ApplicationEnvironment>,
                       service: S1,
                       group: S2,
                       organization: Option<&str>)
                       -> Result<Self>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        if group.as_ref().contains('.') {
            return Err(Error::InvalidGroupName(group.as_ref().to_string()));
        }
        let formatted = Self::format(app_env, service, group, organization);
        Self::validate(&formatted)?;
        Ok(ServiceGroup(formatted))
    }

    fn format<S1, S2>(app_env: Option<&ApplicationEnvironment>,
                      service: S1,
                      group: S2,
                      organization: Option<&str>)
                      -> String
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        match (app_env, organization) {
            (Some(app_env), Some(org)) => {
                format!("{}#{}.{}@{}",
                        app_env,
                        service.as_ref(),
                        group.as_ref(),
                        org)
            }
            (Some(app_env), None) => format!("{}#{}.{}", app_env, service.as_ref(), group.as_ref()),
            (None, Some(org)) => format!("{}.{}@{}", service.as_ref(), group.as_ref(), org),
            (None, None) => format!("{}.{}", service.as_ref(), group.as_ref()),
        }
    }

    pub fn validate(value: &str) -> Result<()> {
        let caps = SG_FROM_STR_RE.captures(value)
                                 .ok_or_else(|| Error::InvalidServiceGroup(value.to_string()))?;
        if caps.name("service").is_none() {
            return Err(Error::InvalidServiceGroup(value.to_string()));
        }
        if caps.name("group").is_none() {
            return Err(Error::InvalidServiceGroup(value.to_string()));
        }
        Ok(())
    }

    pub fn application_environment(&self) -> Option<ApplicationEnvironment> {
        SG_FROM_STR_RE.captures(&self.0)
                      .unwrap()
                      .name("application_environment")
                      .and_then(|v| {
                          Some(
                    ApplicationEnvironment::from_str(v.as_str())
                        .expect("ApplicationEnvironment is valid and parses."),
                )
                      })
    }

    pub fn service(&self) -> &str {
        SG_FROM_STR_RE.captures(&self.0)
                      .unwrap()
                      .name("service")
                      .unwrap()
                      .as_str()
    }

    pub fn group(&self) -> &str {
        SG_FROM_STR_RE.captures(&self.0)
                      .unwrap()
                      .name("group")
                      .unwrap()
                      .as_str()
    }

    pub fn org(&self) -> Option<&str> {
        SG_FROM_STR_RE.captures(&self.0)
                      .unwrap()
                      .name("organization")
                      .and_then(|v| Some(v.as_str()))
    }

    /// Set a new organization for this Service Group.
    ///
    /// This is useful if the organization was lazily loaded or added after creation.
    pub fn set_org<T: AsRef<str>>(&mut self, org: T) {
        self.0 = Self::format(self.application_environment().as_ref(),
                              self.service(),
                              self.group(),
                              Some(org.as_ref()));
    }
}

impl AsRef<str> for ServiceGroup {
    fn as_ref(&self) -> &str { &self.0 }
}

impl Deref for ServiceGroup {
    type Target = String;

    fn deref(&self) -> &String { &self.0 }
}

impl DerefMut for ServiceGroup {
    fn deref_mut(&mut self) -> &mut String { &mut self.0 }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl FromStr for ServiceGroup {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let caps = match SG_FROM_STR_RE.captures(value) {
            Some(c) => c,
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let service = match caps.name("service") {
            Some(s) => s.as_str(),
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let group = match caps.name("group") {
            Some(g) => g.as_str(),
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let app_env = match caps.name("application_environment") {
            Some(a) => Some(ApplicationEnvironment::from_str(a.as_str())?),
            None => None,
        };
        let org = match caps.name("organization") {
            Some(o) => Some(o.as_str()),
            None => None,
        };
        Ok(ServiceGroup(ServiceGroup::format(app_env.as_ref(),
                                             service,
                                             group,
                                             org)))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ApplicationEnvironment(String);

impl ApplicationEnvironment {
    pub fn new<S1, S2>(app: S1, env: S2) -> Result<Self>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        let formatted = Self::format(app, env);
        Self::validate(&formatted)?;
        Ok(ApplicationEnvironment(formatted))
    }

    fn format<S1, S2>(app: S1, env: S2) -> String
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        format!("{}.{}", app.as_ref(), env.as_ref())
    }

    pub fn validate(value: &str) -> Result<()> {
        let caps =
            AE_FROM_STR_RE.captures(value)
                          .ok_or_else(|| Error::InvalidApplicationEnvironment(value.to_string()))?;
        if caps.name("application").is_none() {
            return Err(Error::InvalidApplicationEnvironment(value.to_string()));
        }
        if caps.name("environment").is_none() {
            return Err(Error::InvalidApplicationEnvironment(value.to_string()));
        }
        Ok(())
    }

    pub fn application(&self) -> &str {
        AE_FROM_STR_RE.captures(&self.0)
                      .unwrap()
                      .name("application")
                      .unwrap()
                      .as_str()
    }

    pub fn environment(&self) -> &str {
        AE_FROM_STR_RE.captures(&self.0)
                      .unwrap()
                      .name("environment")
                      .unwrap()
                      .as_str()
    }
}

impl AsRef<str> for ApplicationEnvironment {
    fn as_ref(&self) -> &str { &self.0 }
}

impl Deref for ApplicationEnvironment {
    type Target = String;

    fn deref(&self) -> &String { &self.0 }
}

impl DerefMut for ApplicationEnvironment {
    fn deref_mut(&mut self) -> &mut String { &mut self.0 }
}

impl fmt::Display for ApplicationEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl FromStr for ApplicationEnvironment {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let caps = match AE_FROM_STR_RE.captures(value) {
            Some(c) => c,
            None => return Err(Error::InvalidApplicationEnvironment(value.to_string())),
        };
        let app = match caps.name("application") {
            Some(s) => s.as_str(),
            None => return Err(Error::InvalidApplicationEnvironment(value.to_string())),
        };
        let env = match caps.name("environment") {
            Some(g) => g.as_str(),
            None => return Err(Error::InvalidApplicationEnvironment(value.to_string())),
        };
        Ok(ApplicationEnvironment(ApplicationEnvironment::format(app, env)))
    }
}

/// Represents how far apart to run health checks for individual services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HealthCheckInterval(Duration);

impl HealthCheckInterval {
    pub fn immediately() -> Self { Self::from(0) }
}

impl From<u64> for HealthCheckInterval {
    fn from(seconds: u64) -> Self { Self(Duration::from_secs(seconds)) }
}

impl Into<u64> for HealthCheckInterval {
    fn into(self) -> u64 { self.0.as_secs() }
}

impl fmt::Display for HealthCheckInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}s)", self.0.as_secs())
    }
}

impl Default for HealthCheckInterval {
    fn default() -> Self { Self::from(30) }
}

impl FromStr for HealthCheckInterval {
    type Err = ParseIntError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> { Ok(Self::from(s.parse::<u64>()?)) }
}

impl From<HealthCheckInterval> for Duration {
    fn from(h: HealthCheckInterval) -> Self { h.0 }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn service_group_from_str_with_org() {
        let x = ServiceGroup::from_str("foo.bar").unwrap();
        assert!(x.application_environment().is_none());
        assert_eq!(x.service(), "foo");
        assert_eq!(x.group(), "bar");
        assert!(x.org().is_none());

        let y = ServiceGroup::from_str("foo.bar@baz").unwrap();
        assert!(x.application_environment().is_none());
        assert_eq!(y.service(), "foo");
        assert_eq!(y.group(), "bar");
        assert_eq!(y.org(), Some("baz"));

        assert!(ServiceGroup::from_str("foo@baz").is_err());
    }

    #[test]
    fn service_group_from_str_with_app() {
        let x = ServiceGroup::from_str("oz.prod#foo.bar").unwrap();
        assert_eq!(x.application_environment(),
                   Some(ApplicationEnvironment::from_str("oz.prod").unwrap()));
        assert_eq!(x.service(), "foo");
        assert_eq!(x.group(), "bar");
        assert!(x.org().is_none());
    }

    #[test]
    fn service_group_from_str_with_app_and_org() {
        let x = ServiceGroup::from_str("oz.prod#foo.bar@baz").unwrap();
        assert_eq!(x.application_environment(),
                   Some(ApplicationEnvironment::from_str("oz.prod").unwrap()));
        assert_eq!(x.service(), "foo");
        assert_eq!(x.group(), "bar");
        assert_eq!(x.org(), Some("baz"));

        assert!(ServiceGroup::from_str("f#o#o.bar@baz").is_err());
    }

    #[test]
    fn service_group_from_str_no_group() {
        let group = "foo@baz";
        match ServiceGroup::from_str(group) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_group_from_str_ending_with_at() {
        let group = "not.allowed@";
        match ServiceGroup::from_str(group) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_group_from_str_with_period_in_service() {
        let x = ServiceGroup::from_str("oz.prod#svc.one.group@baz").unwrap();
        assert_eq!(x.application_environment(),
                   Some(ApplicationEnvironment::from_str("oz.prod").unwrap()));
        assert_eq!(x.service(), "svc.one");
        assert_eq!(x.group(), "group");
        assert_eq!(x.org(), Some("baz"));
    }

    #[test]
    fn service_group_from_str_too_many_hashes() {
        let group = "only#one#hash@allowed";
        match ServiceGroup::from_str(group) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_group_from_str_start_with_hash_and_ending_with_at() {
        let group = "#cool.wings@";
        match ServiceGroup::from_str(group) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_group_from_str_starting_with_pound() {
        let group = "#hash.tag";
        match ServiceGroup::from_str(group) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_group_name_with_periods() {
        let service = "service";
        let group = "oh.noes";
        match ServiceGroup::new(None, service, group, None) {
            Err(e) => {
                match e {
                    Error::InvalidGroupName(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_group_from_str_not_enough_periods() {
        let group = "oh-noes";
        match ServiceGroup::from_str(group) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!(group, val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str() {
        let bind_str = "name:app.env#service.group@organization";
        let bind = ServiceBind::from_str(bind_str).unwrap();

        assert_eq!(bind.name, String::from("name"));
        assert_eq!(bind.service_group,
                   ServiceGroup::from_str("app.env#service.group@organization").unwrap());
    }

    #[test]
    fn service_bind_from_str_simple() {
        let bind_str = "name:service.group";
        let bind = ServiceBind::from_str(bind_str).unwrap();

        assert_eq!(bind.name, String::from("name"));
        assert_eq!(bind.service_group,
                   ServiceGroup::from_str("service.group").unwrap());
    }

    #[test]
    fn service_bind_from_str_missing_colon() {
        let bind_str = "uhoh";

        match ServiceBind::from_str(bind_str) {
            Err(e) => {
                match e {
                    Error::InvalidBinding(val) => assert_eq!("uhoh", val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str_too_many_colons() {
        let bind_str = "uhoh:this:is:bad";

        match ServiceBind::from_str(bind_str) {
            Err(e) => {
                match e {
                    Error::InvalidBinding(val) => assert_eq!("uhoh:this:is:bad", val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str_invalid_service_group() {
        let bind_str = "uhoh:nosuchservicegroup@nope";

        match ServiceBind::from_str(bind_str) {
            Err(e) => {
                match e {
                    Error::InvalidServiceGroup(val) => assert_eq!("nosuchservicegroup@nope", val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_to_string() {
        let sg = ServiceGroup::from_str("service.group").expect("valid service group");
        let bind = ServiceBind::new("name", sg);
        assert_eq!("name:service.group", bind.to_string());
    }

    #[test]
    fn service_bind_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: ServiceBind,
        }
        let toml = r#"
            key = "redis:cache.redis#service.group@organization"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!("redis", data.key.name());
        let sg = ServiceGroup::from_str("cache.redis#service.group@organization")
            .expect("good service group");
        assert_eq!(sg, *data.key.service_group());
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn service_bind_toml_deserialize_bad_bind() {
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct Data {
            key: ServiceBind,
        }
        let toml = r#"
            key = "name"
            "#;
        let _data: Data = toml::from_str(toml).unwrap();
    }

    #[test]
    fn application_environment_new() {
        let x = ApplicationEnvironment::new("application", "environment").unwrap();
        assert_eq!(x.application(), "application");
        assert_eq!(x.environment(), "environment");
        assert_eq!(x.as_str(), "application.environment");
    }

    #[test]
    fn application_environment_from_str() {
        let x = ApplicationEnvironment::from_str("foo.bar").unwrap();
        assert_eq!(x.application(), "foo");
        assert_eq!(x.environment(), "bar");
    }

    #[test]
    #[should_panic(expected = "oh-noes")]
    fn application_environment_from_str_missing_period() {
        ApplicationEnvironment::from_str("oh-noes").unwrap();
    }

    #[test]
    #[should_panic(expected = "only.one.period.allowed")]
    fn application_environment_from_str_too_many_periods() {
        ApplicationEnvironment::from_str("only.one.period.allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "@not.allowed")]
    fn application_environment_from_str_with_ats_front() {
        ApplicationEnvironment::from_str("@not.allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "not.allowed@")]
    fn application_environment_from_str_with_ats_end() {
        ApplicationEnvironment::from_str("not.allowed@").unwrap();
    }

    #[test]
    #[should_panic(expected = "ats.not@allowed")]
    fn application_environment_from_str_with_ats_middle() {
        ApplicationEnvironment::from_str("ats.not@allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "#not.allowed")]
    fn application_environment_from_str_with_hashes_front() {
        ApplicationEnvironment::from_str("#not.allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "not.allowed#")]
    fn application_environment_from_str_with_hashes_end() {
        ApplicationEnvironment::from_str("not.allowed#").unwrap();
    }

    #[test]
    #[should_panic(expected = "hashes.not#allowed")]
    fn application_environment_from_str_with_hashes_middle() {
        ApplicationEnvironment::from_str("hashes.not#allowed").unwrap();
    }

    #[test]
    fn default_health_check_interval_has_correct_default() {
        assert_eq!(HealthCheckInterval::default(),
                   HealthCheckInterval::from(30));
    }

    #[test]
    fn health_check_interval_must_be_positive() {
        assert!(HealthCheckInterval::from_str("-123").is_err());
        assert!(HealthCheckInterval::from_str("5").is_ok());
    }

    #[test]
    #[should_panic(expected = "InvalidDigit")]
    fn health_check_interval_from_str_invalid() {
        HealthCheckInterval::from_str("oh-noes").unwrap();
    }

    #[test]
    fn health_check_interval_display() {
        assert_eq!("(5s)".to_owned(),
                   format!("{}", HealthCheckInterval::from_str("5").unwrap()));
    }
}
