// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use hcore::fs::FS_ROOT_PATH;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::util::{deserialize_using_from_str, serialize_using_to_string};

use error::{Error, Result};
use fs;
use util;

const PATH_KEY: &'static str = "PATH";
static LOGKEY: &'static str = "PK";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env(HashMap<String, String>);

impl Deref for Env {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Env {
    /// Modifies PATH env with the full run path for this package. This path is composed of any
    /// binary paths specified by this package, or its TDEPS, plus a path to a BusyBox(non-windows),
    /// plus the existing value of the PATH variable.
    ///
    /// This means we work on any operating system, as long as you can invoke the Supervisor,
    /// without having to worry much about context.
    pub fn new(package: &PackageInstall) -> Result<Self> {
        let mut env = package.runtime_environment()?;
        let path = Self::transform_path(env.get(PATH_KEY))?;
        env.insert(PATH_KEY.to_string(), path);
        Ok(Env(env))
    }

    fn transform_path(path: Option<&String>) -> Result<String> {
        let mut paths: Vec<PathBuf> = match path {
            Some(path) => env::split_paths(&path).collect(),
            None => vec![],
        };

        // Lets join the run paths to the FS_ROOT
        // In most cases, this does nothing and should only mutate
        // the paths in a windows studio where FS_ROOT_PATH will
        // be the studio root path (ie c:\hab\studios\...). In any other
        // environment FS_ROOT will be "/" and this will not make any
        // meaningful change.
        for i in 0..paths.len() {
            if paths[i].starts_with("/") {
                paths[i] = Path::new(&*FS_ROOT_PATH).join(paths[i].strip_prefix("/").unwrap());
            }
        }

        util::path::append_interpreter_and_path(&mut paths)
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
    pub svc_data_path: PathBuf,
    pub svc_files_path: PathBuf,
    pub svc_static_path: PathBuf,
    pub svc_var_path: PathBuf,
    pub svc_pid_file: PathBuf,
    pub svc_run: PathBuf,
    pub svc_user: String,
    pub svc_group: String,
}

impl Pkg {
    pub fn from_install(package: PackageInstall) -> Result<Self> {
        let (svc_user, svc_group) = util::users::get_user_and_group(&package)?;
        let pkg = Pkg {
            svc_path: fs::svc_path(&package.ident.name),
            svc_config_path: fs::svc_config_path(&package.ident.name),
            svc_data_path: fs::svc_data_path(&package.ident.name),
            svc_files_path: fs::svc_files_path(&package.ident.name),
            svc_run: fs::svc_path(&package.ident.name).join("run"),
            svc_static_path: fs::svc_static_path(&package.ident.name),
            svc_var_path: fs::svc_var_path(&package.ident.name),
            svc_pid_file: fs::svc_pid_file(&package.ident.name),
            svc_user: svc_user,
            svc_group: svc_group,
            env: Env::new(&package)?,
            deps: package.tdeps().map_err(|e| {
                sup_error!(Error::BadPackage(package.clone(), e))
            })?,
            exposes: package.exposes().map_err(|e| {
                sup_error!(Error::BadPackage(package.clone(), e))
            })?,
            exports: package.exports().map_err(|e| {
                sup_error!(Error::BadPackage(package.clone(), e))
            })?,
            path: package.installed_path,
            ident: package.ident.clone(),
            origin: package.ident.origin.clone(),
            name: package.ident.name.clone(),
            version: package.ident.version.expect(
                "No package version in PackageInstall",
            ),
            release: package.ident.release.expect(
                "No package release in PackageInstall",
            ),
        };
        Ok(pkg)
    }
}
