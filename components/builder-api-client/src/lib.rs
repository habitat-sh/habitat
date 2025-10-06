use habitat_core::{self as hab_core,
                   util,
                   util::text_render::{tabify,
                                       tabw,
                                       TabularText}};
use habitat_http_client as hab_http;

pub mod builder;
pub mod error;
pub mod response;

use std::str::FromStr;

use regex::Regex;

use std::{fmt,
          io::Write,
          path::Path};

use chrono::{DateTime,
             Utc};
use reqwest::IntoUrl;
use serde::{Deserialize,
            Serialize};

use crate::hab_core::package::PackageIdent;
pub use crate::{builder::{BuilderAPIClient,
                          API_RETRY_COUNT,
                          API_RETRY_DELAY},
                error::{APIFailure,
                        Error,
                        Result}};

pub trait DisplayProgress: Write + Send + Sync {
    fn size(&mut self, size: u64);
    fn finish(&mut self);
}

#[derive(Clone, Deserialize)]
#[serde(rename = "error")]
pub struct NetError {
    pub code: i32,
    pub msg:  String,
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[err: {:?}, msg: {}]", self.code, self.msg)
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct Project {
    pub name:   String,
    pub ident:  String,
    pub state:  String,
    pub job_id: String,
    pub target: String,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{:50} {}", self.ident, self.state);

        if let Ok(j) = self.job_id.parse::<i64>() {
            if j > 0 {
                let job_ids = format!(" (Job ID {})", self.job_id);
                s = s + &job_ids;
            }
        }

        write!(f, "{}", s)
    }
}

#[derive(Default, Deserialize)]
pub struct SchedulerResponse {
    pub id:           String,
    pub state:        String,
    pub projects:     Vec<Project>,
    pub created_at:   String,
    pub project_name: String,
    pub target:       String,
}

impl fmt::Display for SchedulerResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::new();
        output.push(format!("Status for Job Group {} ({}): {}",
                            self.id, self.project_name, self.state));

        if let Ok(c) = DateTime::parse_from_rfc3339(&self.created_at) {
            output.push(format!("Created at: {}", c));
        }

        if !self.projects.is_empty() {
            output.push("".to_string());
            output.push("Reverse dependencies:".to_string());
            let mut projects = self.projects.clone();
            projects.sort_by(|a, b| a.ident.cmp(&b.ident));

            for project in projects.iter() {
                output.push(project.to_string())
            }
        }

        write!(f, "{}", output.join("\n"))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OriginPrivateSigningKey {
    #[serde(with = "util::serde::string")]
    pub id:        u64,
    #[serde(with = "util::serde::string")]
    pub origin_id: u64,
    pub name:      String,
    pub revision:  String,
    pub body:      Vec<u8>,
    #[serde(with = "util::serde::string")]
    pub owner_id:  u64,
}

mod json {
    use serde::Deserialize;
    #[derive(Clone, Deserialize)]
    pub struct PackageIdent {
        pub origin:  String,
        pub name:    String,
        pub version: String,
        pub release: String,
    }

