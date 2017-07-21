// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::fmt;
use std::result;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use regex::Regex;

use error::{Error, Result};

lazy_static! {
    static ref SG_FROM_STR_RE: Regex =
        Regex::new(r"\A((?P<application_environment>[^#@]+)#)?(?P<service>[^#@.]+)\.(?P<group>[^#@.]+)(@(?P<organization>[^#@.]+))?\z").unwrap();

    static ref AE_FROM_STR_RE: Regex =
        Regex::new(r"\A(?P<application>[^#.@]+)\.(?P<environment>[^#.@]+)\z").unwrap();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ServiceGroup(String);

impl ServiceGroup {
    pub fn new<S1, S2>(
        app_env: Option<&ApplicationEnvironment>,
        service: S1,
        group: S2,
        organization: Option<&str>,
    ) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let formatted = Self::format(app_env, service, group, organization);
        Self::validate(&formatted)?;
        Ok(ServiceGroup(formatted))
    }

    fn format<S1, S2>(
        app_env: Option<&ApplicationEnvironment>,
        service: S1,
        group: S2,
        organization: Option<&str>,
    ) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        match (app_env, organization) {
            (Some(app_env), Some(org)) => {
                format!(
                    "{}#{}.{}@{}",
                    app_env,
                    service.as_ref(),
                    group.as_ref(),
                    org
                )
            }
            (Some(app_env), None) => format!("{}#{}.{}", app_env, service.as_ref(), group.as_ref()),
            (None, Some(org)) => format!("{}.{}@{}", service.as_ref(), group.as_ref(), org),
            (None, None) => format!("{}.{}", service.as_ref(), group.as_ref()),
        }
    }

    pub fn validate(value: &str) -> Result<()> {
        let caps = SG_FROM_STR_RE.captures(value).ok_or(
            Error::InvalidServiceGroup(
                value.to_string(),
            ),
        )?;
        if caps.name("service").is_none() {
            return Err(Error::InvalidServiceGroup(value.to_string()));
        }
        if caps.name("group").is_none() {
            return Err(Error::InvalidServiceGroup(value.to_string()));
        }
        Ok(())
    }

    pub fn application_environment(&self) -> Option<ApplicationEnvironment> {
        SG_FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("application_environment")
            .and_then(|v| {
                Some(ApplicationEnvironment::from_str(v.as_str()).expect(
                    "ApplicationEnvironment is valid and parses.",
                ))
            })
    }

    pub fn service(&self) -> &str {
        SG_FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("service")
            .unwrap()
            .as_str()
    }

    pub fn group(&self) -> &str {
        SG_FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("group")
            .unwrap()
            .as_str()
    }

    pub fn org(&self) -> Option<&str> {
        SG_FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("organization")
            .and_then(|v| Some(v.as_str()))
    }

    /// Set a new organization for this Service Group.
    ///
    /// This is useful if the organization was lazily loaded or added after creation.
    pub fn set_org<T: AsRef<str>>(&mut self, org: T) {
        self.0 = Self::format(
            self.application_environment().as_ref(),
            self.service(),
            self.group(),
            Some(org.as_ref()),
        );
    }
}

impl AsRef<str> for ServiceGroup {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ServiceGroup {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for ServiceGroup {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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
        Ok(ServiceGroup(
            ServiceGroup::format(app_env.as_ref(), service, group, org),
        ))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ApplicationEnvironment(String);

impl ApplicationEnvironment {
    pub fn new<S1, S2>(app: S1, env: S2) -> Result<Self>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let formatted = Self::format(app, env);
        Self::validate(&formatted)?;
        Ok(ApplicationEnvironment(formatted))
    }

    fn format<S1, S2>(app: S1, env: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        format!("{}.{}", app.as_ref(), env.as_ref())
    }

    pub fn validate(value: &str) -> Result<()> {
        let caps = AE_FROM_STR_RE.captures(value).ok_or(
            Error::InvalidApplicationEnvironment(value.to_string()),
        )?;
        if caps.name("application").is_none() {
            return Err(Error::InvalidApplicationEnvironment(value.to_string()));
        }
        if caps.name("environment").is_none() {
            return Err(Error::InvalidApplicationEnvironment(value.to_string()));
        }
        Ok(())
    }

    pub fn application(&self) -> &str {
        AE_FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("application")
            .unwrap()
            .as_str()
    }

    pub fn environment(&self) -> &str {
        AE_FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("environment")
            .unwrap()
            .as_str()
    }
}

impl AsRef<str> for ApplicationEnvironment {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ApplicationEnvironment {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for ApplicationEnvironment {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl fmt::Display for ApplicationEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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
        Ok(ApplicationEnvironment(
            ApplicationEnvironment::format(app, env),
        ))
    }
}


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::{ApplicationEnvironment, ServiceGroup};

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
        assert_eq!(
            x.application_environment(),
            Some(ApplicationEnvironment::from_str("oz.prod").unwrap())
        );
        assert_eq!(x.service(), "foo");
        assert_eq!(x.group(), "bar");
        assert!(x.org().is_none());
    }

    #[test]
    fn service_group_from_str_with_app_and_org() {
        let x = ServiceGroup::from_str("oz.prod#foo.bar@baz").unwrap();
        assert_eq!(
            x.application_environment(),
            Some(ApplicationEnvironment::from_str("oz.prod").unwrap())
        );
        assert_eq!(x.service(), "foo");
        assert_eq!(x.group(), "bar");
        assert_eq!(x.org(), Some("baz"));

        assert!(ServiceGroup::from_str("f#o#o.bar@baz").is_err());
    }

    #[test]
    #[should_panic(expected = "foo@baz")]
    fn service_group_from_str_no_group() {
        ServiceGroup::from_str("foo@baz").unwrap();
    }

    #[test]
    #[should_panic(expected = "not.allowed@")]
    fn service_group_from_str_ending_with_at() {
        ServiceGroup::from_str("not.allowed@").unwrap();
    }

    #[test]
    #[should_panic(expected = "only.one.period@allowed")]
    fn service_group_from_str_too_many_periods() {
        ServiceGroup::from_str("only.one.period@allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "only#one#hash@allowed")]
    fn service_group_from_str_too_many_hashes() {
        ServiceGroup::from_str("only#one#hash@allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "#cool.wings@")]
    fn service_group_from_str_start_with_hash_and_ending_with_at() {
        ServiceGroup::from_str("#cool.wings@").unwrap();
    }

    #[test]
    #[should_panic(expected = "#hash.tag")]
    fn service_group_from_str_starting_with_pound() {
        ServiceGroup::from_str("#hash.tag").unwrap();
    }

    #[test]
    #[should_panic(expected = "oh-noes")]
    fn service_group_from_str_not_enough_periods() {
        ServiceGroup::from_str("oh-noes").unwrap();
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
}
