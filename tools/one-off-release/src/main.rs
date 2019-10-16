use env_logger::Builder as LogBuilder;
use log::{debug,
          error,
          info,
          LevelFilter};
use regex::Regex;
use reqwest::Url;
use std::{error,
          fmt,
          fs::{self,
               File},
          io::{self,
               BufRead},
          path::PathBuf,
          process::Command,
          str::FromStr};
use structopt::StructOpt;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"\*\s(?P<ident>.*) \((?P<target>.*)\)").expect("valid regex");
}

#[allow(dead_code)]
const ACCEPTANCE_URL: &str = "https://bldr.acceptance.habitat.sh";
const PACKAGES_ENDPOINT: &str = "v1/depot/pkgs/";
const DOWNLOAD_ENDPOINT: &str = "download";

/// Wraps an owned `String` and implements the `Error` trait for it.
#[derive(Debug)]
struct StringError(String);

impl StringError {
    fn new(error: String) -> Self { Self(error) }
}

impl error::Error for StringError {}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

#[derive(Debug, PartialEq)]
struct Package {
    ident:  String,
    target: String,
}

impl Package {
    /// Create a new package
    fn new(ident: String, target: String) -> Self { Self { ident, target } }

    /// The filename to write the package to
    fn filename(&self) -> PathBuf {
        PathBuf::from_str(&format!("{}-{}.hart", self.ident.replace("/", "-"), self.target)).expect("valid path")
    }

    /// The url to download the package
    fn url(&self, base: &str) -> Result<Url, Box<dyn std::error::Error>> {
        let params = &[("target", &self.target)];
        let url = Url::parse(base)?.join(PACKAGES_ENDPOINT)?
                                   .join(&format!("{}/", self.ident))?
                                   .join(DOWNLOAD_ENDPOINT)?;
        let url = Url::parse_with_params(url.as_str(), params)?;
        Ok(url)
    }

    /// Download the package from `base_url` and save the hart file in `base_dir`
    fn download(&self,
                base_url: &str,
                base_dir: &PathBuf)
                -> Result<(), Box<dyn std::error::Error>> {
        let url = self.url(base_url)?;
        let output_path = base_dir.join(self.filename());
        debug!("Downloading package from '{}' and writing to {:?}",
               url,
               output_path.as_os_str());
        let mut response = reqwest::get(url)?.error_for_status()?;
        let mut output_file = File::create(&output_path)?;
        let written = io::copy(&mut response, &mut output_file)?;
        info!("Wrote {} bytes to {:?}", written, output_path.as_os_str());
        Ok(())
    }

    /// Upload the package that has been saved in `base_dir` using the hab command
    fn upload(&self,
              base_dir: &PathBuf,
              auth: &str,
              channel: &Option<String>,
              force: bool)
              -> Result<(), Box<dyn std::error::Error>> {
        let path = base_dir.join(self.filename());
        let mut command = Command::new("hab");
        command.arg("pkg").arg("upload");
        command.arg("--auth").arg(auth);
        if let Some(channel) = channel {
            command.arg("--channel").arg(channel);
        }
        if force {
            command.arg("--force");
        }
        command.arg(&path);
        let command_str = format!("{:?}", command).replace("\"", "");
        debug!("Uploading {:?} with command '{}'",
               path.as_os_str(),
               command_str);
        let status = command.status()?;
        if !status.success() {
            return Err(Box::new(StringError::new(format!("Failed to execute \
                                                          '{}' with {}",
                                                         command_str, status))));
        };

        info!("Uploaded '{}' with target '{}'", self.ident, self.target);
        Ok(())
    }
}

