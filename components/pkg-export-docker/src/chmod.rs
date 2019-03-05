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

//! Encapsulates code to perform the equivalent of `chmod -R g=u` on a
//! directory.
//!
//! This is needed in order to ensure that the `/hab` directory in a
//! BuildRoot is constructed such that the root group has write
//! permissions, allowing containerized Habitat services to run as
//! non-root users, which is necessary for running in platforms like
//! OpenShift and various HPC situations.
//!
//! Because Docker does not yet provide the ability within a
//! Dockerfile to perform an `ADD` or `COPY` command while also
//! setting permissions on those files, we must set the permissions on
//! the files _before_ they are pulled into the container. We could
//! add a `RUN chmod -R g=u /hab` in the Dockerfile, but that
//! essentially doubles the size of any container, due to how Docker's
//! layered filesystem works. The approach here takes more code, but
//! results in a better experience for end-users.
//!
//! Also note that this is currently a no-op on Windows, since
//! permissions work differently over there, and we don't have
//! non-root Habitat supervisors there either.

use std::{fs,
          os::unix::fs::PermissionsExt,
          path::Path};

use crate::{error::Result,
            hcore::util::posix_perm};

/// Perform the equivalent of `chmod -R g=u path`.
pub fn recursive_g_equal_u<P>(path: P) -> Result<()>
    where P: AsRef<Path>
{
    let path = path.as_ref().to_path_buf();
    let metadata = fs::symlink_metadata(&path)?;
    let permissions = metadata.permissions();
    let filetype = metadata.file_type();

    if filetype.is_symlink() {
        // skip
    } else if filetype.is_dir() {
        set_g_equal_u(&path, &permissions)?;
        for entry in fs::read_dir(&path)? {
            let entry = entry.unwrap().path();
            recursive_g_equal_u(entry)?;
        }
    } else if filetype.is_file() {
        set_g_equal_u(&path, &permissions)?;
    }
    Ok(())
}

/// Set the group permissions of `path` equal to the user permissions.
fn set_g_equal_u<P, Q>(path: P, permissions: &Q) -> Result<()>
    where P: AsRef<Path>,
          Q: PermissionsExt
{
    let current = permissions.mode();
    let new_permissions = g_equals_u(current);
    posix_perm::set_permissions(&path, new_permissions).map_err(From::from)
}

/// Given a u32 representing Linux file permissions, set the group
/// permission bits equal to the user bits.
///
/// This is effectively the same as `chmod g=u`
///
/// And no, we do _not_ need anything more general purpose than this,
/// thankyouverymuch.
fn g_equals_u(perms: u32) -> u32 {
    // On Linux the permission bits are laid out thusly in a 32-bit
    // integer:
    //
    // user  (U)------------> [-]
    // group (G)---------------> [-]
    // other (O)------------------> [-]
    // xxxxxxxxxxxxxxxxxxxxxxx111111111
    //
    // We want the G bits to be identical to the U bits, while
    // preserving all the other bits as given.

    let other = perms & 0o7;
    let user = (perms >> 6) & 0o7;

    // Clear out all the UGO permission bits; we're going to set them as
    // appropriate.
    let cleared = (perms >> 9) << 9;

    // These are the new UGO bits, with U = G
    let new_perms = (((user << 3) | user) << 3) | other;

    cleared | new_perms
}

#[test]
fn test_g_equals_u() {
    let test_cases = vec![(0o00750, 0o00770),
                          (0o00150, 0o00110),
                          (0o00753, 0o00773),
                          (0o40753, 0o40773),
                          (0o02753, 0o02773),
                          (0o42753, 0o42773),
                          (0o00000, 0o00000),
                          (0o00007, 0o00007),
                          (0o00070, 0o00000),
                          (0o00770, 0o00770),
                          (0o00700, 0o00770),];

    for (input, expected) in test_cases {
        let actual = g_equals_u(input);
        assert!(actual == expected,
                "input = {:#o}, expected = {:#o}, actual = {:#o}",
                input,
                expected,
                actual);
    }
}
