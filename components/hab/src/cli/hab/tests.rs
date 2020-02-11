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

    // hab config
    let hab_config1 = sub(&hab1, "config");
    let hab_config2 = sub(&hab2, "config");
    let help1 = help(hab_config1);
    let help2 = help(hab_config2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab config apply
    let hab_config_apply1 = sub(&hab_config1, "apply");
    let hab_config_apply2 = sub(&hab_config2, "apply");
    let help1 = help(hab_config_apply1);
    let help2 = help(hab_config_apply2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab config show
    let hab_config_show1 = sub(&hab_config1, "show");
    let hab_config_show2 = sub(&hab_config2, "show");
    let help1 = help(hab_config_show1);
    let help2 = help(hab_config_show2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab file
    let hab_file1 = sub(&hab1, "file");
    let hab_file2 = sub(&hab2, "file");
    let help1 = help(hab_file1);
    let help2 = help(hab_file2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab file upload
    let hab_file_upload1 = sub(&hab_file1, "upload");
    let hab_file_upload2 = sub(&hab_file2, "upload");
    let help1 = help(hab_file_upload1);
    let help2 = help(hab_file_upload2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin
    let hab_origin1 = sub(&hab1, "origin");
    let hab_origin2 = sub(&hab2, "origin");
    let help1 = help(hab_origin1);
    let help2 = help(hab_origin2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin create
    let hab_origin_create1 = sub(&hab_origin1, "create");
    let hab_origin_create2 = sub(&hab_origin2, "create");
    let help1 = help(hab_origin_create1);
    let help2 = help(hab_origin_create2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin delete
    let hab_origin_delete1 = sub(&hab_origin1, "delete");
    let hab_origin_delete2 = sub(&hab_origin2, "delete");
    let help1 = help(hab_origin_delete1);
    let help2 = help(hab_origin_delete2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin depart
    let hab_origin_depart1 = sub(&hab_origin1, "depart");
    let hab_origin_depart2 = sub(&hab_origin2, "depart");
    let help1 = help(hab_origin_depart1);
    let help2 = help(hab_origin_depart2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations
    let hab_origin_invitations1 = sub(&hab_origin1, "invitations");
    let hab_origin_invitations2 = sub(&hab_origin2, "invitations");
    let help1 = help(hab_origin_invitations1);
    let help2 = help(hab_origin_invitations2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations accept
    let hab_origin_invitations_accept1 = sub(&hab_origin_invitations1, "accept");
    let hab_origin_invitations_accept2 = sub(&hab_origin_invitations2, "accept");
    let help1 = help(hab_origin_invitations_accept1);
    let help2 = help(hab_origin_invitations_accept2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations ignore
    let hab_origin_invitations_ignore1 = sub(&hab_origin_invitations1, "ignore");
    let hab_origin_invitations_ignore2 = sub(&hab_origin_invitations2, "ignore");
    let help1 = help(hab_origin_invitations_ignore1);
    let help2 = help(hab_origin_invitations_ignore2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations list
    let hab_origin_invitations_list1 = sub(&hab_origin_invitations1, "list");
    let hab_origin_invitations_list2 = sub(&hab_origin_invitations2, "list");
    let help1 = help(hab_origin_invitations_list1);
    let help2 = help(hab_origin_invitations_list2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations pending
    let hab_origin_invitations_pending1 = sub(&hab_origin_invitations1, "pending");
    let hab_origin_invitations_pending2 = sub(&hab_origin_invitations2, "pending");
    let help1 = help(hab_origin_invitations_pending1);
    let help2 = help(hab_origin_invitations_pending2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations rescind
    let hab_origin_invitations_rescind1 = sub(&hab_origin_invitations1, "rescind");
    let hab_origin_invitations_rescind2 = sub(&hab_origin_invitations2, "rescind");
    let help1 = help(hab_origin_invitations_rescind1);
    let help2 = help(hab_origin_invitations_rescind2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin invitations send
    let hab_origin_invitations_send1 = sub(&hab_origin_invitations1, "send");
    let hab_origin_invitations_send2 = sub(&hab_origin_invitations2, "send");
    let help1 = help(hab_origin_invitations_send1);
    let help2 = help(hab_origin_invitations_send2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin key
    let hab_origin_key1 = sub(&hab_origin1, "key");
    let hab_origin_key2 = sub(&hab_origin2, "key");
    let help1 = help(hab_origin_key1);
    let help2 = help(hab_origin_key2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin key download
    let hab_origin_key_download1 = sub(&hab_origin_key1, "download");
    let hab_origin_key_download2 = sub(&hab_origin_key2, "download");
    let help1 = help(hab_origin_key_download1);
    let help2 = help(hab_origin_key_download2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin key export
    let hab_origin_key_export1 = sub(&hab_origin_key1, "export");
    let hab_origin_key_export2 = sub(&hab_origin_key2, "export");
    let help1 = help(hab_origin_key_export1);
    let help2 = help(hab_origin_key_export2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin key generate
    let hab_origin_key_generate1 = sub(&hab_origin_key1, "generate");
    let hab_origin_key_generate2 = sub(&hab_origin_key2, "generate");
    let help1 = help(hab_origin_key_generate1);
    let help2 = help(hab_origin_key_generate2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin key import
    let hab_origin_key_import1 = sub(&hab_origin_key1, "import");
    let hab_origin_key_import2 = sub(&hab_origin_key2, "import");
    let help1 = help(hab_origin_key_import1);
    let help2 = help(hab_origin_key_import2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin key upload
    let hab_origin_key_upload1 = sub(&hab_origin_key1, "upload");
    let hab_origin_key_upload2 = sub(&hab_origin_key2, "upload");
    let help1 = help(hab_origin_key_upload1);
    let help2 = help(hab_origin_key_upload2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin secret
    let hab_origin_secret1 = sub(&hab_origin1, "secret");
    let hab_origin_secret2 = sub(&hab_origin2, "secret");
    let help1 = help(hab_origin_secret1);
    let help2 = help(hab_origin_secret2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin secret delete
    let hab_origin_secret_delete1 = sub(&hab_origin_secret1, "delete");
    let hab_origin_secret_delete2 = sub(&hab_origin_secret2, "delete");
    let help1 = help(hab_origin_secret_delete1);
    let help2 = help(hab_origin_secret_delete2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin secret list
    let hab_origin_secret_list1 = sub(&hab_origin_secret1, "list");
    let hab_origin_secret_list2 = sub(&hab_origin_secret2, "list");
    let help1 = help(hab_origin_secret_list1);
    let help2 = help(hab_origin_secret_list2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin secret upload
    let hab_origin_secret_upload1 = sub(&hab_origin_secret1, "upload");
    let hab_origin_secret_upload2 = sub(&hab_origin_secret2, "upload");
    let help1 = help(hab_origin_secret_upload1);
    let help2 = help(hab_origin_secret_upload2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);

    // hab origin transfer
    let hab_origin_transfer1 = sub(&hab_origin1, "transfer");
    let hab_origin_transfer2 = sub(&hab_origin2, "transfer");
    let help1 = help(hab_origin_transfer1);
    let help2 = help(hab_origin_transfer2);
    assert_eq!(help1, help2);
    assert_eq!(help1, help2);
}
