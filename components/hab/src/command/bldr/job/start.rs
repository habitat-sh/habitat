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

use crate::{api_client::Client,
            common::ui::{Status,
                         UIReader,
                         UIWriter,
                         UI},
            hcore::package::{PackageIdent,
                             PackageTarget}};

use crate::{error::{Error,
                    Result},
            PRODUCT,
            VERSION};

pub fn start(ui: &mut UI,
             bldr_url: &str,
             (ident, target): (&PackageIdent, PackageTarget),
             token: &str,
             group: bool)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    if group {
        let rdeps = api_client.fetch_rdeps((ident, target))
                              .map_err(Error::APIClient)?;
        if !rdeps.is_empty() {
            ui.warn("Found the following reverse dependencies:")?;

            for rdep in rdeps {
                ui.warn(rdep.to_string())?;
            }

            let question = "If you choose to start a group build for this package, all of the \
                            above will be built as well. Is this what you want?";

            if !ui.prompt_yes_no(question, Some(true))? {
                ui.fatal("Aborted")?;
                return Ok(());
            }
        }
    }

    ui.status(Status::Creating,
              format!("build job for {} ({})", ident, target))?;

    let id = api_client.schedule_job((ident, target), !group, token)
                       .map_err(Error::APIClient)?;

    ui.status(Status::Created, format!("build job. The id is {}", id))?;

    Ok(())
}
