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
             ident: &PackageIdent,
             target: PackageTarget,
             token: &str,
             group: bool)
             -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    if group {
        let rdeps = api_client.fetch_rdeps(ident, target)
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

    let id = api_client.schedule_job(ident, target, !group, token)
                       .map_err(Error::APIClient)?;

    ui.status(Status::Created, format!("build job. The id is {}", id))?;

    Ok(())
}
