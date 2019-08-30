use habitat_core as hab_core;
use habitat_http_client as hab_http;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

pub mod artifactory;
pub mod builder;
pub mod error;
pub mod response;

use std::str::FromStr;

extern crate regex;
use regex::Regex;

use std::{fmt,
          io::Write,
          path::{Path,
                 PathBuf}};

use chrono::DateTime;
use reqwest::IntoUrl;

pub use crate::error::{Error,
                       Result};

use crate::{artifactory::ArtifactoryClient,
            builder::BuilderAPIClient,
            hab_core::{crypto::keys::box_key_pair::WrappedSealedBox,
                       env,
                       package::{PackageArchive,
                                 PackageIdent,
                                 PackageTarget},
                       ChannelIdent}};

pub trait DisplayProgress: Write + Send {
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
            output.push(format!("Created at: {}", c.to_string()));
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

/// Custom conversion logic to allow `serde` to successfully
/// round-trip `u64` datatypes through JSON serialization.
///
/// To use it, add `#[serde(with = "json_u64")]` to any `u64`-typed struct
/// fields.
mod json_u64 {
    use serde::{self,
                Deserialize,
                Deserializer,
                Serializer};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{}", num);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OriginPrivateSigningKey {
    #[serde(with = "json_u64")]
    pub id: u64,
    #[serde(with = "json_u64")]
    pub origin_id: u64,
    pub name: String,
    pub revision: String,
    pub body: Vec<u8>,
    #[serde(with = "json_u64")]
    pub owner_id: u64,
}

mod json {
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

#[derive(Clone, Copy, Debug)]
pub enum BuildOnUpload {
    PackageDefault,
    Disable,
}

pub trait BuilderAPIProvider: Sync + Send {
    type Progress;

    fn get_origin_schedule(&self, origin: &str, limit: usize) -> Result<Vec<SchedulerResponse>>;

    fn get_schedule(&self, group_id: i64, include_projects: bool) -> Result<SchedulerResponse>;

    fn schedule_job(&self,
                    ident_and_target: (&PackageIdent, PackageTarget),
                    package_only: bool,
                    token: &str)
                    -> Result<(String)>;

    fn fetch_rdeps(&self, ident_and_target: (&PackageIdent, PackageTarget)) -> Result<Vec<String>>;

    fn job_group_promote_or_demote(&self,
                                   group_id: u64,
                                   idents: &[String],
                                   channel: &ChannelIdent,
                                   token: &str,
                                   promote: bool)
                                   -> Result<()>;

    fn job_group_cancel(&self, group_id: u64, token: &str) -> Result<()>;

    fn fetch_origin_public_encryption_key(&self,
                                          origin: &str,
                                          token: &str,
                                          dst_path: &Path,
                                          progress: Option<Self::Progress>)
                                          -> Result<PathBuf>;

    fn create_origin(&self, origin: &str, token: &str) -> Result<()>;

    fn create_origin_secret(&self,
                            origin: &str,
                            token: &str,
                            key: &str,
                            secret: &WrappedSealedBox)
                            -> Result<()>;

    fn delete_origin_secret(&self, origin: &str, token: &str, key: &str) -> Result<()>;

    fn delete_origin(&self, origin: &str, token: &str) -> Result<()>;

    fn list_origin_secrets(&self, origin: &str, token: &str) -> Result<Vec<String>>;

    fn put_package(&self,
                   pa: &mut PackageArchive,
                   token: &str,
                   force_upload: bool,
                   auto_build: BuildOnUpload,
                   progress: Option<Self::Progress>)
                   -> Result<()>;

    fn x_put_package(&self, pa: &mut PackageArchive, token: &str) -> Result<()>;

    fn fetch_package(&self,
                     ident_and_target: (&PackageIdent, PackageTarget),
                     token: Option<&str>,
                     dst_path: &Path,
                     progress: Option<Self::Progress>)
                     -> Result<PackageArchive>;

    fn check_package(&self,
                     ident_and_target: (&PackageIdent, PackageTarget),
                     token: Option<&str>)
                     -> Result<()>;

    fn show_package(&self,
                    ident_and_target: (&PackageIdent, PackageTarget),
                    channel: &ChannelIdent,
                    token: Option<&str>)
                    -> Result<PackageIdent>;

    fn delete_package(&self,
                      ident_and_target: (&PackageIdent, PackageTarget),
                      token: &str)
                      -> Result<()>;

    fn search_package(&self,
                      search_term: &str,
                      limit: usize,
                      token: Option<&str>)
                      -> Result<(Vec<PackageIdent>, usize)>;

    fn create_channel(&self, origin: &str, channel: &ChannelIdent, token: &str) -> Result<()>;

    fn delete_channel(&self, origin: &str, channel: &ChannelIdent, token: &str) -> Result<()>;

    fn list_channels(&self, origin: &str, include_sandbox_channels: bool) -> Result<Vec<String>>;

    fn promote_channel_packages(&self,
                                origin: &str,
                                token: &str,
                                source_channel: &ChannelIdent,
                                target_channel: &ChannelIdent)
                                -> Result<()>;

    fn demote_channel_packages(&self,
                               origin: &str,
                               token: &str,
                               source_channel: &ChannelIdent,
                               target_channel: &ChannelIdent)
                               -> Result<()>;

    fn promote_package(&self,
                       ident_and_target: (&PackageIdent, PackageTarget),
                       channel: &ChannelIdent,
                       token: &str)
                       -> Result<()>;

    fn demote_package(&self,
                      ident_and_target: (&PackageIdent, PackageTarget),
                      channel: &ChannelIdent,
                      token: &str)
                      -> Result<()>;

    fn put_origin_key(&self,
                      origin: &str,
                      revision: &str,
                      src_path: &Path,
                      token: &str,
                      progress: Option<Self::Progress>)
                      -> Result<()>;

    fn fetch_origin_key(&self,
                        origin: &str,
                        revision: &str,
                        token: Option<&str>,
                        dst_path: &Path,
                        progress: Option<Self::Progress>)
                        -> Result<PathBuf>;

    fn put_origin_secret_key(&self,
                             origin: &str,
                             revision: &str,
                             src_path: &Path,
                             token: &str,
                             progress: Option<Self::Progress>)
                             -> Result<()>;

    fn fetch_secret_origin_key(&self,
                               origin: &str,
                               token: &str,
                               dst_path: &Path,
                               progress: Option<Self::Progress>)
                               -> Result<PathBuf>;

    fn show_origin_keys(&self, origin: &str) -> Result<Vec<OriginKeyIdent>>;

    fn package_channels(&self,
                        ident_and_target: (&PackageIdent, PackageTarget),
                        token: Option<&str>)
                        -> Result<Vec<String>>;
}

pub struct Client;
pub type BoxedClient = Box<dyn BuilderAPIProvider<Progress = Box<dyn DisplayProgress>>>;

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<U>(endpoint: U,
                  product: &str,
                  version: &str,
                  fs_root_path: Option<&Path>)
                  -> Result<BoxedClient>
        where U: IntoUrl
    {
        let endpoint = endpoint.into_url().map_err(Error::ReqwestError)?;

        match &env::var("HAB_BLDR_PROVIDER").unwrap_or_else(|_| "builder".to_string())[..] {
            "artifactory" => ArtifactoryClient::create(endpoint, product, version, fs_root_path),
            _ => BuilderAPIClient::create(endpoint, product, version, fs_root_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
