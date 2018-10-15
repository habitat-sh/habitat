// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::fs;
use std::path::Path;

use hcore::util;

use error::Result;
use util::write_file;

/// The default password file contents.
const ETC_PASSWD: &'static str = include_str!("../defaults/etc/passwd");
/// The default group file contents.
const ETC_GROUP: &'static str = include_str!("../defaults/etc/group");
/// The default `resolv.conf` contents.
const ETC_RESOLV_CONF: &'static str = include_str!("../defaults/etc/resolv.conf");
/// The default `nsswitch.conf` contents.
const ETC_NSSWITCH_CONF: &'static str = include_str!("../defaults/etc/nsswitch.conf");

/// Creates a root file system under the given path.
///
/// # Errors
///
/// * If files and/or directories cannot be created
/// * If permissions for files and/or directories cannot be set
#[cfg(unix)]
pub fn create<T>(root: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let root = root.as_ref();
    fs::create_dir_all(root.join("bin"))?;
    fs::create_dir_all(root.join("etc"))?;

    fs::create_dir_all(root.join("root"))?;
    util::posix_perm::set_permissions(root.join("root").to_string_lossy().as_ref(), 0o0750)?;
    fs::create_dir_all(root.join("tmp"))?;
    util::posix_perm::set_permissions(root.join("tmp").to_string_lossy().as_ref(), 0o1777)?;
    fs::create_dir_all(root.join("var/tmp"))?;
    util::posix_perm::set_permissions(root.join("var/tmp").to_string_lossy().as_ref(), 0o1777)?;

    write_file(root.join("etc/passwd"), ETC_PASSWD)?;
    write_file(root.join("etc/group"), ETC_GROUP)?;
    write_file(root.join("etc/resolv.conf"), ETC_RESOLV_CONF)?;
    write_file(root.join("etc/nsswitch.conf"), ETC_NSSWITCH_CONF)?;

    Ok(())
}

#[cfg(unix)]
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    use std::os::unix::fs::MetadataExt;

    use tempdir::TempDir;

    use super::*;

    fn file_content<T: AsRef<Path>>(file: T) -> String {
        let mut content = String::new();
        let mut file = File::open(file).unwrap();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn creates_files_and_dirs() {
        let root = TempDir::new("root").unwrap();
        create(&root).unwrap();

        assert!(root.path().join("bin").is_dir());
        assert!(root.path().join("etc").is_dir());

        let root_home = root.path().join("root");
        assert!(root_home.is_dir());
        assert_eq!(0o40750, root_home.metadata().unwrap().mode()); // the `4` refers to file type

        let tmp = root.path().join("tmp");
        assert!(tmp.is_dir());
        assert_eq!(0o41777, tmp.metadata().unwrap().mode()); // the `4` refers to file type

        let var_tmp = root.path().join("var").join("tmp");
        assert!(var_tmp.is_dir());
        assert_eq!(0o41777, var_tmp.metadata().unwrap().mode()); // the `4` refers to file type

        let etc_passwd = root.path().join("etc").join("passwd");
        assert!(etc_passwd.is_file());
        assert_eq!(ETC_PASSWD, file_content(etc_passwd));

        let etc_group = root.path().join("etc").join("group");
        assert!(etc_group.is_file());
        assert_eq!(ETC_GROUP, file_content(etc_group));

        let etc_resolv_conf = root.path().join("etc").join("resolv.conf");
        assert!(etc_resolv_conf.is_file());
        assert_eq!(ETC_RESOLV_CONF, file_content(etc_resolv_conf));

        let etc_nsswitch_conf = root.path().join("etc").join("nsswitch.conf");
        assert!(etc_nsswitch_conf.is_file());
        assert_eq!(ETC_NSSWITCH_CONF, file_content(etc_nsswitch_conf));
    }
}
