// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod setup {
    use std::io::{self, Write};
    use std::path::Path;
    use std::process;

    use ansi_term::Colour::{Cyan, Green, White};
    use hcore::crypto::SigKeyPair;
    use hcore::env;

    use analytics;
    use command;
    use config::{self, Config};
    use error::Result;

    pub fn start(cache_path: &Path, analytics_path: &Path) -> Result<()> {
        let mut generated_origin = false;

        println!("");
        title("Habitat CLI Setup");
        para("Welcome to hab setup. Let's get started.");

        heading("Set up a default origin");
        para("Every package in Habitat belongs to an origin, which indicates the person or \
              organization responsible for maintaining that package. Each origin also has \
              a key used to cryptographically sign packages in that origin.");
        para("Selecting a default origin tells package building operations such as 'hab pkg \
              build' what key should be used to sign the packages produced. If you do not \
              set a default origin now, you will have to tell package building commands each \
              time what origin to use.");
        para("For more information on origins and how they are used in building packages, \
              please consult the docs at https://www.habitat.sh/docs/build-packages-overview/");
        if try!(ask_default_origin()) {
            println!("");
            para("Enter the name of your origin. If you plan to publish your packages publicly, \
                  we recommend that you select one that is not already in use on the Habitat \
                  build service found at https://app.habitat.sh/.");
            let origin = try!(prompt_origin());
            try!(write_cli_config(&origin));
            println!("");
            if is_origin_in_cache(&origin, cache_path) {
                para(&format!("You already have an origin key for {} created and installed. \
                              Great work!",
                              &origin));
            } else {
                heading("Create origin key pair");
                para(&format!("It doesn't look like you have a signing key for the origin `{}'. \
                               Without it, you won't be able to build new packages successfully.",
                              &origin));
                para("You can either create a new signing key now, or, if you are building \
                      packages for an origin that already exists, ask the owner to give you the \
                      signing key.");
                para("For more information on the use of origin keys, please consult the \
                      documentation at https://www.habitat.sh/docs/concepts-keys/#origin-keys");
                if try!(ask_create_origin(&origin)) {
                    try!(create_origin(&origin, cache_path));
                    generated_origin = true;
                } else {
                    para(&format!("You might want to create an origin key later with: `hab \
                                  origin key generate {}'",
                                  &origin));
                }
            }
        } else {
            para("Okay, maybe another time");
        }
        heading("Analytics");
        para("The `hab` command-line tool will optionally send anonymous usage data to Habitat's \
             Google Analytics account. This is a strictly opt-in activity and no tracking will \
             occur unless you respond affirmatively to the question below.");
        para("We collect this data to help improve Habitat's user experience. For example, we \
              would like to know the category of tasks users are performing, and which ones they \
              are having trouble with (e.g. mistyping command line arguments).");
        para("To see what kinds of data are sent and how they are anonymized, please read more \
             about our analytics here: https://www.habitat.sh/docs/about-analytics/");
        if try!(ask_enable_analytics(analytics_path)) {
            try!(opt_in_analytics(analytics_path, generated_origin));
        } else {
            try!(opt_out_analytics(analytics_path));
        }
        heading("CLI Setup Complete");
        para("That's all for now. Thanks for using Habitat!");
        Ok(())
    }

    fn ask_default_origin() -> Result<bool> {
        prompt_yes_no("Set up a default origin?", Some(true))
    }

    fn ask_create_origin(origin: &str) -> Result<bool> {
        prompt_yes_no(&format!("Create an origin key for `{}'?", origin),
                      Some(true))
    }

    fn write_cli_config(origin: &str) -> Result<()> {
        let mut config = Config::default();
        config.origin = Some(origin.to_string());
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

    fn create_origin(origin: &str, cache_path: &Path) -> Result<()> {
        let result = command::origin::key::generate::start(&origin, cache_path);
        println!("");
        result
    }

    fn prompt_origin() -> Result<String> {
        let config = try!(config::load());
        let default = match config.origin {
            Some(o) => {
                para(&format!("You already have a default origin set up as `{}', but feel free \
                               to change it if you wish.",
                              &o));
                Some(o)
            }
            None => env::var("USER").ok(),
        };
        prompt_ask("Default origin name", default.as_ref().map(|x| &**x))
    }

    fn ask_enable_analytics(analytics_path: &Path) -> Result<bool> {
        let default = match analytics::is_opted_in(analytics_path) {
            Some(val) => Some(val),
            None => Some(true),
        };
        prompt_yes_no("Enable analytics?", default)
    }

    fn opt_in_analytics(analytics_path: &Path, generated_origin: bool) -> Result<()> {
        let result = analytics::opt_in(analytics_path, generated_origin);
        println!("");
        result
    }

    fn opt_out_analytics(analytics_path: &Path) -> Result<()> {
        let result = analytics::opt_out(analytics_path);
        println!("");
        result
    }

    fn title(text: &str) {
        println!("{}", Green.bold().paint(text));
        println!("{}\n",
                 Green.bold().paint(format!("{:=<width$}", "", width = text.chars().count())));
    }

    fn heading(text: &str) {
        println!("{}\n", Green.bold().paint(text));
    }

    fn para(text: &str) {
        print_wrapped(text, 75, 2)
    }

    fn print_wrapped(text: &str, wrap_width: usize, left_indent: usize) {
        for line in text.split("\n\n") {
            let mut buffer = String::new();
            let mut width = 0;
            for word in line.split_whitespace() {
                let wl = word.chars().count();
                if (width + wl + 1) > (wrap_width - left_indent) {
                    println!("{:<width$}{}", " ", buffer, width = left_indent);
                    buffer.clear();
                    width = 0;
                }
                width = width + wl + 1;
                buffer.push_str(word);
                buffer.push(' ');
            }
            if !buffer.is_empty() {
                println!("{:<width$}{}", " ", buffer, width = left_indent);
            }
            println!("");
        }
    }

    fn prompt_yes_no(question: &str, default: Option<bool>) -> Result<bool> {
        let choice = match default {
            Some(yes) => {
                if yes {
                    format!("{}{}{}",
                            White.paint("["),
                            White.bold().paint("Yes"),
                            White.paint("/no/quit]"))
                } else {
                    format!("{}{}{}",
                            White.paint("[yes/"),
                            White.bold().paint("No"),
                            White.paint("/quit]"))
                }
            }
            None => format!("{}", White.paint("[yes/no/quit]")),
        };
        loop {
            try!(io::stdout().flush());
            print!("{} {} ", Cyan.paint(question), choice);
            try!(io::stdout().flush());
            let mut response = String::new();
            try!(io::stdin().read_line(&mut response));
            match response.trim().chars().next().unwrap_or('\n') {
                'y' | 'Y' => return Ok(true),
                'n' | 'N' => return Ok(false),
                'q' | 'Q' => process::exit(0),
                '\n' => {
                    match default {
                        Some(default) => return Ok(default),
                        None => continue,
                    }
                }
                _ => continue,
            }
        }
    }

    fn prompt_ask(question: &str, default: Option<&str>) -> Result<String> {
        let choice = match default {
            Some(d) => {
                format!(" {}{}{}",
                        White.paint("[default: "),
                        White.bold().paint(d),
                        White.paint("]"))
            }
            None => "".to_string(),
        };
        loop {
            try!(io::stdout().flush());
            print!("{}{} ", Cyan.paint(format!("{}:", question)), choice);
            try!(io::stdout().flush());
            let mut response = String::new();
            try!(io::stdin().read_line(&mut response));
            if response.trim().is_empty() {
                match default {
                    Some(d) => return Ok(d.to_string()),
                    None => continue,
                }
            }
            return Ok(response.trim().to_string());
        }
    }
}
