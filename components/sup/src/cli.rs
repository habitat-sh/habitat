use clap::App;

use hab::cli::sup_commands;

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    sup_commands()
}

#[cfg(test)]
mod test {
    use super::cli;
    use clap::ErrorKind;

    macro_rules! assert_cli_cmd {
        ($test:ident, $cmd:expr, $( $key:expr => $value:tt ),+) => {
            #[test]
            fn $test() {
                assert_cmd!(cli(), $cmd, $( $key => $value ),+ );
            }
        }
    }

    #[test]
    fn sup_help_on_run_subcommand() {
        let r = cli().get_matches_from_safe(vec!["hab-sup", "run", "--help"]);
        assert!(r.is_err());
        // not `ErrorKind::InvalidSubcommand`
        assert_eq!(r.unwrap_err().kind, ErrorKind::HelpDisplayed);
    }

    mod sup_run {
        use super::*;
        use crate::common::cli_defaults::{
            GOSSIP_DEFAULT_ADDR, LISTEN_CTL_DEFAULT_ADDR_STRING, LISTEN_HTTP_DEFAULT_ADDR,
        };
        use std::iter::FromIterator;

        assert_cli_cmd!(should_handle_listen_gossip_with_value,
                        "hab-sup run --listen-gossip 1.1.1.1:1111",
                        "LISTEN_GOSSIP" => "1.1.1.1:1111");

        assert_cli_cmd!(listen_gossip_should_have_default_value,
                        "hab-sup run",
                        "LISTEN_GOSSIP" => (&GOSSIP_DEFAULT_ADDR));

