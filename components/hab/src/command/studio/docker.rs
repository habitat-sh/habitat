use crate::{command::studio::enter::{ARTIFACT_PATH_ENVVAR,
                                     CERT_PATH_ENVVAR,
                                     SSL_CERT_FILE_ENVVAR},
            common::ui::UI,
            error::{Error,
                    Result},
            hcore::{crypto::CACHE_KEY_PATH_ENV_VAR,
                    env as henv,
                    fs::{cache_key_path,
                         CACHE_ARTIFACT_PATH,
                         CACHE_KEY_PATH,
                         CACHE_SSL_PATH},
                    os::process,
                    package::target,
                    util::docker},
            license,
            VERSION};
use atty;
use std::{env,
          ffi::{OsStr,
                OsString},
          path::{Path,
                 PathBuf},
          process::{Command,
                    Stdio}};

const DOCKER_IMAGE: &str = "habitat/default-studio";
const DOCKER_WINDOWS_IMAGE: &str = "habitat/win-studio";
const DOCKER_IMAGE_ENVVAR: &str = "HAB_DOCKER_STUDIO_IMAGE";
const DOCKER_OPTS_ENVVAR: &str = "HAB_DOCKER_OPTS";
const DOCKER_SOCKET: &str = "/var/run/docker.sock";
const HAB_STUDIO_SECRET: &str = "HAB_STUDIO_SECRET_";

pub fn start_docker_studio(_ui: &mut UI, args: &[OsString]) -> Result<()> {
    let mut args = args.to_vec();
    if args.get(0) == Some(&OsString::from("rm")) {
        return Err(Error::CannotRemoveDockerStudio);
    }

    let docker_cmd = docker::command_path()?;
    let using_windows_containers = is_serving_windows_containers(&docker_cmd);
    let image = image_identifier_for_active_target(using_windows_containers)?;

    if is_image_present(&docker_cmd, &image) {
        debug!("Found Studio Docker image locally.");
    } else {
        debug!("Failed to find Studio Docker image locally.");
        pull_image(&docker_cmd, &image)?;
    }

    let mnt_prefix = if using_windows_containers { "c:" } else { "" };

    let local_cache_key_path = match henv::var(CACHE_KEY_PATH_ENV_VAR) {
        Ok(val) => PathBuf::from(val),
        Err(_) => cache_key_path(None::<PathBuf>),
    };
    if !local_cache_key_path.exists() {
        return Err(Error::FileNotFound(format!("{}\nRun `hab setup` to \
                                                create an origin or use \
                                                `hab origin key` to \
                                                configure your keys.",
                                               local_cache_key_path.display())));
    }

    let mut volumes = vec![format!("{}:{}{}",
                                   env::current_dir().unwrap().to_string_lossy(),
                                   mnt_prefix,
                                   "/src"),
                           format!("{}:{}/{}",
                                   local_cache_key_path.display(),
                                   mnt_prefix,
                                   CACHE_KEY_PATH),];
    if let Ok(cache_artifact_path) = henv::var(ARTIFACT_PATH_ENVVAR) {
        // Don't use Path::join here as "\" can cause problems in Docker mounts
        volumes.push(format!("{}:{}/{}",
                             cache_artifact_path, mnt_prefix, CACHE_ARTIFACT_PATH));
    }
    if let Ok(cache_ssl_path) = henv::var(CERT_PATH_ENVVAR) {
        // Don't use Path::join here as "\" can cause problems in Docker mounts
        volumes.push(format!("{}:{}/{}", cache_ssl_path, mnt_prefix, CACHE_SSL_PATH));
    }
    if !using_windows_containers
       && (Path::new(DOCKER_SOCKET).exists() || cfg!(target_os = "windows"))
    {
        volumes.push(format!("{}:{}", DOCKER_SOCKET, DOCKER_SOCKET));
    }

    let mut env_vars = vec![String::from("DEBUG"),
                            String::from("DO_CHECK"),
                            String::from("HAB_AUTH_TOKEN"),
                            String::from("HAB_BLDR_URL"),
                            String::from("HAB_BLDR_CHANNEL"),
                            String::from("HAB_NOCOLORING"),
                            String::from("HAB_LICENSE"),
                            String::from("HAB_ORIGIN"),
                            String::from("HAB_ORIGIN_KEYS"),
                            String::from("HAB_STUDIO_BACKLINE_PKG"),
                            String::from("HAB_STUDIO_NOSTUDIORC"),
                            String::from("HAB_STUDIO_SUP"),
                            String::from("HAB_UPDATE_STRATEGY_FREQUENCY_MS"),
                            String::from("http_proxy"),
                            String::from("https_proxy"),
                            String::from("RUST_LOG"),
                            String::from("SSL_CERT_FILE"),];

    for (key, _) in env::vars() {
        if key.starts_with(HAB_STUDIO_SECRET) {
            env_vars.push(key);
        }
    }

    // We need to strip out the -D if it exists to avoid
    // it getting passed to the sup on entering the studio
    let to_cull = OsString::from("-D");
    if let Some(index) = args.iter().position(|x| *x == to_cull) {
        args.remove(index);
    }

    // When a user sets SSL_CERT_FILE, we need to modify the absolute
    // path to the file to reflect the location of the file inside the studio
    update_ssl_cert_file_envvar(&mnt_prefix);

    // Windows containers do not use filesystem sharing for
    // local mounts
    if !using_windows_containers {
        check_mounts(&docker_cmd, volumes.iter(), &image)?;
    }
    run_container(docker_cmd,
                  &args,
                  volumes.iter(),
                  env_vars.iter(),
                  image,
                  using_windows_containers)
}

