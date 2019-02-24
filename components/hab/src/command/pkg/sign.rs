// Copyright (c) 2019 Chef Software Inc. and/or applicable contributors
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

use std::path::Path;

use crate::{common::ui::{Status,
                         UIWriter,
                         UI},
            hcore::crypto::{artifact,
                            SigKeyPair}};

use crate::error::Result;

pub fn start(ui: &mut UI, origin: &SigKeyPair, src: &Path, dst: &Path) -> Result<()> {
    ui.begin(format!("Signing {}", src.display()))?;
    ui.status(
        Status::Signing,
        format!(
            "{} with {} to create {}",
            src.display(),
            &origin.name_with_rev(),
            dst.display()
        ),
    )?;
    artifact::sign(src, dst, origin)?;
    ui.end(format!("Signed artifact {}.", dst.display()))?;
    Ok(())
}
