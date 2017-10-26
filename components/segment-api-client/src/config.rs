// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

/// URL to Segment API endpoint
pub const DEFAULT_SEGMENT_URL: &'static str = "https://api.segment.io";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct SegmentCfg {
    /// URL to Segment API
    #[serde(default = "default_url")]
    pub url: String,
    /// Write key used for Segment API requests
    pub write_key: String,
}

impl Default for SegmentCfg {
    fn default() -> Self {
        SegmentCfg {
            url: DEFAULT_SEGMENT_URL.to_string(),
            write_key: "".to_string(),
        }
    }
}

fn default_url() -> String {
    DEFAULT_SEGMENT_URL.to_string()
}
