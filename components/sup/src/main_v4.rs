#[cfg(feature = "v4")]
pub(crate) fn main_v4() {
    // We bring these 'use' statements inside `main` because they generate unnecessary warnings
    // when compiled with `feature v2` for now.
    use std::process;

    use log::debug;
    use tokio::{self,
                runtime::Builder as RuntimeBuilder};

    use habitat_common::{ui::UI,
                         FeatureFlag};

    use habitat_core::env::Config;
    use habitat_launcher_client::ERR_NO_RETRY_EXCODE;

    use crate::TokioThreadCount;

    // Set up signal handlers before anything else happens to ensure
    // that all threads spawned thereafter behave properly.
    habitat_core::os::signals::init();
    habitat_sup::logger::init();

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

#[cfg(not(feature = "v4"))]
pub(crate) fn main_v4() {
    unreachable! {}
}
