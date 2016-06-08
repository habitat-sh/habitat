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
        para("Welcome to hab setup, let's get started.");

        heading("Setup a default origin");
        para("Far far away, behind the word mountains, far from the countries Vokalia and \
             Consonantia, there live the blind texts. Separated they live in Bookmarksgrove \
             right at the coast of the Semantics, a large language ocean. A small river named \
             Duden flows by their place and supplies it with the necessary regelialia. It is a \
             paradisematic country, in which roasted parts of sentences fly into your mouth. \
             Even the all-powerful Pointing has no control about the blind texts it is an almost \
             unorthographic life One day however a small line of blind text by the name of Lorem \
             Ipsum decided to leave for the far World of Grammar.");
        para("https://www.habitat.sh/docs/reference/habitat-cli/#hab-studio");
        if try!(ask_default_origin()) {
            println!("");
            para("When she reached the first hills of the Italic Mountains, she had a last view \
                 back on the skyline of her hometown Bookmarksgrove, the headline of Alphabet \
                 Village and the subline of her own road, the Line Lane.");
            let origin = try!(prompt_origin());
            try!(write_cli_config(&origin));
            println!("");
            if is_origin_in_cache(&origin, cache_path) {
                para(&format!("You already have an origin key for {} created and installed, \
                              great work!",
                              &origin));
            } else {
                heading("Create origin key pair");
                para("Far far away, behind the word mountains, far from the countries Vokalia \
                      and Consonantia, there live the blind texts. Separated they live in \
                      Bookmarksgrove right at the coast of the Semantics, a large language \
                      ocean. A small river named Duden flows by their place and supplies it with \
                      the necessary regelialia. It is a paradisematic country, in which roasted \
                      parts of sentences fly into your mouth. Even the all-powerful Pointing has \
                      no control about the blind texts it is an almost unorthographic life One \
                      day however a small line of blind text by the name of Lorem Ipsum decided \
                      to leave for the far World of Grammar.");
                para("https://www.habitat.sh/docs/concepts-keys/#origin-keys");
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
             occur unless you respond affirmatively to the question during `hab setup`. If you \
             do not use `hab setup`, no data will ever be sent.");
        para("We collect this data to help improve Habitat's user experience: for example, to \
             know what tasks users are performing, and which ones they are having trouble with \
             (e.g. mistyping command line arguments).");
        para("By anonymous we mean that all identifying information about you is removed before \
             we send the data. This includes the removal of any information about what packages \
             you are building, or what origins you are using. For example, if you were building \
             the package `yourname/yourapp`, and you typed `hab pkg build -k yourkey \
             yourname/yourapp`, the fact that you were performing the `pkg build` operation \
             would be transmitted. Neither the name of the specific package you are building, \
             nor the fact that you are using the `yourkey` key to sign that package would be \
             transmitted.");
        para("Please do not hesitate to contact us at support@habitat.sh if you have \
             questions or concerns about the use of Google Analytics within the Habitat product.");
        if try!(ask_enable_analytics(analytics_path)) {
            try!(opt_in_analytics(analytics_path, generated_origin));
        } else {
            try!(opt_out_analytics(analytics_path));
        }
        heading("CLI Setup Complete");
        para("That's all, thanks for playing along!");
        Ok(())
    }

    fn ask_default_origin() -> Result<bool> {
        prompt_yes_no("Set up a default origin key?", Some(true))
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
