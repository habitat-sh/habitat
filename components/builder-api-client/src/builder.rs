use crate::{allow_std_io::AllowStdIo,
            error::{Error,
                    Result},
            hab_http::ApiClient,
            response,
            BuildOnUpload,
            DisplayProgress,
            OriginInfoResponse,
            OriginKeyIdent,
            OriginSecret,
            Package,
            PendingOriginInvitationsResponse,
            ReverseDependencies,
            SchedulerResponse,
            UserOriginInvitationsResponse};
use broadcast::BroadcastWriter;
use bytes::BytesMut;
use futures::stream::TryStreamExt;
use habitat_core::{crypto::keys::box_key_pair::WrappedSealedBox,
                   fs::{AtomicWriter,
                        Permissions,
                        DEFAULT_CACHED_ARTIFACT_PERMISSIONS},
                   package::{Identifiable,
                             PackageArchive,
                             PackageIdent,
                             PackageTarget},
                   ChannelIdent};
use percent_encoding::{percent_encode,
                       AsciiSet,
                       CONTROLS};
use reqwest::{header::CONTENT_LENGTH,
              Body,
              IntoUrl,
              RequestBuilder,
              StatusCode};
use std::{fs::{self,
               File},
          future::Future,
          io::{self,
               Cursor},
          path::{Path,
                 PathBuf},
          string::ToString};
use tee::TeeReader;
use tokio::task;
use tokio_util::codec::{BytesCodec,
                        FramedRead};
use url::Url;

const X_FILENAME: &str = "x-filename";

const DEFAULT_API_PATH: &str = "/v1";

// The characters in this set are copied from
// https://docs.rs/percent-encoding/1.0.1/percent_encoding/struct.PATH_SEGMENT_ENCODE_SET.html
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ')
                                                    .add(b'"')
                                                    .add(b'#')
                                                    .add(b'<')
                                                    .add(b'>')
                                                    .add(b'`')
                                                    .add(b'?')
                                                    .add(b'{')
                                                    .add(b'}')
                                                    .add(b'%')
                                                    .add(b'/');

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
    pub id:        u64,
    #[serde(with = "json_u64")]
    pub origin_id: u64,
    pub name:      String,
    pub revision:  String,
    pub body:      Vec<u8>,
    #[serde(with = "json_u64")]
    pub owner_id:  u64,
}

mod json {
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

pub struct BuilderAPIClient(ApiClient);

impl BuilderAPIClient {
    pub fn new<U>(endpoint: U,
                  product: &str,
                  version: &str,
                  fs_root_path: Option<&Path>)
                  -> Result<Self>
        where U: IntoUrl
    {
        let mut endpoint = endpoint.into_url().map_err(Error::ReqwestError)?;
        if !endpoint.cannot_be_a_base() && endpoint.path() == "/" {
            endpoint.set_path(DEFAULT_API_PATH);
        }
        let client = BuilderAPIClient(
            ApiClient::new(endpoint, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        );
        Ok(client)
    }

    pub fn create<U>(endpoint: U,
                     product: &str,
                     version: &str,
                     fs_root_path: Option<&Path>)
                     -> Result<BuilderAPIClient>
        where U: IntoUrl
    {
        Self::new(endpoint, product, version, fs_root_path)
    }

    fn maybe_add_authz(&self, rb: RequestBuilder, token: Option<&str>) -> RequestBuilder {
        match token {
            Some(token) => rb.bearer_auth(token),
            None => rb,
        }
    }

    async fn download<'a>(&'a self,
                          rb: RequestBuilder,
                          dst_path: &'a Path,
                          token: Option<&'a str>,
                          permissions: Permissions,
                          progress: Option<Box<dyn DisplayProgress>>)
                          -> Result<PathBuf> {
        debug!("Downloading file to path: {}", dst_path.display());
        let resp = self.maybe_add_authz(rb, token).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        fs::create_dir_all(&dst_path)?;
        let file_name = response::get_header(&resp, X_FILENAME)?;
        let dst_file_path = dst_path.join(file_name);
        let w = AtomicWriter::new_with_permissions(&dst_file_path, permissions)?;
        let content_length = response::get_header(&resp, CONTENT_LENGTH);
        let mut body = Cursor::new(resp.bytes().await?);
        // Blocking IO is used because of `DisplayProgress` which relies on the `Write` trait.
        task::spawn_blocking(move || {
            w.with_writer(|mut f| {
                 // There will be no CONTENT_LENGTH header if an on prem
                 // builder is using chunked transfer encoding
                 match (progress, content_length) {
                     (Some(mut progress), Ok(content_length)) => {
                         let size = content_length.parse().map_err(Error::ParseIntError)?;
                         progress.size(size);
                         let mut writer = BroadcastWriter::new(&mut f, progress);
                         io::copy(&mut body, &mut writer).map_err(Error::IO)
                     }
                     _ => io::copy(&mut body, &mut f).map_err(Error::IO),
                 }
             })?;
            Ok(dst_file_path)
        }).await?
    }

