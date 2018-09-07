// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Encapsulate operations for creating a supervised service's service
//! directories (i.e., `/hab/svc/$NAME`).

use std::convert::From;
use std::fs as stdfs;
use std::path::Path;

use error::{Error, Result};
use fs;
use hcore::util::perm;
use manager::service::package::Pkg;
use sys::abilities;
use sys::users::assert_pkg_user_and_group;

/// Permissions that service-owned service directories should
/// have. The user and group will be `SVC_USER` / `SVC_GROUP`.
const SVC_DIR_PERMISSIONS: u32 = 0o770;

// NOTE: This is the same log key from the manager, from which this
// functionality was originally extracted. Having a separate log key
// for creating directories seemed excessive.
static LOGKEY: &'static str = "SR";

/// Represents the service directory for a given package.
pub struct SvcDir<'a> {
    service_name: &'a str,
    svc_user: &'a str,
    svc_group: &'a str,
}

impl<'a> SvcDir<'a> {
    // TODO (CM): When / if data intended solely for templated content
    // is separated out of Pkg, we could just wrap a &Pkg directly,
    // instead of extracting name, user, and group. Until then,
    // however, we're being explicit to avoid confusion and needless
    // intertwining of code.

    // The fact that all our references are coming from a single Pkg
    // (with a single lifetime) is why we only take a single lifetime
    // parameter; beyond that, there's no intrinsic requirement for
    // the lifetimes of the three struct members to be the same.
    //
    // (They could also be Strings and not references, but there's
    // really no need to make copies of that data.)
    pub fn new(pkg: &'a Pkg) -> Self {
        SvcDir {
            service_name: &pkg.name,
            svc_user: &pkg.svc_user,
            svc_group: &pkg.svc_group,
        }
    }

    /// Create a service directory, including all necessary
    /// sub-directories. Ownership and permissions are handled as
    /// well.
    pub fn create(&self) -> Result<()> {
        if abilities::can_run_services_as_svc_user() {
            // The only reason we assert that these users exist is
            // because our `set_owner` calls will fail if they
            // don't. If we don't have the ability to to change
            // ownership, however, it doesn't really matter!
            assert_pkg_user_and_group(&self.svc_user, &self.svc_group)?;
        }

        self.create_svc_root()?;
        self.create_all_sup_owned_dirs()?;
        self.create_all_svc_owned_dirs()?;

        Ok(())
    }

    fn create_svc_root(&self) -> Result<()> {
        Self::create_dir_all(fs::svc_path(&self.service_name))
    }

    /// Creates all to sub-directories in a service directory that are
    /// owned by the Supervisor (that is, the user the current thread
    /// is running as).
    fn create_all_sup_owned_dirs(&self) -> Result<()> {
        Self::create_dir_all(fs::svc_hooks_path(&self.service_name))?;
        Self::create_dir_all(fs::svc_logs_path(&self.service_name))?;
        Ok(())
    }

    /// Creates all to sub-directories in a service directory that are
    /// owned by the service user by default.
    ///
    /// If the Supervisor (i.e., the current thread) is not running as
    /// a user that has the ability to change file and directory
    /// ownership, however, they will be owned by the Supervisor
    /// instead.
    fn create_all_svc_owned_dirs(&self) -> Result<()> {
        self.create_svc_owned_dir(fs::svc_config_path(&self.service_name))?;
        self.create_svc_owned_dir(fs::svc_data_path(&self.service_name))?;
        self.create_svc_owned_dir(fs::svc_files_path(&self.service_name))?;
        self.create_svc_owned_dir(fs::svc_var_path(&self.service_name))?;
        self.create_svc_owned_dir(fs::svc_static_path(&self.service_name))?;
        Ok(())
    }

    fn create_svc_owned_dir<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        Self::create_dir_all(&path)?;
        if cfg!(linux) {
            if abilities::can_run_services_as_svc_user() {
                perm::set_owner(&path, &self.svc_user, &self.svc_group)?;
            }
            perm::set_permissions(&path, SVC_DIR_PERMISSIONS).map_err(From::from)
        } else if cfg!(windows) {
            if path.as_ref().exists() {
                Ok(())
            } else {
                Err(sup_error!(Error::FileNotFound(format!(
                    "Invalid path {:?}",
                    path.as_ref().display()
                ))))
            }
        } else {
            unreachable!();
        }
    }

    fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        debug!("Creating dir with subdirs: {:?}", &path.as_ref());
        if let Err(e) = stdfs::create_dir_all(&path) {
            Err(sup_error!(Error::Permissions(format!(
                "Can't create {:?}, {}",
                &path.as_ref(),
                e
            ),)))
        } else {
            Ok(())
        }
    }
}
