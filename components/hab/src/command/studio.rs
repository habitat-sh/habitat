// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::env;
use std::ffi::OsString;

use hcore::env as henv;

use config;
use error::Result;

pub fn start(args: Vec<OsString>) -> Result<()> {
    // If the `$HAB_ORIGIN` environment variable is not present, then see if a default is set in
    // the CLI config. If so, set it as the `$HAB_ORIGIN` environment variable for the `hab-studio`
    // or `docker` execv call.
    if henv::var("HAB_ORIGIN").is_err() {
        let config = try!(config::load());
        if let Some(default_origin) = config.origin {
            debug!("Setting default origin {} via CLI config", &default_origin);
            env::set_var("HAB_ORIGIN", default_origin);
        }
    }
    inner::start(args)
}

#[cfg(target_os = "linux")]
mod inner {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::str::FromStr;

    use hcore::crypto::{init, default_cache_key_path};
    use hcore::env as henv;
    use hcore::fs::find_command;
    use hcore::package::PackageIdent;

    use error::{Error, Result};
    use exec;

    const STUDIO_CMD: &'static str = "hab-studio";
    const STUDIO_CMD_ENVVAR: &'static str = "HAB_STUDIO_BINARY";
    const STUDIO_PACKAGE_IDENT: &'static str = "core/hab-studio";

    pub fn start(args: Vec<OsString>) -> Result<()> {
        let command = match henv::var(STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let ident = try!(PackageIdent::from_str(STUDIO_PACKAGE_IDENT));
                try!(exec::command_from_pkg(STUDIO_CMD, &ident, &default_cache_key_path(None), 0))
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            try!(exec::exec_command(cmd, args));
        } else {
            return Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()));
        }
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use std::env;
    use std::ffi::OsString;

    use hcore::crypto::default_cache_key_path;
    use hcore::env as henv;
    use hcore::fs::{CACHE_KEY_PATH, find_command};

    use error::{Error, Result};
    use exec;

    const DOCKER_CMD: &'static str = "docker";
    const DOCKER_CMD_ENVVAR: &'static str = "HAB_DOCKER_BINARY";

    const DOCKER_IMAGE: &'static str = "habitat-docker-registry.bintray.io/studio";
    const DOCKER_IMAGE_ENVVAR: &'static str = "HAB_DOCKER_STUDIO_IMAGE";

    pub fn start(args: Vec<OsString>) -> Result<()> {
        let docker = henv::var(DOCKER_CMD_ENVVAR).unwrap_or(DOCKER_CMD.to_string());
        let image = henv::var(DOCKER_IMAGE_ENVVAR).unwrap_or(DOCKER_IMAGE.to_string());

        let cmd = match find_command(&docker) {
            Some(cmd) => cmd,
            None => return Err(Error::ExecCommandNotFound(docker.to_string())),
        };

        let mut cmd_args: Vec<OsString> = vec!["run".into(),
                                               "--rm".into(),
                                               "--tty".into(),
                                               "--interactive".into(),
                                               "--privileged".into()];
        let env_vars = vec!["HAB_DEPOT_URL", "HAB_ORIGIN", "http_proxy", "https_proxy"];
        for var in env_vars {
            if let Ok(val) = henv::var(var) {
                debug!("Propagating environment variable into container: {}={}",
                       var,
                       val);
                cmd_args.push("--env".into());
                cmd_args.push(format!("{}={}", var, val).into());
            }
        }
        cmd_args.push("--volume".into());
        cmd_args.push("/var/run/docker.sock:/var/run/docker.sock".into());
        cmd_args.push("--volume".into());
        cmd_args.push(format!("{}:/{}",
                              default_cache_key_path(None).to_string_lossy(),
                              CACHE_KEY_PATH)
            .into());
        cmd_args.push("--volume".into());
        cmd_args.push(format!("{}:/src", env::current_dir().unwrap().to_string_lossy()).into());
        cmd_args.push(image.into());
        cmd_args.extend_from_slice(args.as_slice());

        for var in vec!["http_proxy", "https_proxy"] {
            if let Ok(_) = henv::var(var) {
                debug!("Unsetting proxy environment variable '{}' before calling `{}'",
                       var,
                       docker);
                env::remove_var(var);
            }
        }

        exec::exec_command(cmd, cmd_args)
    }
}
