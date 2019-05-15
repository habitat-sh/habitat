use std::path::Path;

use crate::{common::ui::{UIWriter,
                         UI},
            hcore::{crypto::BoxKeyPair,
                    service::ServiceGroup}};

use crate::error::Result;

pub fn start(ui: &mut UI, org: &str, service_group: &ServiceGroup, cache: &Path) -> Result<()> {
    ui.begin(format!("Generating service key for {} in {}", &service_group, org))?;
    let pair = BoxKeyPair::generate_pair_for_service(org, &service_group.to_string())?;
    pair.to_pair_files(cache)?;
    ui.end(format!("Generated service key pair {}.", &pair.name_with_rev()))?;
    Ok(())
}
