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

use std::path::Path;
use std::process::Command;

use serde_json::{self, Value as JsonValue};

use error::{Error, Result};
use runner::docker::DockerExporterSpec;
use runner::workspace::Workspace;

// TODO fn: The horror... well, it's not that bad. There isn't a quick win for recursive chown'ing
// a path, so we'll use the `chown` binary as provided by busybox and guarenteed by the Supervisor.
// I'm wincing here right now, honest.
pub fn chown_recursive<P: AsRef<Path>>(path: P, uid: u32, gid: u32) -> Result<()> {
    let mut cmd = Command::new("chown");
    cmd.arg("-R"); // Recursively apply ownership
    cmd.arg(format!("{}:{}", uid, gid));
    cmd.arg(path.as_ref());
    debug!("building chown command, cmd={:?}", &cmd);

    debug!("spawning chown command");
    let mut child = cmd.spawn().map_err(|e| {
        Error::Chown(path.as_ref().to_path_buf(), uid, gid, e)
    })?;
    let exit_status = child.wait().map_err(Error::ChownWait)?;
    debug!("completed chown command, status={:?}", exit_status);
    Ok(())
}

// TODO fn: Here's a sure sign you want more data integrity throughout the system. Most of this
// validation should happen way, way upstream, but that's future refactoring work. Also, if the
// payloads are known structures then serde derive and structs will make better work of things.
// For the moment though, this is validation by hand. Welcome to our cave of shame, folks.

/// Validate integration data in job.
pub fn validate_integrations(workspace: &Workspace) -> Result<()> {
    // Validate project integration
    {
        let prj_integrations = workspace.job.get_project_integrations();
        if prj_integrations.is_empty() {
            // No project integrations, that's cool, we're done!
            return Ok(());
        }

        let prj_integration = prj_integrations.first().unwrap();

        // TODO fn: use a struct and serde to do heavy lifting
        let opts: JsonValue = match serde_json::from_str(prj_integration.get_body()) {
            Ok(json) => json,
            Err(err) => {
                return Err(Error::InvalidIntegrations(format!(
                    "project integration body does not deserialize as JSON: {:?}",
                    err
                )))
            }
        };
        // Required keys with string values
        for str_key in vec!["docker_hub_repo_name"].iter() {
            match opts.get(str_key) {
                Some(val) => {
                    if val.is_string() {
                        if val.as_str().unwrap().is_empty() {
                            return Err(Error::InvalidIntegrations(format!(
                                "project integration {} value must be a nonempty string",
                                str_key
                            )));
                        }
                    } else {
                        return Err(Error::InvalidIntegrations(format!(
                            "project integration {} value must be a string",
                            str_key
                        )));
                    }
                }
                None => {
                    return Err(Error::InvalidIntegrations(
                        format!("project integration {} missing", str_key),
                    ));
                }
            }
        }
        // Required keys with boolean values
        for bool_key in vec!["latest_tag", "version_tag", "version_release_tag"].iter() {
            match opts.get(bool_key) {
                Some(val) => {
                    if !val.is_boolean() {
                        return Err(Error::InvalidIntegrations(format!(
                            "project integration {} value must be a bool",
                            bool_key
                        )));
                    }
                }
                None => {
                    return Err(Error::InvalidIntegrations(
                        format!("project integration {} missing", bool_key),
                    ));
                }
            }
        }
        // Optional keys with string values
        if let Some(val) = opts.get("custom_tag") {
            if !val.is_string() {
                return Err(Error::InvalidIntegrations(format!(
                    "project integration custom_tag value must be a string"
                )));
            }
        }
    }
    // Validate origin integration
    {
        let org_integrations = workspace.job.get_integrations();
        if org_integrations.is_empty() {
            return Err(Error::InvalidIntegrations(format!(
                "missing Docker credentials from origin integrations"
            )));
        }
        let org_integration = org_integrations.first().unwrap();

        // TODO fn: use a struct and serde to do heavy lifting
        let creds: JsonValue = match serde_json::from_str(org_integration.get_body()) {
            Ok(json) => json,
            Err(err) => {
                return Err(Error::InvalidIntegrations(format!(
                    "origin integration body does not deserialize as JSON: {:?}",
                    err
                )))
            }
        };
        // Required keys with string values
        for str_key in vec!["username", "password"].iter() {
            match creds.get(str_key) {
                Some(s) => {
                    if s.is_string() {
                        if s.as_str().unwrap().is_empty() {
                            return Err(Error::InvalidIntegrations(format!(
                                "origin integration {} value must be a nonempty string",
                                str_key
                            )));
                        }
                    } else {
                        return Err(Error::InvalidIntegrations(format!(
                            "origin integration {} value must be a string",
                            str_key
                        )));
                    }
                }
                None => {
                    return Err(Error::InvalidIntegrations(
                        format!("origin integration {} missing", str_key),
                    ));
                }
            }
        }
    }
    debug!("validated integrations");
    Ok(())
}


/// Builds the Docker exporter details from the origin and project integrations.
pub fn docker_exporter_spec(workspace: &Workspace) -> DockerExporterSpec {
    // TODO fn: Using this value struct was done to keep the validation, JSON parsing, and
    // craziness in one place and out of the Docker-specific code. Oi, this is embarrassing.

    // Note: There are a lot of `.expect()` calls in this function. That is because the
    // integrations data is assumed to have been already validated via `validate_integrations()`
    // above. As a result, Any panics that occur are most likely due to programmer error and not
    // input validation.

    let origin_integration = workspace.job.get_integrations().first().expect(
        "Origin integrations must not be empty",
    );

    let creds: JsonValue = serde_json::from_str(origin_integration.get_body()).expect(
        "Origin integrations body must be JSON",
    );

    let opts: JsonValue = serde_json::from_str(
        workspace
            .job
            .get_project_integrations()
            .first()
            .expect("Project integrations must not be empty")
            .get_body(),
    ).expect("Project integrations body must be JSON");

    let custom_tag = get_optional_args(&opts, String::from("custom_tag"));
    let registry_url = get_optional_args(&creds, String::from("registry_url"));
    let registry_type = origin_integration.get_integration().to_string();

    DockerExporterSpec {
        username: creds
            .get("username")
            .expect("username key is present")
            .as_str()
            .expect("username value is a string")
            .to_string(),
        password: creds
            .get("password")
            .expect("password key is present")
            .as_str()
            .expect("password value is a string")
            .to_string(),
        registry_type: registry_type,
        registry_url: registry_url,
        docker_hub_repo_name: opts.get("docker_hub_repo_name")
            .expect("docker_hub_repo_name key is present")
            .as_str()
            .expect("docker_hub_repo_name value is a string")
            .to_string(),
        latest_tag: opts.get("latest_tag")
            .expect("latest_tag key is present")
            .as_bool()
            .expect("latest_tag value is a bool"),
        version_tag: opts.get("version_tag")
            .expect("version_tag key is present")
            .as_bool()
            .expect("version_tag value is a bool"),
        version_release_tag: opts.get("version_release_tag")
            .expect("version_release_tag key is present")
            .as_bool()
            .expect("version_release_tag value is a bool"),
        custom_tag: custom_tag,
    }
}

fn get_optional_args(opts: &JsonValue, arg: String) -> Option<String> {
    match opts.get(arg) {
        Some(key) => {
            let key = key.as_str().unwrap_or("");
            if key.is_empty() {
                None
            } else {
                Some(key.to_string())
            }
        }
        None => None,
    }
}
