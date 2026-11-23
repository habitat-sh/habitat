use habitat_core::{self as hab_core,
                   util,
                   util::text_render::{TabularText,
                                       tabify,
                                       tabw}};
use habitat_http_client as hab_http;

pub mod builder;
pub mod error;
pub mod response;

use std::str::FromStr;

use regex::Regex;

use std::{fmt,
          io::Write,
          path::Path};

use jiff::civil::DateTime;
use reqwest::IntoUrl;
use serde::{Deserialize,
            Serialize};
use std::sync::LazyLock;

use crate::hab_core::package::PackageIdent;
pub use crate::{builder::{API_RETRY_COUNT,
                          API_RETRY_DELAY,
                          BuilderAPIClient},
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

        if let Ok(j) = self.job_id.parse::<i64>()
           && j > 0
        {
            let job_ids = format!(" (Job ID {})", self.job_id);
            s = s + &job_ids;
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

        if let Some(c) = scheduler_date(&self.created_at) {
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
    #[serde(deserialize_with = "deserialize_invitation_date")]
    pub created_at:   BuilderDateTime,
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

/// Builder supplies a civil date and time. Invitations use UTC, while job status
/// retains the supplied offset. Keep leap-second labels for display because Jiff
/// represents second 60 as second 59.
#[derive(Clone)]
pub struct BuilderDateTime {
    datetime:    DateTime,
    leap_second: bool,
}

impl BuilderDateTime {
    fn parse(value: &str) -> std::result::Result<Self, jiff::Error> {
        let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
        // Builder can supply more than nine digits. Its output has nanosecond precision.
        let value = if fraction.len() > 9 && fraction.bytes().all(|b| b.is_ascii_digit()) {
            &value[..whole.len() + 1 + 9]
        } else {
            value
        };
        let datetime = DateTime::strptime("%Y-%m-%dT%H:%M:%S%.f", value)?;
        Ok(Self { datetime,
                  leap_second: whole.ends_with(":60") })
    }
}

impl fmt::Display for BuilderDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = self.datetime;
        let second = if self.leap_second { 60 } else { dt.second() };
        write!(f, "{}:{second:02}", dt.strftime("%Y-%m-%d %H:%M"))?;
        let nanos = dt.subsec_nanosecond();
        let precision = util::timestamp_precision(nanos);
        if precision > 0 {
            write!(f, ".{}", &format!("{nanos:09}")[..precision])?;
        }
        Ok(())
    }
}

fn deserialize_invitation_date<'de, D: serde::Deserializer<'de>>(
    deserializer: D)
    -> std::result::Result<BuilderDateTime, D::Error> {
    BuilderDateTime::parse(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
}

fn scheduler_date(value: &str) -> Option<String> {
    static RFC3339: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(r"^([0-9]{4}-[0-9]{2}-[0-9]{2})[Tt ]",
                           r"([0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?)",
                           r"([Zz]|[+−-](?:[01][0-9]|2[0-3]):[0-5][0-9])$",)).unwrap()
    });
    let parts = RFC3339.captures(value)?;
    let datetime = BuilderDateTime::parse(&format!("{}T{}", &parts[1], &parts[2])).ok()?;
    let offset = parts[3].replace('−', "-");
    let offset = match offset.as_str() {
        "Z" | "z" | "-00:00" => "+00:00",
        offset => offset,
    };
    Some(format!("{datetime} {offset}"))
}

impl TabularText for UserOriginInvitationsResponse {
    fn as_tabbed(&self) -> std::result::Result<String, habitat_core::error::Error> {
        let tw = tabw().padding(2).minwidth(5);
        if !self.0.is_empty() {
            let mut body = vec![String::from("Invitation Id\tOrigin Name\tAccount \
                                              Name\tCreation Date\tIgnored")];
            for invitation in self.0.iter() {
                body.push(format!("{}\t{}\t{}\t{} UTC\t{}",
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
                body.push(format!("{}\t{}\t{} UTC\t{}",
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
    fn timestamp_scheduler_preserves_offset_and_omits_invalid_dates() {
        for (input, expected) in
            [("2024-03-01T00:01:02.12+05:30", Some("2024-03-01 00:01:02.120 +05:30")),
             ("2024-03-01t00:01:02z", Some("2024-03-01 00:01:02 +00:00")),
             ("2016-12-31 23:59:60.1234567891Z", Some("2016-12-31 23:59:60.123456789 +00:00")),
             ("2024-02-29T20:00:00−04:30", Some("2024-02-29 20:00:00 -04:30")),
             ("2024-02-30T00:00:00Z", None),
             ("2024-03-01T00:00:00", None),
             ("2024-03-01T00:00:00+0530", None)]
        {
            let job = SchedulerResponse { created_at: input.into(),
                                          ..Default::default() };
            let mut output = String::from("Status for Job Group  (): ");
            if let Some(date) = expected {
                output.push_str(&format!("\nCreated at: {date}"));
            }
            assert_eq!(job.to_string(), output);
        }
    }

    #[test]
    fn timestamp_invitations_parse_offset_free_dates_and_render_utc_in_both_tables() {
        for (fraction, expected) in [("", ""),
                                     (".1", ".100"),
                                     (".1234", ".123400"),
                                     (".123456780", ".123456780")]
        {
            let mut record = json!({"id":"1", "account_id":"2", "account_name":"sam",
                                    "created_at":format!("2024-02-29T01:02:03{fraction}"),
                                    "ignored":false, "origin":"core", "owner_id":"3", "updated_at":""});
            let invitation: OriginInvitation = serde_json::from_value(record.clone()).unwrap();
            let user = UserOriginInvitationsResponse(vec![invitation.clone()]);
            let pending = PendingOriginInvitationsResponse { origin:      "core".into(),
                                                             invitations: vec![invitation], };
            for table in [user.as_tabbed().unwrap(), pending.as_tabbed().unwrap()] {
                assert!(table.contains(&format!("2024-02-29 01:02:03{expected} UTC")));
            }
            record["created_at"] = json!("2024-02-29T01:02:03Z");
            assert!(serde_json::from_value::<OriginInvitation>(record).is_err());
        }
    }

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