impl FromStr for Package {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(&s).ok_or("failed to parse")?;
        Ok(Self::new(String::from(&captures["ident"]),
                     String::from(&captures["target"])))
    }
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Builder live authentication token
    #[structopt(long, short = "z")]
    auth: String,

    /// The builder acceptance url
    #[structopt(default_value = ACCEPTANCE_URL, long)]
    acceptance_url: String,

    /// The channel to tag packages with when promoting to live
    #[structopt(long, short)]
    channel: Option<String>,

    /// Force upload the packages to live
    #[structopt(long)]
    force: bool,

    /// The temporary directory to write hart files to
    #[structopt(default_value = "one-off-release-tmp-dir", long)]
    tmp_dir: PathBuf,

    /// Log level
    #[structopt(default_value = "info",
                short,
                long,
                possible_values(&["trace", "debug", "info", "warn", "error"]))]
    log_level: LevelFilter,
}

fn main() {
    let Opt { auth,
              acceptance_url,
              channel,
              force,
              tmp_dir,
              log_level, } = Opt::from_args();

    LogBuilder::new().format_timestamp(None)
                     .format_module_path(false)
                     .filter_level(log_level)
                     .filter(Some("hyper"), LevelFilter::Warn)
                     .filter(Some("reqwest"), LevelFilter::Warn)
                     .filter(Some("tokio_reactor"), LevelFilter::Warn)
                     .init();

    // Create the temporary output directory
    if let Err(e) = fs::create_dir(&tmp_dir) {
        error!("Failed to create {:?}, err: {}", tmp_dir, e);
        return;
    };

    // Read all lines from stdin until an empty line
    let stdin = io::stdin();
    let lines = stdin.lock()
                     .lines()
                     .map(|l| l.expect("to read a line"))
                     .take_while(|l| !l.is_empty())
                     .collect::<Vec<_>>();

    // Parse each line into a package
    let packages = lines.iter()
                        .filter_map(|l| {
                            let package_result = l.parse();
                            if package_result.is_err() {
                                error!("Failed to parse line '{}'", l);
                                return None;
                            }
                            package_result.ok()
                        })
                        .collect::<Vec<Package>>();

    // Download all packages
    for p in packages.iter() {
        if let Err(e) = p.download(&acceptance_url, &tmp_dir) {
            error!("Failed to download {:?}, err: {}", p, e);
        }
    }

    // Upload all packages
    for p in packages.iter() {
        if let Err(e) = p.upload(&tmp_dir, &auth, &channel, force) {
            error!("Failed to upload {:?}, err: {}", p, e);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert!("bad input".parse::<Package>().is_err());
        assert_eq!("* package (target)".parse::<Package>().unwrap(),
                   Package::new(String::from("package"), String::from("target")));
        assert_eq!("* core/hab-backline/0.84.33/20190819190300 (x86_64-linux)".parse::<Package>()
                                                                              .unwrap(),
                   Package::new(String::from("core/hab-backline/0.84.33/20190819190300"),
                                String::from("x86_64-linux")));
    }

    #[test]
    fn test_package_url() {
        let p = Package::new(String::from("package"), String::from("target"));
        assert!(p.url("bad url").is_err());
        assert_eq!(p.url("https://test.url").unwrap().as_str(),
                   "https://test.url/v1/depot/pkgs/package/download?target=target");
        let p = Package::new(String::from("core/hab-backline/0.84.33/20190819190300"),
                             String::from("x86_64-linux"));
        assert_eq!(p.url(ACCEPTANCE_URL).unwrap().as_str(),
                   format!("{}/{}core/hab-backline/0.84.33/20190819190300/{}?target=x86_64-linux",
                           ACCEPTANCE_URL, PACKAGES_ENDPOINT, DOWNLOAD_ENDPOINT));
    }

    #[test]
    fn test_package_filename() {
        let p = Package::new(String::from("package"), String::from("target"));
        assert_eq!(p.filename().as_os_str(), "package-target.hart");
        let p = Package::new(String::from("core/hab-backline/0.84.33/20190819190300"),
                             String::from("x86_64-linux"));
        assert_eq!(p.filename().as_os_str(),
                   "core-hab-backline-0.84.33-20190819190300-x86_64-linux.hart");
    }
}
