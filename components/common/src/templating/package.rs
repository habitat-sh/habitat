use std::{collections::HashMap,
          env,
          ops::Deref,
          path::PathBuf,
          result};

use crate::hcore::{fs,
                   os::{process::{ShutdownSignal,
                                  ShutdownTimeout},
                        users},
                   package::{PackageIdent,
                             PackageInstall},
                   util::{deserialize_using_from_str,
                          serialize_using_to_string}};
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};

use crate::{error::{Error,
                    Result},
            util::path};

const DEFAULT_USER: &str = "hab";
const DEFAULT_GROUP: &str = "hab";

const PATH_KEY: &str = "PATH";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env(HashMap<String, String>);

impl Deref for Env {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<HashMap<String, String>> for Env {
    fn from(inner_map: HashMap<String, String>) -> Self { Env(inner_map) }
}

impl Env {
    /// Modifies PATH env with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus a path to a BusyBox(non-windows),
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    pub fn new(package: &PackageInstall) -> Result<Self> {
        let mut env = package.environment_for_command()?;
        let path = Self::transform_path(env.get(PATH_KEY))?;
        env.insert(PATH_KEY.to_string(), path);
        Ok(Env(env))
    }

    fn transform_path(path: Option<&String>) -> Result<String> {
        let mut paths: Vec<PathBuf> = match path {
            Some(path) => env::split_paths(&path).collect(),
            None => vec![],
        };

        path::append_interpreter_and_path(&mut paths)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pkg {
    #[serde(deserialize_with = "deserialize_using_from_str",
            serialize_with = "serialize_using_to_string")]
    pub ident: PackageIdent,
    pub origin: String,
    pub name: String,
    pub version: String,
    pub release: String,
    pub deps: Vec<PackageIdent>,
    pub env: Env,
    pub exposes: Vec<String>,
    pub exports: HashMap<String, String>,
    pub path: PathBuf,
    pub svc_path: PathBuf,
    pub svc_config_path: PathBuf,
    pub svc_config_install_path: PathBuf,
    pub svc_data_path: PathBuf,
    pub svc_files_path: PathBuf,
    pub svc_static_path: PathBuf,
    pub svc_var_path: PathBuf,
    pub svc_pid_file: PathBuf,
    pub svc_run: PathBuf,
    pub svc_user: String,
    pub svc_group: String,
    pub shutdown_signal: ShutdownSignal,
    pub shutdown_timeout: ShutdownTimeout,
}

impl Pkg {
    pub fn from_install(package: &PackageInstall) -> Result<Self> {
        let (svc_user, svc_group) = get_user_and_group(&package)?;
        let pkg =
            Pkg { svc_path: fs::svc_path(&package.ident.name),
                  svc_config_path: fs::svc_config_path(&package.ident.name),
                  svc_config_install_path: fs::svc_config_install_path(&package.ident.name),
                  svc_data_path: fs::svc_data_path(&package.ident.name),
                  svc_files_path: fs::svc_files_path(&package.ident.name),
                  svc_run: fs::svc_path(&package.ident.name).join("run"),
                  svc_static_path: fs::svc_static_path(&package.ident.name),
                  svc_var_path: fs::svc_var_path(&package.ident.name),
                  svc_pid_file: fs::svc_pid_file(&package.ident.name),
                  svc_user,
                  svc_group,
                  env: Env::new(&package)?,
                  deps: package.tdeps()?,
                  exposes: package.exposes()?,
                  exports: package.exports()?,
                  path: package.installed_path.clone(),
                  ident: package.ident.clone(),
                  origin: package.ident.origin.clone(),
                  name: package.ident.name.clone(),
                  version: package.ident
                                  .version
                                  .clone()
                                  .expect("No package version in PackageInstall"),
                  release: package.ident
                                  .release
                                  .clone()
                                  .expect("No package release in PackageInstall"),
                  shutdown_signal: package.shutdown_signal()?.unwrap_or_else(Default::default),
                  shutdown_timeout: package.shutdown_timeout()?.unwrap_or_else(Default::default) };
        Ok(pkg)
    }
}

/// This is a proxy struct to represent the data about a Pkg that we actually want to be
/// serialized, similar to ServiceProxy
pub struct PkgProxy<'a> {
    pkg: &'a Pkg,
}

