// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use common::ui::UI;
use hcore::crypto::SigKeyPair;
use hcore::env;
use hcore::package::ident;
use hcore::Error::InvalidOrigin;

use {AUTH_TOKEN_ENVVAR, ORIGIN_ENVVAR};
use analytics;
use command;
use config;
use error::Result;

pub fn start(ui: &mut UI, cache_path: &Path, analytics_path: &Path) -> Result<()> {
    let mut generated_origin = false;

    ui.br()?;
    ui.title("Habitat CLI Setup")?;
    ui.para("Welcome to hab setup. Let's get started.")?;

    ui.heading("Set up a default origin")?;
    ui.para(
        "Every package in Habitat belongs to an origin, which indicates the person or \
               organization responsible for maintaining that package. Each origin also has \
               a key used to cryptographically sign packages in that origin.",
    )?;
    ui.para(
        "Selecting a default origin tells package building operations such as 'hab \
                  pkg build' what key should be used to sign the packages produced. If you \
                  do not set a default origin now, you will have to tell package building \
                  commands each time what origin to use.",
    )?;
    ui.para(
        "For more information on origins and how they are used in building packages, \
               please consult the docs at https://www.habitat.sh/docs/create-packages-build/",
    )?;
    if ask_default_origin(ui)? {
        ui.br()?;
        ui.para(
            "Enter the name of your origin. If you plan to publish your packages \
                      publicly, we recommend that you select one that is not already in use \
                      on the Habitat build service found at https://bldr.habitat.sh/.",
        )?;
        ui.para(&format!(
            "Origins must begin with a lowercase letter or number. \
                Allowed characters include lowercase letters, numbers, _, -. \
                No more than 255 characters."
        ))?;
        let mut origin = prompt_origin(ui)?;

        while !ident::is_valid_origin_name(&origin) {
            ui.br()?;
            ui.fatal(&format!("{}", InvalidOrigin(origin)))?;
            ui.br()?;

            origin = prompt_origin(ui)?;
        }
        write_cli_config_origin(&origin)?;
        ui.br()?;
        if is_origin_in_cache(&origin, cache_path) {
            ui.para(&format!(
                "You already have an origin key for {} created and \
                                   installed. Great work!",
                &origin
            ))?;
        } else {
            ui.heading("Create origin key pair")?;
            ui.para(&format!(
                "It doesn't look like you have a signing key for the origin \
                                `{}'. Without it, you won't be able to build new packages \
                                successfully.",
                &origin
            ))?;
            ui.para(
                "You can either create a new signing key now, or, if you are building \
                       packages for an origin that already exists, ask the owner to give \
                       you the signing key.",
            )?;
            ui.para(
                "For more information on the use of origin keys, please consult \
                          the documentation at \
                          https://www.habitat.sh/docs/concepts-keys/#origin-keys",
            )?;
            if ask_create_origin(ui, &origin)? {
                create_origin(ui, &origin, cache_path)?;
                generated_origin = true;
            } else {
                ui.para(&format!(
                    "You might want to create an origin key later with: \
                                       `hab origin key generate {}'",
                    &origin
                ))?;
            }
        }
    } else {
        ui.para("Okay, maybe another time.")?;
    }
    ui.heading("GitHub Access Token")?;
    ui.para(
        "While you can build and run Habitat packages without sharing them on the public \
               depot, doing so allows you to collaborate with the Habitat community. In \
               addition, it is how you can perform continuous deployment with Habitat.",
    )?;
    ui.para(
        "If you would like to share your packages on the depot, please enter your GitHub \
               access token. Otherwise, just enter No.",
    )?;
    ui.para(
        "For more information on sharing packages on the depot, please read the \
          documentation at https://www.habitat.sh/docs/share-packages-overview/",
    )?;
    if ask_default_auth_token(ui)? {
        ui.br()?;
        ui.para("Enter your GitHub access token.")?;
        let auth_token = prompt_auth_token(ui)?;
        write_cli_config_auth_token(&auth_token)?;
    } else {
        ui.para("Okay, maybe another time.")?;
    }
    ui.heading("Analytics")?;
    ui.para(
        "The `hab` command-line tool will optionally send anonymous usage data to \
               Habitat's Google Analytics account. This is a strictly opt-in activity and \
               no tracking will occur unless you respond affirmatively to the question \
               below.",
    )?;
    ui.para(
        "We collect this data to help improve Habitat's user experience. For example, we \
               would like to know the category of tasks users are performing, and which ones \
               they are having trouble with (e.g. mistyping command line arguments).",
    )?;
    ui.para(
        "To see what kinds of data are sent and how they are anonymized, please read more \
               about our analytics here: https://www.habitat.sh/docs/about-analytics/",
    )?;
    if ask_enable_analytics(ui, analytics_path)? {
        opt_in_analytics(ui, analytics_path, generated_origin)?;
    } else {
        opt_out_analytics(ui, analytics_path)?;
    }
    ui.heading("CLI Setup Complete")?;
    ui.para("That's all for now. Thanks for using Habitat!")?;
    Ok(())
}

fn ask_default_origin(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no("Set up a default origin?", Some(true))?)
}

fn ask_create_origin(ui: &mut UI, origin: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!("Create an origin key for `{}'?", origin),
        Some(true),
    )?)
}

fn write_cli_config_origin(origin: &str) -> Result<()> {
    let mut config = config::load()?;
    config.origin = Some(origin.to_string());
    config::save(&config)
}

fn write_cli_config_auth_token(auth_token: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config::save(&config)
}

fn is_origin_in_cache(origin: &str, cache_path: &Path) -> bool {
    match SigKeyPair::get_latest_pair_for(origin, cache_path, None) {
        Ok(pair) => {
            match pair.secret() {
                Ok(_) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn create_origin(ui: &mut UI, origin: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &origin, cache_path);
    ui.br()?;
    result
}

fn prompt_origin(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.origin {
        Some(o) => {
            ui.para(&format!(
                "You already have a default origin set up as `{}', but feel \
                                free to change it if you wish.",
                &o
            ))?;
            Some(o)
        }
        None => env::var(ORIGIN_ENVVAR).or(env::var("USER")).ok(),
    };
    Ok(ui.prompt_ask(
        "Default origin name",
        default.as_ref().map(|x| &**x),
    )?)
}

fn ask_default_auth_token(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        "Set up a default GitHub access token?",
        Some(true),
    )?)
}

fn prompt_auth_token(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.auth_token {
        Some(o) => {
            ui.para(
                "You already have a default auth token set up, but feel free to change it \
                       if you wish.",
            )?;
            Some(o)
        }
        None => env::var(AUTH_TOKEN_ENVVAR).ok(),
    };
    Ok(ui.prompt_ask(
        "GitHub access token",
        default.as_ref().map(|x| &**x),
    )?)
}

fn ask_enable_analytics(ui: &mut UI, analytics_path: &Path) -> Result<bool> {
    let default = match analytics::is_opted_in(analytics_path) {
        Some(val) => Some(val),
        None => Some(true),
    };
    Ok(ui.prompt_yes_no("Enable analytics?", default)?)
}

fn opt_in_analytics(ui: &mut UI, analytics_path: &Path, generated_origin: bool) -> Result<()> {
    let result = analytics::opt_in(ui, analytics_path, generated_origin);
    ui.br()?;
    result
}

fn opt_out_analytics(ui: &mut UI, analytics_path: &Path) -> Result<()> {
    let result = analytics::opt_out(ui, analytics_path);
    ui.br()?;
    result
}
