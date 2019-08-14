use crate::{error::{Error,
                    Result},
            hab_core::{crypto::keys::box_key_pair::WrappedSealedBox,
                       fs::AtomicWriter,
                       package::{Identifiable,
                                 PackageArchive,
                                 PackageIdent,
                                 PackageTarget},
                       ChannelIdent},
            hab_http::ApiClient,
            response::ResponseExt,
            BoxedClient,
            BuildOnUpload,
            BuilderAPIProvider,
            DisplayProgress,
            OriginKeyIdent,
            SchedulerResponse};
use broadcast::BroadcastWriter;
use reqwest::{header::{HeaderName,
                       CONTENT_LENGTH},
              Body,
              IntoUrl,
              RequestBuilder,
              StatusCode};
use std::{collections::HashMap,
          fs::{self,
               File},
          io::{self,
               Read},
          path::{Path,
                 PathBuf},
          str::FromStr,
          string::ToString};
use tee::TeeReader;
use url::Url;

const X_JFROG_ART_API: &str = "x-jfrog-art-api";
const X_ARTIFACTORY_FILENAME: &str = "x-artifactory-filename";

#[derive(Clone, Serialize, Deserialize, Debug)]
struct LatestVersion {
    pub version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Properties {
    pub properties: InnerProperties,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct InnerProperties {
    pub channels: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct FileInfo {
    pub uri: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct FolderInfo {
    pub children: Vec<FileInfo>,
}

pub struct ArtifactoryClient(ApiClient);

impl ArtifactoryClient {
    pub fn new<U>(endpoint: U,
                  product: &str,
                  version: &str,
                  fs_root_path: Option<&Path>)
                  -> Result<Self>
        where U: IntoUrl
    {
        let endpoint = endpoint.into_url().map_err(Error::ReqwestError)?;

        debug!("ArtifactoryClient::new, endpoint = {:?}", endpoint);
        let client = ArtifactoryClient(
            ApiClient::new(endpoint, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        );
        Ok(client)
    }

    pub fn create<U>(endpoint: U,
                     product: &str,
                     version: &str,
                     fs_root_path: Option<&Path>)
                     -> Result<BoxedClient>
        where U: IntoUrl
    {
        Self::new(endpoint, product, version, fs_root_path).map(|c| Box::new(c) as _)
    }

    fn add_authz(&self, rb: RequestBuilder, token: &str) -> RequestBuilder {
        rb.header(HeaderName::from_static(X_JFROG_ART_API), token)
    }

    fn download(&self,
                rb: RequestBuilder,
                dst_path: &Path,
                token: &str,
                progress: Option<<ArtifactoryClient as BuilderAPIProvider>::Progress>)
                -> Result<PathBuf> {
        let mut resp = self.add_authz(rb, token).send()?;
        resp.ok_if(StatusCode::OK)?;
        fs::create_dir_all(&dst_path)?;

        let file_name = resp.get_header(X_ARTIFACTORY_FILENAME)?;
        let dst_file_path = dst_path.join(file_name);
        let w = AtomicWriter::new(&dst_file_path)?;

        w.with_writer(|mut f| {
             match progress {
                 Some(mut progress) => {
                     let size = resp.get_header(CONTENT_LENGTH)?
                                    .parse()
                                    .map_err(Error::ParseIntError)?;

                     progress.size(size);
                     let mut writer = BroadcastWriter::new(&mut f, progress);
                     io::copy(&mut resp, &mut writer).map_err(Error::IO)
                 }
                 None => io::copy(&mut resp, &mut f).map_err(Error::IO),
             }
         })?;
        Ok(dst_file_path)
    }

    fn set_properties(&self,
                      path: &str,
                      properties: &HashMap<&str, String>,
                      token: &str)
                      -> Result<()> {
        let prop_arr: Vec<String> = properties.iter()
                                              .map(|(k, v)| format!("{}={}", k, v))
                                              .collect();
        let prop_str = &prop_arr.join(";");
        debug!("Setting properties on path {}: {}", path, prop_str);

        let custom = |url: &mut Url| {
            url.query_pairs_mut()
               .append_pair("properties", &prop_str.to_string());
        };

        self.add_authz(self.0.put_with_custom_url(&path, custom), token)
            .send()?
            .ok_if(StatusCode::NO_CONTENT)
    }

    fn get_properties(&self, path: &str, token: &str) -> Result<Properties> {
        debug!("Getting properties on path {}", path);

        let custom = |url: &mut Url| {
            url.query_pairs_mut().append_pair("properties", "");
        };

        let mut resp = self.add_authz(self.0.get_with_custom_url(&path, custom), token)
                           .send()?;
        resp.ok_if(StatusCode::OK)?;

        let mut encoded = String::new();
        resp.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Body: {:?}", encoded);

        let properties: Properties = serde_json::from_str::<Properties>(&encoded)?;

        Ok(properties)
    }

    fn repo_for_origin(&self, origin: &str) -> String {
        format!("habitat-artifact-store-local.{}", origin)
    }

    fn api_path_for_package(&self, ident: &PackageIdent, target: PackageTarget) -> String {
        let hart_name = ident.archive_name_with_target(target)
                             .expect("ident is fully qualified");

        format!("artifactory/api/storage/{}/pkgs/{}/{}/{}/{}/{}",
                self.repo_for_origin(ident.origin()),
                target,
                ident.origin(),
                ident.name(),
                ident.version().unwrap(), // unwrap Ok
                hart_name)
    }

    fn api_path_for_latest_package(&self, ident: &PackageIdent, target: PackageTarget) -> String {
        if ident.version().is_some() {
            format!("artifactory/api/versions/{}/pkgs/{}/{}/{}/{}",
                    self.repo_for_origin(ident.origin()),
                    target,
                    ident.origin(),
                    ident.name(),
                    ident.version().unwrap(), /* unwrap Ok */)
        } else {
            format!("artifactory/api/versions/{}/pkgs/{}/{}/{}",
                    self.repo_for_origin(ident.origin()),
                    target,
                    ident.origin(),
                    ident.name(),)
        }
    }

    fn url_path_for_package(&self, ident: &PackageIdent, target: PackageTarget) -> String {
        let hart_name = ident.archive_name_with_target(target)
                             .expect("ident is fully qualified");

        format!("artifactory/{}/pkgs/{}/{}/{}/{}/{}",
                self.repo_for_origin(ident.origin()),
                target,
                ident.origin(),
                ident.name(),
                ident.version().unwrap(), // unwrap Ok
                hart_name)
    }

    fn url_path_for_key(&self, origin: &str, revision: &str) -> String {
        format!("artifactory/{}/keys/{}",
                self.repo_for_origin(origin),
                &format!("{}-{}.pub", origin, revision))
    }

    fn api_path_for_key_folder(&self, origin: &str) -> String {
        format!("artifactory/api/storage/{}/keys",
                self.repo_for_origin(origin))
    }

    fn url_path_for_secret_key(&self, origin: &str, revision: &str) -> String {
        format!("artifactory/{}/secret_keys/{}",
                self.repo_for_origin(origin),
                &format!("{}-{}.sig.key", origin, revision))
    }

    fn api_path_for_secret_key(&self, origin: &str, revision: &str) -> String {
        format!("artifactory/api/storage/{}/secret_keys/{}-{}.sig.key",
                self.repo_for_origin(origin),
                origin,
                revision)
    }

    fn api_path_for_latest_secret_key(&self, origin: &str) -> String {
        format!("artifactory/api/versions/{}/secret_keys",
                self.repo_for_origin(origin))
    }
}

impl BuilderAPIProvider for ArtifactoryClient {
    type Progress = Box<dyn DisplayProgress>;

    fn fetch_origin_key(&self,
                        origin: &str,
                        revision: &str,
                        token: Option<&str>,
                        dst_path: &Path,
                        progress: Option<Self::Progress>)
                        -> Result<PathBuf> {
        self.download(self.0.get(&self.url_path_for_key(origin, revision)),
                      dst_path.as_ref(),
                      token.unwrap_or_else(|| ""),
                      progress)
    }

    fn fetch_secret_origin_key(&self,
                               origin: &str,
                               token: &str,
                               dst_path: &Path,
                               progress: Option<Self::Progress>)
                               -> Result<PathBuf> {
        let path = self.api_path_for_latest_secret_key(origin);
        let mut resp = self.add_authz(self.0.get(&path), token).send()?;
        resp.ok_if(StatusCode::OK)?;

        let mut encoded = String::new();
        resp.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Body: {:?}", encoded);

        let version: LatestVersion = serde_json::from_str::<LatestVersion>(&encoded)?;
        debug!("Latest secret key version: {:?}", version.version);

        let key_path = self.url_path_for_secret_key(origin, &version.version);
        self.download(self.0.get(&key_path), dst_path.as_ref(), token, progress)
    }

    fn show_origin_keys(&self, origin: &str) -> Result<Vec<OriginKeyIdent>> {
        let mut resp = self.0.get(&self.api_path_for_key_folder(origin)).send()?;
        resp.ok_if(StatusCode::OK)?;

        let mut encoded = String::new();
        resp.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Body: {:?}", encoded);

        let folder_info: FolderInfo = serde_json::from_str::<FolderInfo>(&encoded)?;

        let revisions: Vec<OriginKeyIdent> =
            folder_info.children
                       .iter()
                       .map(|f| OriginKeyIdent::from_str(&f.uri[1..]).unwrap())
                       .collect();

        Ok(revisions)
    }

    fn package_channels(&self,
                        (ident, target): (&PackageIdent, PackageTarget),
                        token: Option<&str>)
                        -> Result<Vec<String>> {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let path = self.api_path_for_package(&ident, target);

        let properties = self.get_properties(&path, token.unwrap_or_else(|| ""))?;
        debug!("Got properties: {:?}", properties);

        Ok(properties.properties.channels.clone())
    }

    fn put_origin_key(&self,
                      origin: &str,
                      revision: &str,
                      src_path: &Path,
                      token: &str,
                      progress: Option<Self::Progress>)
                      -> Result<()> {
        let path = self.url_path_for_key(origin, revision);

        let file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file.metadata()
                            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
                            .len();

        let body = if let Some(mut progress) = progress {
            progress.size(file_size);
            let reader = TeeReader::new(file, progress);
            Body::sized(reader, file_size)
        } else {
            Body::sized(file, file_size)
        };

        let mut resp = self.add_authz(self.0.put(&path), token).body(body).send()?;
        resp.ok_if(StatusCode::CREATED)
    }

    fn put_origin_secret_key(&self,
                             origin: &str,
                             revision: &str,
                             src_path: &Path,
                             token: &str,
                             progress: Option<Self::Progress>)
                             -> Result<()> {
        let path = self.url_path_for_secret_key(origin, revision);

        let file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file.metadata()
                            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
                            .len();

        let body = if let Some(mut progress) = progress {
            progress.size(file_size);
            let reader = TeeReader::new(file, progress);
            Body::sized(reader, file_size)
        } else {
            Body::sized(file, file_size)
        };

        let mut resp = self.add_authz(self.0.put(&path), token).body(body).send()?;
        resp.ok_if(StatusCode::CREATED)?;

        let properties_path = self.api_path_for_secret_key(origin, revision);

        let mut properties: HashMap<&str, String> = HashMap::new();
        properties.insert("version", revision.to_string());

        self.set_properties(&properties_path, &properties, token)
    }

    fn fetch_package(&self,
                     (ident, target): (&PackageIdent, PackageTarget),
                     token: Option<&str>,
                     dst_path: &Path,
                     progress: Option<Self::Progress>)
                     -> Result<PackageArchive> {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let req_builder = self.0.get(&self.url_path_for_package(ident, target));

        match self.download(req_builder,
                            dst_path.as_ref(),
                            token.unwrap_or_else(|| ""),
                            progress)
        {
            Ok(file) => Ok(PackageArchive::new(file)),
            Err(e) => Err(e),
        }
    }

    fn check_package(&self,
                     (package, target): (&PackageIdent, PackageTarget),
                     token: Option<&str>)
                     -> Result<()> {
        if !package.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let url = self.api_path_for_package(package, target);

        self.add_authz(self.0.get(&url), token.unwrap_or_else(|| ""))
            .send()?
            .ok_if(StatusCode::OK)
    }

    fn show_package(&self,
                    (package, target): (&PackageIdent, PackageTarget),
                    channel: &ChannelIdent,
                    token: Option<&str>)
                    -> Result<PackageIdent> {
        let path = self.api_path_for_latest_package(package, target);

        let mut resp = self.add_authz(self.0.get_with_custom_url(&path, |u| {
                                                u.set_query(Some(&format!("channels={}", channel)))
                                            }),
                                      token.unwrap_or_else(|| ""))
                           .send()?;
        resp.ok_if(StatusCode::OK)?;

        let mut encoded = String::new();
        resp.read_to_string(&mut encoded)
            .map_err(Error::BadResponseBody)?;
        debug!("Body: {:?}", encoded);

        let version: LatestVersion = serde_json::from_str::<LatestVersion>(&encoded)?;
        debug!("Latest package version: {:?}", version);

        let ident = format!("{}/{}/{}", package.origin, package.name, version.version);
        PackageIdent::from_str(&ident).map_err(Error::HabitatCore)
    }

    fn put_package(&self,
                   pa: &mut PackageArchive,
                   token: &str,
                   _force_upload: bool,
                   _auto_build: BuildOnUpload,
                   progress: Option<Self::Progress>)
                   -> Result<()> {
        let checksum = pa.checksum()?;
        let ident = pa.ident()?;
        let target = pa.target()?;

        let file = File::open(&pa.path).map_err(|e| Error::PackageReadError(pa.path.clone(), e))?;
        let file_size = file.metadata()
                            .map_err(|e| Error::PackageReadError(pa.path.clone(), e))?
                            .len();

        let path = self.url_path_for_package(&ident, target);

        let body = if let Some(mut progress) = progress {
            progress.size(file_size);
            let reader = TeeReader::new(file, progress);
            Body::sized(reader, file_size)
        } else {
            Body::sized(file, file_size)
        };

        let mut resp = self.add_authz(self.0.put(&path), token).body(body).send()?;
        resp.ok_if(StatusCode::OK)?;

        let properties_path = self.api_path_for_package(&ident, target);

        let mut properties: HashMap<&str, String> = HashMap::new();
        properties.insert("version",
                          format!("{}/{}", ident.version.unwrap(), ident.release.unwrap())); // Unwrap ok
        properties.insert("target", target.to_string());
        properties.insert("channels", ChannelIdent::unstable().to_string());
        properties.insert("checksum", checksum);

        self.set_properties(&properties_path, &properties, token)
    }

    fn delete_package(&self,
                      (ident, target): (&PackageIdent, PackageTarget),
                      token: &str)
                      -> Result<()> {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let path = self.url_path_for_package(ident, target);
        self.add_authz(self.0.delete(&path), token)
            .send()?
            .ok_if(StatusCode::NO_CONTENT)
    }

    fn promote_package(&self,
                       (ident, target): (&PackageIdent, PackageTarget),
                       channel: &ChannelIdent,
                       token: &str)
                       -> Result<()> {
        let properties_path = self.api_path_for_package(&ident, target);

        let curr_properties = self.get_properties(&properties_path, token)?;

        if curr_properties.properties
                          .channels
                          .iter()
                          .any(|c| c == &channel.to_string())
        {
            debug!("Already has channel {}", channel.to_string());
            return Ok(());
        }

        let curr_channels = &curr_properties.properties.channels.join(",");
        let new_channels = format!("{},{}", curr_channels, channel.to_string());

        let mut properties: HashMap<&str, String> = HashMap::new();
        properties.insert("channels", new_channels);

        self.set_properties(&properties_path, &properties, token)
    }

    fn demote_package(&self,
                      (ident, target): (&PackageIdent, PackageTarget),
                      channel: &ChannelIdent,
                      token: &str)
                      -> Result<()> {
        let properties_path = self.api_path_for_package(&ident, target);

        let curr_properties = self.get_properties(&properties_path, token)?;

        if !curr_properties.properties
                           .channels
                           .iter()
                           .any(|c| c == &channel.to_string())
        {
            debug!("Does not have channel {}", channel.to_string());
            return Ok(());
        }

        let filtered_channels: Vec<String> = curr_properties.properties
                                                            .channels
                                                            .into_iter()
                                                            .filter(|c| c != &channel.to_string())
                                                            .collect();
        let new_channels = filtered_channels.join(",");
        debug!("New channels: {}", new_channels);

        let mut properties: HashMap<&str, String> = HashMap::new();
        properties.insert("channels", new_channels);

        self.set_properties(&properties_path, &properties, token)
    }

    // No-op functions - return success

    fn create_channel(&self, _origin: &str, _channel: &ChannelIdent, _token: &str) -> Result<()> {
        Ok(())
    }

    fn delete_channel(&self, _origin: &str, _channel: &ChannelIdent, _token: &str) -> Result<()> {
        Ok(())
    }

    fn list_channels(&self, _origin: &str, _include_sandbox_channels: bool) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    // Not supported functions - return error

    fn search_package(&self,
                      _search_term: &str,
                      _limit: usize,
                      _token: Option<&str>)
                      -> Result<(Vec<PackageIdent>, usize)> {
        Err(Error::NotSupported)
    }

    fn x_put_package(&self, _pa: &mut PackageArchive, _token: &str) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn get_origin_schedule(&self, _origin: &str, _limit: usize) -> Result<Vec<SchedulerResponse>> {
        Err(Error::NotSupported)
    }

    fn get_schedule(&self, _group_id: i64, _include_projects: bool) -> Result<SchedulerResponse> {
        Err(Error::NotSupported)
    }

    fn schedule_job(&self,
                    (_ident, _target): (&PackageIdent, PackageTarget),
                    _package_only: bool,
                    _token: &str)
                    -> Result<(String)> {
        Err(Error::NotSupported)
    }

    fn fetch_rdeps(&self,
                   (_ident, _target): (&PackageIdent, PackageTarget))
                   -> Result<Vec<String>> {
        Err(Error::NotSupported)
    }

    fn job_group_promote_or_demote(&self,
                                   _group_id: u64,
                                   _idents: &[String],
                                   _channel: &ChannelIdent,
                                   _token: &str,
                                   _promote: bool)
                                   -> Result<()> {
        Err(Error::NotSupported)
    }

    fn job_group_cancel(&self, _group_id: u64, _token: &str) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn fetch_origin_public_encryption_key(&self,
                                          _origin: &str,
                                          _token: &str,
                                          _dst_path: &Path,
                                          _progress: Option<Self::Progress>)
                                          -> Result<PathBuf> {
        Err(Error::NotSupported)
    }

    fn create_origin(&self, _origin: &str, _token: &str) -> Result<()> { Err(Error::NotSupported) }

    fn create_origin_secret(&self,
                            _origin: &str,
                            _token: &str,
                            _key: &str,
                            _secret: &WrappedSealedBox)
                            -> Result<()> {
        Err(Error::NotSupported)
    }

    fn delete_origin_secret(&self, _origin: &str, _token: &str, _key: &str) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn delete_origin(&self, _origin: &str, _token: &str) -> Result<()> { Err(Error::NotSupported) }

    fn list_origin_secrets(&self, _origin: &str, _token: &str) -> Result<Vec<String>> {
        Err(Error::NotSupported)
    }
}
