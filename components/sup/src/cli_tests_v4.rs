use crate::TokioThreadCount;
use habitat_core::env::Config;

mod tokio_thread_count {
    use super::*;
    use habitat_core::locked_env_var;

    locked_env_var!(HAB_TOKIO_THREAD_COUNT, lock_thread_count);

    #[test]
    fn default_is_number_of_cpus() {
        let tc = lock_thread_count();
        tc.unset();

        assert_eq!(TokioThreadCount::configured_value().0, num_cpus::get());
    }

    #[test]
    fn can_be_overridden_by_env_var() {
        let tc = lock_thread_count();
        tc.set("128");
        assert_eq!(TokioThreadCount::configured_value().0, 128);
    }
}

mod manager_config {
    use clap_v4 as clap;

    use std::{net::{SocketAddr,
                    ToSocketAddrs},
              str::FromStr};

    use clap::Parser;
    use futures::executor;
    use tempfile::TempDir;

    use habitat_common::FeatureFlag;
    use habitat_core::ChannelIdent;
    use habitat_sup::manager::ManagerConfig;

    use habitat_common::types::{GossipListenAddr,
                                HttpListenAddr,
                                ListenCtlAddr};

    use habitat_core::crypto::keys::{Key,
                                     KeyCache,
                                     RingKey};

    use crate::cli_v4::{split_apart_sup_run,
                        HabSup};

    use habitat_core::locked_env_var;

    locked_env_var!(HAB_CACHE_KEY_PATH, lock_var);

    fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

    fn config_from_cmd_str(cmd: &str) -> ManagerConfig {
        let hab_sup = HabSup::try_parse_from(cmd.split_whitespace());
        assert!(hab_sup.is_ok(), "{:#?}", hab_sup.err().unwrap());

        let hab_sup = hab_sup.unwrap();
        assert!(matches!(hab_sup, HabSup::Run(..)));

        match hab_sup {
            HabSup::Run(sup_run_options) => {
                executor::block_on(split_apart_sup_run(sup_run_options, no_feature_flags()))
                    .expect(
                        "Could not get split apart \
                                                                     SupRun",
                    )
                    .0
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn auto_update_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --auto-update");
        assert!(config.auto_update);

        let config = config_from_cmd_str("hab-sup run");
        assert!(!config.auto_update);
    }

    #[test]
    fn update_url_should_be_set() {
        let config = config_from_cmd_str("hab-sup run -u http://fake.example.url");
        assert_eq!(config.update_url, "http://fake.example.url/");
    }

    #[test]
    fn update_url_is_set_to_default_when_not_specified() {
        let config = config_from_cmd_str("hab-sup run");
        assert_eq!(config.update_url, habitat_core::url::default_bldr_url());
    }

    #[test]
    fn update_channel_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --channel unstable");
        assert_eq!(config.update_channel, ChannelIdent::unstable());
    }

    #[test]
    fn update_channel_is_set_to_default_when_not_specified() {
        let config = config_from_cmd_str("hab-sup run");
        assert_eq!(config.update_channel, ChannelIdent::stable());
    }

    #[test]
    fn gossip_listen_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --listen-gossip 1.1.1.1:1111");
        let expected_addr =
            GossipListenAddr::from_str("1.1.1.1:1111").expect("Could not create GossipListenAddr");
        assert_eq!(config.gossip_listen, expected_addr);
    }

    #[test]
    fn gossip_listen_is_set_to_default_when_not_specified() {
        let config = config_from_cmd_str("hab-sup run");
        let expected_addr = GossipListenAddr::default();
        assert_eq!(config.gossip_listen, expected_addr);
    }

    #[test]
    fn http_listen_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --listen-http 2.2.2.2:2222");
        let expected_addr =
            HttpListenAddr::from_str("2.2.2.2:2222").expect("Could not create http listen addr");
        assert_eq!(config.http_listen, expected_addr);
    }

    #[test]
    fn http_listen_is_set_default_when_not_specified() {
        let config = config_from_cmd_str("hab-sup run");
        let expected_addr = HttpListenAddr::default();
        assert_eq!(config.http_listen, expected_addr);
    }

    #[test]
    fn http_disable_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --http-disable");
        assert!(config.http_disable);

        let config = config_from_cmd_str("hab-sup run");
        assert!(!config.http_disable);
    }

    #[test]
    fn ctl_listen_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --listen-ctl 3.3.3.3:3333");
        let expected_addr =
            ListenCtlAddr::from_str("3.3.3.3:3333").expect("Could not create ctl listen addr");
        assert_eq!(config.ctl_listen, expected_addr);

        let config = config_from_cmd_str("hab-sup run");
        let expected_addr = ListenCtlAddr::default();
        assert_eq!(config.ctl_listen, expected_addr);
    }

    #[test]
    fn peers_should_have_a_default_port_set() {
        let config = config_from_cmd_str("hab-sup run --peer 1.1.1.1 2.2.2.2 3.3.3.3");
        let expected_peers: Vec<SocketAddr> =
            vec!["1.1.1.1", "2.2.2.2", "3.3.3.3"].into_iter()
                                                 .map(|peer| {
                                                     format!("{}:{}",
                                                             peer,
                                                             GossipListenAddr::DEFAULT_PORT)
                                                 })
                                                 .flat_map(|peer| {
                                                     peer.to_socket_addrs()
                                                         .expect("Failed getting addrs")
                                                 })
                                                 .collect();
        assert_eq!(config.gossip_peers, expected_peers);
    }

    #[test]
    fn watch_peer_file_should_be_set() {
        let config = config_from_cmd_str("hab-sup run --peer-watch-file foobar");
        assert_eq!(config.watch_peer_file, Some("foobar".to_string()));

        let config = config_from_cmd_str("hab-sup run");
        assert_eq!(config.watch_peer_file, None);
    }

    #[test]
    fn ring_key_is_set_properly_by_name() {
        let temp_dir = TempDir::new().expect("Could not create tempdir");

        let cache = KeyCache::new(temp_dir.path());
        let lock = lock_var();
        lock.set(temp_dir.path());

        let key_content =
            "SYM-SEC-1\nfoobar-20160504220722\n\nRCFaO84j41GmrzWddxMdsXpGdn3iuIy7Mw3xYrjPLsE=";
        let key: RingKey = key_content.parse().unwrap();
        cache.write_key(&key).unwrap();

        let config = config_from_cmd_str("hab-sup run --ring foobar");

        assert_eq!(config.ring_key
                         .expect("No ring key on manager config")
                         .named_revision(),
                   key.named_revision());
    }
}
