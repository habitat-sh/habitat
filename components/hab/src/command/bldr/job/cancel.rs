// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::api_client;
use crate::common::ui::{Status, UIReader, UIWriter, UI};

use crate::error::{Error, Result};
use crate::{PRODUCT, VERSION};

pub fn start(ui: &mut UI, bldr_url: &str, group_id: &str, token: &str, force: bool) -> Result<()> {
    if !force {
        // TODO (SA): Show all the in-progress builds that will get canceled
        let question =
            "If you choose to cancel a group build, \
             all of the builds that are in progress will be canceled. Is this what you want?";

        if !ui.prompt_yes_no(question, Some(true))? {
            ui.fatal("Aborted")?;
            return Ok(());
        }
    }

    let api_client =
        api_client::Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;
    let gid = match group_id.parse::<u64>() {
        Ok(g) => g,
        Err(e) => {
            ui.fatal(format!("Failed to parse group id: {}", e))?;
            return Err(Error::ParseIntError(e));
        }
    };

    ui.status(Status::Canceling, format!("job group {}", group_id))?;

    match api_client.job_group_cancel(gid, token) {
        Ok(_) => {
            ui.status(Status::Canceled, format!("job group {}", group_id))?;
        }

        Err(e) => {
            return Err(Error::JobGroupCancel(e));
        }
    };

    Ok(())
}
