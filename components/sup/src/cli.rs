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
        use tempfile::{NamedTempFile, TempDir};

        const BAD_ADDRS: [&str; 3] = ["1.1.1:1111", "1.1.1.256:1111", "1.1.1.1.1:1111"];

        assert_cli_cmd!(listen_gossip_should_have_default_value,
                        "hab-sup run",
                        "LISTEN_GOSSIP" => (&GOSSIP_DEFAULT_ADDR));

        #[test]
        fn invalid_listen_gossip_address_should_fail() {
            for addr in BAD_ADDRS.iter() {
                let cmd_str = format!("hab-sup run --listen-gossip {}", addr);
                let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
                assert!(cli().get_matches_from_safe(cmd_vec).is_err());
            }
        }

        assert_cli_cmd!(listen_http_should_have_default_value,
                        "hab-sup run",
                        "LISTEN_HTTP" => (&LISTEN_HTTP_DEFAULT_ADDR));

        #[test]
        fn invalid_listen_http_address_should_fail() {
            for addr in BAD_ADDRS.iter() {
                let cmd_str = format!("hab-sup run --listen-http {}", addr);
                let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
                assert!(cli().get_matches_from_safe(cmd_vec).is_err());
            }
        }

        assert_cli_cmd!(listen_ctl_should_have_a_default_value,
                        "hab-sup run",
                        "LISTEN_CTL" => (&LISTEN_CTL_DEFAULT_ADDR_STRING));

        #[test]
        fn invalid_listen_ctl_address_should_fail() {
            for addr in BAD_ADDRS.iter() {
                let cmd_str = format!("hab-sup run --listen-ctl {}", addr);
                let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
                assert!(cli().get_matches_from_safe(cmd_vec).is_err());
            }
        }

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

        #[test]
        fn peer_watch_file_conflicts_with_peer() {
            let cmd_vec = Vec::from_iter(
                "hab-sup run --peer-watch-file foobar --peer 1.1.1.1:1111".split_whitespace(),
            );
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn ring_conflicts_with_ring_key() {
            let cmd_vec =
                Vec::from_iter("hab-sup run --ring myring --ring-key foobar".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(bldr_url_should_have_a_default_value,
                        "hab-sup run",
                        "BLDR_URL" => "https://bldr.habitat.sh");

        #[test]
        fn invalid_bldr_url_should_fail() {
            let cmd_vec = Vec::from_iter("hab-sup run -u bad-url".split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn should_handle_valid_config_dir() {
            let config_dir = TempDir::new().expect("Could not create tempdir");
            let path_str = config_dir.path().to_str().unwrap();
            let cmd_str = format!("hab-sup run --config-from {}", path_str);
            let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
            let matches = cli()
                .get_matches_from_safe(cmd_vec)
                .expect("Could not get ArgMatches");
            let subcommand = matches
                .subcommand_matches("run")
                .expect("Failed to get matches for sup run");
            assert_eq!(subcommand.value_of("CONFIG_DIR"), Some(path_str));
        }

        #[test]
        fn config_dir_should_fail_when_dir_does_not_exist() {
            let cmd_vec = Vec::from_iter(
                "hab-sup run --config-from really-fake/not-real/directory/no-really/go-away"
                    .split_whitespace(),
            );
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_handle_auto_update,
                        "hab-sup run --auto-update",
                        "AUTO_UPDATE" => true);

        assert_cli_cmd!(auto_update_should_have_a_short_flag,
                        "hab-sup run -A",
                        "AUTO_UPDATE" => true);

        #[test]
        fn should_take_cert_file_and_key_file() {
            let key_file = NamedTempFile::new()
                .expect("Failed to create temporary key_file")
                .into_temp_path();
            let cert_file = NamedTempFile::new()
                .expect("Failed to create temporary cert_file")
                .into_temp_path();
            let key_file_str = key_file.to_str().unwrap();
            let cert_file_str = cert_file.to_str().unwrap();
            let cmd_str = format!(
                "hab-sup run --key {} --certs {}",
                key_file_str, cert_file_str
            );
            let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
            let matches = cli()
                .get_matches_from_safe(cmd_vec)
                .expect("Could not get ArgMatches");
            let subcommand = matches
                .subcommand_matches("run")
                .expect("Failed to get matches for sup run");
            assert_eq!(subcommand.value_of("KEY_FILE"), Some(key_file_str));
            assert_eq!(subcommand.value_of("CERT_FILE"), Some(cert_file_str));
        }

        #[test]
        fn key_file_requires_cert_file() {
            let key_file = NamedTempFile::new()
                .expect("Failed to create temporary key_file")
                .into_temp_path();
            let key_file_str = key_file.to_str().unwrap();
            let cmd_str = format!("hab-sup run --key {}", key_file_str);
            let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn cert_file_requires_key_file() {
            let cert_file = NamedTempFile::new()
                .expect("Failed to create temporary cert_file")
                .into_temp_path();
            let cert_file_str = cert_file.to_str().unwrap();
            let cmd_str = format!("hab-sup run --certs {}", cert_file_str);
            let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        assert_cli_cmd!(should_take_pkg_or_artifact_as_arg,
                        "hab-sup run core/redis",
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

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

        assert_cli_cmd!(group_should_have_default_value,
                        "hab-sup run",
                        "GROUP" => "default");

        assert_cli_cmd!(should_handle_standalone_topology,
                        "hab-sup run --topology standalone",
                        "TOPOLOGY" => "standalone");

        assert_cli_cmd!(should_handle_leader_topology,
                        "hab-sup run --topology leader",
                        "TOPOLOGY" => "leader");

        #[test]
        fn topology_should_fail_when_a_bad_value_is_passed() {
            for bad_topo in ["none", "at-once", "rolling", "foobar"].iter() {
                let cmd_str = format!("hab-sup run --topology {}", bad_topo);
                let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
                assert!(cli().get_matches_from_safe(cmd_vec).is_err());
            }
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

        assert_cli_cmd!(strategy_should_have_a_default_value,
                        "hab-sup run",
                        "STRATEGY" => "none");

        #[test]
        fn strategy_should_fail_when_a_bad_value_is_passed() {
            for bad_strat in ["standalone", "leader", "foobar"].iter() {
                let cmd_str = format!("hab-sup run --strategy {}", bad_strat);
                let cmd_vec = Vec::from_iter(cmd_str.split_whitespace());
                assert!(cli().get_matches_from_safe(cmd_vec).is_err());
            }
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

        assert_cli_cmd!(health_check_interval_should_have_a_default_value,
                        "hab-sup run",
                        "HEALTH_CHECK_INTERVAL" => "30");

    }

}
