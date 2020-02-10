use crate::cli;
use clap::App;
use habitat_common::FeatureFlag;
use std::str;

fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

fn config_file_enabled() -> FeatureFlag {
    let mut f = FeatureFlag::empty();
    f.insert(FeatureFlag::CONFIG_FILE);
    f
}

fn help(app: &App) -> String {
    let mut help = Vec::new();
    app.write_help(&mut help).expect("to write help");
    String::from(str::from_utf8(&help).expect("to convert help to utf8"))
}

fn sub<'a>(app: &'a App, name: &str) -> &'a App<'a, 'a> {
    app.p
       .subcommands()
       .find(|s| s.p.meta.name == name)
       .expect("to find subcommand")
}

#[test]
fn hab_help() {
    // hab
    let hab1 = cli::get(no_feature_flags()).after_help("");
    let hab2 = cli::get(config_file_enabled()).after_help("");
    let help1 = help(&hab1);
    let help2 = help(&hab2);
    assert_eq!(help1, help2);

    // hab license
    let hab_license1 = sub(&hab1, "license");
    let hab_license2 = sub(&hab2, "license");
    let help1 = help(hab_license1);
    let help2 = help(hab_license2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab license accept
    let hab_license_accept1 = sub(&hab_license1, "accept");
    let hab_license_accept2 = sub(&hab_license2, "accept");
    let help1 = help(hab_license_accept1);
    let help2 = help(hab_license_accept2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);
}
