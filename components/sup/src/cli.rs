use clap::App;

use hab::cli::{
    sub_sup_bash, sub_sup_depart, sub_sup_run, sub_sup_secret, sub_sup_sh, sub_sup_term,
    sub_svc_status,
};
use VERSION;

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    clap_app!(("hab-sup") =>
        (about: "The Habitat Supervisor")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        // set custom usage string, otherwise the binary
        // is displayed confusingly as `hab-sup`
        // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
        (usage: "hab sup <SUBCOMMAND>")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        // this is the _full_ list of supervisor related cmds
        // they are all enumerated here so that the entire help menu
        // can be displayed from `hab sup --help`
        (subcommand: sub_sup_bash().aliases(&["b", "ba", "bas"]))
        (subcommand: sub_sup_depart().aliases(&["d", "de", "dep", "depa", "depart"]))
        (subcommand: sub_sup_run().aliases(&["r", "ru"]))
        (subcommand: sub_sup_secret().aliases(&["sec", "secr"]))
        (subcommand: sub_sup_sh().aliases(&[]))
        (subcommand: sub_svc_status().aliases(&["stat", "statu"]))
        (subcommand: sub_sup_term().aliases(&["ter"]))
    )
}

#[cfg(test)]
mod test {
    use super::cli;

    macro_rules! assert_cli_cmd {
        ($test:ident, $cmd:expr, $( $key:expr => $value:tt ),+) => {
            #[test]
            fn $test() {
                assert_cmd!(cli(), $cmd, $( $key => $value ),+ );
            }
        }
    }

    mod sup_run {
        use super::*;

        assert_cli_cmd!(should_handle_multiple_peer_flags,
                        "hab-sup run --peer 1.1.1.1 --peer 2.2.2.2",
                        "PEER" => ["1.1.1.1", "2.2.2.2"]);

        assert_cli_cmd!(should_handle_single_peer_flag_with_multiple_values,
                        "hab-sup run --peer 1.1.1.1 2.2.2.2",
                        "PEER" => ["1.1.1.1", "2.2.2.2"]);

        assert_cli_cmd!(should_handle_peer_flag_with_arguments,
                        "hab-sup run --peer 1.1.1.1 2.2.2.2 -- core/redis",
                        "PEER" => ["1.1.1.1", "2.2.2.2"],
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

        assert_cli_cmd!(should_handle_multiple_bind_flags,
                        "hab-sup run --bind service.group1 --bind service.group2",
                        "BIND" => ["service.group1", "service.group2"]);

        assert_cli_cmd!(should_handle_single_bind_flag_with_multiple_values,
                        "hab-sup run --bind service.group1 service.group2",
                        "BIND" => ["service.group1", "service.group2"]);

        assert_cli_cmd!(should_handle_bind_flag_with_arguments,
                        "hab-sup run --bind service.group1 service.group2 -- core/redis",
                        "BIND" => ["service.group1", "service.group2"],
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

    }

}
