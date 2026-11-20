use crate::{common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result},
            hcore::{fs::{FS_ROOT_PATH,
                         ROOT_PATH},
                    os::net::hostname}};
use flate2::{Compression,
             write::GzEncoder};
use jiff::Zoned;
use std::{env,
          fs::{self,
               File},
          path::{MAIN_SEPARATOR,
                 Path},
          process};

fn lookup_hostname() -> Result<String> {
    match hostname() {
        Ok(hostname) => Ok(hostname),
        Err(_) => Err(Error::NameLookup),
    }
}

pub fn start(ui: &mut UI) -> Result<()> {
    let dt = Zoned::now();
    ui.status(Status::Generating, bundle_status(&dt))?;
    let host = match lookup_hostname() {
        Ok(host) => host,
        Err(e) => {
            let host = String::from("localhost");
            ui.warn(format!("Hostname lookup failed; using fallback of {} ({})", host, e))?;
            host
        }
    };
    let cwd = env::current_dir().unwrap();
    let tarball_name = bundle_filename(&host, &dt);

    let sup_root = Path::new(&*FS_ROOT_PATH).join(ROOT_PATH).join("sup");
    let tar_gz = File::create(&tarball_name)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.follow_symlinks(false);

    if sup_root.exists() {
        ui.status(Status::Adding, format!("files from {}", sup_root.display()))?;
        if let Err(why) = tar.append_dir_all(format!("hab{}sup", MAIN_SEPARATOR), &sup_root) {
            ui.fatal(format!("Failed to add all files into the tarball: {}", why))?;
            fs::remove_file(&tarball_name)?;
            process::exit(1);
        }
    } else {
        ui.fatal(format!("Failed to find Supervisor root directory {}",
                         sup_root.display()))?;
        process::exit(1)
    }

    ui.status(Status::Created,
              format!("{}{}{}", cwd.display(), MAIN_SEPARATOR, tarball_name))?;

    Ok(())
}

fn bundle_status(dt: &Zoned) -> String {
    format!("New Support Bundle at {}", dt.strftime("%Y-%m-%d %H:%M:%S"))
}

fn bundle_filename(host: &str, dt: &Zoned) -> String {
    format!("support-bundle-{}-{}.tar.gz",
            host,
            dt.strftime("%Y%m%d%H%M%S"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_bundle_uses_local_date_for_status_and_filename() {
        let dt: Zoned = "2024-03-01T00:01:02.999+05:30[+05:30]".parse().unwrap();
        assert_eq!(bundle_status(&dt),
                   "New Support Bundle at 2024-03-01 00:01:02");
        assert_eq!(bundle_filename("node", &dt),
                   "support-bundle-node-20240301000102.tar.gz");
    }
}
