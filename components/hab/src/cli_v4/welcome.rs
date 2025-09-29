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

use crate::error::Result as HabResult;
use clap_v4 as clap;
use clap::Parser;
use habitat_common::ui::{UI, UIWriter};

#[derive(Clone, Debug, Parser)]
#[command(name = "welcome",
          about = "Display a welcome message for Habitat",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct WelcomeOpts;

impl WelcomeOpts {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        ui.begin("Welcome to Habitat!")?;
        ui.para("Habitat is Chef's application automation framework that builds, deploys, and manages applications.")?;
        ui.para("To get started, try running 'hab --help' to see available commands.")?;
        ui.end("Happy automating!")?;
        Ok(())
    }
}