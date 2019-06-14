use broadcast::BroadcastWriter;
use hyper::{client::{Body,
                     IntoUrl,
                     RequestBuilder,
                     Response},
            status::StatusCode,
            Url};
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
            BoxedClient,
            BuilderAPIProvider,
            DisplayProgress,
            OriginKeyIdent,
            SchedulerResponse};

header! { (XFileName, "X-Artifactory-Filename") => [String] }
header! { (XJFrogArtApi, "X-JFrog-Art-Api") => [String] }

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
        let endpoint = endpoint.into_url().map_err(Error::UrlParseError)?;

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
        let endpoint = endpoint.into_url().map_err(Error::UrlParseError)?;

        debug!("ArtifactoryClient::new, endpoint = {:?}", endpoint);
        let client = ArtifactoryClient(
            ApiClient::new(endpoint, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        );
        Ok(Box::new(client))
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder<'_> {
        rb.header(XJFrogArtApi(token.to_owned()))
    }

    fn download<'a>(&'a self,
                    rb: RequestBuilder<'a>,
                    dst_path: &Path,
                    token: &str,
                    progress: Option<<ArtifactoryClient as BuilderAPIProvider>::Progress>)
                    -> Result<PathBuf> {
        let mut res = self.add_authz(rb, token).send()?;
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }

        fs::create_dir_all(&dst_path)?;

        let file_name = res.headers
                           .get::<XFileName>()
                           .expect("X-Artifactory-Filename missing from response")
                           .to_string();
        let dst_file_path = dst_path.join(file_name);
        let w = AtomicWriter::new(&dst_file_path)?;
        w.with_writer(|mut f| {
             match progress {
                 Some(mut progress) => {
                     let size: u64 = res.headers
                                        .get::<hyper::header::ContentLength>()
                                        .map_or(0, |v| **v);
                     progress.size(size);
                     let mut writer = BroadcastWriter::new(&mut f, progress);
                     io::copy(&mut res, &mut writer)
                 }
                 None => io::copy(&mut res, &mut f),
             }
         })
         .map_err(Error::BadResponseBody)?;
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

        let res = self.add_authz(self.0.put_with_custom_url(&path, custom), token)
                      .send()?;
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::NoContent {
            Err(err_from_response(res))
        } else {
            Ok(())
        }
    }

    fn get_properties(&self, path: &str, token: &str) -> Result<Properties> {
        debug!("Getting properties on path {}", path);

        let custom = |url: &mut Url| {
            url.query_pairs_mut().append_pair("properties", "");
        };

        let mut res = self.add_authz(self.0.get_with_custom_url(&path, custom), token)
                          .send()?;
        debug!("Response: {:?}", res);

        if res.status != hyper::status::StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)
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
        let mut res = self.add_authz(self.0.get(&path), token).send()?;

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)
           .map_err(Error::BadResponseBody)?;
        debug!("Body: {:?}", encoded);

        let version: LatestVersion = serde_json::from_str::<LatestVersion>(&encoded)?;
        debug!("Latest secret key version: {:?}", version.version);

        let key_path = self.url_path_for_secret_key(origin, &version.version);
        self.download(self.0.get(&key_path), dst_path.as_ref(), token, progress)
    }

    fn show_origin_keys(&self, origin: &str) -> Result<Vec<OriginKeyIdent>> {
        let mut res = self.0.get(&self.api_path_for_key_folder(origin)).send()?;
        debug!("Response: {:?}", res);

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        };

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)
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

        let mut file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file.metadata()
                            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
                            .len();

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.0.put(&path), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.0.put(&path), token)
                .body(Body::SizedBody(&mut file, file_size))
                .send()
        };
        debug!("Response: {:?}", result);

        match result {
            Ok(Response { status: StatusCode::Created,
                          .. }) => Ok(()),
            Ok(response) => Err(err_from_response(response)),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn put_origin_secret_key(&self,
                             origin: &str,
                             revision: &str,
                             src_path: &Path,
                             token: &str,
                             progress: Option<Self::Progress>)
                             -> Result<()> {
        let path = self.url_path_for_secret_key(origin, revision);

        let mut file =
            File::open(src_path).map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?;
        let file_size = file.metadata()
                            .map_err(|e| Error::KeyReadError(src_path.to_path_buf(), e))?
                            .len();

        let result = if let Some(mut progress) = progress {
            progress.size(file_size);
            let mut reader = TeeReader::new(file, progress);
            self.add_authz(self.0.put(&path), token)
                .body(Body::SizedBody(&mut reader, file_size))
                .send()
        } else {
            self.add_authz(self.0.put(&path), token)
                .body(Body::SizedBody(&mut file, file_size))
                .send()
        };
        debug!("Response: {:?}", result);

        match result {
            Ok(Response { status: StatusCode::Created,
                          .. }) => (),
            Ok(response) => return Err(err_from_response(response)),
            Err(e) => return Err(Error::from(e)),
        };

        let properties_path = self.api_path_for_secret_key(origin, revision);

        let mut properties: HashMap<&str, String> = HashMap::new();
        properties.insert("version", revision.to_string());

        self.set_properties(&properties_path, &properties, token)?;

        Ok(())
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

        let res = self.add_authz(self.0.get(&url), token.unwrap_or_else(|| ""))
                      .send()?;

        debug!("Response: {:?}", res);

        if res.status != StatusCode::Ok {
            Err(err_from_response(res))
        } else {
            Ok(())
        }
    }

    fn show_package(&self,
                    (package, target): (&PackageIdent, PackageTarget),
                    channel: &ChannelIdent,
                    token: Option<&str>)
                    -> Result<PackageIdent> {
        let path = self.api_path_for_latest_package(package, target);

        let mut res = self.add_authz(self.0.get_with_custom_url(&path, |u| {
                                               u.set_query(Some(&format!("channels={}", channel)))
                                           }),
                                     token.unwrap_or_else(|| ""))
                          .send()?;
        debug!("Response: {:?}", res);

        if res.status != StatusCode::Ok {
            return Err(err_from_response(res));
        }

        let mut encoded = String::new();
        res.read_to_string(&mut encoded)
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

        let mut reader: Box<dyn Read> = if let Some(mut progress) = progress {
            progress.size(file_size);
            Box::new(TeeReader::new(file, progress))
        } else {
            Box::new(file)
        };

        let result = self.add_authz(self.0.put(&path), token)
                         .body(Body::SizedBody(&mut reader, file_size))
                         .send();
        debug!("Response: {:?}", result);

        match result {
            Ok(Response { status: StatusCode::Created,
                          .. }) => (),
            Ok(response) => return Err(err_from_response(response)),
            Err(e) => return Err(Error::from(e)),
        };

        let properties_path = self.api_path_for_package(&ident, target);

        let mut properties: HashMap<&str, String> = HashMap::new();
        properties.insert("version",
                          format!("{}/{}", ident.version.unwrap(), ident.release.unwrap())); // Unwrap ok
        properties.insert("target", target.to_string());
        properties.insert("channels", ChannelIdent::unstable().to_string());
        properties.insert("checksum", checksum);

        self.set_properties(&properties_path, &properties, token)?;

        Ok(())
    }

    fn delete_package(&self,
                      (ident, target): (&PackageIdent, PackageTarget),
                      token: &str)
                      -> Result<()> {
        if !ident.fully_qualified() {
            return Err(Error::IdentNotFullyQualified);
        }

        let path = self.url_path_for_package(ident, target);
        let res = self.add_authz(self.0.delete(&path), token).send()?;
        debug!("Response: {:?}", res);

        if res.status != StatusCode::NoContent {
            return Err(err_from_response(res));
        };

        Ok(())
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

        self.set_properties(&properties_path, &properties, token)?;

        Ok(())
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

        self.set_properties(&properties_path, &properties, token)?;

        Ok(())
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

fn err_from_response(mut response: hyper::client::Response) -> Error {
    if response.status == StatusCode::Unauthorized {
        return Error::APIError(response.status,
                               "Please check that you have specified a valid Artifactory API \
                                Key."
                                     .to_string());
    }

    let mut buff = String::new();
    if response.read_to_string(&mut buff).is_err() {
        buff.truncate(0)
    }
    Error::APIError(response.status, buff)
}
