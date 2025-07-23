use std::process;

use log::debug;
use tokio::{self,
            runtime::Builder as RuntimeBuilder};

use habitat_common::{liveliness_checker,
                     outputln,
                     ui::UI,
                     FeatureFlag};
use habitat_core::{env::Config,
                   os::signals};
use habitat_launcher_client::ERR_NO_RETRY_EXCODE;

use habitat_sup::{error::{Error,
                          Result},
                  logger};

use crate::TokioThreadCount;

static LOGKEY: &str = "MN";

#[cfg(feature = "v4")]
pub(crate) fn main_v4() {
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

    let result = runtime.block_on(start_rsr_imlw_mlw_gsw_smw_rhw_msw(flags));
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

pub(crate) async fn start_rsr_imlw_mlw_gsw_smw_rhw_msw(feature_flags: FeatureFlag) -> Result<()> {
    if feature_flags.contains(FeatureFlag::TEST_BOOT_FAIL) {
        outputln!("Simulating boot failure");
        return Err(Error::TestBootFail);
    }
    liveliness_checker::spawn_thread_alive_checker();
    let launcher = crate::cli_common::boot();

    Ok(())
}
