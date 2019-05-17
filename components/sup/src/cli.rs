use clap::App;
use hab::cli::sup_commands;
use habitat_common::FeatureFlag;

pub fn cli<'a, 'b>(feature_flags: FeatureFlag) -> App<'a, 'b> { sup_commands(feature_flags) }

#[cfg(test)]
mod test {
    use super::cli;
    use clap::ErrorKind;
    use habitat_common::FeatureFlag;

    fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

    macro_rules! assert_cli_cmd {
        ($test:ident, $flags:expr, $cmd:expr, $( $key:expr => $value:tt ),+) => {
            #[test]
            fn $test() {
                assert_cmd!(cli($flags), $cmd, $( $key => $value ),+ );
            }
        }
    }

    #[test]
    fn sup_help_on_run_subcommand() {
        let r = cli(no_feature_flags()).get_matches_from_safe(vec!["hab-sup", "run", "--help"]);
        assert!(r.is_err());
        // not `ErrorKind::InvalidSubcommand`
        assert_eq!(r.unwrap_err().kind, ErrorKind::HelpDisplayed);
    }

    mod sup_run {
        use super::*;
        use std::iter::FromIterator as _;

        assert_cli_cmd!(should_handle_multiple_peer_flags,
                        no_feature_flags(),
                        "hab-sup run --peer 1.1.1.1 --peer 2.2.2.2",
                        "PEER" => ["1.1.1.1", "2.2.2.2"]);

        assert_cli_cmd!(should_handle_single_peer_flag_with_multiple_values,
                        no_feature_flags(),
                        "hab-sup run --peer 1.1.1.1 2.2.2.2",
                        "PEER" => ["1.1.1.1", "2.2.2.2"]);

        assert_cli_cmd!(should_handle_peer_flag_with_arguments,
                        no_feature_flags(),
                        "hab-sup run --peer 1.1.1.1 2.2.2.2 -- core/redis",
                        "PEER" => ["1.1.1.1", "2.2.2.2"],
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

        assert_cli_cmd!(should_handle_multiple_bind_flags,
                        no_feature_flags(),
                        "hab-sup run --bind service.group1 --bind service.group2",
                        "BIND" => ["service.group1", "service.group2"]);

        assert_cli_cmd!(should_handle_single_bind_flag_with_multiple_values,
                        no_feature_flags(),
                        "hab-sup run --bind service.group1 service.group2",
                        "BIND" => ["service.group1", "service.group2"]);

        assert_cli_cmd!(should_handle_bind_flag_with_arguments,
                        no_feature_flags(),
                        "hab-sup run --bind service.group1 service.group2 -- core/redis",
                        "BIND" => ["service.group1", "service.group2"],
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

        #[test]
        fn local_gossip_mode_and_listen_gossip_are_mutually_exclusive() {
            let cmd_vec = Vec::from_iter("hab-sup run --listen-gossip 1.1.1.1:1111 \
                                          --local-gossip-mode"
                                                              .split_whitespace());
            assert!(cli(no_feature_flags()).get_matches_from_safe(cmd_vec)
                                           .is_err());
        }

        #[test]
        fn local_gossip_mode_and_peer_are_mutually_exclusive() {
            let cmd_vec = Vec::from_iter(
                "hab-sup run --peer 1.1.1.1:1111 --local-gossip-mode".split_whitespace(),
            );
            assert!(cli(no_feature_flags()).get_matches_from_safe(cmd_vec)
                                           .is_err());
        }

        #[test]
        fn local_gossip_mode_and_peer_watch_file_are_mutually_exclusive() {
            let cmd_vec = Vec::from_iter("hab-sup run --local-gossip-mode --peer-watch-file \
                                          foobar"
                                                 .split_whitespace());
            assert!(cli(no_feature_flags()).get_matches_from_safe(cmd_vec)
                                           .is_err());
        }

        #[test]
        fn peer_watch_file_and_peer_are_mutually_exclusive() {
            let cmd_vec = Vec::from_iter("hab-sup run --peer 1.1.1.1:1111 --peer-watch-file \
                                          foobar"
                                                 .split_whitespace());
            assert!(cli(no_feature_flags()).get_matches_from_safe(cmd_vec)
                                           .is_err());
        }

    }

}