    impl From<PackageIdent> for super::PackageIdent {
        fn from(ident: PackageIdent) -> Self {
            super::PackageIdent { origin:  ident.origin,
                                  name:    ident.name,
                                  version: Some(ident.version),
                                  release: Some(ident.release), }
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Package {
    pub ident:    PackageIdent,
    pub checksum: String,
    pub manifest: String,
    pub deps:     Vec<PackageIdent>,
    pub tdeps:    Vec<PackageIdent>,
    pub exposes:  Vec<u32>,
    pub config:   String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageResults<T> {
    pub range_start: isize,
    pub range_end:   isize,
    pub total_count: isize,
    pub data:        Vec<T>,
}

#[derive(Clone, Deserialize)]
pub struct OriginChannelIdent {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OriginInfoResponse {
    pub default_package_visibility: String,
    pub name: String,
    #[serde(with = "util::serde::string")]
    pub owner_id: u64,
    pub owner_account: String,
    pub private_key_name: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct OriginInvitation {
    #[serde(with = "util::serde::string")]
    pub id:           u64,
    #[serde(with = "util::serde::string")]
    pub account_id:   u64,
    pub account_name: String,
    #[serde(with = "json_date_format")]
    pub created_at:   DateTime<Utc>,
    pub ignored:      bool,
    pub origin:       String,
    #[serde(with = "util::serde::string")]
    pub owner_id:     u64,
    pub updated_at:   String,
}

#[derive(Clone, Deserialize)]
pub struct UserOriginInvitationsResponse(pub Vec<OriginInvitation>);

#[derive(Clone, Deserialize)]
pub struct PendingOriginInvitationsResponse {
    pub origin:      String,
    pub invitations: Vec<OriginInvitation>,
}

// Custom conversion logic to allow `serde` to successfully
// deserialize `DateTime<Utc>` datatypes.
//
// To use it, add `#[serde(with = "json_date_format")]` to any
// `DateTime<Utc>`-typed struct fields.
mod json_date_format {
    use chrono::{DateTime,
                 NaiveDateTime,
                 Utc};
    use serde::{self,
                Deserialize,
                Deserializer};
    const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, DATE_FORMAT).map(|v| v.and_utc())
                                                      .map_err(serde::de::Error::custom)
    }
}

impl TabularText for UserOriginInvitationsResponse {
    fn as_tabbed(&self) -> std::result::Result<String, habitat_core::error::Error> {
        let tw = tabw().padding(2).minwidth(5);
        if !self.0.is_empty() {
            let mut body = vec![String::from("Invitation Id\tOrigin Name\tAccount \
                                              Name\tCreation Date\tIgnored")];
            for invitation in self.0.iter() {
                body.push(format!("{}\t{}\t{}\t{}\t{}",
                                  invitation.id,
                                  invitation.origin,
                                  invitation.account_name,
                                  invitation.created_at,
                                  invitation.ignored));
            }
            tabify(tw, &body.join("\n"))
        } else {
            Ok(String::from(""))
        }
    }
}

impl TabularText for PendingOriginInvitationsResponse {
    fn as_tabbed(&self) -> std::result::Result<String, habitat_core::error::Error> {
        let tw = tabw().padding(2).minwidth(5);
        if !self.invitations.is_empty() {
            let mut body =
                vec![String::from("Invitation Id\tAccount Name\tCreation Date\tIgnored")];
            for invitation in self.invitations.iter() {
                body.push(format!("{}\t{}\t{}\t{}",
                                  invitation.id,
                                  invitation.account_name,
                                  invitation.created_at,
                                  invitation.ignored));
            }
            tabify(tw, &body.join("\n"))
        } else {
            Ok(String::from(""))
        }
    }
}

impl TabularText for OriginInfoResponse {
    fn as_tabbed(&self) -> std::result::Result<String, habitat_core::error::Error> {
        let tw = tabw().padding(2).minwidth(5);
        let mut body = Vec::new();
        body.push(String::from("Owner Id\tOwner Account\tPrivate Key\tPackage Visibility"));
        let key_display = match &self.private_key_name {
            Some(key) => key.as_str(),
            None => "None",
        };
        body.push(format!("{}\t{}\t{}\t{}",
                          self.owner_id,
                          self.owner_account,
                          key_display,
                          self.default_package_visibility));
        tabify(tw, &body.join("\n"))
    }
}

#[derive(Clone, Deserialize)]
pub struct OriginSecret {
    pub id:        String,
    pub origin_id: String,
    pub name:      String,
    pub value:     String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OriginKeyIdent {
    pub origin:   String,
    pub revision: String,
    pub location: String,
}

// Expected format: "origin-revision.extension"
impl FromStr for OriginKeyIdent {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"([\w]+)([-])([\d]+)").unwrap();

        let caps = match re.captures(s) {
            Some(caps) => caps,
            None => return Err(Error::NotSupported),
        };

        Ok(OriginKeyIdent { origin:   caps.get(1).unwrap().as_str().to_string(),
                            revision: caps.get(3).unwrap().as_str().to_string(),
                            location: "".to_string(), })
    }
}

#[derive(Deserialize)]
pub struct ReverseDependencies {
    pub origin: String,
    pub name:   String,
    pub rdeps:  Vec<String>,
}

pub struct Client;

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<U>(endpoint: U,
                  product: &str,
                  version: &str,
                  fs_root_path: Option<&Path>)
                  -> Result<BuilderAPIClient>
        where U: IntoUrl
    {
        let endpoint = endpoint.into_url().map_err(Error::ReqwestError)?;

        let client = BuilderAPIClient::new(endpoint, product, version, fs_root_path)?;

        Ok(client)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OriginMemberRoleResponse {
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_round_trip_u64_fields() {
        let pre = OriginPrivateSigningKey { id:        705_705_315_793_903_646,
                                            origin_id: 705_705_305_031_319_582,
                                            name:      "core".to_string(),
                                            revision:  "20160810182414".to_string(),
                                            body:      vec![1, 2, 3],
                                            owner_id:  0, };

        let as_json = serde_json::to_value(&pre).unwrap();
        let expected = json!({
            "id": "705705315793903646",
            "origin_id": "705705305031319582",
            "name": "core",
            "revision": "20160810182414",
            "body": [
                1,
                2,
                3
            ],
            "owner_id": "0"
        });
        assert_eq!(as_json, expected);

        let post: OriginPrivateSigningKey = serde_json::from_value(as_json).unwrap();
        assert_eq!(pre.id, post.id);
        assert_eq!(pre.origin_id, post.origin_id);
        assert_eq!(pre.owner_id, post.owner_id);
    }
}