fn update_ssl_cert_file_envvar(mnt_prefix: &str) {
    if let Ok(ssl_cert_file) = env::var(SSL_CERT_FILE_ENVVAR) {
        if let Some(cert_file_name) = Path::new(&ssl_cert_file).file_name() {
            if let Some(cert_file_name) = cert_file_name.to_str() {
                // Don't use Path::join here in order to work around platform
                // differences with paths on Windows with linux containers enabled
                env::set_var(SSL_CERT_FILE_ENVVAR,
                             format!("{}/{}/{}", mnt_prefix, CACHE_SSL_PATH, cert_file_name));
            } else {
                warn!("Unable to format {:?} for use inside studio", ssl_cert_file);
            }
        } else {
            warn!("Invalid SSL_CERT_FILE value: {:?}", ssl_cert_file);
        }
    }
}

fn is_image_present(docker_cmd: &Path, image: &str) -> bool {
    let mut cmd = Command::new(docker_cmd);
    cmd.arg("images").arg(image).arg("-q");
    debug!("Running command: {:?}", cmd);
    let result = cmd.output().expect("Docker command failed to spawn");

    !String::from_utf8_lossy(&result.stdout).as_ref().is_empty()
}

fn is_serving_windows_containers(docker_cmd: &Path) -> bool {
    let mut cmd = Command::new(docker_cmd);
    cmd.arg("version").arg("--format='{{.Server.Os}}'");
    debug!("Running command: {:?}", cmd);
    let result = cmd.output().expect("Docker command failed to spawn");
    String::from_utf8_lossy(&result.stdout).contains("windows")
}

fn pull_image(docker_cmd: &Path, image: &str) -> Result<()> {
    let mut cmd = Command::new(docker_cmd);
    cmd.arg("pull")
       .arg(image)
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());
    debug!("Running command: {:?}", cmd);
    let result = cmd.spawn()
                    .expect("Docker command failed to spawn")
                    .wait_with_output()
                    .expect("Failed to wait on child process");

    if result.status.success() {
        debug!("Docker image '{}' is present locally.", image);
    } else {
        debug!("Pulling Docker image '{}' failed with exit code: {:?}",
               image, result.status);

        let err_output = String::from_utf8_lossy(&result.stderr);

        if err_output.contains("image") && err_output.contains("not found") {
            return Err(Error::DockerImageNotFound(image.to_string()));
        } else if err_output.contains("Cannot connect to the Docker daemon") {
            return Err(Error::DockerDaemonDown);
        } else {
            return Err(Error::DockerNetworkDown(image.to_string()));
        }
    }

    Ok(())
}