    async fn upload_body(src_path: &Path,
                         progress: Option<Box<dyn DisplayProgress>>)
                         -> Result<Body> {
        let file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file.metadata()
                            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
                            .len();

        // Blocking IO is used because of `DisplayProgress` which relies on the `Write` trait.
        task::spawn_blocking(move || {
            Ok(if let Some(mut progress) = progress {
                progress.size(file_size);
                let reader = AllowStdIo::new(TeeReader::new(file, progress));
                let reader = FramedRead::new(reader, BytesCodec::new()).map_ok(BytesMut::freeze);
                Body::wrap_stream(reader)
            } else {
                let reader = AllowStdIo::new(file);
                let reader = FramedRead::new(reader, BytesCodec::new()).map_ok(BytesMut::freeze);
                Body::wrap_stream(reader)
            })
        }).await?
    }

    async fn seach_package_with_range(&self,
                                      search_term: &str,
                                      token: Option<&str>,
                                      range: usize)
                                      -> Result<(PackageResults<PackageIdent>, bool)> {
        debug!("Searching for package {} with range {}", search_term, range);
        let req = self.0
                      .get_with_custom_url(&package_search(search_term), |url| {
                          url.set_query(Some(&format!("range={:?}&distinct=true", range)));
                      });
        let resp = self.maybe_add_authz(req, token).send().await?;
        let status = resp.status();
        debug!("Response Status: {:?}", status);

        if status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT {
            let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
            trace!(target: "habitat_http_client::api_client::search_package", "{:?}", encoded);

            Ok((serde_json::from_str(&encoded)?, status == StatusCode::PARTIAL_CONTENT))
        } else {
            Err(response::err_from_response(resp).await)
        }
    }

    async fn search_package_impl<'a, F>(&'a self,
                                        search_term: &'a str,
                                        limit: usize,
                                        token: Option<&'a str>,
                                        search_with_range: impl Fn(&'a BuilderAPIClient,
                                           &'a str,
                                           Option<&'a str>,
                                           usize)
                                           -> F)
                                        -> Result<(Vec<PackageIdent>, usize)>
        where F: Future<Output = Result<(PackageResults<PackageIdent>, bool)>>
    {
        let mut packages = Vec::new();
        loop {
            let (mut package_results, more_to_come) =
                search_with_range(self, search_term, token, packages.len()).await?;
            packages.append(&mut package_results.data);

            if packages.len() >= limit || !more_to_come {
                packages.truncate(limit);
                return Ok((packages, package_results.total_count as usize));
            }
        }
    }

