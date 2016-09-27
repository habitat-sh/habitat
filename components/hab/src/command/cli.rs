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

pub mod setup {
    use std::path::Path;

    use common::ui::UI;
    use hcore::crypto::SigKeyPair;
    use hcore::env;

    use analytics;
    use command;
    use config;
    use error::Result;

    pub fn start(ui: &mut UI, cache_path: &Path, analytics_path: &Path) -> Result<()> {
        let mut generated_origin = false;

        try!(ui.br());
        try!(ui.title("Habitat CLI Setup"));
        try!(ui.para("Welcome to hab setup. Let's get started."));

        try!(ui.heading("Set up a default origin"));
        try!(ui.para("Every package in Habitat belongs to an origin, which indicates the person or \
                   organization responsible for maintaining that package. Each origin also has \
                   a key used to cryptographically sign packages in that origin."));
        try!(ui.para("Selecting a default origin tells package building operations such as 'hab \
                      pkg build' what key should be used to sign the packages produced. If you \
                      do not set a default origin now, you will have to tell package building \
                      commands each time what origin to use."));
        try!(ui.para("For more information on origins and how they are used in building packages, \
                   please consult the docs at https://www.habitat.sh/docs/create-packages-build/"));
        if try!(ask_default_origin(ui)) {
            try!(ui.br());
            try!(ui.para("Enter the name of your origin. If you plan to publish your packages \
                          publicly, we recommend that you select one that is not already in use \
                          on the Habitat build service found at https://app.habitat.sh/."));
            let origin = try!(prompt_origin(ui));
            try!(write_cli_config_origin(&origin));
            try!(ui.br());
            if is_origin_in_cache(&origin, cache_path) {
                try!(ui.para(&format!("You already have an origin key for {} created and \
                                       installed. Great work!",
                                      &origin)));
            } else {
                try!(ui.heading("Create origin key pair"));
                try!(ui.para(&format!("It doesn't look like you have a signing key for the origin \
                                    `{}'. Without it, you won't be able to build new packages \
                                    successfully.",
                                   &origin)));
                try!(ui.para("You can either create a new signing key now, or, if you are building \
                           packages for an origin that already exists, ask the owner to give \
                           you the signing key."));
                try!(ui.para("For more information on the use of origin keys, please consult \
                              the documentation at \
                              https://www.habitat.sh/docs/concepts-keys/#origin-keys"));
                if try!(ask_create_origin(ui, &origin)) {
                    try!(create_origin(ui, &origin, cache_path));
                    generated_origin = true;
                } else {
                    try!(ui.para(&format!("You might want to create an origin key later with: \
                                           `hab origin key generate {}'",
                                          &origin)));
                }
            }
        } else {
            try!(ui.para("Okay, maybe another time."));
        }
        try!(ui.heading("GitHub Access Token"));
        try!(ui.para("While you can build and run Habitat packages without sharing them on the \
                      public depot, doing so allows you to collaborate with the Habitat \
                      community. In addition, it is how you can perform continuous deployment \
                      with Habitat."));
        try!(ui.para("The depot uses GitHub authentication by personal access token with the \
                      user:email and read:org scopes (https://help.github.\
                      com/articles/creating-an-access-token-for-command-line-use/)."));
        try!(ui.para("If you would like to share your packages on the depot, please enter your \
                      GitHub access token. Otherwise, just enter No."));
        try!(ui.para("For more information on sharing packages on the depot, please read the \
              documentation at https://www.habitat.sh/docs/share-packages-overview/"));
        if try!(ask_default_auth_token(ui)) {
            try!(ui.br());
            try!(ui.para("Enter your GitHub access token."));
            let auth_token = try!(prompt_auth_token(ui));
            try!(write_cli_config_auth_token(&auth_token));
        } else {
            try!(ui.para("Okay, maybe another time."));
        }
        try!(ui.heading("Analytics"));
        try!(ui.para("The `hab` command-line tool will optionally send anonymous usage data to \
                   Habitat's Google Analytics account. This is a strictly opt-in activity and \
                   no tracking will occur unless you respond affirmatively to the question \
                   below."));
        try!(ui.para("We collect this data to help improve Habitat's user experience. For example, \
                   we would like to know the category of tasks users are performing, and which \
                   ones they are having trouble with (e.g. mistyping command line arguments)."));
        try!(ui.para("To see what kinds of data are sent and how they are anonymized, please read \
                   more about our analytics here: https://www.habitat.sh/docs/about-analytics/"));
        if try!(ask_enable_analytics(ui, analytics_path)) {
            try!(opt_in_analytics(ui, analytics_path, generated_origin));
        } else {
            try!(opt_out_analytics(ui, analytics_path));
        }
        try!(ui.heading("CLI Setup Complete"));
        try!(ui.para("That's all for now. Thanks for using Habitat!"));
        Ok(())
    }

    fn ask_default_origin(ui: &mut UI) -> Result<bool> {
        Ok(try!(ui.prompt_yes_no("Set up a default origin?", Some(true))))
    }

    fn ask_create_origin(ui: &mut UI, origin: &str) -> Result<bool> {
        Ok(try!(ui.prompt_yes_no(&format!("Create an origin key for `{}'?", origin),
                                 Some(true))))
    }

    fn write_cli_config_origin(origin: &str) -> Result<()> {
        let mut config = try!(config::load());
        config.origin = Some(origin.to_string());
        config::save(&config)
    }

    fn write_cli_config_auth_token(auth_token: &str) -> Result<()> {
        let mut config = try!(config::load());
        config.auth_token = Some(auth_token.to_string());
        config::save(&config)
    }

    fn is_origin_in_cache(origin: &str, cache_path: &Path) -> bool {
        match SigKeyPair::get_latest_pair_for(origin, cache_path) {
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
        try!(ui.br());
        result
    }

    fn prompt_origin(ui: &mut UI) -> Result<String> {
        let config = try!(config::load());
        let default = match config.origin {
            Some(o) => {
                try!(ui.para(&format!("You already have a default origin set up as `{}', but feel \
                                    free to change it if you wish.",
                                   &o)));
                Some(o)
            }
            None => env::var("USER").ok(),
        };
        Ok(try!(ui.prompt_ask("Default origin name", default.as_ref().map(|x| &**x))))
    }

    fn ask_default_auth_token(ui: &mut UI) -> Result<bool> {
        Ok(try!(ui.prompt_yes_no("Set up a default GitHub access token?", Some(true))))
    }

    fn prompt_auth_token(ui: &mut UI) -> Result<String> {
        let config = try!(config::load());
        let default = match config.auth_token {
            Some(o) => {
                try!(ui.para("You already have a default auth token set up, but feel free to \
                              change it if you wish."));
                Some(o)
            }
            None => None,
        };
        Ok(try!(ui.prompt_ask("GitHub access token", default.as_ref().map(|x| &**x))))
    }

    fn ask_enable_analytics(ui: &mut UI, analytics_path: &Path) -> Result<bool> {
        let default = match analytics::is_opted_in(analytics_path) {
            Some(val) => Some(val),
            None => Some(true),
        };
        Ok(try!(ui.prompt_yes_no("Enable analytics?", default)))
    }

    fn opt_in_analytics(ui: &mut UI, analytics_path: &Path, generated_origin: bool) -> Result<()> {
        let result = analytics::opt_in(ui, analytics_path, generated_origin);
        try!(ui.br());
        result
    }

    fn opt_out_analytics(ui: &mut UI, analytics_path: &Path) -> Result<()> {
        let result = analytics::opt_out(ui, analytics_path);
        try!(ui.br());
        result
    }
}