/// Checks whether or not the volume mounts are working.
///
/// We need to ensure that filesystem sharing has been enabled, otherwise the user will be
/// greeted with a horrible error message that's difficult to make sense of. To mitigate this,
/// we check the studio version. This will cause Docker to go through the mounting steps, so we
/// can watch stderr for failure, but has the advantage of not requiring a TTY.
fn check_mounts<I, S>(docker_cmd: &Path, volumes: I, image: &str) -> Result<()>
    where I: IntoIterator<Item = S>,
          S: AsRef<OsStr>
{
    let mut cmd_args: Vec<OsString> = vec!["run".into(), "--rm".into()];

    for vol in volumes {
        cmd_args.push("--volume".into());
        cmd_args.push(vol.as_ref().into());
    }
    cmd_args.push(image.into());
    cmd_args.push("-V".into());
    let version_output = Command::new(docker_cmd).args(&cmd_args)
                                                 .output()
                                                 .expect("docker failed to start");

    let stderr = String::from_utf8(version_output.stderr).unwrap();
    if !stderr.is_empty()
       && (stderr.as_str().contains("Mounts denied")
           || stderr.as_str().contains("drive is not shared"))
    {
        return Err(Error::DockerFileSharingNotEnabled);
    }
    Ok(())
}

fn run_container<I, J, S, T>(docker_cmd: PathBuf,
                             args: &[OsString],
                             volumes: I,
                             env_vars: J,
                             image: String,
                             using_windows_containers: bool)
                             -> Result<()>
    where I: IntoIterator<Item = S>,
          J: IntoIterator<Item = T>,
          S: AsRef<OsStr>,
          T: AsRef<str>
{
    let mut cmd_args: Vec<OsString> = vec!["run".into(), "--rm".into()];

    if !using_windows_containers {
        cmd_args.push("--privileged".into());
    }

    if atty::is(atty::Stream::Stderr) || atty::is(atty::Stream::Stdout) {
        cmd_args.push("--tty".into());
        cmd_args.push("--interactive".into());
    }

    if let Ok(opts) = henv::var(DOCKER_OPTS_ENVVAR) {
        let opts = opts
            .split_whitespace()
            .map(|v| v.into())
            // Ensure we're not passing something like `--tty` again here.
            .filter(|v| !cmd_args.contains(v))
            .collect::<Vec<_>>();

        if !opts.is_empty() {
            debug!("Adding extra Docker options from {} = {:?}",
                   DOCKER_OPTS_ENVVAR, opts);
            cmd_args.extend_from_slice(opts.as_slice());
        }
    }

    for var in env_vars {
        if let Ok(val) = henv::var(var.as_ref()) {
            debug!("Setting container env var: {:?}='{}'", var.as_ref(), val);
            cmd_args.push("--env".into());
            cmd_args.push(format!("{}={}", var.as_ref(), val).into());
        } else if var.as_ref() == "HAB_LICENSE" && license::license_exists() {
            debug!("Hab license already accepted. Setting container env var: \
                    HAB_LICENSE=accept-no-persist");
            cmd_args.push("--env".into());
            cmd_args.push("HAB_LICENSE=accept-no-persist".to_string().into());
        }
    }

    for vol in volumes {
        cmd_args.push("--volume".into());
        cmd_args.push(vol.as_ref().into());
    }

    cmd_args.push(image.into());
    cmd_args.extend_from_slice(args);

    if using_windows_containers {
        cmd_args.push("-n".into());
        cmd_args.push("-o".into());
        cmd_args.push("c:/".into());
    }

    unset_proxy_env_vars();
    process::become_command(docker_cmd, &cmd_args)?;
    Ok(())
}

fn unset_proxy_env_vars() {
    for var in &["http_proxy", "https_proxy"] {
        if henv::var(var).is_ok() {
            debug!("Unsetting process environment variable '{}'", var);
            env::remove_var(var);
        }
    }
}

fn image_identifier_for_active_target(using_windows_containers: bool) -> Result<String> {
    let windows_base_tag = if using_windows_containers {
        Some(docker::default_base_tag_for_host()?)
    } else {
        None
    };
    Ok(image_identifier(windows_base_tag, target::PackageTarget::active_target()))
}

/// Returns the Docker Studio image with tag for the desired version which corresponds to the
/// same version (minus release) as this program.
fn image_identifier(windows_base_tag: Option<&str>, target: target::PackageTarget) -> String {
    let version: Vec<&str> = VERSION.split('/').collect();
    let (img, tag) = if let Some(t) = windows_base_tag {
        (DOCKER_WINDOWS_IMAGE, format!("{}-{}", t, version[0]))
    } else {
        (DOCKER_IMAGE, version[0].to_string())
    };
    let studio_target = studio_target(windows_base_tag.is_some(), target);

    henv::var(DOCKER_IMAGE_ENVVAR).unwrap_or_else(|_| format!("{}-{}:{}", img, studio_target, tag))
}