    /// Retrieves the status of every group job in an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn get_origin_schedule(&self,
                                     origin: &str,
                                     limit: usize)
                                     -> Result<Vec<SchedulerResponse>> {
        debug!("Retrieving status for job groups in the {} origin (limit: {})",
               origin, limit);

        let path = format!("depot/pkgs/schedule/{}/status", origin);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("limit", &limit.to_string());
        };

        let resp = self.0.get_with_custom_url(&path, custom).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        Ok(resp.json().await?)
    }

    /// Retrieves the status of a group job
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn get_schedule(&self,
                              group_id: i64,
                              include_projects: bool)
                              -> Result<SchedulerResponse> {
        debug!("Retrieving schedule for job group {} (include_projects: {})",
               group_id, include_projects);

        let path = format!("depot/pkgs/schedule/{}", group_id);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("include_projects", &include_projects.to_string());
        };

        let resp = self.0.get_with_custom_url(&path, custom).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        Ok(resp.json().await?)
    }

    /// Schedules a job for a package ident
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    pub async fn schedule_job(&self,
                              (ident, target): (&PackageIdent, PackageTarget),
                              package_only: bool,
                              token: &str)
                              -> Result<String> {
        debug!("Scheduling job for {}, {}", ident, target);

        let path = format!("depot/pkgs/schedule/{}/{}", ident.origin(), ident.name());

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("package_only", &package_only.to_string())
               .append_pair("target", &target.to_string());
        };

        let resp = self.0
                       .post_with_custom_url(&path, custom)
                       .bearer_auth(token)
                       .send()
                       .await?;
        debug!("Response Status: {:?}", resp.status());

        if resp.status() == StatusCode::CREATED || resp.status() == StatusCode::OK {
            let sr: SchedulerResponse = resp.json().await?;
            Ok(sr.id)
        } else {
            Err(response::err_from_response(resp).await)
        }
    }

    /// Fetch the reverse dependencies for a package
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub async fn fetch_rdeps(&self,
                             (ident, target): (&PackageIdent, PackageTarget),
                             token: &str)
                             -> Result<Vec<String>> {
        debug!("Fetching the reverse dependencies for {}", ident);

        let url = format!("rdeps/{}", ident);

        let resp = self.0
                       .get_with_custom_url(&url, |u| {
                           u.set_query(Some(&format!("target={}", &target.to_string())))
                       })
                       .bearer_auth(token)
                       .send()
                       .await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
        trace!(target: "habitat_http_client::api_client::fetch_rdeps", "{:?}", encoded);

        let rd: ReverseDependencies = serde_json::from_str(&encoded).map_err(Error::Json)?;
        Ok(rd.rdeps.to_vec())
    }

    /// Promote/Demote a job group to/from a channel
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub async fn job_group_promote_or_demote(&self,
                                             group_id: u64,
                                             idents: &[String],
                                             channel: &ChannelIdent,
                                             token: &str,
                                             promote: bool)
                                             -> Result<()> {
        debug!("{} for group: {}, channel: {}",
               group_id,
               channel,
               if promote { "Promote" } else { "Demote" });

        let json_idents = json!(idents);
        let body = json!({ "idents": json_idents });

        let url = format!("jobs/group/{}/{}/{}",
                          group_id,
                          if promote { "promote" } else { "demote" },
                          channel);

        response::ok_if_unit(self.0
                                 .post(&url)
                                 .bearer_auth(token)
                                 .json(&body)
                                 .send()
                                 .await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Cancel a job group
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    pub async fn job_group_cancel(&self, group_id: u64, token: &str) -> Result<()> {
        debug!("Canceling job group: {}", group_id);

        let url = format!("jobs/group/{}/cancel", group_id);

        response::ok_if_unit(self.0.post(&url).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Download a public encryption key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub async fn fetch_origin_public_encryption_key<'a>(&'a self,
                                                        origin: &'a str,
                                                        token: &'a str,
                                                        dst_path: &'a Path,
                                                        progress: Option<Box<dyn DisplayProgress>>)
                                                        -> Result<PathBuf> {
        self.download(self.0
                          .get(&format!("depot/origins/{}/encryption_key", origin)),
                      dst_path.as_ref(),
                      Some(token),
                      Permissions::Standard,
                      progress)
            .await
    }

    /// Create an origin
    ///
    ///  # Failures
    ///
    ///  * Remote builder is not available
    ///  * Unable to authenticate
    pub async fn create_origin(&self, origin: &str, token: &str) -> Result<()> {
        let body = json!({
            "name": origin,
        });

        response::ok_if_unit(self.0
                                 .post("depot/origins")
                                 .bearer_auth(token)
                                 .json(&body)
                                 .send()
                                 .await?,
                             &[StatusCode::CREATED]).await
    }

    /// Create secret for an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn create_origin_secret(&self,
                                      origin: &str,
                                      token: &str,
                                      key_name: &str,
                                      secret: &WrappedSealedBox<'_>)
                                      -> Result<()> {
        debug!("Creating origin secret: {}, {}", origin, key_name);

        let path = format!("depot/origins/{}/secret", origin);
        let body = json!({
            "name": key_name,
            "value": secret
        });

        response::ok_if_unit(self.0
                                 .post(&path)
                                 .bearer_auth(token)
                                 .json(&body)
                                 .send()
                                 .await?,
                             &[StatusCode::CREATED]).await
    }

    /// Delete a secret for an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn delete_origin_secret(&self,
                                      origin: &str,
                                      token: &str,
                                      key_name: &str)
                                      -> Result<()> {
        debug!("Deleting origin secret: {}, {}", origin, key_name);

        let path = format!("depot/origins/{}/secret/{}", origin, key_name);

        // Originally, we only returned an Ok result if the response was StatusCode::NO_CONTENT
        // (HTTP 204). However the Bldr API appears to always have returned HTTP 200. We'll accept
        // either as indicators of a successful operation moving forward.
        response::ok_if_unit(self.0.delete(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT, StatusCode::OK]).await
    }

    /// Check an origin exists
    ///
    ///  # Failures
    ///
    ///  * Origin is not found
    ///  * Remote Builder is not available
    pub async fn check_origin(&self, origin: &str, token: &str) -> Result<()> {
        debug!("Checking for existence of origin: {}", origin);

        let path = format!("depot/origins/{}", origin);

        response::ok_if_unit(self.0.get(&path).bearer_auth(token).send().await?,
                             &[StatusCode::OK]).await
    }

    /// Delete an origin
    ///
    ///  # Failures
    ///
    ///  * Remote builder is not available
    ///  * Origin is populated with > 0 packages
    ///  * Submitted Origin is not found
    pub async fn delete_origin(&self, origin: &str, token: &str) -> Result<()> {
        debug!("Deleting origin: {}", origin);

        let path = format!("depot/origins/{}", origin);

        response::ok_if_unit(self.0.delete(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Transfer ownership of an origin to a new account
    ///
    ///  # Failures
    ///
    ///  * Remote builder is not available
    ///  * Origin is not owned by the account of auth token
    ///  * Account is not a member of the origin
    ///  * Account does not exist
    ///  * Origin does not exist
    pub async fn transfer_origin_ownership(&self,
                                           origin: &str,
                                           token: &str,
                                           account: &str)
                                           -> Result<()> {
        debug!("Transferring ownership of {} origin to {}", origin, account);

        let path = format!("depot/origins/{}/transfer/{}", origin, account);

        response::ok_if_unit(self.0.post(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    ///  Depart membership from an origin
    ///
    ///  # Failures
    ///
    ///  * Remote builder is not available
    ///  * Origin is owned by the account of auth token
    ///  * Account is not a member of the origin
    ///  * Account does not exist
    ///  * Origin does not exist
    pub async fn depart_origin(&self, origin: &str, token: &str) -> Result<()> {
        debug!("Departing membership of origin {}", origin);

        let path = format!("depot/origins/{}/depart", origin);

        response::ok_if_unit(self.0.post(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Accepts an origin member invitation
    ///
    ///  # Builder API endpiont (api.raml permalink)
    ///    * https://github.com/habitat-sh/builder/blob/da72e9fb86e24d9076268b6b1c913b7531c83ed9/components/builder-api/doc/api.raml#L883
    ///
    ///  # Arguments
    ///    * Origin: &str - origin name where membership invitation exists
    ///    * Token: &str - bearer token for authentication/authorization
    ///    * Invitation Id: u64 - id of invitation to accept
    ///
    ///  # Expected API response on success
    ///    * HTTP 204
    ///
    ///  # Return
    ///    * Result<()>
    pub async fn accept_origin_invitation(&self,
                                          origin: &str,
                                          token: &str,
                                          invitation_id: u64)
                                          -> Result<()> {
        debug!("Accepting invitation id {} in origin {}",
               invitation_id, origin);

        let path = format!("depot/origins/{}/invitations/{}", origin, invitation_id);

        response::ok_if_unit(self.0.put(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Marks an origin member invitation ignored
    ///
    ///  After ignoring, the user will no longer see the invitation
    ///
    ///  # Builder API endpiont (api.raml permalink)
    ///    * https://github.com/habitat-sh/builder/blob/da72e9fb86e24d9076268b6b1c913b7531c83ed9/components/builder-api/doc/api.raml#L903
    ///
    ///  # Arguments
    ///    * Origin: &str - origin name where membership invitation exists
    ///    * Token: &str - bearer token for authentication/authorization
    ///    * Invitation Id: u64 - id of invitation to accept
    ///
    ///  # Expected API response on success
    ///    * HTTP 204
    ///
    ///  # Return
    ///    * Result<()>
    pub async fn ignore_origin_invitation(&self,
                                          origin: &str,
                                          token: &str,
                                          invitation_id: u64)
                                          -> Result<()> {
        debug!("Marking invitation {} in origin {} ignored",
               invitation_id, origin);

        let path = format!("depot/origins/{}/invitations/{}/ignore",
                           origin, invitation_id);

        response::ok_if_unit(self.0.put(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Retrieves origin member invitations for current user
    ///
    ///  The usecase is for any user to discover any origin invitations sent to their
    ///  account without having to know the origin.
    ///
    ///  # Builder API endpiont (api.raml permalink)
    ///    * https://github.com/habitat-sh/builder/blob/da72e9fb86e24d9076268b6b1c913b7531c83ed9/components/builder-api/doc/api.raml#L614
    ///
    ///  # Arguments
    ///    * Token: &str - bearer token for authentication/authorization
    ///
    ///  # Expected API response on success
    ///    * HTTP 200
    ///
    ///  # Return
    ///    * Result<UserOriginInvitationsResponse>
    pub async fn list_user_invitations(&self,
                                       token: &str)
                                       -> Result<UserOriginInvitationsResponse> {
        let path = "user/invitations";

        let resp = self.0.get(&path).bearer_auth(token).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        Ok(resp.json().await?)
    }

    /// Retrieves public metadata for an origin
    ///
    ///  # Arguments
    ///    * Token: &str - bearer token for authentication/authorization
    ///
    ///  # Expected API response on success
    ///    * HTTP 200
    ///
    ///  # Return
    ///    * Result<OriginInfoResponse>
    pub async fn origin_info(&self, token: &str, origin: &str) -> Result<OriginInfoResponse> {
        let path = format!("depot/origins/{}", origin);

        let resp = self.0.get(&path).bearer_auth(token).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        Ok(resp.json().await?)
    }

    /// Retrieves pending origin member invitations
    ///
    ///  The usecase here is for an origin owner (or simply origin member) to see the outstanding
    ///  invitations for a given origin.
    ///
    ///  # Builder API endpiont (api.raml permalink)
    ///    * https://github.com/habitat-sh/builder/blob/da72e9fb86e24d9076268b6b1c913b7531c83ed9/components/builder-api/doc/api.raml#L881
    ///
    ///  # Arguments
    ///    * Origin: &str - origin name where membership invitation exists
    ///    * Token: &str - bearer token for authentication/authorization
    ///
    ///  # Expected API response on success
    ///    * HTTP 200
    ///
    ///  # Return
    ///    * Result<PendingOriginInvitationsResponse>
    pub async fn list_pending_origin_invitations(&self,
                                                 origin: &str,
                                                 token: &str)
                                                 -> Result<PendingOriginInvitationsResponse> {
        debug!("Retrieving pending invitations in origin {}", origin);
        let path = format!("depot/origins/{}/invitations", origin);

        let resp = self.0.get(&path).bearer_auth(token).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        Ok(resp.json().await?)
    }

    /// Rescind an origin member invitation
    ///
    ///  Rescind an invitation that hasn't already been ignored. The invitation will be deleted.
    ///
    ///  # Builder API endpiont (api.raml permalink)
    ///    * https://github.com/habitat-sh/builder/blob/da72e9fb86e24d9076268b6b1c913b7531c83ed9/components/builder-api/doc/api.raml#L893
    ///
    ///  # Arguments
    ///    * Origin: &str - origin name where membership invitation exists
    ///    * Token: &str - bearer token for authentication/authorization
    ///    * Invitation Id: u64 - id of invitation to accept
    ///
    ///  # Expected API response on success
    ///    * HTTP 204
    ///
    ///  # Return
    ///    * Result<()>
    pub async fn rescind_origin_invitation(&self,
                                           origin: &str,
                                           token: &str,
                                           invitation_id: u64)
                                           -> Result<()> {
        debug!("Rescinding invitation {} in origin {}",
               invitation_id, origin);

        let path = format!("depot/origins/{}/invitations/{}", origin, invitation_id);

        response::ok_if_unit(self.0.delete(&path).bearer_auth(token).send().await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Send an origin member invitation
    ///
    ///  Invite a user into an origin. The invite will show up under pending origin invitations
    ///  and listed under a user's direct invitations.
    ///
    ///  # Builder API endpiont (api.raml permalink)
    ///    * https://github.com/habitat-sh/builder/blob/da72e9fb86e24d9076268b6b1c913b7531c83ed9/components/builder-api/doc/api.raml#L812
    ///
    ///  # Arguments
    ///    * Origin: &str - origin name where membership invitation exists
    ///    * Token: &str - bearer token for authentication/authorization
    ///    * Invitee Account: &str - account name to invite into the origin
    ///
    ///  # Expected API response on success
    ///    * HTTP 201
    ///
    ///  # Return
    ///    * Result<()>
    pub async fn send_origin_invitation(&self,
                                        origin: &str,
                                        token: &str,
                                        invitee_account: &str)
                                        -> Result<()> {
        debug!("Sending an invitation to {} for origin {}",
               invitee_account, origin);

        let path = format!("depot/origins/{}/users/{}/invitations",
                           origin, invitee_account);

        response::ok_if_unit(self.0.post(&path).bearer_auth(token).send().await?,
                             &[StatusCode::CREATED]).await
    }

    /// List all secrets keys for an origin
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn list_origin_secrets(&self, origin: &str, token: &str) -> Result<Vec<String>> {
        debug!("Listing origin secret: {}", origin);

        let path = format!("depot/origins/{}/secret", origin);
        let resp = self.0.get(&path).bearer_auth(token).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
        trace!(target: "habitat_http_client::api_client::list_origin_secrets", "{:?}", encoded);

        Ok(serde_json::from_str::<Vec<OriginSecret>>(&encoded)?.into_iter()
                                                               .map(|s| s.name)
                                                               .collect())
    }

    /// Download a public key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub async fn fetch_origin_key<'a>(&'a self,
                                      origin: &'a str,
                                      revision: &'a str,
                                      _token: Option<&'a str>,
                                      dst_path: &'a Path,
                                      progress: Option<Box<dyn DisplayProgress>>)
                                      -> Result<PathBuf> {
        self.download(self.0
                          .get(&format!("depot/origins/{}/keys/{}", origin, revision)),
                      dst_path.as_ref(),
                      None,
                      Permissions::Standard,
                      progress)
            .await
    }

    /// Download a secret key from a remote Builder to the given filepath.
    ///
    /// # Failures
    ///
    /// * Key cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub async fn fetch_secret_origin_key<'a>(&'a self,
                                             origin: &'a str,
                                             token: &'a str,
                                             dst_path: &'a Path,
                                             progress: Option<Box<dyn DisplayProgress>>)
                                             -> Result<PathBuf> {
        self.download(self.0
                          .get(&format!("depot/origins/{}/secret_keys/latest", origin)),
                      dst_path.as_ref(),
                      Some(token),
                      Permissions::Standard,
                      progress)
            .await
    }

    pub async fn show_origin_keys(&self, origin: &str) -> Result<Vec<OriginKeyIdent>> {
        debug!("Showing origin keys: {}", origin);

        let resp = self.0.get(&origin_keys_path(origin)).send().await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
        trace!(target: "habitat_http_client::api_client::show_origin_keys", "{:?}", encoded);

        Ok(serde_json::from_str::<Vec<OriginKeyIdent>>(&encoded)?)
    }

    /// Return a list of channels for a given package
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * Package does not exist
    pub async fn package_channels(&self,
                                  (ident, target): (&PackageIdent, PackageTarget),
                                  token: Option<&str>)
                                  -> Result<Vec<String>> {
        debug!("Retrieving channels for {}, target {}", ident, target);

        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let path = package_channels_path(ident);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("target", &target.to_string());
        };

        let resp = self.maybe_add_authz(self.0.get_with_custom_url(&path, custom), token)
                       .send()
                       .await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
        trace!(target: "habitat_http_client::api_client::package_channels", "{:?}", encoded);

        Ok(serde_json::from_str::<Vec<String>>(&encoded)?.into_iter()
                                                         .collect())
    }

    /// Upload a public origin key to a remote Builder.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub async fn put_origin_key<'a>(&'a self,
                                    origin: &'a str,
                                    revision: &'a str,
                                    src_path: &'a Path,
                                    token: &'a str,
                                    progress: Option<Box<dyn DisplayProgress>>)
                                    -> Result<()> {
        debug!("Uploading origin key: {}, {}", origin, revision);

        let path = format!("depot/origins/{}/keys/{}", &origin, &revision);
        let body = Self::upload_body(src_path, progress).await?;
        let resp = self.0
                       .post(&path)
                       .bearer_auth(token)
                       .body(body)
                       .send()
                       .await?;
        response::ok_if_unit(resp, &[StatusCode::OK, StatusCode::CREATED]).await
    }

    /// Upload a secret origin key to a remote Builder.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub async fn put_origin_secret_key<'a>(&'a self,
                                           origin: &'a str,
                                           revision: &'a str,
                                           src_path: &'a Path,
                                           token: &'a str,
                                           progress: Option<Box<dyn DisplayProgress>>)
                                           -> Result<()> {
        debug!("Uploading origin secret key: {}, {}", origin, revision);

        let path = format!("depot/origins/{}/secret_keys/{}", &origin, &revision);
        let body = Self::upload_body(src_path, progress).await?;
        let resp = self.0
                       .post(&path)
                       .bearer_auth(token)
                       .body(body)
                       .send()
                       .await?;
        response::ok_if_unit(resp, &[StatusCode::OK]).await
    }

    /// Download the latest release of a package.
    ///
    /// By the time this function is called, the ident must be fully qualified. The download URL in
    /// the depot requires a fully qualified ident to work. If you want the latest version of
    /// a package, e.g. core/redis, you can display package details for that via a different URL,
    /// e.g. /pkgs/core/redis/latest but that only _shows_ you the details - it doesn't download
    /// the package.
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Builder is not available
    /// * File cannot be created and written to
    pub async fn fetch_package<'a>(&'a self,
                                   (ident, target): (&'a PackageIdent, PackageTarget),
                                   token: Option<&'a str>,
                                   dst_path: &'a Path,
                                   progress: Option<Box<dyn DisplayProgress>>)
                                   -> Result<PackageArchive> {
        // Ensure ident is fully qualified.
        //
        // TODO fn: this will be removed when we can describe a fully qualified ident by type as a
        // param to this function
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let req_builder = self.0.get_with_custom_url(&package_download(ident), |u| {
                                    u.set_query(Some(&format!("target={}", target)))
                                });
        let path = self.download(req_builder,
                                 dst_path.as_ref(),
                                 token,
                                 DEFAULT_CACHED_ARTIFACT_PERMISSIONS,
                                 progress)
                       .await?;
        Ok(PackageArchive::new(path)?)
    }

    /// Checks whether a specified package exists
    ///
    /// The package ident must be fully qualified
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Builder is not available
    pub async fn check_package(&self,
                               (package, target): (&PackageIdent, PackageTarget),
                               token: Option<&str>)
                               -> Result<()> {
        debug!("Checking package existence for {}, target {}",
               package, target);

        if !package.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let url = channel_package_path(&ChannelIdent::unstable(), package);

        response::ok_if_unit(self.maybe_add_authz(self.0.get_with_custom_url(&url, |u| {
                                                            u.set_query(Some(&format!("target={}",
                                                                                      target)))
                                                        }),
                                                  token)
                                 .send()
                                 .await?,
                             &[StatusCode::OK]).await
    }

    /// Returns a package ident struct for the latest package. Arguably should be renamed
    ///
    /// An optional version can be specified which will scope the release returned to the latest
    /// release of that package.
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Builder is not available
    pub async fn show_package(&self,
                              (package, target): (&PackageIdent, PackageTarget),
                              channel: &ChannelIdent,
                              token: Option<&str>)
                              -> Result<PackageIdent> {
        debug!("Retrieving package ident for {}, target {}",
               package, target);

        let package = self.show_package_metadata((package, target), channel, token)
                          .await?;
        Ok(package.ident)
    }

    /// Returns a package struct for the latest package.
    ///
    /// An optional version can be specified which will scope the release returned to the latest
    /// release of that package.
    ///
    /// # Failures
    ///
    /// * Package cannot be found
    /// * Remote Builder is not available
    pub async fn show_package_metadata(&self,
                                       (package, target): (&PackageIdent, PackageTarget),
                                       channel: &ChannelIdent,
                                       token: Option<&str>)
                                       -> Result<Package> {
        debug!("Retrieving package metadata for {}, target {}",
               package, target);

        let mut url = channel_package_path(channel, package);

        if !package.fully_qualified() {
            url.push_str("/latest");
        }

        let resp = self.maybe_add_authz(self.0.get_with_custom_url(&url, |u| {
                                                  u.set_query(Some(&format!("target={}", target)))
                                              }),
                                        token)
                       .send()
                       .await?;
        let resp = response::ok_if(resp, &[StatusCode::OK]).await?;

        let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
        trace!(target: "habitat_http_client::api_client::show_package_metadata", "{:?}", encoded);

        let package: Package = serde_json::from_str::<Package>(&encoded)?;
        Ok(package)
    }

    /// Upload a package to a remote Builder.
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * File cannot be read
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub async fn put_package<'a>(&'a self,
                                 pa: &'a mut PackageArchive,
                                 token: &'a str,
                                 force_upload: bool,
                                 auto_build: BuildOnUpload,
                                 progress: Option<Box<dyn DisplayProgress>>)
                                 -> Result<()> {
        let checksum = pa.checksum()?;
        let ident = pa.ident()?;
        let target = pa.target()?;

        debug!("Uploading package {}, target {} (forced: {})",
               ident, target, force_upload);

        let path = package_path(&ident);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("checksum", &checksum)
               .append_pair("target", &target.to_string())
               .append_pair("forced", &force_upload.to_string());

            // Builder uses presence of the `builder` param to disable builds.
            // Only send the parameter when we the user requests builds be disabled.
            if let BuildOnUpload::Disable = auto_build {
                url.query_pairs_mut().append_pair("builder", "true");
            }
        };

        debug!("Reading from {}", &pa.path.display());
        let body = Self::upload_body(&pa.path, progress).await?;

        let resp = self.0
                       .post_with_custom_url(&path, custom)
                       .bearer_auth(token)
                       .body(body)
                       .send()
                       .await?;

        response::ok_if_unit(resp, &[StatusCode::OK, StatusCode::CREATED]).await
    }

    /// Delete a package from Builder
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    /// * If package does not exist in Builder
    /// * If the package does not qualify for deletion
    /// * Authorization token was not set on client
    pub async fn delete_package(&self,
                                (ident, target): (&PackageIdent, PackageTarget),
                                token: &str)
                                -> Result<()> {
        debug!("Deleting package {}, target {}", ident, target);
        let path = package_path(ident);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("target", &target.to_string());
        };

        response::ok_if_unit(self.0
                                 .delete_with_custom_url(&path, custom)
                                 .bearer_auth(token)
                                 .send()
                                 .await?,
                             &[StatusCode::NO_CONTENT]).await
    }

    /// Promote a package to a given channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    ///
    /// # Panics
    ///
    /// * If package does not exist in Builder
    /// * Authorization token was not set on client
    pub async fn promote_package(&self,
                                 (ident, target): (&PackageIdent, PackageTarget),
                                 channel: &ChannelIdent,
                                 token: &str)
                                 -> Result<()> {
        debug!("Promoting package {}, target {} to channel {}",
               ident, target, channel);

        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }
        let path = channel_package_promote(channel, ident);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("target", &target.to_string());
        };

        response::ok_if_unit(self.0
                                 .put_with_custom_url(&path, custom)
                                 .bearer_auth(token)
                                 .send()
                                 .await?,
                             &[StatusCode::OK]).await
    }

    /// Demote a package from a given channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    ///
    /// # Panics
    ///
    /// * If package does not exist in Builder
    /// * Authorization token was not set on client
    pub async fn demote_package(&self,
                                (ident, target): (&PackageIdent, PackageTarget),
                                channel: &ChannelIdent,
                                token: &str)
                                -> Result<()> {
        debug!("Demoting package {}, target {} from channel {}",
               ident, target, channel);

        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }
        let path = channel_package_demote(channel, ident);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("target", &target.to_string());
        };

        response::ok_if_unit(self.0
                                 .put_with_custom_url(&path, custom)
                                 .bearer_auth(token)
                                 .send()
                                 .await?,
                             &[StatusCode::OK]).await
    }

    /// Create a custom channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn create_channel(&self,
                                origin: &str,
                                channel: &ChannelIdent,
                                token: &str)
                                -> Result<()> {
        debug!("Creating channel {} for origin {}", channel, origin);

        let path = format!("depot/channels/{}/{}", origin, channel);
        response::ok_if_unit(self.0.post(&path).bearer_auth(token).send().await?,
                             &[StatusCode::CREATED]).await
    }

    /// Delete a custom channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn delete_channel(&self,
                                origin: &str,
                                channel: &ChannelIdent,
                                token: &str)
                                -> Result<()> {
        debug!("Deleting channel {} for origin {}", channel, origin);

        let path = format!("depot/channels/{}/{}", origin, channel);
        response::ok_if_unit(self.0.delete(&path).bearer_auth(token).send().await?,
                             &[StatusCode::OK]).await
    }

    /// Promote all packages in channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn promote_channel_packages(&self,
                                          origin: &str,
                                          token: &str,
                                          source_channel: &ChannelIdent,
                                          target_channel: &ChannelIdent)
                                          -> Result<()> {
        debug!("Promoting packages in channel {:?} to channel {:?}",
               source_channel, target_channel);

        let path = format!("depot/channels/{}/{}/pkgs/promote", origin, source_channel);

        response::ok_if_unit(self.0
                                 .put_with_custom_url(&path, |url| {
                                     url.query_pairs_mut()
                                        .append_pair("channel", target_channel.as_str());
                                 })
                                 .bearer_auth(token)
                                 .send()
                                 .await?,
                             &[StatusCode::OK]).await
    }

    /// Demote all packages from channel
    ///
    /// # Failures
    ///
    /// * Remote Builder is not available
    pub async fn demote_channel_packages(&self,
                                         origin: &str,
                                         token: &str,
                                         source_channel: &ChannelIdent,
                                         target_channel: &ChannelIdent)
                                         -> Result<()> {
        debug!("Demoting packages selected from channel {:?} in {:?}",
               source_channel, target_channel);

        let path = format!("depot/channels/{}/{}/pkgs/demote", origin, source_channel);

        response::ok_if_unit(self.0
                                 .put_with_custom_url(&path, |url| {
                                     url.query_pairs_mut()
                                        .append_pair("channel", target_channel.as_str());
                                 })
                                 .bearer_auth(token)
                                 .send()
                                 .await?,
                             &[StatusCode::OK]).await
    }

    /// Returns a vector of PackageIdent structs
    ///
    /// # Failures
    ///
    /// * Remote depot unavailable
    pub async fn search_package(&self,
                                search_term: &str,
                                limit: usize,
                                token: Option<&str>)
                                -> Result<(Vec<PackageIdent>, usize)> {
        self.search_package_impl(search_term, limit, token, Self::seach_package_with_range)
            .await
    }

    /// Return a list of channels for a given origin
    ///
    /// # Failures
    /// * Remote Builder is not available
    /// * Authorization token was not set on client
    pub async fn list_channels(&self,
                               origin: &str,
                               include_sandbox_channels: bool)
                               -> Result<Vec<String>> {
        debug!("Listing channels for origin {}", origin);

        let path = format!("depot/channels/{}", origin);
        let resp = if include_sandbox_channels {
            self.0
                .get_with_custom_url(&path, |url| url.set_query(Some("sandbox=true")))
                .send()
                .await?
        } else {
            self.0.get(&path).send().await?
        };
        debug!("Response Status: {:?}", resp.status());

        match resp.status() {
            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
                let encoded = resp.text().await.map_err(Error::BadResponseBody)?;
                let results: Vec<OriginChannelIdent> = serde_json::from_str(&encoded)?;
                let channels = results.into_iter().map(|o| o.name).collect();
                Ok(channels)
            }
            _ => Err(response::err_from_response(resp).await),
        }
    }
}

fn origin_keys_path(origin: &str) -> String { format!("depot/origins/{}/keys", origin) }

fn package_download(package: &PackageIdent) -> String {
    format!("{}/download", package_path(package))
}

fn package_path(package: &PackageIdent) -> String { format!("depot/pkgs/{}", package) }

fn package_search(term: &str) -> String {
    let encoded_term = percent_encode(term.as_bytes(), PATH_SEGMENT_ENCODE_SET);
    format!("depot/pkgs/search/{}", encoded_term)
}

fn channel_package_path(channel: &ChannelIdent, package: &PackageIdent) -> String {
    let mut path = format!("depot/channels/{}/{}/pkgs/{}",
                           package.origin(),
                           channel,
                           package.name());
    if let Some(version) = package.version() {
        path.push_str("/");
        path.push_str(version);
        if let Some(release) = package.release() {
            path.push_str("/");
            path.push_str(release);
        }
    }
    path
}

fn package_channels_path(package: &PackageIdent) -> String {
    format!("depot/pkgs/{}/{}/{}/{}/channels",
            package.origin(),
            package.name(),
            package.version().unwrap(),
            package.release().unwrap())
}

fn channel_package_promote(channel: &ChannelIdent, package: &PackageIdent) -> String {
    format!("depot/channels/{}/{}/pkgs/{}/{}/{}/promote",
            package.origin(),
            channel,
            package.name(),
            package.version().unwrap(),
            package.release().unwrap())
}

fn channel_package_demote(channel: &ChannelIdent, package: &PackageIdent) -> String {
    format!("depot/channels/{}/{}/pkgs/{}/{}/{}/demote",
            package.origin(),
            channel,
            package.name(),
            package.version().unwrap(),
            package.release().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::{self,
                          Ready};
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

    fn get_test_ident(name: &str) -> PackageIdent {
        PackageIdent { origin:  String::from("test"),
                       name:    String::from(name),
                       version: None,
                       release: None, }
    }

    fn seach_generator<'a>(
        data: &'a [&str],
        step: usize)
        -> impl Fn(&BuilderAPIClient,
              &'a str,
              Option<&'a str>,
              usize) -> Ready<Result<(PackageResults<PackageIdent>, bool)>> {
        move |_client, search_term, _token, range| {
            let filtered = data.iter()
                               .filter(|d| d.contains(search_term))
                               .collect::<Vec<_>>();

            if filtered.is_empty() {
                return future::ready(Ok((PackageResults { range_start: 0,
                                                          range_end:   0,
                                                          total_count: 0,
                                                          data:        vec![], },
                                         false)));
            }

            let total = filtered.len();
            let last = total - 1;
            let (start, end) = if range >= last {
                (last, last)
            } else {
                (range, (range + step).min(last))
            };
            let filtered_range = filtered[start..=end].iter()
                                                      .map(|s| get_test_ident(**s))
                                                      .collect::<Vec<_>>();
            let result = PackageResults { range_start: start as isize,
                                          range_end:   end as isize,
                                          total_count: total as isize,
                                          data:        filtered_range, };
            future::ready(Ok((result, end < last)))
        }
    }

    #[tokio::test]
    async fn package_search() {
        let client = BuilderAPIClient::new("http://test.com", "", "", None).expect("valid client");

        let sample_data = vec!["one_a", "one_b", "one_c", "one_d", "one_e", "two_a", "two_b",
                               "two_c", "two_d", "two_e",];

        let searcher = seach_generator(sample_data.as_slice(), 2);
        let r = client.search_package_impl("one", 10, None, searcher)
                      .await
                      .expect("valid search");
        assert_eq!(r.0.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                   vec!["one_a", "one_b", "one_c", "one_d", "one_e"]);
        assert_eq!(r.1, 5);

        let searcher = seach_generator(sample_data.as_slice(), 2);
        let r = client.search_package_impl("_", 3, None, searcher)
                      .await
                      .expect("valid search");
        assert_eq!(r.0.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                   vec!["one_a", "one_b", "one_c"]);
        assert_eq!(r.1, 10);

        let searcher = seach_generator(sample_data.as_slice(), 10);
        let r = client.search_package_impl("a", 2, None, searcher)
                      .await
                      .expect("valid search");
        assert_eq!(r.0.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                   vec!["one_a", "two_a"]);
        assert_eq!(r.1, 2);

        let searcher = seach_generator(sample_data.as_slice(), 10);
        let r = client.search_package_impl("does_not_exist", 100, None, searcher)
                      .await
                      .expect("valid search");
        assert_eq!(r.0.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                   Vec::<String>::new());
        assert_eq!(r.1, 0);
    }

    #[tokio::test]
    #[ignore = "takes too long to run regularly; should run on CI"]
    async fn package_search_large() {
        let client = BuilderAPIClient::new("http://test.com", "", "", None).expect("valid client");

        let count = 100_000;
        let sample_data = std::iter::repeat("test").take(count).collect::<Vec<_>>();

        let searcher = seach_generator(sample_data.as_slice(), 50);
        let r = client.search_package_impl("test", count, None, searcher)
                      .await
                      .expect("valid search");
        assert_eq!(r.1, count);
    }
}
