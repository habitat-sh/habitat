use std::process;

use log::debug;
use tokio::{self,
            runtime::Builder as RuntimeBuilder};

use habitat_common::{ui::UI,
                     FeatureFlag};
use habitat_core::{env::Config,
                   os::signals};
use habitat_launcher_client::ERR_NO_RETRY_EXCODE;

use habitat_sup::logger;

use crate::TokioThreadCount;

pub(crate) fn main_v2() {
    // Set up signal handlers before anything else happens to ensure
    // that all threads spawned thereafter behave properly.
    signals::init();
    logger::init();

    let runtime = RuntimeBuilder::new_multi_thread()
        .worker_threads(TokioThreadCount::configured_value().into())
        .enable_all()
        .build()
        .expect("Couldn't build Tokio Runtime!");

    let mut ui = UI::default_with_env();
    let flags = FeatureFlag::from_env(&mut ui);

    let result = runtime.block_on(crate::cli::start_rsr_imlw_mlw_gsw_smw_rhw_msw(flags));
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
