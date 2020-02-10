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

fn help(app: App) -> String {
    let mut help = Vec::new();
    app.write_help(&mut help).expect("to write help");
    String::from(str::from_utf8(&help).expect("to convert help to utf8"))
}

#[test]
fn hab_help() {
    let sub_help1 = help(cli::get(no_feature_flags()));
    let sub_help2 = help(cli::get(config_file_enabled()));
    assert_eq!(sub_help1, sub_help2);
}
