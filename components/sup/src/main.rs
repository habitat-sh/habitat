pub(crate) mod cli_v4;

pub(crate) mod cli_common;

habitat_core::env_config_int!(/// Represents how many threads to start for our main Tokio runtime
                              #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
                              TokioThreadCount,
                              usize,
                              HAB_TOKIO_THREAD_COUNT,
                              // This is the same internal logic used in Tokio itself.
                              // https://docs.rs/tokio/0.1.12/src/tokio/runtime/builder.rs.html#68
                              num_cpus::get().max(1));

fn main() {
    // We bring these 'use' statements inside `main` because they generate unnecessary warnings
    // when compiled with `feature v2` for now.
    use std::process;

    use log::debug;
    use tokio::{self,
                runtime::Builder as RuntimeBuilder};

    use habitat_common::{FeatureFlag,
                         ui::UI};

    use habitat_core::env::Config;
    use habitat_launcher_client::ERR_NO_RETRY_EXCODE;

    use crate::TokioThreadCount;

    // Set up signal handlers before anything else happens to ensure
    // that all threads spawned thereafter behave properly.
    habitat_core::os::signals::init();
    habitat_sup::logger::init();

    // Initialize rustls crypto provider. This must be done once before any TLS operations.
    // The aws-lc-rs feature enables the crypto backend, but we must explicitly install it
    // as the default provider for the process.
    rustls::crypto::aws_lc_rs::default_provider().install_default()
                                                 .expect("Failed to install rustls crypto \
                                                          provider. This typically indicates \
                                                          that a TLS crypto provider has already \
                                                          been installed earlier in the process, \
                                                          or that the configured aws-lc-rs \
                                                          backend is incompatible with the \
                                                          current runtime or build \
                                                          configuration. Ensure no other rustls \
                                                          crypto provider is installed before \
                                                          the Habitat Supervisor starts, and \
                                                          verify that the aws-lc-rs feature is \
                                                          enabled and correctly linked.");

    let runtime = RuntimeBuilder::new_multi_thread()
        .worker_threads(TokioThreadCount::configured_value().into())
        .enable_all()
        .build()
        .expect("Couldn't build Tokio Runtime!");

    let mut ui = UI::default_with_env();
    let flags = FeatureFlag::from_env(&mut ui);

    let result = runtime.block_on(crate::cli_v4::start_rsr_imlw_mlw_gsw_smw_rhw_msw(flags));
    let exit_code = match result {
        Ok(_) => 0,
        Err(ref err) => {
            println!("{}", err);
            ERR_NO_RETRY_EXCODE
        }
    };
    debug!("start() returned {:?}; Exiting {}", result, exit_code);
    process::exit(exit_code);
}

#[cfg(test)]
mod cli_tests_v4;
