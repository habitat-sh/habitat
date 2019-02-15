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

use crate::{
    api_client::Client,
    common::ui::{Status, UIWriter, UI},
    hcore::ChannelIdent,
};

use crate::{
    error::{Error, Result},
    PRODUCT, VERSION,
};

pub fn start(
    ui: &mut UI,
    bldr_url: &str,
    token: &str,
    origin: &str,
    channel: &ChannelIdent,
) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Creating, format!("channel {}.", channel))?;

    api_client
        .create_channel(origin, channel, token)
        .map_err(Error::APIClient)?;

    ui.status(Status::Created, format!("channel {}.", channel))?;

    Ok(())
}
