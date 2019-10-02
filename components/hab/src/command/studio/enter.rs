use std::{env,
          ffi::OsString,
          fs as stdfs,
          path::{Path,
                 PathBuf}};

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::{crypto::CACHE_KEY_PATH_ENV_VAR,
                    env as henv,
                    fs}};

use crate::{config,
            error::{Error,
                    Result},
            BLDR_URL_ENVVAR,
            ORIGIN_ENVVAR};

use habitat_core::AUTH_TOKEN_ENVVAR;
use same_file::is_same_file;

pub const ARTIFACT_PATH_ENVVAR: &str = "ARTIFACT_PATH";
pub const CERT_PATH_ENVVAR: &str = "CERT_PATH";
pub const SSL_CERT_FILE_ENVVAR: &str = "SSL_CERT_FILE";

const STUDIO_CMD: &str = "hab-studio";
const STUDIO_CMD_ENVVAR: &str = "HAB_STUDIO_BINARY";
const STUDIO_PACKAGE_IDENT: &str = "core/hab-studio";

#[derive(Clone, Copy)]
enum Sensitivity {
    PrintValue,
    NoPrintValue,
}

fn set_env_var_from_config(env_var: &str, config_val: Option<String>, sensitive: Sensitivity) {
    if henv::var(env_var).is_err() {
        if let Some(val) = config_val {
            match sensitive {
                Sensitivity::NoPrintValue => {
                    debug!("Setting {}=REDACTED (sensitive) via config file", env_var)
                }
                Sensitivity::PrintValue => debug!("Setting {}={} via config file", env_var, val),
            }
            env::set_var(env_var, val);
        }
    }
}

fn cache_ssl_cert_file(cert_file: &str, cert_cache_dir: &Path) -> Result<()> {
    let cert_path = Path::new(&cert_file);

    let cert_filename = match cert_path.file_name() {
        Some(cert_filename) => cert_filename,
        None => return Err(Error::CacheSslCertError(format!("{:?} is not a file", &cert_file))),
    };
    let cache_file = cert_cache_dir.join(&cert_filename);

    if cache_file.exists() && cert_path.exists() && is_same_file(&cache_file, &cert_path)? {
        return Err(Error::CacheSslCertError("Source and destination certificate are the same \
                                             file"
                                                  .to_string()));
    }

    debug!("Caching SSL_CERT_FILE {:?} => {:?}", cert_file, cache_file);
    stdfs::copy(cert_file, &cache_file)?;

    Ok(())
}

pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    let config = config::load()?;

    set_env_var_from_config(AUTH_TOKEN_ENVVAR,
                            config.auth_token,
                            Sensitivity::NoPrintValue);
    set_env_var_from_config(BLDR_URL_ENVVAR, config.bldr_url, Sensitivity::PrintValue);
    set_env_var_from_config(ORIGIN_ENVVAR, config.origin, Sensitivity::PrintValue);

    if config.ctl_secret.is_some() {
        ui.warn("Your Supervisor CtlGateway secret is not being copied to the Studio \
                 environment because the Studio's Supervisor is local. If you wish to contact a \
                 remote Supervisor from the Studio, please set the HAB_CTL_SECRET variable with \
                 your secret.")?;
    }

    if henv::var(CACHE_KEY_PATH_ENV_VAR).is_err() {
        let path = fs::cache_key_path(None::<&str>);
        debug!("Setting {}={}", CACHE_KEY_PATH_ENV_VAR, path.display());
        env::set_var(CACHE_KEY_PATH_ENV_VAR, &path);
    };

    let artifact_path = match henv::var(ARTIFACT_PATH_ENVVAR) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let path = fs::cache_artifact_path(None::<&str>);
            debug!("Setting {}={}", ARTIFACT_PATH_ENVVAR, path.display());
            env::set_var(ARTIFACT_PATH_ENVVAR, &path);
            path
        }
    };
    if !artifact_path.is_dir() {
        debug!("Creating artifact_path at: {}", artifact_path.display());
        stdfs::create_dir_all(&artifact_path)?;
    }

    let ssl_path = match henv::var(CERT_PATH_ENVVAR) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let path = fs::cache_ssl_path(None::<&str>);
            debug!("Setting {}={}", CERT_PATH_ENVVAR, path.display());
            env::set_var(CERT_PATH_ENVVAR, &path);
            path
        }
    };
    if !ssl_path.is_dir() {
        debug!("Creating ssl_path at: {}", ssl_path.display());
        stdfs::create_dir_all(&ssl_path)?;
    }

    if let Ok(ssl_cert_file) = env::var(SSL_CERT_FILE_ENVVAR) {
        if let Err(err) = cache_ssl_cert_file(&ssl_cert_file, &ssl_path) {
            warn!("Unable to cache SSL_CERT_FILE: {}", err);
        }
    }

    inner::start(ui, args)
}