fn studio_target(windows: bool, target: target::PackageTarget) -> target::PackageTarget {
    if windows {
        #[cfg(feature = "supported_targets")]
        return target::X86_64_WINDOWS;
    }
    match target {
        #[cfg(feature = "supported_targets")]
        target::X86_64_DARWIN => target::X86_64_LINUX,
        #[cfg(feature = "supported_targets")]
        target::X86_64_LINUX => target::X86_64_LINUX,
        #[cfg(feature = "supported_targets")]
        target::X86_64_LINUX_KERNEL2 => target::X86_64_LINUX_KERNEL2,
        #[cfg(feature = "supported_targets")]
        target::X86_64_WINDOWS => target::X86_64_LINUX,
        #[cfg(feature = "aarch64-linux")]
        target::AARCH64_LINUX => panic!("{} is not supported", target::AARCH64_LINUX),
        // This is only needed for the case that we have no target enabled. In that case, we get a
        // non-exhaustive patterns error because the match statement is empty.
        #[cfg(not(any(feature = "supported_targets", feature = "aarch64-linux")))]
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{image_identifier,
                update_ssl_cert_file_envvar,
                DOCKER_IMAGE,
                DOCKER_WINDOWS_IMAGE};
    use crate::VERSION;

    use crate::{command::studio::enter::SSL_CERT_FILE_ENVVAR,
                hcore::{fs::CACHE_SSL_PATH,
                        package::target}};

    use lazy_static::lazy_static;
    habitat_common::locked_env_var!(SSL_CERT_FILE, lock_ssl_cert_file_env_var);

    #[test]
    #[cfg(feature = "supported_targets")]
    fn retrieve_supported_image_identifier() {
        assert_eq!(image_identifier(None, target::X86_64_DARWIN),
                   format!("{}-{}:{}", DOCKER_IMAGE, "x86_64-linux", VERSION));
        assert_eq!(image_identifier(None, target::X86_64_LINUX),
                   format!("{}-{}:{}", DOCKER_IMAGE, "x86_64-linux", VERSION));
        assert_eq!(image_identifier(None, target::X86_64_LINUX_KERNEL2),
                   format!("{}-{}:{}", DOCKER_IMAGE, "x86_64-linux-kernel2", VERSION));
        assert_eq!(image_identifier(None, target::X86_64_WINDOWS),
                   format!("{}-{}:{}", DOCKER_IMAGE, "x86_64-linux", VERSION));
        assert_eq!(image_identifier(Some("ltsc2016"), target::X86_64_WINDOWS),
                   format!("{}-{}:{}-{}",
                           DOCKER_WINDOWS_IMAGE, "x86_64-windows", "ltsc2016", VERSION));
        assert_eq!(image_identifier(Some("ltsc2016"), target::X86_64_LINUX),
                   format!("{}-{}:{}-{}",
                           DOCKER_WINDOWS_IMAGE, "x86_64-windows", "ltsc2016", VERSION));
    }

    #[should_panic]
    #[cfg(feature = "aarch64-linux")]
    fn retrieve_aarch64_image_identifier() { image_identifier(None, target::AARCH64_LINUX); }

    #[test]
    fn update_ssl_cert_file_envvar_not_set() {
        let mnt_prefix = "";
        let env_var = lock_ssl_cert_file_env_var();
        env_var.unset();

        update_ssl_cert_file_envvar(mnt_prefix);
        assert!(std::env::var(SSL_CERT_FILE_ENVVAR).is_err());
    }

    #[test]
    fn update_ssl_cert_file_envvar_has_value() {
        let mnt_prefix = "/some/prefix";
        let key_name = "ssl-test-cert.pem";

        let env_var = lock_ssl_cert_file_env_var();
        env_var.set(format!("/path/to/{}", key_name));

        update_ssl_cert_file_envvar(mnt_prefix);

        // Don't use Path::join here because we format! the path above,
        // in order to work around platform differences with paths on
        // windows with linux containers enabled
        let internal_cert_path = format!("{}/{}/{}", mnt_prefix, CACHE_SSL_PATH, key_name);

        assert_eq!(std::env::var(SSL_CERT_FILE_ENVVAR), Ok(internal_cert_path));
    }
}