        #[test]
        fn invalid_listen_gossip_address_should_fail() {
            let cmd_vec =
                Vec::from_iter("hab-sup run --listen-gossip bad-address".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_listen_http,
                        "hab-sup run --listen-http 1.1.1.1:1111",
                        "LISTEN_HTTP" => "1.1.1.1:1111");

        assert_cli_cmd!(listen_http_should_have_default_value,
                        "hab-sup run",
                        "LISTEN_HTTP" => (&LISTEN_HTTP_DEFAULT_ADDR));

        #[test]
        fn invalid_listen_http_address_should_fail() {
            let cmd_vec =
                Vec::from_iter("hab-sup run --listen-http bad-address".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_http_disable,
                        "hab-sup run --http-disable",
                        "HTTP_DISABLE" => true);

        assert_cli_cmd!(http_disable_should_have_short_flag,
                        "hab-sup run -D",
                        "HTTP_DISABLE" => true);

        assert_cli_cmd!(should_handle_listen_ctl,
                        "hab-sup run --listen-ctl 1.1.1.1:1111",
                        "LISTEN_CTL" => "1.1.1.1:1111");

        assert_cli_cmd!(listen_ctl_should_have_a_default_value,
                        "hab-sup run",
                        "LISTEN_CTL" => (&LISTEN_CTL_DEFAULT_ADDR_STRING));

        #[test]
        fn invalid_listen_ctl_address_should_fail() {
            let cmd_vec = Vec::from_iter("hab-sup run --listen-ctl bad-address".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_org,
                        "hab-sup run --org monkeypants",
                        "ORGANIZATION" => "monkeypants");

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

        assert_cli_cmd!(should_handle_permanent_peer,
                        "hab-sup run --permanent-peer",
                        "PERMANENT_PEER" => true);

        assert_cli_cmd!(permanent_peer_should_have_short_flag,
                        "hab-sup run -I",
                        "PERMANENT_PEER" => true);

        assert_cli_cmd!(should_handle_peer_watch_file,
                        "hab-sup run --peer-watch-file foobar",
                        "PEER_WATCH_FILE" => "foobar");

        #[test]
        fn peer_watch_file_conflicts_with_peer() {
            let cmd_vec = Vec::from_iter(
                "hab-sup run --peer-watch-file foobar --peer 1.1.1.1:1111".split_whitespace(),
            );
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_ring,
                        "hab-sup run --ring fizbang",
                        "RING" => "fizbang");

        assert_cli_cmd!(ring_should_have_short_flag,
                        "hab-sup run -r flippitygibbit",
                        "RING" => "flippitygibbit");

        #[test]
        fn ring_conflicts_with_ring_key() {
            let cmd_vec =
                Vec::from_iter("hab-sup run --ring myring --ring-key foobar".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_ring_key,
                        "hab-sup run --ring-key secrit",
                        "RING_KEY" => "secrit");

        assert_cli_cmd!(should_handle_channel,
                        "hab-sup run --channel dev",
                        "CHANNEL" => "dev");

        assert_cli_cmd!(should_handle_bldr_url,
                        "hab-sup run --url http://not.a.real.com",
                        "BLDR_URL" => "http://not.a.real.com");

        assert_cli_cmd!(bldr_url_should_have_a_short_flag,
                        "hab-sup run -u http://not.a.real.com",
                        "BLDR_URL" => "http://not.a.real.com");

        assert_cli_cmd!(bldr_url_should_have_a_default_value,
                        "hab-sup run",
                        "BLDR_URL" => "https://bldr.habitat.sh");

        #[test]
        fn invalid_bldr_url_should_fail() {
            let cmd_vec = Vec::from_iter("hab-sup run -u bad-url".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        // TODO: config dir test

        // TODO: bad config dir test

        assert_cli_cmd!(should_handle_auto_update,
                        "hab-sup run --auto-update",
                        "AUTO_UPDATE" => true);

        assert_cli_cmd!(auto_update_should_have_a_short_flag,
                        "hab-sup run -A",
                        "AUTO_UPDATE" => true);

        // TODO: key file cert file test

        // TODO: key file requires test

        // TODO: cert file requires test

        assert_cli_cmd!(should_take_pkg_or_artifact_as_arg,
                        "hab-sup run core/redis",
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

        assert_cli_cmd!(should_handle_application_and_environment,
                        "hab-sup run --application foobar --environment fizbang",
                        "APPLICATION" => "foobar",
                        "ENVIRONMENT" => "fizbang");

        assert_cli_cmd!(application_and_environment_should_have_short_flags,
                        "hab-sup run -a foobar -e fizbang",
                        "APPLICATION" => "foobar",
                        "ENVIRONMENT" => "fizbang");

        #[test]
        fn application_requires_environment() {
            let cmd_vec = Vec::from_iter("hab-sup run -a foobar".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn environment_requires_application() {
            let cmd_vec = Vec::from_iter("hab-sup run -e fizbang".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_group,
                        "hab-sup run --group groupy",
                        "GROUP" => "groupy");

        assert_cli_cmd!(group_should_have_default_value,
                        "hab-sup run",
                        "GROUP" => "default");

        assert_cli_cmd!(should_handle_standalone_topology,
                        "hab-sup run --topology standalone",
                        "TOPOLOGY" => "standalone");

        assert_cli_cmd!(should_handle_leader_topology,
                        "hab-sup run --topology leader",
                        "TOPOLOGY" => "leader");

        assert_cli_cmd!(topology_should_have_a_short_flag,
                        "hab-sup run -t standalone",
                        "TOPOLOGY" => "standalone");

        #[test]
        fn topology_should_fail_when_a_bad_value_is_passed() {
            let cmd_vec = Vec::from_iter("hab-sup run --topology foobar".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_none_strategy,
                        "hab-sup run --strategy none",
                        "STRATEGY" => "none");

        assert_cli_cmd!(should_handle_at_once_strategy,
                        "hab-sup run --strategy at-once",
                        "STRATEGY" => "at-once");

        assert_cli_cmd!(should_handle_rolling_strategy,
                        "hab-sup run --strategy rolling",
                        "STRATEGY" => "rolling");

        assert_cli_cmd!(strategy_should_have_short_flag,
                        "hab-sup run -s rolling",
                        "STRATEGY" => "rolling");

        assert_cli_cmd!(strategy_should_have_a_default_value,
                        "hab-sup run",
                        "STRATEGY" => "none");

        #[test]
        fn strategy_should_fail_when_a_bad_value_is_passed() {
            let cmd_vec = Vec::from_iter("hab-sup run --strategy foobar".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

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

        assert_cli_cmd!(should_handle_strict_binding_mode,
                        "hab-sup run --binding-mode strict",
                        "BINDING_MODE" => "strict");

        assert_cli_cmd!(should_handle_relaxed_binding_mode,
                        "hab-sup run --binding-mode relaxed",
                        "BINDING_MODE" => "relaxed");

        assert_cli_cmd!(binding_mode_should_have_default_value,
                        "hab-sup run",
                        "BINDING_MODE" => "strict");

        #[test]
        fn binding_mode_should_fail_when_a_bad_value_is_passed() {
            let cmd_vec = Vec::from_iter("hab-sup run --binding-mode foobar".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_verbose,
                        "hab-sup run -v",
                        "VERBOSE" => true);

        assert_cli_cmd!(should_handle_no_color,
                        "hab-sup run --no-color",
                        "NO_COLOR" => true);

        assert_cli_cmd!(should_handle_json_logging,
                        "hab-sup run --json-logging",
                        "JSON" => true);

        assert_cli_cmd!(should_handle_health_check_interval,
                        "hab-sup run --health-check-interval 10",
                        "HEALTH_CHECK_INTERVAL" => "10");

        assert_cli_cmd!(health_check_interval_should_have_short_flag,
                        "hab-sup run -i 10",
                        "HEALTH_CHECK_INTERVAL" => "10");

        assert_cli_cmd!(health_check_interval_should_have_a_default_value,
                        "hab-sup run",
                        "HEALTH_CHECK_INTERVAL" => "30");

    }

}
