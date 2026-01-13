use crate::util::write_file;
use anyhow::Result;
use std::{fs,
          path::Path};

/// The default password file contents.
const ETC_PASSWD: &str = include_str!("../defaults/etc/passwd");
/// The default group file contents.
const ETC_GROUP: &str = include_str!("../defaults/etc/group");
/// The default `resolv.conf` contents.
const ETC_RESOLV_CONF: &str = include_str!("../defaults/etc/resolv.conf");
/// The default `nsswitch.conf` contents.
const ETC_NSSWITCH_CONF: &str = include_str!("../defaults/etc/nsswitch.conf");

/// Creates a root file system under the given path.
///
/// # Errors
///
/// * If files and/or directories cannot be created
/// * If permissions for files and/or directories cannot be set
#[cfg(unix)]
pub(crate) fn create<T>(root: T) -> Result<()>
    where T: AsRef<Path>
{
    let root = root.as_ref();

    // We'll create links to all the busybox utilities here, as well
    // as links to `hab` and user package binaries.
    fs::create_dir_all(root.join("bin"))?;

    // We'll put all our custom /etc content in the rootfs, as well.
    fs::create_dir_all(root.join("etc"))?;
    write_file(root.join("etc/passwd"), ETC_PASSWD)?;
    write_file(root.join("etc/group"), ETC_GROUP)?;
    write_file(root.join("etc/resolv.conf"), ETC_RESOLV_CONF)?;
    write_file(root.join("etc/nsswitch.conf"), ETC_NSSWITCH_CONF)?;

    // Note that other required directories are currently handled
    // directly in the Dockerfile.

    Ok(())
}

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File,
              io::Read};
    use tempfile::TempDir;

    fn file_content<T: AsRef<Path>>(file: T) -> String {
        let mut content = String::new();
        let mut file = File::open(file).unwrap();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn creates_files_and_dirs() {
        let root = TempDir::new().unwrap();
        create(&root).unwrap();

        assert!(root.path().join("bin").is_dir());
        assert!(root.path().join("etc").is_dir());

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