#[cfg(target_os = "linux")]
mod inner {
    use crate::{command::studio::docker,
                common::ui::{UIWriter,
                             UI},
                error::{Error,
                        Result},
                exec,
                hcore::{crypto::init,
                        env as henv,
                        fs::{am_i_root,
                             find_command},
                        os::process,
                        package::{PackageIdent,
                                  PackageInstall},
                        users::linux as group},
                VERSION};
    use std::{env,
              ffi::OsString,
              path::PathBuf,
              str::FromStr};

    const SUDO_CMD: &str = "sudo";

    pub fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
        rerun_with_sudo_if_needed(ui, &args)?;
        if is_docker_studio(&args) {
            docker::start_docker_studio(ui, args)
        } else {
            let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
                Ok(command) => PathBuf::from(command),
                Err(_) => {
                    init();
                    let version: Vec<&str> = VERSION.split('/').collect();
                    let ident = PackageIdent::from_str(&format!("{}/{}",
                                                                super::STUDIO_PACKAGE_IDENT,
                                                                version[0]))?;
                    let command = exec::command_from_min_pkg(ui, super::STUDIO_CMD, &ident)?;
                    // This is a duplicate of the code in `hab pkg exec` and
                    // should be refactored as part of or after:
                    // https://github.com/habitat-sh/habitat/issues/6633
                    // https://github.com/habitat-sh/habitat/issues/6634
                    let pkg_install = PackageInstall::load(&ident, None)?;
                    let cmd_env = pkg_install.environment_for_command()?;
                    for (key, value) in cmd_env.into_iter() {
                        debug!("Setting: {}='{}'", key, value);
                        env::set_var(key, value);
                    }

                    let mut display_args = super::STUDIO_CMD.to_string();
                    for arg in args {
                        display_args.push(' ');
                        display_args.push_str(arg.to_string_lossy().as_ref());
                    }
                    debug!("Running: {}", display_args);

                    command
                }
            };