impl<'a> PkgProxy<'a> {
    pub fn new(p: &'a Pkg) -> Self { PkgProxy { pkg: &p } }

    pub fn dependencies(&self) -> Vec<String> {
        self.pkg.deps.iter().map(PackageIdent::to_string).collect()
    }
}

impl<'a> Serialize for PkgProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let p = &self.pkg;
        let mut strukt = serializer.serialize_struct("pkg", 21)?;
        strukt.serialize_field("ident", &p.ident.to_string())?;
        strukt.serialize_field("origin", &p.origin)?;
        strukt.serialize_field("name", &p.name)?;
        strukt.serialize_field("version", &p.version)?;
        strukt.serialize_field("release", &p.release)?;
        strukt.serialize_field("deps", &p.deps)?;
        strukt.serialize_field("dependencies", &self.dependencies())?;
        strukt.serialize_field("env", &p.env)?;
        strukt.serialize_field("exposes", &p.exposes)?;
        strukt.serialize_field("exports", &p.exports)?;
        strukt.serialize_field("path", &p.path)?;
        strukt.serialize_field("svc_path", &p.svc_path)?;
        strukt.serialize_field("svc_config_path", &p.svc_config_path)?;
        strukt.serialize_field("svc_config_install_path", &p.svc_config_install_path)?;
        strukt.serialize_field("svc_data_path", &p.svc_data_path)?;
        strukt.serialize_field("svc_files_path", &p.svc_files_path)?;
        strukt.serialize_field("svc_static_path", &p.svc_static_path)?;
        strukt.serialize_field("svc_var_path", &p.svc_var_path)?;
        strukt.serialize_field("svc_pid_file", &p.svc_pid_file)?;
        strukt.serialize_field("svc_run", &p.svc_run)?;
        strukt.serialize_field("svc_user", &p.svc_user)?;
        strukt.serialize_field("svc_group", &p.svc_group)?;
        strukt.end()
    }
}

/// check and see if a user/group is specified in package metadata.
/// if not, we'll try and use hab/hab.
/// If hab/hab doesn't exist, try to use (current username, current group).
/// If that doesn't work, then give up.
#[cfg(unix)]
fn get_user_and_group(pkg_install: &PackageInstall) -> Result<(String, String)> {
    if let Some((user, group)) = get_pkg_user_and_group(&pkg_install)? {
        Ok((user, group))
    } else {
        let defaults = default_user_and_group()?;
        Ok(defaults)
    }
}

/// check and see if a user/group is specified in package metadata.
/// if not, we'll try and use hab/hab.
/// If hab/hab doesn't exist, try to use (current username, current group).
/// If that doesn't work, then give up.
/// Windows will also check if hab exists if it was the given user name
/// If it does not exist then fall back to the current username
/// This is because historically windows plans defaulted to
/// the hab pkg_svc_user even if not explicitly provided
#[cfg(windows)]
fn get_user_and_group(pkg_install: &PackageInstall) -> Result<(String, String)> {
    match get_pkg_user_and_group(&pkg_install)? {
        Some((ref user, ref _group)) if user == DEFAULT_USER => Ok(default_user_and_group()?),
        Some((user, group)) => Ok((user, group)),
        _ => Ok(default_user_and_group()?),
    }
}

/// This function checks to see if a custom SVC_USER and SVC_GROUP has
/// been specified as part of the package metadata.
/// If pkg_svc_user and pkg_svc_group have NOT been defined, return None.
fn get_pkg_user_and_group(pkg_install: &PackageInstall) -> Result<Option<(String, String)>> {
    let svc_user = pkg_install.svc_user()?;
    let svc_group = pkg_install.svc_group()?;
    match (svc_user, svc_group) {
        (Some(user), Some(group)) => Ok(Some((user, group))),
        _ => {
            debug!("User/group not specified in package, running with default");
            Ok(None)
        }
    }
}

/// checks to see if hab/hab exists, if not, fall back to
/// current user/group. If that fails, then return an error.
fn default_user_and_group() -> Result<(String, String)> {
    let uid = users::get_uid_by_name(DEFAULT_USER);
    let gid = users::get_gid_by_name(DEFAULT_GROUP);
    match (uid, gid) {
        (Some(_), Some(_)) => Ok((DEFAULT_USER.to_string(), DEFAULT_GROUP.to_string())),
        _ => {
            debug!("hab:hab does NOT exist");
            let user = users::get_current_username();
            let group = users::get_current_groupname();
            match (user, group) {
                (Some(user), Some(group)) => {
                    debug!("Running as {}/{}", user, group);
                    Ok((user, group))
                }
                _ => {
                    Err(Error::PermissionFailed("Can't determine current \
                                                 user:group"
                                                            .to_string()))
                }
            }
        }
    }
}
