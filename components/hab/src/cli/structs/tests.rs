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

    // hab cli
    let hab_cli1 = sub(&hab1, "cli");
    let hab_cli2 = sub(&hab2, "cli");
    let help1 = help(hab_cli1);
    let help2 = help(hab_cli2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab cli completers
    let hab_cli_completers1 = sub(&hab_cli1, "completers");
    let hab_cli_completers2 = sub(&hab_cli2, "completers");
    let help1 = help(hab_cli_completers1);
    let help2 = help(hab_cli_completers2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab cli setup
    let hab_cli_setup1 = sub(&hab_cli1, "setup");
    let hab_cli_setup2 = sub(&hab_cli2, "setup");
    let help1 = help(hab_cli_setup1);
    let help2 = help(hab_cli_setup2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr
    let hab_bldr1 = sub(&hab1, "bldr");
    let hab_bldr2 = sub(&hab2, "bldr");
    let help1 = help(hab_bldr1);
    let help2 = help(hab_bldr2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr channel
    let hab_bldr_channel1 = sub(&hab_bldr1, "channel");
    let hab_bldr_channel2 = sub(&hab_bldr2, "channel");
    let help1 = help(hab_bldr_channel1);
    let help2 = help(hab_bldr_channel2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr channel create
    let hab_bldr_channel_create1 = sub(&hab_bldr_channel1, "create");
    let hab_bldr_channel_create2 = sub(&hab_bldr_channel2, "create");
    let help1 = help(hab_bldr_channel_create1);
    let help2 = help(hab_bldr_channel_create2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr channel demote
    let hab_bldr_channel_demote1 = sub(&hab_bldr_channel1, "demote");
    let hab_bldr_channel_demote2 = sub(&hab_bldr_channel2, "demote");
    let help1 = help(hab_bldr_channel_demote1);
    let help2 = help(hab_bldr_channel_demote2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr channel destroy
    let hab_bldr_channel_destroy1 = sub(&hab_bldr_channel1, "destroy");
    let hab_bldr_channel_destroy2 = sub(&hab_bldr_channel2, "destroy");
    let help1 = help(hab_bldr_channel_destroy1);
    let help2 = help(hab_bldr_channel_destroy2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr channel list
    let hab_bldr_channel_list1 = sub(&hab_bldr_channel1, "list");
    let hab_bldr_channel_list2 = sub(&hab_bldr_channel2, "list");
    let help1 = help(hab_bldr_channel_list1);
    let help2 = help(hab_bldr_channel_list2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr channel promote
    let hab_bldr_channel_promote1 = sub(&hab_bldr_channel1, "promote");
    let hab_bldr_channel_promote2 = sub(&hab_bldr_channel2, "promote");
    let help1 = help(hab_bldr_channel_promote1);
    let help2 = help(hab_bldr_channel_promote2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr job
    let hab_bldr_job1 = sub(&hab_bldr1, "job");
    let hab_bldr_job2 = sub(&hab_bldr2, "job");
    let help1 = help(hab_bldr_job1);
    let help2 = help(hab_bldr_job2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr job cancel
    let hab_bldr_job_cancel1 = sub(&hab_bldr_job1, "cancel");
    let hab_bldr_job_cancel2 = sub(&hab_bldr_job2, "cancel");
    let help1 = help(hab_bldr_job_cancel1);
    let help2 = help(hab_bldr_job_cancel2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr job demote
    let hab_bldr_job_demote1 = sub(&hab_bldr_job1, "demote");
    let hab_bldr_job_demote2 = sub(&hab_bldr_job2, "demote");
    let help1 = help(hab_bldr_job_demote1);
    let help2 = help(hab_bldr_job_demote2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr job promote
    let hab_bldr_job_promote1 = sub(&hab_bldr_job1, "promote");
    let hab_bldr_job_promote2 = sub(&hab_bldr_job2, "promote");
    let help1 = help(hab_bldr_job_promote1);
    let help2 = help(hab_bldr_job_promote2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr job start
    let hab_bldr_job_start1 = sub(&hab_bldr_job1, "start");
    let hab_bldr_job_start2 = sub(&hab_bldr_job2, "start");
    let help1 = help(hab_bldr_job_start1);
    let help2 = help(hab_bldr_job_start2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab bldr job start
    let hab_bldr_job_status1 = sub(&hab_bldr_job1, "status");
    let hab_bldr_job_status2 = sub(&hab_bldr_job2, "status");
    let help1 = help(hab_bldr_job_status1);
    let help2 = help(hab_bldr_job_status2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);
}