            if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
                process::become_command(cmd, args)?;
                Ok(())
            } else {
                Err(Error::ExecCommandNotFound(command))
            }
        }
    }

    fn is_docker_studio(args: &[OsString]) -> bool {
        if cfg!(not(target_os = "linux")) {
            return false;
        }

        for arg in args.iter() {
            let str_arg = arg.to_string_lossy();
            if str_arg == "-D" {
                return true;
            }
        }

        false
    }

    fn has_docker_group() -> bool {
        let current_user = group::get_current_username().unwrap();
        let docker_members = group::get_members_by_groupname("docker");
        docker_members.map_or(false, |d| d.contains(&current_user))
    }

    fn rerun_with_sudo_if_needed(ui: &mut UI, args: &[OsString]) -> Result<()> {
        // If I have root permissions or if I am executing a docker studio
        // and have the appropriate group - early return, we are done.
        if am_i_root() || (is_docker_studio(args) && has_docker_group()) {
            return Ok(());
        }

        // Otherwise we will try to re-run this program using `sudo`
        match find_command(SUDO_CMD) {
            Some(sudo_prog) => {
                let mut args: Vec<OsString> = vec!["-p".into(),
                                                   "[sudo hab-studio] password for %u: ".into(),
                                                   "-E".into(),];
                args.append(&mut env::args_os().collect());
                process::become_command(sudo_prog, &args)?;
                Ok(())
            }
            None => {
                ui.warn(format!("Could not find the `{}' command, is it in your PATH?",
                                SUDO_CMD))?;
                ui.warn("Running Habitat Studio requires root or administrator privileges. \
                         Please retry this command as a super user or use a privilege-granting \
                         facility such as sudo.")?;
                ui.br()?;
                Err(Error::RootRequired)
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use crate::{command::studio::docker,
                common::ui::UI,
                error::{Error,
                        Result},
                exec,
                hcore::{crypto::init,
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent},
                VERSION};
    use std::{ffi::OsString,
              path::PathBuf,
              str::FromStr};

    pub fn start(_ui: &mut UI, args: &[OsString]) -> Result<()> {
        if is_windows_studio(&args) {
            start_windows_studio(_ui, args)
        } else {
            docker::start_docker_studio(_ui, args)
        }
    }

    pub fn start_windows_studio(ui: &mut UI, args: &[OsString]) -> Result<()> {
        let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split('/').collect();
                let ident = PackageIdent::from_str(&format!("{}/{}",
                                                            super::STUDIO_PACKAGE_IDENT,
                                                            version[0]))?;
                exec::command_from_min_pkg(ui, super::STUDIO_CMD, &ident)?
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            process::become_command(cmd, args)?;
        } else {
            return Err(Error::ExecCommandNotFound(command));
        }
        Ok(())
    }

    fn is_windows_studio(args: &[OsString]) -> bool {
        if cfg!(not(target_os = "windows")) {
            return false;
        }

        for arg in args.iter() {
            let str_arg = arg.to_string_lossy();
            if str_arg == "-D" {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::cache_ssl_cert_file;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn cache_ssl_cert_file_caches_file() -> std::io::Result<()> {
        let cert_name = "ssl-test-cert.pem";
        let cert_cache_dir = TempDir::new()?.into_path();
        let ssl_cert_dir = TempDir::new()?;
        let ssl_cert_filepath = ssl_cert_dir.path().join(cert_name);
        File::create(&ssl_cert_filepath)?;

        cache_ssl_cert_file(&ssl_cert_filepath.to_str().unwrap(), &cert_cache_dir).unwrap();
        assert!(cert_cache_dir.join(cert_name).exists());

        Ok(())
    }

    #[test]
    fn cache_ssl_cert_file_replaces_already_cached() -> std::io::Result<()> {
        let cert_name = "ssl-test-cert.pem";

        let cert_cache_dir = TempDir::new()?.into_path();
        let cached_cert = cert_cache_dir.join(&cert_name);
        File::create(&cached_cert)?;

        let ssl_cert_dir = TempDir::new()?;
        let ssl_cert_filepath = ssl_cert_dir.path().join(&cert_name);
        std::fs::write(&ssl_cert_filepath, "new cert from environment")?;

        cache_ssl_cert_file(&ssl_cert_filepath.to_str().unwrap(), &cert_cache_dir).unwrap();

        let contents = std::fs::read_to_string(&cached_cert)?;

        assert_eq!(contents, "new cert from environment");

        Ok(())
    }
    #[test]
    fn cache_ssl_cert_file_invalid_file() -> std::io::Result<()> {
        let cert_cache_dir = TempDir::new()?.into_path();

        let non_existant_file_name = "i_shouldnt_exist";
        let non_existant_file = TempDir::new()?.path().join(non_existant_file_name);

        assert!(cache_ssl_cert_file(non_existant_file.to_str().unwrap(), &cert_cache_dir).is_err());

        Ok(())
    }

    #[test]
    fn cache_ssl_cert_file_cert_file_is_dir() -> std::io::Result<()> {
        let cert_cache_dir = TempDir::new()?.into_path();
        let ssl_cert_dir = TempDir::new()?.into_path();

        assert!(cache_ssl_cert_file(ssl_cert_dir.to_str().unwrap(), &cert_cache_dir).is_err());

        Ok(())
    }

    #[test]
    fn cache_ssl_cert_file_cert_file_is_cached_file() -> std::io::Result<()> {
        let cached_cert_dir = TempDir::new()?.into_path();
        let cached_cert = cached_cert_dir.join("ssl-cert-file.pem");
        File::create(&cached_cert)?;

        assert!(cache_ssl_cert_file(cached_cert.to_str().unwrap(), &cached_cert_dir).is_err());

        Ok(())
    }

    #[test]
    fn cache_ssl_cert_file_is_empty_string() -> std::io::Result<()> {
        let cached_cert_dir = TempDir::new()?.into_path();

        assert!(cache_ssl_cert_file("", &cached_cert_dir).is_err());

        Ok(())
    }
}
